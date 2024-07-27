use std::env;
use std::panic;

mod nil;
use crate::nil::lexer;
use crate::nil::parser;
use crate::nil::evaluate;
use parser::ParserSettings;
use crate::nil::errorhandler;

const USAGE: &'static str = "USEAGE:\n nil <path> [(-l | -p)]

Options:
    -l Display Lexer Output
    -p Display Parser Output
";

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut lexer_log = false;
    let mut parser_log = false;

    panic::set_hook(Box::new(|_info| {
        // do nothing
    }));

    match args.len() {
        1 => {
            println!("{}", USAGE);
            panic!();
        }
        _ => {
            let path = &args[1];

            for flag in &args[2..] {
                match flag.as_str() {
                    "-l" => lexer_log = true,
                    "-p" => parser_log = true,
                    _ => {
                        println!("\x1b[91mError\x1b[0m Unkown Argument Passed: {}", flag);
                        panic!();
                    }
                }
            }

            match path.as_ref() {
                "--help" => println!("{}", USAGE),
                _ => {
                    let parts: &Vec<&str> = &path.as_str().split('.').collect();
                    if parts.len() == 1 {
                        println!("\x1b[91mError\x1b[0m Unkown Argument Passed");
                    panic!();
                    } else {
                        let ent = parts[parts.len()-1];
                        if ent != "nil" {
                            println!("\x1b[91mError\x1b[0m File has unexpected file extension of .{}", ent);
                            panic!();
                        }

                        let content: String;
                        match std::fs::read_to_string(&path) {
                            Ok(x) => content = x,
                            Err(_e) => {
                                println!("\x1b[91mError\x1b[0m Could Not Find File at Path: `{}`", path);
                                panic!();
                            }
                        }

                        //Start of Processing
                        let err_hand = errorhandler::ErrorHandler::new(content.clone(), path.to_string());
                        
                        let mut tokens = lexer::tokenizer(content).unwrap_or_else(|err| err_hand.throw_err(err));
                        
                        if lexer_log {
                            println!("\n{:?}\n", &tokens);
                        }
                        let tree = parser::parser(&mut tokens, &mut ParserSettings::default()).unwrap_or_else(|err| err_hand.throw_err(err));

                        if parser_log {
                            println!("\n{:#?}\n", &tree);
                        }

                        evaluate::eval_ast(tree);
                        
                    }
                }
            }
        }
    }
}
