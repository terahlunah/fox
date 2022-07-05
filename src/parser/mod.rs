pub mod ast;

use crate::parser::ast::{Ast, Literal, Term, TermList};
use chumsky::{
    prelude::*,
    text::{digits, whitespace},
};

pub fn root() -> impl Parser<char, TermList, Error = Simple<char>> {
    term_list()
}

pub fn term_list() -> impl Parser<char, TermList, Error = Simple<char>> {
    term().separated_by(whitespace())
}

pub fn term() -> impl Parser<char, Term, Error = Simple<char>> {
    choice((literal().map(Term::Literal),))
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
