use crate::parsing::lexer::Literal;

#[derive(Debug)]
pub struct Ast {
    pub defs: Vec<Definition>,
}

#[derive(Debug)]
pub enum Definition {
    FunctionDef(FunctionDefinition),
    //TypeDef,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDefinition {
    pub name: String,
    pub body: Block,
}

pub type Block = Vec<Expr>;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Term(String),
    List(Vec<Block>),
    Dict,
    Tuple,
    Quote(Block),
}
