use std::marker::PhantomData;

#[derive(
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
pub struct Hash<T> {
    pub(super) hash: [u8; 32],
    pub(super) _t: PhantomData<T>,
}

impl<T> Clone for Hash<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Hash<T> {}
