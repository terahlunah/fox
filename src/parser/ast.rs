#[derive(Debug)]
pub struct Ast {
    pub defs: Vec<Definition>,
}

#[derive(Debug)]
pub enum Definition {
    FunctionDef(FunctionDefinition),
    //TypeDef,
}

#[derive(Debug)]
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

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Char(char),
    String(Vec<char>),
}
