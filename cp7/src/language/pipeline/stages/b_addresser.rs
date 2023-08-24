use std::collections::HashMap;

use crate::language::tokens::{
    self, AddressedToken, LeftNeighborAddress, Token, TokenAddress, Tokens,
};

pub fn address(tokens: Vec<Token>) -> Tokens {
    let addresses_as_left_neighbor = addresses_as_left_neighbor(&tokens);

    let mut by_address = HashMap::new();
    let mut right_neighbors = HashMap::new();
    let mut left_to_right = HashMap::new();
    let mut right_to_left = HashMap::new();

    let addresser_output = address_token(
        None,
        tokens,
        addresses_as_left_neighbor,
        &mut by_address,
        &mut right_neighbors,
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

        right_neighbors,

        left_to_right,
        right_to_left,
    }
}

fn addresses_as_left_neighbor(tokens: &[Token]) -> Vec<LeftNeighborAddress> {
    let mut addresses = Vec::new();
    let mut current_hash = None;

    for token in tokens {
        let hash = hash(token, current_hash);
        current_hash = Some(hash);

        addresses.push(LeftNeighborAddress { hash });
    }

    addresses
}

fn address_token(
    left_neighbor: Option<tokens::LeftNeighborAddress>,
    tokens: impl IntoIterator<Item = Token>,
    addresses_as_left_neighbor: impl IntoIterator<Item = LeftNeighborAddress>,
    by_address: &mut HashMap<TokenAddress, Token>,
    right_neighbors: &mut HashMap<TokenAddress, TokenAddress>,
    left_to_right: &mut HashMap<tokens::RightNeighborAddress, AddressedToken>,
    right_to_left: &mut HashMap<tokens::LeftNeighborAddress, AddressedToken>,
) -> Option<(TokenAddress, tokens::LeftNeighborAddress)> {
    let mut tokens = tokens.into_iter();
    let mut addresses_as_left_neighbor = addresses_as_left_neighbor.into_iter();

    let token = tokens.next()?;
    let token_as_left_neighbor = addresses_as_left_neighbor.next()?;

    let addresser_output = address_token(
        Some(token_as_left_neighbor),
        tokens,
        addresses_as_left_neighbor,
        by_address,
        right_neighbors,
        left_to_right,
        right_to_left,
    );
    let (right_neighbor, rightmost) = match addresser_output {
        Some((right_neighbor, rightmost)) => (Some(right_neighbor), rightmost),
        None => (None, token_as_left_neighbor),
    };

    let token_as_right_neighbor = tokens::RightNeighborAddress {
        hash: hash(
            &token,
            right_neighbor.map(|address| address.as_right_neighbor.hash),
        ),
    };

    let address = TokenAddress {
        as_left_neighbor: token_as_left_neighbor,
        as_right_neighbor: token_as_right_neighbor,
    };
    let addressed_token = AddressedToken {
        token: address,
        left_neighbor,
        right_neighbor: right_neighbor.map(|address| address.as_right_neighbor),
    };
    by_address.insert(address, token);

    if let Some(right_neighbor) = right_neighbor {
        right_neighbors.insert(address, right_neighbor);
    }

    left_to_right.insert(token_as_right_neighbor, addressed_token.clone());
    right_to_left.insert(token_as_left_neighbor, addressed_token);

    Some((address, rightmost))
}

fn hash(token: &Token, neighbor: Option<blake3::Hash>) -> blake3::Hash {
    let mut hasher = blake3::Hasher::new();

    hasher.update(token.to_string().as_bytes());
    if let Some(neighbor) = neighbor {
        hasher.update(neighbor.as_bytes());
    }

    hasher.finalize()
}
