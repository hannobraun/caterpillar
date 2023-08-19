use std::collections::HashMap;

use crate::language::tokens::{
    AddressedToken, Token, TokenAddressLeft, TokenAddressRight, Tokens,
};

pub fn address(tokens: impl IntoIterator<Item = Token>) -> Tokens {
    let mut left_to_right = HashMap::new();
    let mut right_to_left = HashMap::new();

    let leftmost =
        address_token(None, tokens, &mut left_to_right, &mut right_to_left);

    Tokens {
        leftmost,
        left_to_right,
        right_to_left,
    }
}

fn address_token(
    left: Option<TokenAddressLeft>,
    tokens: impl IntoIterator<Item = Token>,
    left_to_right: &mut HashMap<TokenAddressRight, AddressedToken>,
    right_to_left: &mut HashMap<TokenAddressLeft, AddressedToken>,
) -> Option<TokenAddressRight> {
    let mut tokens = tokens.into_iter();
    let token = tokens.next()?;

    let address_left = TokenAddressLeft {
        hash: hash(&token, left.map(|address| address.hash)),
    };
    let right =
        address_token(Some(address_left), tokens, left_to_right, right_to_left);
    let address_right = TokenAddressRight {
        hash: hash(&token, right.map(|address| address.hash)),
    };

    let addressed_token = AddressedToken {
        token,
        left_neighbor: left,
        right_neighbor: right,
    };

    left_to_right.insert(address_right, addressed_token.clone());
    right_to_left.insert(address_left, addressed_token);

    Some(address_right)
}

fn hash(token: &Token, neighbor: Option<blake3::Hash>) -> blake3::Hash {
    let mut hasher = blake3::Hasher::new();

    hasher.update(token.to_string().as_bytes());
    if let Some(neighbor) = neighbor {
        hasher.update(neighbor.as_bytes());
    }

    hasher.finalize()
}
