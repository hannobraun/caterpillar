/// # The type of a value
pub enum Type {
    /// # A function
    Function {
        /// # The inputs that the function consumes
        inputs: Vec<Type>,

        /// # The outputs that the function produces
        outputs: Vec<Type>,
    },
    Number,
}
