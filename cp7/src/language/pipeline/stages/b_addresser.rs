use itertools::Itertools;

use crate::language::tokens::{
    LeftNeighborAddress, RightNeighborAddress, Token, TokenAddress, Tokens,
};

pub fn address(tokens: Vec<Token>) -> Tokens {
    let addresses_as_left_neighbor = addresses_as_left_neighbor(&tokens);
    let addresses_as_right_neighbor = addresses_as_right_neighbor(&tokens);

    let addresses =
        addresses(&addresses_as_left_neighbor, &addresses_as_right_neighbor);

    let by_address = addresses.iter().copied().zip(tokens).collect();

    let left_to_right = addresses
        .iter()
        .copied()
        .tuple_windows()
        .map(|(a, b)| (a, b))
        .collect();
    let right_to_left = addresses
        .iter()
        .copied()
        .rev()
        .tuple_windows()
        .map(|(a, b)| (a, b))
        .collect();

    let leftmost = addresses.first().copied();
    let rightmost = addresses.last().copied();

    Tokens {
        by_address,

        leftmost,
        rightmost,

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

fn hash(token: &Token, neighbor: Option<blake3::Hash>) -> blake3::Hash {
    let mut hasher = blake3::Hasher::new();

    hasher.update(token.to_string().as_bytes());
    if let Some(neighbor) = neighbor {
        hasher.update(neighbor.as_bytes());
    }

    hasher.finalize()
}
