use crate::engine::env::Env;
use crate::engine::error::Error;
use crate::engine::expr::Expr;
use crate::engine::stack::Stack;
use crate::engine::value::Value;
use std::fmt::{Debug, Formatter, Write};

#[derive(Clone)]
pub enum Function {
    Fox {
        name: String,
        items: Vec<Expr>,
    },
    Native {
        name: String,
        handler: fn(&Env<Function>, &Env<Value>, &mut Stack) -> Result<(), Error>,
    },
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Function")
    }
}

impl Function {
    pub fn fox(name: String, items: Vec<Expr>) -> Self {
        Function::Fox { name, items }
    }

    pub fn native(
        name: impl Into<String>,
        handler: fn(&Env<Function>, &Env<Value>, &mut Stack) -> Result<(), Error>,
    ) -> Self
where {
        Function::Native {
            name: name.into(),
            handler,
        }
    }

    pub fn call(
        &self,
        definitions: &Env<Function>,
        locals: &Env<Value>,
        stack: &mut Stack,
    ) -> Result<(), Error> {
        match self {
            Function::Fox { name: _, items: _ } => {
                // Implement the behavior for Fox functions
                unimplemented!();
            }
            Function::Native { name, handler } => handler(definitions, locals, stack),
        }
    }
}
