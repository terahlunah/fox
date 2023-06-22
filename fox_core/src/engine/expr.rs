#[derive(Debug, Clone)]
pub enum Expr {
    Bool(bool),
    Num(f64),
    Char(char),
    Symbol(String),
}

impl Expr {
    pub fn bool(b: bool) -> Self {
        Self::Bool(b).into()
    }

    pub fn char(c: char) -> Self {
        Self::Char(c).into()
    }

    pub fn num(n: f64) -> Self {
        Self::Num(n).into()
    }

    pub fn symbol(s: impl Into<String>) -> Self {
        Self::Symbol(s.into()).into()
    }

    pub fn repr(&self) -> String {
        match self {
            Expr::Bool(v) => v.to_string(),
            Expr::Num(v) => v.to_string(),
            Expr::Char(v) => v.to_string(),
            Expr::Symbol(v) => v.to_string(),
        }
    }
}
