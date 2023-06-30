use crate::cp::{
    data_stack::Value,
    pipeline::{
        channel::StageInput,
        ir::{
            analyzer_output::{AnalyzerEvent, AnalyzerOutput},
            syntax::{SyntaxElement, SyntaxTree},
        },
    },
    Bindings, Functions, Module, PipelineError,
};

pub fn analyze(
    mut syntax_elements: StageInput<SyntaxElement>,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<AnalyzerEvent, PipelineError<AnalyzerError>> {
    loop {
        let syntax_element = syntax_elements.peek()?;
        let Analysis { event } = analyze_syntax_element(
            syntax_element,
            Module::none(),
            bindings,
            functions,
            tests,
        )
        .map_err(PipelineError::Stage)?;

        syntax_elements.read()?;
        syntax_elements.take();

        if let Some(event) = event {
            return Ok(event);
        }
    }
}

fn analyze_syntax_element(
    syntax_element: &SyntaxElement,
    module: Module,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<Analysis, AnalyzerError> {
    let expression = match syntax_element {
        SyntaxElement::Array { syntax_tree } => {
            let expressions = analyze_syntax_tree(
                syntax_tree,
                module,
                bindings,
                functions,
                tests,
            )?;
            AnalyzerEvent::Array { expressions }
        }
        SyntaxElement::Binding { idents } => {
            for ident in idents {
                bindings.declare(ident.clone());
            }

            let idents = idents.clone();
            AnalyzerEvent::Binding { idents }
        }
        SyntaxElement::Block { syntax_tree } => {
            let expressions = analyze_syntax_tree(
                syntax_tree,
                module,
                bindings,
                functions,
                tests,
            )?;
            AnalyzerEvent::Value(Value::Block(expressions))
        }
        SyntaxElement::Function { name, body } => {
            // The analyzer directly mutates the namespace, here and in other
            // places. This is not necessarily desirable.
            //
            // There are other pieces of code that want to be informed about
            // these mutations, for example to know which functions were
            // updated, which can then be used to calculate a list of tests that
            // need to be run.
            //
            // For use cases like these, it would be better to generate a
            // sequence of events, which can then be aggregated and applied to
            // the namespace state, but also used to keep track of changes.
            //
            // However, this piece of code here is a bit problematic in that
            // regard. It generates two events, declare function and define
            // function, and needs the first one applied before it can generate
            // the second one. So this would need to be distributed over two
            // invocations of the analyzer.
            //
            // However, that the first event was generated is not reflected in
            // the input, so the second invocation would not know that the
            // function is already declared.
            //
            // I've started addressing this, by querying the namespace to check
            // whether the function is already declared. If it is, I skip the
            // declaration and jump directly to the definition. This way, I can
            // emit a declaration event that will be applied to the namespace
            // outside of this function. The updated state will be available the
            // next time this function is called.

            functions.declare(name.clone());

            let body = analyze_syntax_tree(
                body,
                Module::none(),
                bindings,
                functions,
                tests,
            )?;
            functions.define(Module::none(), name.clone(), body);

            return Ok(Analysis { event: None });
        }
        SyntaxElement::Module { name, body } => {
            let name = name.clone();
            let body = analyze_syntax_tree(
                body,
                Module::some(&name),
                bindings,
                functions,
                tests,
            )?;
            AnalyzerEvent::Module { name, body }
        }
        SyntaxElement::Test { name, body } => {
            let name = name.clone();
            let body =
                analyze_syntax_tree(body, module, bindings, functions, tests)?;

            tests.declare(name.clone());
            tests.define(module, name, body);

            return Ok(Analysis { event: None });
        }
        SyntaxElement::Value(value) => AnalyzerEvent::Value(value.clone()),
        SyntaxElement::Word(word) => {
            let refers_to_binding = bindings.is_declared(word);
            let refers_to_function = functions.is_declared(word);

            if refers_to_binding {
                let event = AnalyzerEvent::EvalBinding { name: word.clone() };
                return Ok(Analysis { event: Some(event) });
            }
            if refers_to_function {
                let event = AnalyzerEvent::EvalFunction { name: word.clone() };
                return Ok(Analysis { event: Some(event) });
            }

            return Err(AnalyzerError::UnrecognizedWord(word.clone()));
        }
    };

    Ok(Analysis {
        event: Some(expression),
    })
}

fn analyze_syntax_tree(
    syntax_tree: &SyntaxTree,
    module: Module,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<AnalyzerOutput, AnalyzerError> {
    let mut expressions = AnalyzerOutput { events: Vec::new() };

    let mut syntax_elements = syntax_tree.into_iter().peekable();
    while let Some(syntax_element) = syntax_elements.peek() {
        // I've been moving towards a new way of declaring/defining functions
        // within `analyze_syntax_element`, which consists of emitting events
        // that can be applied to the namespace externally, instead of mutating
        // the namespace directly. See the long comment about that within the
        // function.
        //
        // However, I can't pull the trigger and finish that work, because of
        // this use of the function here. The problem is that we're not applying
        // those events the state here, causing an infinite loop, as functions
        // will never be declared.
        //
        // I can't think of a way to fix this. If the event application
        // machinery were passed in here, it would have to be threaded through
        // all of the analyzer methods. At that point, we've made everything
        // more complicated without gaining anything.
        //
        // I think that means the complete design is wrong. It's probably better
        // to just revert to what we had before, and just track events within
        // `Functions` instead.
        let Analysis { event } = analyze_syntax_element(
            syntax_element,
            module,
            bindings,
            functions,
            tests,
        )?;

        syntax_elements.next();

        if let Some(event) = event {
            expressions.events.push(event);
        }
    }

    Ok(expressions)
}

struct Analysis {
    event: Option<AnalyzerEvent>,
}

#[derive(Debug, thiserror::Error)]
pub enum AnalyzerError {
    #[error("Unrecognized word: {0}")]
    UnrecognizedWord(String),
}
