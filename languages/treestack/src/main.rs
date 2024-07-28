mod error;
mod interpreter;
mod lexer;
mod parser;
mod tree;
//mod compiler;
mod repl;
//use crate::parser::Parser;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use clap::{arg, command, Parser};
use crossterm::terminal::disable_raw_mode;
use error::{Positioned, RangeError};
use parser::Node;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    file: Option<String>,

    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    // Proper Clap stuff
    if let Some(ref file) = args.file {
        run_file(&file, args.debug);
    } else {
        repl::start_repl(args.debug);
    }
}

fn run_file(file: &str, debug: bool) {
    let program = match load_file(file) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let ast = match compile_ast(program.clone(), debug) {
        Ok(ast) => ast,
        Err(err) => {
            err.pretty_print(&program, true);
            return;
        }
    };

    if let Err(err) = Interpreter::new(debug).parse(ast) {
        disable_raw_mode().unwrap();
        err.pretty_print(&program, true);
    }
}

fn compile_ast(program: String, debug: bool) -> Result<Vec<Positioned<Node>>, RangeError> {
    let tokens = Lexer::new(program).parse();
    if debug {
        println!("{tokens:?}");
    } // FIT behind debug flag

    let ast = parser::Parser::new(tokens).parse()?;
    if debug {
        println!("{ast:?}");
    } // FIT behind debug flag

    Ok(ast)
}

fn load_file(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|e| format!("Error while loading program: {:?}", e.kind()))
}
