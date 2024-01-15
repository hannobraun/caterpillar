use crate::repr::eval::fragments::FragmentId;

#[derive(Clone, Debug)]
pub struct FunctionName {
    pub value: String,
    pub fragment: Option<FragmentId>,
}
