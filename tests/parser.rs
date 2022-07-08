use chumsky::{prelude::*, Parser};
use fox::{self, parser, parser::ast::Literal};
use pretty_assertions::assert_eq;

fn test_parser<O, P>(p: P) -> impl Fn(&str) -> Result<O, Vec<P::Error>>
where
    P: Parser<char, O, Error = Simple<char>>,
{
    let parser = p.then_ignore(end());
    move |s: &str| parser.parse(s)
}

#[test]
fn test_parse_int() {
    let parse = test_parser(parser::integer());

    assert_eq!(parse("0"), Ok(0));
    assert_eq!(parse("1"), Ok(1));
    assert_eq!(parse("-1"), Ok(-1));

    assert!(parse("+1").is_err());
    assert!(parse("01").is_err());
    assert!(parse("3.14").is_err());
    assert!(parse("foo").is_err());
}

#[test]
fn test_parse_string() {
    let parse = test_parser(parser::string());
    assert_eq!(parse(r#""f o o""#), Ok("f o o".chars().collect()));

    assert!(parse(r#"""foo""#).is_err());
    assert!(parse("1").is_err());
    assert!(parse("0.0").is_err());
}

#[test]
fn test_parse_char() {
    let parse = test_parser(parser::character());
    assert_eq!(parse("'a'"), Ok('a'));
    assert_eq!(parse("'1'"), Ok('1'));

    assert!(parse("1").is_err());
}

#[test]
fn test_parse_float() {
    let parse = test_parser(parser::float());

    assert_eq!(parse("0.0"), Ok(0.0));
    assert_eq!(parse("-3.14"), Ok(-3.14));

    assert!(parse("+0.0").is_err());
    assert!(parse("01.0").is_err());
    assert!(parse("3").is_err());
    assert!(parse(".14").is_err());
    assert!(parse("foo").is_err());
}

#[test]
fn test_parse_literal() {
    let parse = test_parser(parser::literal());

    assert_eq!(parse("0"), Ok(Literal::Int(0)));
    assert_eq!(parse("0.0"), Ok(Literal::Float(0.0)));
    assert_eq!(parse("'c'"), Ok(Literal::Char('c')));
    assert_eq!(
        parse(r#""foo""#),
        Ok(Literal::String("foo".chars().collect()))
    );
}

#[test]
fn test_parse_term() {
    let parse = test_parser(parser::term());

    assert_eq!(parse("foo"), Ok("foo".to_string()));
    assert_eq!(parse(">5"), Ok(">5".to_string()));
    assert_eq!(parse("hasFlag?"), Ok("hasFlag?".to_string()));
    assert_eq!(
        parse(r#">bar<_=-+?!*/%|~"#),
        Ok(">bar<_=-+?!*/%|~".to_string())
    );

    assert!(parse("0").is_err());
    assert!(parse("Foo").is_err());
}

#[test]
fn test_parse_block() {
    let parse = test_parser(parser::block());

    assert!(parse("{ \n}").is_ok());
    assert!(parse("a").is_err());
}
