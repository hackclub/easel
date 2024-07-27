use std::io;
use std::io::Write;
use std::collections::HashMap;
use crate::{get_num, get_bool};

use crate::nil::grammar::Value;
use crate::nil::errorhandler::Error;

/*macro_rules! check_args {
    ( $parms:expr, $types:expr ) => {
        if $parms.len() != $types.len() {
            return Err(String::from("Incorrect Number of Args"));
        }
        for i in 0..$parms.len() {
            if $parms[i] != $types[i] {
                return Err(String::from("missing proper types"))
            }
        }
    };
}*/

type Callback = fn(Vec<Value>) -> Result<Value, String>;
//user def fn call speial function that calls eval

pub struct SpecialForms {
    pub map: HashMap<String, Callback>
}

impl SpecialForms {
    pub fn new() -> Self {
        let mut temp: HashMap<String, Callback> = HashMap::new();

        temp.insert(String::from("+"), add);
        temp.insert(String::from("-"), sub);
        temp.insert(String::from("*"), mul);
        temp.insert(String::from("/"), div);
        temp.insert(String::from("%"), rim);
        
        temp.insert(String::from("||"), and);
        temp.insert(String::from("&&"), or);
        temp.insert(String::from("=="), equal);
        temp.insert(String::from("!="), nequal);
        temp.insert(String::from(">"), more);
        temp.insert(String::from(">="), moreequal);
        temp.insert(String::from("<="), lessequal);
        temp.insert(String::from("<"), less);

        temp.insert(String::from("output"), output);
        temp.insert(String::from("num_input"), num_input);
        temp.insert(String::from("str_input"), str_input);
        temp.insert(String::from("round"), round);

        SpecialForms {map: temp}
    }
}

//----------------------- Bulit-In Functions -----------------------

fn add(args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Num(get_num!(args[0]) + get_num!(args[1])))
}

fn sub(args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Num(get_num!(args[0]) - get_num!(args[1])))
}

fn mul(args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Num(get_num!(args[0]) * get_num!(args[1])))
}

fn div(args: Vec<Value>) -> Result<Value, String> {
    //let parms = vec!(Value::Num, Value::Num);
    //let Value::Num(num1) = args[0];
    //check_args!(args, parms);
    if get_num!(args[1]) == 0.0 {
        return Err(String::from("Div by zero"));
    }
    Ok(Value::Num(get_num!(args[0]) / get_num!(args[1])))
}

fn rim(args: Vec<Value>) -> Result<Value, String> {
    //let parms = vec!(Value::Num, Value::Num);
    //let Value::Num(num1) = args[0];
    //check_args!(args, parms);
    if get_num!(args[1]) == 0.0 {
        return Err(String::from("Div by zero"));
    }
    Ok(Value::Num(get_num!(args[0]) % get_num!(args[1])))
}

fn equal(args: Vec<Value>) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (&Value::Num(ref v1), &Value::Num(ref v2)) => Ok(Value::Bool(v1 == v2)),
        (&Value::String(ref v1), &Value::String(ref v2)) => Ok(Value::Bool(v1 == v2)),
        (&Value::Bool(ref v1), &Value::Bool(ref v2)) => Ok(Value::Bool(v1 == v2)),
        _ => Ok(Value::Bool(false))
    }
}
fn nequal(args: Vec<Value>) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (&Value::Num(ref v1), &Value::Num(ref v2)) => Ok(Value::Bool(v1 != v2)),
        (&Value::String(ref v1), &Value::String(ref v2)) => Ok(Value::Bool(v1 != v2)),
        (&Value::Bool(ref v1), &Value::Bool(ref v2)) => Ok(Value::Bool(v1 != v2)),
        _ => Ok(Value::Bool(true))
    }
}
fn more(args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Bool(get_num!(args[0]) > get_num!(args[1])))
}
fn moreequal(args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Bool(get_num!(args[0]) >= get_num!(args[1])))
}
fn less(args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Bool(get_num!(args[0]) < get_num!(args[1])))
}
fn lessequal(args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Bool(get_num!(args[0]) <= get_num!(args[1])))
}

fn and(args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Bool(get_bool!(args[0]) && get_bool!(args[1])))
}

fn or(args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Bool(get_bool!(args[0]) || get_bool!(args[1])))
}

fn output(args: Vec<Value>) -> Result<Value, String> {
    let mut out: Vec<char> = vec![];
    for i in 0..args.len() {
        //let str_val = args[i].to_string();
        let str_val = match &args[i] {
            Value::Num(num) => num.to_string(),
            Value::String(str) => str.clone(),
            Value::Bool(bool) => if *bool {String::from("True")} else {String::from("False")}
        };
        
        out.append(&mut str_val.chars().collect());
        if i != args.len()-1 {
            out.push(' ');
        }
    }

    println!("{}", out.into_iter().collect::<String>());
    Ok(Value::Bool(true))
}

fn str_input(args: Vec<Value>) -> Result<Value, String> {
    let mut input = String::new();

    match args.first() {
        Some(Value::String(val)) => {
            print!("{}", val);
            io::stdout().flush().unwrap();
        },
        _ => {}
    }

    std::io::stdin().read_line(&mut input).unwrap();

    Ok(Value::String(input.replace("\n","")))
}

fn num_input(args: Vec<Value>) -> Result<Value, String> {
    let mut input = String::new();

    match args.first() {
        Some(Value::String(val)) => {
            print!("{}", val);
            io::stdout().flush().unwrap();
        },
        _ => {}
    }
    
    std::io::stdin().read_line(&mut input).unwrap();

    match input.replace("\n","").parse() {
        Ok(num) => Ok(Value::Num(num)),
        Err(_) => Err(String::from("Unable to convert input to Number"))
    }
}

fn round(args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Num(get_num!(args[0]).round()))
}
