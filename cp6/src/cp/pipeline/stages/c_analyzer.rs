use crate::cp::{
    pipeline::{
        channel::StageInput,
        ir::{
            analyzer_output::{AnalyzerEvent, AnalyzerOutput},
            syntax::{SyntaxElement, SyntaxTree},
        },
    },
    runtime::data_stack::Value,
    Bindings, Functions, Module, PipelineError,
};

pub fn analyze(
    mut syntax_elements: StageInput<SyntaxElement>,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<AnalyzerEvent, PipelineError<AnalyzerError>> {
    loop {
        let syntax_element = syntax_elements.read()?;
        let event = analyze_syntax_element(
            syntax_element,
            Module::none(),
            bindings,
            functions,
            tests,
        )
        .map_err(PipelineError::Stage)?;

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
) -> Result<Option<AnalyzerEvent>, AnalyzerError> {
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
            functions.declare(name.clone());

            let body = analyze_syntax_tree(
                body,
                Module::none(),
                bindings,
                functions,
                tests,
            )?;
            let is_test = false;

            functions.define(Module::none(), name.clone(), body, is_test);

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
            AnalyzerEvent::Module { name, body }
        }
        SyntaxElement::Test { name, body } => {
            let name = name.clone();
            let body =
                analyze_syntax_tree(body, module, bindings, functions, tests)?;
            let is_test = true;

            tests.declare(name.clone());
            tests.define(module, name, body, is_test);

            return Ok(None);
        }
        SyntaxElement::Value(value) => AnalyzerEvent::Value(value.clone()),
        SyntaxElement::Word(word) => {
            let refers_to_binding = bindings.is_declared(word);
            let refers_to_function = functions.is_declared(word);

            if refers_to_binding {
                let event = AnalyzerEvent::EvalBinding { name: word.clone() };
                return Ok(Some(event));
            }
            if refers_to_function {
                let event = AnalyzerEvent::EvalFunction { name: word.clone() };
                return Ok(Some(event));
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
) -> Result<AnalyzerOutput, AnalyzerError> {
    let mut analyzer_output = AnalyzerOutput { events: Vec::new() };

    for syntax_element in syntax_tree {
        let event = analyze_syntax_element(
            syntax_element,
            module,
            bindings,
            functions,
            tests,
        )?;

        if let Some(event) = event {
            analyzer_output.events.push(event);
        }
    }

    Ok(analyzer_output)
}

#[derive(Debug, thiserror::Error)]
pub enum AnalyzerError {
    #[error("Unrecognized word: {0}")]
    UnrecognizedWord(String),
}
