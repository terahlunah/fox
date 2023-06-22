extern crate core;

pub mod engine;
// pub mod parsing;
pub mod stdlib;

// use std::fs;

// use chumsky::Parser;

// pub fn execute_script(path: String) {
//     let source = fs::read_to_string(path.clone()).expect("Something went wrong reading the file");
//
//     match parsing::lexer::root().parse(source.as_str()) {
//         Ok(tokens) => {
//             println!("Tokens: {:?}\n", tokens);
//             match parsing::parser::root().parse(tokens) {
//                 Ok(ast) => {
//                     println!("Ast: {:?}\n", ast);
//                 }
//                 Err(err) => {
//                     for e in err {
//                         println!("{:?}\n", e);
//                     }
//                 }
//             };
//         }
//         Err(err) => {
//             for e in err {
//                 println!("{}", e);
//             }
//         }
//     };
// }
