use std::collections::HashMap;
use std::convert::TryInto;
use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let code: String = fs::read_to_string(&args[1])?;
    let instructions: Vec<&str> = code.lines().collect();

    let stdin = io::stdin();
    let mut line_num: usize = 0;
    let mut variables: HashMap<&str, String> = HashMap::new();
    let mut labels: HashMap<&str, String> = HashMap::new();
    loop {
        if line_num >= instructions.len() {
            break;
        }

        let line: Vec<&str> = instructions[line_num].split_whitespace().collect();
        if line[0] == "label" {
            labels.insert(line[1], line_num.to_string());
        }

        line_num += 1;
    }

    line_num = 0;
    loop {
        if line_num >= instructions.len() {
            break;
        }

        let line: Vec<&str> = instructions[line_num].split_whitespace().collect();
        if line[0] == "calculate" {
            let argument1: i32 = variables.get(line[1]).expect("REASON").parse().unwrap();
            let argument2: i32 = variables.get(line[3]).expect("REASON").parse().unwrap();
            if line[2] == "+" {
                variables.insert(line[4], (argument1 + argument2).to_string());

            } else if line[2] == "-" {
                variables.insert(line[4], (argument1 - argument2).to_string());

            } else if line[2] == "*" {
                variables.insert(line[4], (argument1 * argument2).to_string());

            } else if line[2] == "/" {
                variables.insert(line[4], (argument1 / argument2).to_string());

            } else if line[2] == "%" {
                variables.insert(line[4], (argument1 % argument2).to_string());

            } else if line[2] == "^" {
                variables.insert(line[4], i32::pow(argument1, argument2.try_into().unwrap()).to_string());
            }

        } else if line[0] == "compare" {
            if variables.get(line[1]).unwrap().eq("true") || variables.get(line[1]).unwrap().eq("false") || variables.get(line[3]).unwrap().eq("true") || variables.get(line[3]).unwrap().eq("false") {
                let boolean1: bool = variables.get(line[1]).expect("REASON").parse().unwrap();
                let boolean2: bool = variables.get(line[3]).expect("REASON").parse().unwrap();
                if line[2] == "&&" {
                    variables.insert(line[4], (boolean1 && boolean2).to_string());

                } else if line[2] == "||" {
                    variables.insert(line[4], (boolean1 || boolean2).to_string());
                }

            } else {
                let argument1: i32 = variables.get(line[1]).expect("REASON").parse().unwrap();
                let argument2: i32 = variables.get(line[3]).expect("REASON").parse().unwrap();
                if line[2] == "==" {
                    variables.insert(line[4], (argument1 == argument2).to_string());

                } else if line[2] == "!=" {
                    variables.insert(line[4], (argument1 != argument2).to_string());

                } else if line[2] == ">" {
                    variables.insert(line[4], (argument1 > argument2).to_string());

                } else if line[2] == "<" {
                    variables.insert(line[4], (argument1 < argument2).to_string());

                } else if line[2] == ">=" {
                    variables.insert(line[4], (argument1 >= argument2).to_string());

                } else if line[2] == "<=" {
                    variables.insert(line[4], (argument1 <= argument2).to_string());
                }
            }

        } else if line[0] == "concat" {
            variables.insert(line[3], variables.get(line[1]).unwrap().to_owned() + variables.get(line[2]).unwrap());

        } else if line[0] == "data" {
            variables.insert(line[1], line[2].to_string());

        } else if line[0] == "exit" {
            process::exit(0);

        } else if line[0] == "get_char" {
            let index: usize = variables.get(line[2]).expect("REASON").parse().unwrap();
            variables.insert(line[3], variables.get(line[1]).expect("REASON").as_bytes()[index].to_string());

        } else if line[0] == "jump" {
            line_num = labels.get(line[1]).expect("REASON").parse().unwrap();

        } else if line[0] == "jump_if" {
            if variables.get(line[1]).expect("REASON").parse().unwrap() {
                line_num = labels.get(line[2]).expect("REASON").parse().unwrap();
            }

        } else if line[0] == "jump_if_else" {
            if variables.get(line[1]).expect("REASON").parse().unwrap() {
                line_num = labels.get(line[2]).expect("REASON").parse().unwrap();

            } else {
                line_num = labels.get(line[3]).expect("REASON").parse().unwrap();
            }

        } else if line[0] == "label" {

        } else if line[0] == "move" {
            variables.insert(line[2], variables.get(line[1]).expect("REASON").to_string());

        } else if line[0] == "print" {
            println!("{}", variables.get(line[1]).unwrap());

        } else if line[0] == "read" {
            let mut user_input = String::new();
            stdin.read_line(&mut user_input)?;
            variables.insert(line[1], user_input.split_whitespace().collect::<Vec<&str>>()[0].to_string());

        } else if line[0] == "set_char" {
            let index: usize = variables.get(line[2]).expect("REASON").parse().unwrap();
            let mut bytes: Vec<u8> = variables.get(line[1]).expect("REASON").as_bytes().to_vec();
            bytes[index] = variables.get(line[3]).expect("REASON").as_bytes()[0];
            variables.insert(line[1], String::from_utf8(bytes).expect("REASON"));

        } else {
            println!("error: this instruction does not exist");
        }

        line_num += 1;
    }

    Ok(())
}