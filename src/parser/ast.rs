#[derive(Debug)]
pub struct Ast {
    defs: Vec<Definition>,
}

#[derive(Debug)]
pub enum Definition {
    FunctionDef(FunctionDefinition),
    //TypeDef,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    name: String,
    body: TermList,
}

pub type TermList = Vec<Term>;

#[derive(Debug, PartialEq)]
pub enum Term {
    Literal(Literal),
    Term(String),
    List(Vec<TermList>),
    Dict,
    Tuple,
    Block,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Char(char),
    String(Vec<char>),
}
