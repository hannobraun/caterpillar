use std::collections::VecDeque;

use super::{
    parser::SyntaxTreeNode,
    values::{Color, Value},
};

pub type Stack = VecDeque<Value>;

pub fn evaluate(
    syntax_tree: impl IntoIterator<Item = SyntaxTreeNode>,
    stack: &mut Stack,
) {
    for node in syntax_tree {
        evaluate_node(node, stack);
    }
}

fn evaluate_node(node: SyntaxTreeNode, stack: &mut Stack) {
    match node {
        SyntaxTreeNode::Fn { name } => {
            evaluate_fn(name, stack);
        }
        SyntaxTreeNode::Array { syntax_tree } => {
            let mut array_stack = VecDeque::new();
            evaluate(syntax_tree, &mut array_stack);
            stack.push_back(Value::Array(array_stack));
        }
    }
}

fn evaluate_fn(name: String, stack: &mut Stack) {
    match name.as_str() {
        "color" => {
            let Some(Value::Array(mut color)) = stack.pop_back() else {
                return;
            };

            let b = color.pop_back();
            let g = color.pop_back();
            let r = color.pop_back();

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
