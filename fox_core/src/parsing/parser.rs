use std::collections::HashMap;

use chumsky::prelude::*;

use crate::parsing::lexer::{Literal, Token};

#[derive(Debug, Clone)]
pub struct Ast {
    pub defs: Vec<Definition>,
}

#[derive(Debug, Clone)]
pub enum Definition {
    Function(FunctionDefinition),
    Type(TypeDefinition),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeDefinition {
    pub name: String,
    pub vars: Vec<String>,
    pub variants: Vec<VariantDefinition>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariantDefinition {
    pub name: String,
    pub items: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub ftype: Option<FunctionType>,
    pub body: ExprList,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionType {
    inputs: Vec<ParamType>,
    output: Vec<ParamType>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ValueType {
    name: String,
    module: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParamType {
    Function(FunctionType),
    Value(ValueType),
}

pub type ExprList = Vec<Expr>;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Literal(Literal),
    Term { name: String, module: Vec<String> },
    Local(Vec<String>),
    Quote(ExprList),
}

impl Expr {
    pub fn term(full_name: impl Into<String>) -> Self {
        let s: String = full_name.into();
        let s: Vec<_> = s.split('.').collect();

        let (name, module) = match s.as_slice() {
            [] => unreachable!(),
            [tail @ .., head] => (
                head.to_string(),
                tail.iter().map(|it| it.to_string()).collect(),
            ),
        };

        Self::Term { name, module }
    }
}

pub fn root() -> impl Parser<Token, Ast, Error = Simple<Token>> {
    definition()
        .repeated()
        .then_ignore(end())
        .map(|defs| Ast { defs })
}

pub fn definition() -> impl Parser<Token, Definition, Error = Simple<Token>> {
    choice((
        function_def().map(Definition::Function),
        type_def().map(Definition::Type),
    ))
}

pub fn type_def() -> impl Parser<Token, TypeDefinition, Error = Simple<Token>> {
    let type_name = type_name().then(type_var().repeated());

    keyword(Token::Type)
        .ignore_then(type_name.clone())
        .then_ignore(keyword(Token::Eq))
        .then(
            type_variant()
                .separated_by(just(Token::Pipe))
                .allow_leading(),
        )
        .map(|((name, vars), variants)| TypeDefinition {
            name,
            vars,
            variants,
        })
}

pub fn type_variant() -> impl Parser<Token, VariantDefinition, Error = Simple<Token>> {
    let tuple_var = upper_name()
        .then(lower_or_upper_name().repeated())
        .map(|(name, variants)| {
            let mut items = HashMap::new();
            let mut n = 0;
            for v in variants {
                let key = format!("_{n}");
                items.insert(key, v);
                n += 1;
            }
            VariantDefinition { name, items }
        });

    let record_var = upper_name()
        .then(
            lower_name()
                .then_ignore(just(Token::Colon))
                .then(lower_or_upper_name())
                .separated_by(just(Token::Comma))
                .delimited_by(just(Token::LBrace), just(Token::RBrace)),
        )
        .map(|(name, variants)| {
            let items = variants.into_iter().collect();
            VariantDefinition { name, items }
        });

    choice((record_var, tuple_var))
}

pub fn function_def() -> impl Parser<Token, FunctionDefinition, Error = Simple<Token>> {
    keyword(Token::Def)
        .ignore_then(term_name())
        .then(function_type().or_not())
        .then_ignore(keyword(Token::Eq))
        .then(expr().repeated().flatten())
        .map(|((name, ftype), body)| FunctionDefinition { name, ftype, body })
}

pub fn function_type() -> impl Parser<Token, FunctionType, Error = Simple<Token>> {
    recursive(|ftype| {
        let plist = choice((
            value_type().map(ParamType::Value),
            ftype.map(ParamType::Function),
        ))
        .separated_by(just(Token::Comma));

        let input = plist.clone().map(|i| FunctionType {
            inputs: i,
            output: vec![],
        });
        let output = just(Token::Arrow)
            .ignore_then(plist.clone())
            .map(|o| FunctionType {
                inputs: vec![],
                output: o,
            });
        let input_output = plist
            .clone()
            .then(just(Token::Arrow).ignore_then(plist.clone()))
            .map(|(i, o)| FunctionType {
                inputs: i,
                output: o,
            });

        choice((input_output, output, input)).delimited_by(just(Token::LParen), just(Token::RParen))
    })
}

pub fn value_type() -> impl Parser<Token, ValueType, Error = Simple<Token>> + Clone {
    module_name()
        .separated_by(just(Token::Dot))
        .at_least(1)
        .map(|modules| match modules.as_slice() {
            [] => unreachable!(),
            [tail @ .., head] => ValueType {
                name: head.clone(),
                module: tail.iter().map(|it| it.to_string()).collect(),
            },
        })
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

pub fn expr() -> impl Parser<Token, ExprList, Error = Simple<Token>> {
    recursive(|expr| {
        let literal = select! { Token::Literal(l) => l.clone() }.map(Expr::Literal);

        let local = just(Token::Arrow)
            .ignore_then(term_name().separated_by(just(Token::Comma)))
            .map(Expr::Local);

        let quote = expr
            .clone()
            .repeated()
            .flatten()
            .delimited_by(just(Token::LBrace), just(Token::RBrace))
            .map(Expr::Quote);

        let lambda = just(Token::Lambda)
            .ignore_then(choice((literal.clone(), term())))
            .map(|l| Expr::Quote(vec![l]));

        choice((
            vec(literal),
            vec(term()),
            vec(local),
            vec(quote),
            vec(lambda),
            tuple(expr.clone()),
            list(expr.clone()),
            table(expr.clone()),
            then_else(expr.clone()),
        ))
    })
}

pub fn lower_name() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
    select! { Token::LowerName(t) => t.clone() }
}

pub fn upper_name() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
    select! { Token::UpperName(t) => t.clone() }
}

pub fn lower_or_upper_name() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
    choice((upper_name(), lower_name()))
}

pub fn term_name() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
    lower_name().labelled("term name")
}

pub fn module_name() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
    upper_name().labelled("module name")
}

