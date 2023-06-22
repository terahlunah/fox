use std::fmt::Debug;

use crate::engine::value::{NativeObject, Value};

#[derive(Debug, Clone)]
pub struct List(pub Vec<Value>);

impl NativeObject for List {
    fn repr(&self) -> String {
        let items: Vec<String> = self.0.iter().map(|it| it.data_ref().repr()).collect();
        format!("[{}]", items.join(", "))
    }

    fn cloned(&self) -> Box<dyn NativeObject> {
        let cloned: List = Clone::clone(self);
        Box::new(cloned)
    }
}
