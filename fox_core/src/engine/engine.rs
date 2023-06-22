use crate::engine::env::Env;
use crate::engine::error::Error;
use crate::engine::expr::Expr;
use crate::engine::function::Function;
use crate::engine::stack::Stack;
use crate::engine::value::{Value, ValueData};

pub struct Engine {
    pub definitions: Env<Function>,
    pub stack: Stack,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            definitions: Env::new(None),
            stack: Stack::new(),
        }
    }

    pub fn eval(&mut self, exprs: &[Expr]) -> Result<(), Error> {
        let mut locals: Env<Value> = Env::new(None);

        for expr in exprs {
            println!("{:?}", self.stack);
            match expr {
                Expr::Bool(b) => self.stack.push_bool(*b),
                Expr::Num(n) => self.stack.push_num(*n),
                Expr::Char(c) => self.stack.push_char(*c),
                Expr::Symbol(s) => {
                    if let Some(local) = locals.get(s.as_str()) {
                        match local.data_ref() {
                            ValueData::Bool(_)
                            | ValueData::Char(_)
                            | ValueData::Num(_)
                            | ValueData::Native(_) => {
                                self.stack.push(local.clone());
                            }
                            ValueData::Symbol(s) => {
                                let f = self.definitions.get(&s);
                                if let Some(f) = f {
                                    self.call(&f, &mut locals)?;
                                }
                            }
                        }
                    } else if let Some(def) = self.definitions.get(s.as_str()) {
                        self.call(&def, &mut locals)?;
                    } else {
                        return Err(Error::UnknownSymbol(s.to_string()));
                    }
                }
            }
        }

        Ok(())
    }

    fn call(&mut self, f: &Function, locals: &mut Env<Value>) -> Result<(), Error> {
        match f {
            Function::Fox { items, .. } => {
                self.eval(items.as_slice());
                Ok(())
            }
            Function::Native { handler, .. } => handler(&self.definitions, locals, &mut self.stack),
        }
    }
}
