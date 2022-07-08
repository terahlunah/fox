pub mod ast;

use crate::parser::ast::{Ast, Block, Definition, Expr, FunctionDefinition, Literal};
use chumsky::{
    prelude::*,
    text::{digits, whitespace},
};

pub fn root() -> impl Parser<char, Ast, Error = Simple<char>> {
    definition()
        .repeated()
        .then_ignore(end())
        .map(|defs| Ast { defs })
}

pub fn definition() -> impl Parser<char, Definition, Error = Simple<char>> {
    choice((function().map(Definition::FunctionDef),))
}

pub fn function() -> impl Parser<char, FunctionDefinition, Error = Simple<char>> {
    just("def")
        .ignore_then(function_name())
        .then(block())
        .map(|(name, body)| FunctionDefinition { name, body })
}

pub fn function_name() -> impl Parser<char, String, Error = Simple<char>> {
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
}

pub fn block() -> impl Parser<char, Block, Error = Simple<char>> {
    recursive(|block| {
        expr(block)
            .separated_by(whitespace())
            .padded()
            .delimited_by(just('{'), just('}'))
            .padded()
    })
}

pub fn expr(
    block: impl Parser<char, Block, Error = Simple<char>>,
) -> impl Parser<char, Expr, Error = Simple<char>> {
    choice((
        literal().map(Expr::Literal),
        term().map(Expr::Term),
        lambda(),
        block.map(Expr::Quote),
    ))
}

pub fn term() -> impl Parser<char, String, Error = Simple<char>> {
    function_name()
}

pub fn lambda() -> impl Parser<char, Expr, Error = Simple<char>> {
    just('\\')
        .ignore_then(choice((
            literal().map(Expr::Literal),
            term().map(Expr::Term),
        )))
        .map(|l| Expr::Quote(vec![l]))
}

pub fn literal() -> impl Parser<char, Literal, Error = Simple<char>> {
    choice((
        float().map(Literal::Float),
        integer().map(Literal::Int),
        string().map(Literal::String),
        character().map(Literal::Char),
    ))
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

pub fn string() -> impl Parser<char, Vec<char>, Error = Simple<char>> {
    just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<Vec<char>>()
}

pub fn character() -> impl Parser<char, char, Error = Simple<char>> {
    just("'").ignore_then(any()).then_ignore(just("'"))
}
