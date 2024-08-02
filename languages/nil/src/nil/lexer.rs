use regex::Regex;

use crate::nil::errorhandler::Error;
use crate::nil::grammar::Value;
use crate::nil::token::{Token, TokenVal, TypeOf};
use TokenVal::*;

pub fn tokenizer(input: String) -> Result<Vec<Token>, Error> {
    let mut tokens = vec![];

    let mut by_lines: Vec<&str> = input.split("\n").collect();

    by_lines.reverse();

    let mut l = by_lines.len();
    let mut in_segment = false;

    for line in by_lines {
        let mut ended = false;
        let start = match line.find("/*") {
            Some(i) => {
                in_segment = false;
                ended = true;
                i + 2
            }
            None => 0,
        };

        let end = match line.find("*/") {
            Some(i) => {
                in_segment = true;
                i
            }
            None => line.len(),
        };

        if in_segment || ended {
            match &mut tokenize_line(&line[start..end], l) {
                Ok(new) => tokens.append(new),
                Err(err) => {
                    return Err(err.to_owned());
                }
            }
        }
        l -= 1;
    }

    Ok(tokens)
}

fn tokenize_line(line: &str, line_num: usize) -> Result<Vec<Token>, Error> {
    let token_re = Regex::new(concat!(
        r"(?P<string>,(.*?)*,)|",
        r"(?P<ident>\p{Alphabetic}\w*)|",
        r"(?P<number>\d+\.?\d*)|",
        r"(?P<logical>((=|>|<|!)=?|(\|\||&&)))|",
        r"(?P<delimiter>;)|",
        r"(?P<oppar>\()|",
        r"(?P<clpar>\))|",
        r"(?P<opbar>\{)|",
        r"(?P<clbar>\})|",
        r"(?P<operator>\S)"
    ))
    .unwrap();

    let mut temp: Vec<Token> = vec![];

    for caputure in token_re.captures_iter(line) {
        let c = caputure.get(0).unwrap().start();

        let token = if caputure.name("logical").is_some() {
            //println!("{:?}", caputure);
            match caputure.name("logical").unwrap().as_str() {
                "=" => Assignment,
                log => Logical(log.to_owned()),
            }
        } else if caputure.name("string").is_some() {
            let name = caputure.name("string").unwrap().as_str();
            Value(Value::String(name[1..(name.len()-1)].to_owned()))
        } else if caputure.name("ident").is_some() {
            match caputure.name("ident").unwrap().as_str() {
                //use hashmap
                "true" => Value(Value::Bool(true)),
                "false" => Value(Value::Bool(false)),
                "def" => Def,
                "extern" => Extern,
                "noop" => NWhile,
                "nif" => NIf,
                "else" => Else,
                "num" => Type(TypeOf::Num),
                "bool" => Type(TypeOf::Bool),
                "str" => Type(TypeOf::String),
                ident => Ident(ident.to_owned()),
            }
        } else if caputure.name("number").is_some() {
            match caputure.name("number").unwrap().as_str().parse() {
                Ok(number) => Value(Value::Num(number)),
                Err(_) => {
                    return Err(Error::at_mes_pt(
                        "Logical Format Unrecognized",
                        &format!(
                            "Number starting at {}:{} was not able to be parsed",
                            line_num, c
                        ),
                        (line_num, c),
                    ))
                }
            }
        } else if caputure.name("delimiter").is_some() {
            Delimiter
        } else if caputure.name("oppar").is_some() {
            OpeningPars
        } else if caputure.name("clpar").is_some() {
            ClosingPars
        } else if caputure.name("opbar").is_some() {
            OpeningBrac
        } else if caputure.name("clbar").is_some() {
            ClosingBrac
        } else {
            let name = caputure.name("operator").unwrap();
            Operator(name.as_str().to_owned())
        };

        temp.push(Token {
            value: token,
            pos: (line_num, c),
        });
    }

    Ok(temp)
}
