use crate::{
    parsing,
    parsing::{
        lexer::{Literal, Token},
        parser::Ast,
    },
};
use ariadne::{sources, Color, Fmt, FnCache, Label, Report, ReportKind, Source, Span};
use chumsky::{
    error::{Simple, SimpleReason},
    Parser,
};
use std::fs;
use tap::Tap;

pub fn execute_script(path: String) {
    let source = fs::read_to_string(path.clone()).expect("Something went wrong reading the file");

    match parsing::lexer::root().parse(source.as_str()) {
        Ok(tokens) => {
            println!("Tokens: {:?}\n", tokens);
            match parsing::parser::root().parse(tokens) {
                Ok(ast) => {
                    println!("Ast: {:?}\n", ast);
                }
                Err(err) => {
                    for e in err {
                        println!("{:?}\n", e);
                    }
                }
            };
        }
        Err(err) => {
            for e in err {
                println!("{}", e);
            }
        }
    };

    // match optimized_ast {
    //     Ok(ast) => {
    //         println!("Final Ast: {:?}", ast);
    //     }
    //     Err(errs) => {
    //         for err in errs {
    //             let report = Report::build(ReportKind::Error, (), err.span().start);
    //             let report = match err.reason() {
    //                 SimpleReason::Unexpected => report
    //                     .with_message(format!(
    //                         "{}, expected {}",
    //                         if err.found().is_some() {
    //                             "Unexpected token in input"
    //                         } else {
    //                             "Unexpected end of input"
    //                         },
    //                         if err.expected().len() == 0 {
    //                             "something else".to_string()
    //                         } else {
    //                             err.expected()
    //                                 .map(|expected| match expected {
    //                                     Some(expected) => expected.to_string(),
    //                                     None => "end of input".to_string(),
    //                                 })
    //                                 .collect::<Vec<_>>()
    //                                 .join(", ")
    //                         }
    //                     ))
    //                     .with_label(
    //                         Label::new(err.span())
    //                             .with_message(format!(
    //                                 "Unexpected token {}",
    //                                 err.found()
    //                                     .map(char::to_string)
    //                                     .unwrap_or("end of file".to_string())
    //                                     .fg(Color::Red)
    //                             ))
    //                             .with_color(Color::Red),
    //                     ),
    //                 SimpleReason::Unclosed { .. } => report,
    //                 SimpleReason::Custom(_) => report,
    //             };
    //             report.finish().print(Source::from(&source)).unwrap();
    //         }
    //     }
    // }
}
