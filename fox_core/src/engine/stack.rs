use crate::engine::error::Error;
use crate::engine::value::{NativeObject, Value, ValueData};

#[derive(Debug)]
pub struct Stack {
    stack: Vec<Value>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    pub fn push(&mut self, value: impl Into<Value>) {
        self.stack.push(value.into());
    }

    pub fn pop(&mut self) -> Result<Value, Error> {
        self.stack
            .pop()
            .ok_or(Error::StackError("Stack is empty".into()))
    }

    pub fn peek(&self) -> Result<Value, Error> {
        self.stack
            .last()
            .ok_or(Error::StackError("Stack is empty".into()))
            .cloned()
    }

    pub fn push_bool(&mut self, b: bool) {
        self.push(ValueData::Bool(b));
    }

    pub fn push_char(&mut self, r: char) {
        self.push(ValueData::Char(r));
    }

    pub fn push_num(&mut self, d: f64) {
        self.push(ValueData::Num(d));
    }

    pub fn push_symbol(&mut self, s: String) {
        self.push(ValueData::Symbol(s));
    }

    pub fn push_native(&mut self, obj: impl NativeObject) {
        self.push(ValueData::Native(Box::new(obj)));
    }
}
