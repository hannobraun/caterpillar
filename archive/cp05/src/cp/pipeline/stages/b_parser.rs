use crate::cp::{
    pipeline::{
        channel::StageInput,
        ir::{
            syntax::{SyntaxElement, SyntaxTree},
            tokens::{Keyword, Literal, Token},
        },
    },
    runtime::data_stack::Value,
    PipelineError,
};

pub fn parse(mut tokens: StageInput<Token>) -> Result<SyntaxElement> {
    let syntax_element = parse_syntax_element(&mut tokens)?;
    tokens.take();
    Ok(syntax_element)
}

fn parse_syntax_element(
    tokens: &mut StageInput<Token>,
) -> Result<SyntaxElement> {
    let syntax_element = match tokens.peek()? {
        Token::BindingOperator => {
            let idents = parse_binding(tokens)?;
            SyntaxElement::Binding { idents }
        }
        Token::CurlyBracketOpen => {
            let syntax_tree = parse_block(tokens)?;
            SyntaxElement::Block { syntax_tree }
        }
        Token::SquareBracketOpen => {
            let syntax_tree = parse_array(tokens)?;
            SyntaxElement::Array { syntax_tree }
        }
        Token::Keyword(Keyword::Fn) => {
            let (name, body) = parse_fn(tokens)?;
            SyntaxElement::Function { name, body }
        }
        Token::Keyword(Keyword::Mod) => {
            let (name, body) = parse_mod(tokens)?;
            SyntaxElement::Module { name, body }
        }
        Token::Keyword(Keyword::Test) => {
            let (name, body) = parse_test(tokens)?;
            SyntaxElement::Test { name, body }
        }
        Token::Literal(Literal::Number(_)) => {
            let number = parse_number(tokens)?;
            SyntaxElement::Value(Value::U8(number))
        }
        Token::Literal(Literal::String(_)) => {
            let s = parse_string(tokens)?;
            SyntaxElement::Value(Value::String(s))
        }
        Token::Ident(_) => {
            let ident = parse_ident(tokens)?;
            SyntaxElement::Word(ident)
        }
        token => {
            return Err(PipelineError::Stage(ParserError::UnexpectedToken(
                token.clone(),
            )));
        }
    };

    Ok(syntax_element)
}

fn parse_binding(tokens: &mut StageInput<Token>) -> Result<Vec<String>> {
    expect_token(tokens, Token::BindingOperator)?;

    let mut idents = Vec::new();

    loop {
        match tokens.peek()? {
            Token::Period => {
                let _ = tokens.read();
                return Ok(idents);
            }
            _ => {
                let ident = parse_ident(tokens)?;
                idents.push(ident);
            }
        }
    }
}

fn parse_array(tokens: &mut StageInput<Token>) -> Result<SyntaxTree> {
    expect_token(tokens, Token::SquareBracketOpen)?;

    let mut syntax_tree = SyntaxTree {
        elements: Vec::new(),
    };

    loop {
        match tokens.peek()? {
            Token::SquareBracketClose => {
                let _ = tokens.read();
                return Ok(syntax_tree);
            }
            _ => {
                let syntax_element = parse_syntax_element(tokens)?;
                syntax_tree.elements.push(syntax_element)
            }
        }
    }
}

fn parse_block(tokens: &mut StageInput<Token>) -> Result<SyntaxTree> {
    expect_token(tokens, Token::CurlyBracketOpen)?;

    let mut syntax_tree = SyntaxTree {
        elements: Vec::new(),
    };

    loop {
        match tokens.peek()? {
            Token::CurlyBracketClose => {
                let _ = tokens.read();
                return Ok(syntax_tree);
            }
            _ => {
                let syntax_element = parse_syntax_element(tokens)?;
                syntax_tree.elements.push(syntax_element)
            }
        }
    }
}

fn parse_fn(tokens: &mut StageInput<Token>) -> Result<(String, SyntaxTree)> {
    expect_token(tokens, Token::Keyword(Keyword::Fn))?;
    let name = parse_ident(tokens)?;
    let body = parse_block(tokens)?;

    Ok((name, body))
}

fn parse_mod(tokens: &mut StageInput<Token>) -> Result<(String, SyntaxTree)> {
    expect_token(tokens, Token::Keyword(Keyword::Mod))?;
    let name = parse_ident(tokens)?;
    let body = parse_block(tokens)?;

    Ok((name, body))
}

fn parse_test(tokens: &mut StageInput<Token>) -> Result<(String, SyntaxTree)> {
    expect_token(tokens, Token::Keyword(Keyword::Test))?;
    let name = parse_string(tokens)?;
    let body = parse_block(tokens)?;

    Ok((name, body))
}

fn parse_number(tokens: &mut StageInput<Token>) -> Result<u8> {
    let token = tokens.read()?;
    let Token::Literal(Literal::Number(number)) = token else {
        return Err(PipelineError::Stage(ParserError::UnexpectedToken(
            token.clone()))
        );
    };
    Ok(*number)
}

fn parse_string(tokens: &mut StageInput<Token>) -> Result<String> {
    let token = tokens.read()?;
    let Token::Literal(Literal::String(s)) = token else {
        return Err(PipelineError::Stage(ParserError::UnexpectedToken(
            token.clone()))
        );
    };
    Ok(s.clone())
}

fn parse_ident(tokens: &mut StageInput<Token>) -> Result<String> {
    let token = tokens.read()?;
    let Token::Ident(ident) = token else {
        return Err(PipelineError::Stage(ParserError::UnexpectedToken(
            token.clone()
        )));
    };
    Ok(ident.clone())
}

fn expect_token(tokens: &mut StageInput<Token>, expected: Token) -> Result<()> {
    let token = tokens.read()?;

    if token != &expected {
        return Err(PipelineError::Stage(ParserError::UnexpectedToken(
            token.clone(),
        )));
    };

    Ok(())
}

type Result<T> = std::result::Result<T, PipelineError<ParserError>>;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Unexpected token: `{0:?}`")]
    UnexpectedToken(Token),
}
