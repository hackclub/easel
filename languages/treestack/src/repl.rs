use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{stdout, Write};

pub fn start_repl(debug: bool) {
    enable_raw_mode().expect("Error while trying to start repl");

    let mut interpreter = Interpreter::new(debug);

    let mut stdout = stdout();

    let mut input = String::new();
    let mut cursor = 0;
    let mut scrollback = 0;
    let mut commands: Vec<String> = Vec::new();
    print!("> ");
    let _ = stdout.flush();

    loop {
        let event = match read().unwrap() {
            Event::Key(key) => key,
            _ => continue,
        };

        if event.kind == KeyEventKind::Release {
            continue;
        }

        match event.code {
            KeyCode::Char('c') if event.modifiers == KeyModifiers::CONTROL => {
                break;
            }
            KeyCode::Char(c) => {
                if cursor == input.len() {
                    input.push(c);
                } else {
                    input.insert(cursor, c);
                }
                cursor += 1;
            }
            KeyCode::Backspace => {
                if cursor == 0 {
                    continue;
                }
                input.remove(cursor - 1);
                cursor -= 1;
            }
            KeyCode::Left => {
                if cursor != 0 {
                    cursor -= 1;
                }
            }
            KeyCode::Right => {
                if cursor != input.len() {
                    cursor += 1;
                }
            }
            KeyCode::Up => {
                if scrollback == commands.len() {
                    continue;
                }
                scrollback += 1;
                input = commands[commands.len() - scrollback].clone();
                cursor = input.len();
            }
            KeyCode::Down => {
                if scrollback == 0 { continue; }
                scrollback -= 1;
                input = commands[commands.len() - scrollback].clone();
                cursor = input.len();
            }
            KeyCode::Enter => {
                print!("\n\r");
                disable_raw_mode().unwrap();
                let tokens = Lexer::new(input.clone()).parse();
                let ast = Parser::new(tokens).parse().unwrap();
                let result = interpreter.parse(ast);
                if let Err(msg) = result {
                    msg.pretty_print(&input, false);
                }
                if commands.contains(&input) { commands.retain(|x| *x != input); }
                commands.push(input.clone());
                input.clear();
                cursor = 0;
                scrollback = 0;
                print!("\n\r> ");
                enable_raw_mode().unwrap();
            }
            _ => {}
        }

        print!("\x1b[2K\r> {}\x1b[0m\r\x1b[{}C", &input, cursor + 2);
        stdout.flush().unwrap();
    }

    disable_raw_mode().unwrap();
}

