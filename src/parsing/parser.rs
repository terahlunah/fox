use crate::parsing::lexer::{Literal, Token};
use chumsky::{
    prelude::*,
    text::{digits, whitespace},
};
use std::collections::HashMap;

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
    pub body: ExprList,
}

pub type ExprList = Vec<Expr>;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Term(String),
    Tuple(Vec<ExprList>),
    List(Vec<ExprList>),
    Table(Vec<(ExprList, ExprList)>),
    Quote(ExprList),
    Local(Vec<String>),
}

pub fn root() -> impl Parser<Token, Ast, Error = Simple<Token>> {
    definition()
        .repeated()
        .then_ignore(end())
        .map(|defs| Ast { defs })
}

pub fn definition() -> impl Parser<Token, Definition, Error = Simple<Token>> {
    choice((function().map(Definition::FunctionDef),))
}

pub fn function() -> impl Parser<Token, FunctionDefinition, Error = Simple<Token>> {
    keyword(Token::Def)
        .ignore_then(term_name())
        .then_ignore(keyword(Token::Eq))
        .then(expr().repeated())
        .map(|(name, body)| FunctionDefinition { name, body })
}

pub fn keyword(token: Token) -> impl Parser<Token, Token, Error = Simple<Token>> {
    // select! { t if t == token => t.clone() }
    filter_map(move |span, x: Token| match x {
        t if t == token => Ok(t.clone()),
        _ => Err(chumsky::Error::expected_input_found(
            span,
            None,
            Some(token.clone()),
        )),
    })
}

pub fn expr() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    recursive(|expr| {
        let literal = select! { Token::Literal(l) => l.clone() }.map(Expr::Literal);

        let term = term_name().map(Expr::Term);

        let lambda = just(Token::Lambda)
            .ignore_then(select! {
                Token::Literal(l) => Expr::Literal(l),
                Token::Term(t) => Expr::Term(t),
            })
            .map(|l| Expr::Quote(vec![l]));

        let quote = expr
            .clone()
            .repeated()
            .delimited_by(just(Token::LBrace), just(Token::RBrace))
            .map(Expr::Quote);

        let tuple = expr
            .clone()
            .repeated()
            .separated_by(just(Token::Comma))
            .delimited_by(just(Token::LParen), just(Token::RParen))
            .map(Expr::Tuple);

        let list = expr
            .clone()
            .repeated()
            .separated_by(just(Token::Comma))
            .delimited_by(just(Token::LBracket), just(Token::RBracket))
            .map(Expr::List);

        let table_pair = expr
            .clone()
            .repeated()
            .then_ignore(just(Token::Colon))
            .then(expr.clone().repeated());
        let table = table_pair
            .separated_by(just(Token::Comma))
            .delimited_by(just(Token::LBracket), just(Token::RBracket))
            .map(Expr::Table);

        let local = just(Token::Local)
            .ignore_then(term_name().separated_by(just(Token::Comma)))
            .map(Expr::Local);

        choice((literal, term, quote, lambda, tuple, list, table, local))
    })
}

pub fn term_name() -> impl Parser<Token, String, Error = Simple<Token>> {
    select! { Token::Term(t) => t.clone() }.labelled("term")
}
