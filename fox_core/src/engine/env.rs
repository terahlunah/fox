use std::collections::HashMap;

#[derive(Debug)]
pub struct Env<T> {
    env: HashMap<String, T>,
    parent: Option<Box<Env<T>>>,
}

impl<T: Clone> Env<T> {
    pub fn new(parent: Option<Box<Env<T>>>) -> Self {
        Env {
            env: HashMap::new(),
            parent,
        }
    }

    pub fn has(&self, key: &str) -> bool {
        self.env.contains_key(key) || self.parent.as_ref().map_or(false, |p| p.has(key))
    }

    pub fn get(&self, key: &str) -> Option<T> {
        self.env
            .get(key)
            .cloned()
            .or_else(|| self.parent.as_ref().and_then(|p| p.get(key)))
    }

    pub fn set(&mut self, key: String, value: T) {
        self.env.insert(key, value);
    }
}
