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
        let Analysis {
            event,
            consumed_syntax_element,
        } = analyze_syntax_element(
            syntax_element,
            Module::none(),
            bindings,
            functions,
            tests,
        )
        .map_err(PipelineError::Stage)?;

        if consumed_syntax_element {
            syntax_elements.read()?;
            syntax_elements.take();
        }

        match event {
            Some(event) => return Ok(event),
            None => continue,
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
            // I can think of several ways to address this:
            //
            // - Split function declaration into declaration and definition
            //   before this stage.
            //   - The parser could emit it like that, but this would be weird.
            //     Then the syntax tree emitted by the parser would not reflect
            //     the actual syntax. I don't know how much difference this
            //     would make in practice.
            //   - There could be another stage between parser and analyzer,
            //     which normalizes the syntax tree. This would require another
            //     intermediate representation, which seems like too much
            //     overhead to be worth it. But maybe there would be other use
            //     cases too.
            // - Introduce another bit of state to track whether the function
            //   was already declared. This would be awkward, as that state
            //   would just be used for this specific thing, while for anything
            //   else, the input tracks the state just fine.
            // - Query the namespace to check whether the function is already
            //   declared. If it is, skip the declaration and jump directly to
            //   the definition.
            //   This seems like a bit of a hack at first, but I actually can't
            //   think of a reason why it wouldn't work well. It seems like the
            //   most practical solution.
            //   If a function is re-defined, then the declaration would always
            //   be skipped. I don't think this matters. Any code that wants to
            //   keep track of changes is most likely interested in the
            //   definition anyway.

            if !functions.is_declared(name) {
                functions.declare(name.clone());
            }

            let body = analyze_syntax_tree(
                body,
                Module::none(),
                bindings,
                functions,
                tests,
            )?;
            functions.define(Module::none(), name.clone(), body);

            return Ok(Analysis {
                event: None,
                consumed_syntax_element: true,
            });
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

            return Ok(Analysis {
                event: None,
                consumed_syntax_element: true,
            });
        }
        SyntaxElement::Value(value) => AnalyzerEvent::Value(value.clone()),
        SyntaxElement::Word(word) => {
            let refers_to_binding = bindings.is_declared(word);
            let refers_to_function = functions.is_declared(word);

            if refers_to_binding {
                let event = AnalyzerEvent::EvalBinding { name: word.clone() };
                return Ok(Analysis {
                    event: Some(event),
                    consumed_syntax_element: true,
                });
            }
            if refers_to_function {
                let event = AnalyzerEvent::EvalFunction { name: word.clone() };
                return Ok(Analysis {
                    event: Some(event),
                    consumed_syntax_element: true,
                });
            }

            return Err(AnalyzerError::UnrecognizedWord(word.clone()));
        }
    };

    Ok(Analysis {
        event: Some(expression),
        consumed_syntax_element: true,
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
        let Analysis {
            event,
            consumed_syntax_element,
        } = analyze_syntax_element(
            syntax_element,
            module,
            bindings,
            functions,
            tests,
        )?;

        if consumed_syntax_element {
            syntax_elements.next();
        }

        if let Some(event) = event {
            expressions.events.push(event);
        }
    }

    Ok(expressions)
}

struct Analysis {
    event: Option<AnalyzerEvent>,
    consumed_syntax_element: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum AnalyzerError {
    #[error("Unrecognized word: {0}")]
    UnrecognizedWord(String),
}
