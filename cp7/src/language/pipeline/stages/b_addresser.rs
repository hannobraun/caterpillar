use std::collections::HashMap;

use crate::language::tokens::{
    self, AddressedToken, LeftNeighborAddress, RightNeighborAddress, Token,
    TokenAddress, Tokens,
};

pub fn address(tokens: Vec<Token>) -> Tokens {
    let addresses_as_left_neighbor = addresses_as_left_neighbor(&tokens);
    let addresses_as_right_neighbor = addresses_as_right_neighbor(&tokens);
    let addresses =
        addresses(&addresses_as_left_neighbor, &addresses_as_right_neighbor);
    let by_address = addresses.iter().copied().zip(tokens).collect();

    let mut right_neighbors = HashMap::new();
    let mut left_to_right = HashMap::new();
    let mut right_to_left = HashMap::new();

    let addresser_output = address_token(
        None,
        addresses_as_left_neighbor,
        addresses_as_right_neighbor,
        addresses,
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
    tokens
        .iter()
        .scan(None, |current_hash, token| {
            let hash = hash(token, *current_hash);
            *current_hash = Some(hash);

            Some(LeftNeighborAddress { hash })
        })
        .collect()
}

fn addresses_as_right_neighbor(tokens: &[Token]) -> Vec<RightNeighborAddress> {
    let mut addresses = Vec::new();
    let mut current_hash = None;

    for token in tokens.iter().rev() {
        let hash = hash(token, current_hash);
        current_hash = Some(hash);

        addresses.push(RightNeighborAddress { hash });
    }
    addresses.reverse();

    addresses
}

fn addresses(
    as_left_neighbor: &[LeftNeighborAddress],
    as_right_neighbor: &[RightNeighborAddress],
) -> Vec<TokenAddress> {
    as_left_neighbor
        .iter()
        .copied()
        .zip(as_right_neighbor.iter().copied())
        .map(|(as_left_neighbor, as_right_neighbor)| TokenAddress {
            as_left_neighbor,
            as_right_neighbor,
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn address_token(
    left_neighbor: Option<tokens::LeftNeighborAddress>,
    addresses_as_left_neighbor: impl IntoIterator<Item = LeftNeighborAddress>,
    addresses_as_right_neighbor: impl IntoIterator<Item = RightNeighborAddress>,
    addresses: impl IntoIterator<Item = TokenAddress>,
    right_neighbors: &mut HashMap<TokenAddress, TokenAddress>,
    left_to_right: &mut HashMap<tokens::RightNeighborAddress, AddressedToken>,
    right_to_left: &mut HashMap<tokens::LeftNeighborAddress, AddressedToken>,
) -> Option<(TokenAddress, tokens::LeftNeighborAddress)> {
    let mut addresses_as_left_neighbor = addresses_as_left_neighbor.into_iter();
    let mut addresses_as_right_neighbor =
        addresses_as_right_neighbor.into_iter();
    let mut addresses = addresses.into_iter();

    let token_as_left_neighbor = addresses_as_left_neighbor.next()?;
    let token_as_right_neighbor = addresses_as_right_neighbor.next()?;
    let address = addresses.next()?;

    let addresser_output = address_token(
        Some(token_as_left_neighbor),
        addresses_as_left_neighbor,
        addresses_as_right_neighbor,
        addresses,
        right_neighbors,
        left_to_right,
        right_to_left,
    );
    let (right_neighbor, rightmost) = match addresser_output {
        Some((right_neighbor, rightmost)) => (Some(right_neighbor), rightmost),
        None => (None, token_as_left_neighbor),
    };

    let addressed_token = AddressedToken {
        token: address,
        left_neighbor,
        right_neighbor: right_neighbor.map(|address| address.as_right_neighbor),
    };

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
