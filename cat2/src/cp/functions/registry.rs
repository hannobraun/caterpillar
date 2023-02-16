use super::Function;

pub struct Registry {
    inner: Vec<Function>,
}

impl Registry {
    pub fn new() -> Self {
        let inner = Vec::new();
        Self { inner }
    }

    pub fn define(&mut self, name: impl Into<String>, body: &str) {
        self.inner.push(Function::new(name, body));
    }

    pub fn resolve(&self, name: &str) -> Option<&Function> {
        self.inner.iter().find(|function| function.name == name)
    }
}