pub fn type_name() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
    upper_name().labelled("type name")
}

pub fn type_var() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
    term_name().labelled("var name")
}

pub fn term() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    let module = module_name()
        .separated_by(just(Token::Dot))
        .then_ignore(just(Token::Dot));
    module
        .or_not()
        .then(term_name())
        .map(|(module, name)| Expr::Term {
            name: name.clone(),
            module: module.unwrap_or_default(),
        })
}

pub fn vec(
    expr: impl Parser<Token, Expr, Error = Simple<Token>>,
) -> impl Parser<Token, ExprList, Error = Simple<Token>> {
    expr.map(|it| vec![it])
}

pub fn list(
    expr: impl Parser<Token, ExprList, Error = Simple<Token>>,
) -> impl Parser<Token, ExprList, Error = Simple<Token>> {
    expr.repeated()
        .flatten()
        .separated_by(just(Token::Comma))
        .delimited_by(just(Token::LBracket), just(Token::RBracket))
        .map(|items| {
            let mut res = vec![Expr::term("Core.List.empty")];
            for mut e in items {
                res.append(&mut e);
                res.push(Expr::term("Core.List.push"));
            }
            res
        })
}

pub fn tuple(
    expr: impl Parser<Token, ExprList, Error = Simple<Token>>,
) -> impl Parser<Token, ExprList, Error = Simple<Token>> {
    expr.repeated()
        .flatten()
        .separated_by(just(Token::Comma))
        .at_most(9)
        .delimited_by(just(Token::LParen), just(Token::RParen))
        .map(|items| {
            let n = items.len();
            let mut res = vec![];
            for mut e in items {
                res.append(&mut e);
            }
            res.push(Expr::term(format!("Core.Tuple{n}")));
            res
        })
}

pub fn table(
    expr: impl Parser<Token, ExprList, Error = Simple<Token>> + Clone,
) -> impl Parser<Token, ExprList, Error = Simple<Token>> {
    let table_pair = expr
        .clone()
        .repeated()
        .flatten()
        .then_ignore(just(Token::Colon))
        .then(expr.clone().repeated().flatten());
    table_pair
        .separated_by(just(Token::Comma))
        .delimited_by(just(Token::LBracket), just(Token::RBracket))
        .map(|pairs| {
            let mut res = vec![Expr::term("Core.Table.empty")];
            for (mut k, mut v) in pairs {
                res.append(&mut k);
                res.append(&mut v);
                res.push(Expr::term("Core.Table.set"));
            }
            res
        })
}

pub fn then_else(
    expr: impl Parser<Token, ExprList, Error = Simple<Token>> + Clone,
) -> impl Parser<Token, ExprList, Error = Simple<Token>> {
    just(Token::Then)
        .ignore_then(
            expr.clone()
                .repeated()
                .flatten()
                .delimited_by(just(Token::LBrace), just(Token::RBrace)),
        )
        .then_ignore(just(Token::Else))
        .then(
            expr.clone()
                .repeated()
                .flatten()
                .delimited_by(just(Token::LBrace), just(Token::RBrace)),
        )
        .map(|it| vec![Expr::Quote(it.0), Expr::Quote(it.1), Expr::term("Core.??")])
}
