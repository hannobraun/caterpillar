use std::collections::HashMap;

use crate::language::tokens::{
    self, AddressedToken, Token, TokenAddress, Tokens,
};

pub fn address(tokens: impl IntoIterator<Item = Token>) -> Tokens {
    let mut by_address = HashMap::new();
    let mut left_to_right = HashMap::new();
    let mut right_to_left = HashMap::new();

    let addresser_output = address_token(
        None,
        tokens,
        &mut by_address,
        &mut left_to_right,
        &mut right_to_left,
    );
    let (leftmost, rightmost) = match addresser_output {
        Some((leftmost, rightmost)) => (Some(leftmost), Some(rightmost)),
        None => (None, None),
    };

    Tokens {
        by_address,
        leftmost,
        rightmost,
        left_to_right,
        right_to_left,
    }
}

fn address_token(
    left_neighbor: Option<tokens::LeftNeighborAddress>,
    tokens: impl IntoIterator<Item = Token>,
    by_address: &mut HashMap<TokenAddress, Token>,
    left_to_right: &mut HashMap<tokens::RightNeighborAddress, AddressedToken>,
    right_to_left: &mut HashMap<tokens::LeftNeighborAddress, AddressedToken>,
) -> Option<(tokens::RightNeighborAddress, tokens::LeftNeighborAddress)> {
    let mut tokens = tokens.into_iter();
    let token = tokens.next()?;

    let token_as_left_neighbor = tokens::LeftNeighborAddress {
        hash: hash(&token, left_neighbor.map(|address| address.hash)),
    };

    let addresser_output = address_token(
        Some(token_as_left_neighbor),
        tokens,
        by_address,
        left_to_right,
        right_to_left,
    );
    let (right_neighbor, rightmost) = match addresser_output {
        Some((right_neighbor, rightmost)) => (Some(right_neighbor), rightmost),
        None => (None, token_as_left_neighbor),
    };

    let token_as_right_neighbor = tokens::RightNeighborAddress {
        hash: hash(&token, right_neighbor.map(|address| address.hash)),
    };

    let address = TokenAddress {
        as_left_neighbor: token_as_left_neighbor,
        as_right_neighbor: token_as_right_neighbor,
    };
    let addressed_token = AddressedToken {
        token: address,
        left_neighbor,
        right_neighbor,
    };
    by_address.insert(address, token);

    left_to_right.insert(token_as_right_neighbor, addressed_token.clone());
    right_to_left.insert(token_as_left_neighbor, addressed_token);

    Some((token_as_right_neighbor, rightmost))
}

fn hash(token: &Token, neighbor: Option<blake3::Hash>) -> blake3::Hash {
    let mut hasher = blake3::Hasher::new();

    hasher.update(token.to_string().as_bytes());
    if let Some(neighbor) = neighbor {
        hasher.update(neighbor.as_bytes());
    }

    hasher.finalize()
}
