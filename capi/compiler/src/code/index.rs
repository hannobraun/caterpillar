use std::marker::PhantomData;

/// # The index of a named function in the root context
#[derive(
    Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Index<T> {
    pub value: u32,
    pub t: PhantomData<T>,
}

impl<T> Clone for Index<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Index<T> {}

impl<T> udigest::Digestable for Index<T> {
    fn unambiguously_encode<B: udigest::Buffer>(
        &self,
        encoder: udigest::encoding::EncodeValue<B>,
    ) {
        self.value.unambiguously_encode(encoder);
    }
}
