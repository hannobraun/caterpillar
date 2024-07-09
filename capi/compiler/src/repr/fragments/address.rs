use super::FragmentId;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct FragmentAddress {
    pub function: FragmentAddressParent,
    pub next: Option<FragmentId>,
}

impl FragmentAddress {
    pub(super) fn hash(&self, hasher: &mut blake3::Hasher) {
        let FragmentAddressParent::Function { name } = &self.function;
        hasher.update(name.as_bytes());
        if let Some(next) = self.next {
            hasher.update(next.hash.as_bytes());
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum FragmentAddressParent {
    Function { name: String },
}
