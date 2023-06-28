use crate::cp::{
    data_stack::Value,
    expressions::{Expression, Expressions},
    syntax::{SyntaxElement, SyntaxTree},
    Bindings, Functions, Module,
};

use super::{stage_input::StageInputReader, PipelineError};

pub fn analyze(
    mut syntax_elements: StageInputReader<SyntaxElement>,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<Expression, PipelineError<AnalyzerError>> {
    loop {
        let syntax_element = syntax_elements.read()?;
        let expression = analyze_syntax_element(
            syntax_element,
            Module::none(),
            bindings,
            functions,
            tests,
        )
        .map_err(PipelineError::Stage)?;
        syntax_elements.take();

        match expression {
            Some(expression) => return Ok(expression),
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
) -> Result<Option<Expression>, AnalyzerError> {
    let expression = match syntax_element {
        SyntaxElement::Array { syntax_tree } => {
            let expressions = analyze_syntax_tree(
                syntax_tree,
                module,
                bindings,
                functions,
                tests,
            )?;
            Expression::Array { expressions }
        }
        SyntaxElement::Binding { idents } => {
            for ident in idents {
                bindings.declare(ident.clone());
            }

            let idents = idents.clone();
            Expression::Binding { idents }
        }
        SyntaxElement::Block { syntax_tree } => {
            let expressions = analyze_syntax_tree(
                syntax_tree,
                module,
                bindings,
                functions,
                tests,
            )?;
            Expression::Value(Value::Block(expressions))
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

            functions.declare(name.clone());

            let body = analyze_syntax_tree(
                body,
                Module::none(),
                bindings,
                functions,
                tests,
            )?;
            functions.define(Module::none(), name.clone(), body);

            return Ok(None);
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
            Expression::Module { name, body }
        }
        SyntaxElement::Test { name, body } => {
            let name = name.clone();
            let body =
                analyze_syntax_tree(body, module, bindings, functions, tests)?;

            tests.define(module, name, body);

            return Ok(None);
        }
        SyntaxElement::Value(value) => Expression::Value(value.clone()),
        SyntaxElement::Word(word) => {
            let refers_to_binding = bindings.is_declared(word);
            let refers_to_function = functions.is_declared(word);

            if refers_to_binding {
                return Ok(Some(Expression::EvalBinding {
                    name: word.clone(),
                }));
            }
            if refers_to_function {
                return Ok(Some(Expression::EvalFunction {
                    name: word.clone(),
                }));
            }

            return Err(AnalyzerError::UnrecognizedWord(word.clone()));
        }
    };

    Ok(Some(expression))
}

fn analyze_syntax_tree(
    syntax_tree: &SyntaxTree,
    module: Module,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<Expressions, AnalyzerError> {
    let mut expressions = Expressions {
        elements: Vec::new(),
    };

    for syntax_element in syntax_tree {
        let expression = analyze_syntax_element(
            syntax_element,
            module,
            bindings,
            functions,
            tests,
        )?;
        if let Some(expression) = expression {
            expressions.elements.push(expression);
        }
    }

    Ok(expressions)
}

#[derive(Debug, thiserror::Error)]
pub enum AnalyzerError {
    #[error("Unrecognized word: {0}")]
    UnrecognizedWord(String),
}
