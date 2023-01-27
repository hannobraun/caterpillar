use std::collections::VecDeque;

use super::{
    parser::SyntaxTree,
    values::{Color, Value},
};

pub type Stack = VecDeque<Value>;

pub fn evaluate(
    syntax_tree: impl IntoIterator<Item = SyntaxTree>,
    stack: &mut Stack,
) {
    for node in syntax_tree {
        evaluate_node(node, stack);
    }
}

fn evaluate_node(node: SyntaxTree, stack: &mut Stack) {
    match node {
        SyntaxTree::Fn { name } => {
            evaluate_fn(name, stack);
        }
        SyntaxTree::Array { .. } => {}
    }
}

fn evaluate_fn(name: String, stack: &mut Stack) {
    match name.as_str() {
        "color" => {
            let b = stack.pop_back();
            let g = stack.pop_back();
            let r = stack.pop_back();

            if let (
                Some(Value::U8(r)),
                Some(Value::U8(g)),
                Some(Value::U8(b)),
            ) = (r, g, b)
            {
                let color = Color { r, g, b };
                let value = Value::Color(color);

                stack.push_back(value);
            }
        }
        "drop" => {
            stack.pop_back();
        }
        _ => {
            if let Ok(value) = name.parse::<u8>() {
                let value = Value::U8(value);
                stack.push_back(value);
            }
        }
    }
}
