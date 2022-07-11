use chumsky::{
    prelude::*,
    text::{digits, whitespace},
};
use eq_float::F64;
use std::{
    fmt,
    fmt::Display,
    hash::{Hash, Hasher},
};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Token {
    Def,
    Type,
    Eq,
    Lambda,
    Dot,
    Comma,
    Colon,
    Pipe,
    Term(String),
    Module(String),
    Literal(Literal),
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Arrow,
    Then,
    Else,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, "TODO"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Char(char),
    String(String),
}

impl Hash for Literal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Literal::Int(x) => x.hash(state),
            Literal::Float(x) => F64(*x).hash(state),
            Literal::Char(x) => x.hash(state),
            Literal::String(x) => x.hash(state),
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::Int(a), Literal::Int(b)) => a == b,
            (Literal::Float(a), Literal::Float(b)) => F64(*a) == F64(*b),
            (Literal::Char(a), Literal::Char(b)) => a == b,
            (Literal::String(a), Literal::String(b)) => a == b,
            _ => false,
        }
    }
}
impl Eq for Literal {}

pub fn root() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    let token = choice((keyword(), literal(), term(), module())).padded();

    let with_comments = comment()
        .repeated()
        .ignore_then(token)
        .then_ignore(comment().repeated());

    with_comments.repeated().then_ignore(end())
}

pub fn keyword() -> impl Parser<char, Token, Error = Simple<char>> {
    choice((
        just("def").map(|_| Token::Def),
        just("type").map(|_| Token::Type),
        just("=").map(|_| Token::Eq),
        just("\\").map(|_| Token::Lambda),
        just(".").map(|_| Token::Dot),
        just(",").map(|_| Token::Comma),
        just(":").map(|_| Token::Colon),
        just("|").map(|_| Token::Pipe),
        just("(").map(|_| Token::LParen),
        just(")").map(|_| Token::RParen),
        just("{").map(|_| Token::LBrace),
        just("}").map(|_| Token::RBrace),
        just("[").map(|_| Token::LBracket),
        just("]").map(|_| Token::RBracket),
        just("->").map(|_| Token::Arrow),
        just("then").map(|_| Token::Then),
        just("else").map(|_| Token::Else),
    ))
}

pub fn term() -> impl Parser<char, Token, Error = Simple<char>> {
    filter(|c| match c {
        'a'..='z' => true,
        c if "><_=-+?!*/%|~".contains(*c) => true,
        _ => false,
    })
    .then(
        filter(|c| match c {
            'a'..='z' => true,
            'A'..='Z' => true,
            '0'..='9' => true,
            c if "><_=-+?!*/%|~".contains(*c) => true,
            _ => false,
        })
        .repeated(),
    )
    .padded()
    .map(|(head, tail)| [vec![head], tail].concat())
    .collect()
    .map(Token::Term)
}

pub fn module() -> impl Parser<char, Token, Error = Simple<char>> {
    filter(|c| match c {
        'A'..='Z' => true,
        _ => false,
    })
    .then(
        filter(|c| match c {
            'a'..='z' => true,
            'A'..='Z' => true,
            '0'..='9' => true,
            _ => false,
        })
        .repeated(),
    )
    .padded()
    .map(|(head, tail)| [vec![head], tail].concat())
    .collect()
    .map(Token::Module)
}

pub fn comment() -> impl Parser<char, (), Error = Simple<char>> {
    just("#")
        .then(take_until(text::newline().or(end())))
        .ignored()
}

pub fn literal() -> impl Parser<char, Token, Error = Simple<char>> {
    choice((
        float().map(Literal::Float),
        integer().map(Literal::Int),
        string().map(Literal::String),
        character().map(Literal::Char),
    ))
    .map(Token::Literal)
}

pub fn integer() -> impl Parser<char, i64, Error = Simple<char>> {
    let number = text::int(10).map(|s: String| s.parse::<i64>().unwrap());
    let negative = just('-').or_not().map(|neg| neg.is_some());

    negative
        .then(number)
        .map(|(neg, i)| if neg { -i } else { i })
}

pub fn float() -> impl Parser<char, f64, Error = Simple<char>> {
    let number = text::int(10).map(|s: String| s.parse::<i64>().unwrap());
    let fraction = digits(10).map(|s: String| s.parse::<i64>().unwrap());
    let point = just('.');
    let float = number
        .then_ignore(point)
        .then(fraction)
        .map(|(l, r)| format!("{}.{}", l, r).parse::<f64>().unwrap());
    let negative = just('-').or_not().map(|neg| neg.is_some());

    negative
        .then(float)
        .map(|(neg, f)| if neg { -f } else { f })
}

pub fn string() -> impl Parser<char, String, Error = Simple<char>> {
    just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect()
}

pub fn character() -> impl Parser<char, char, Error = Simple<char>> {
    just("'").ignore_then(any()).then_ignore(just("'"))
}
