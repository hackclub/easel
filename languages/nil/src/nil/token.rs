use std::fmt;

use crate::nil::grammar::Value;

pub struct Token {
    pub value: TokenVal,
    pub pos: (usize, usize),
}

#[derive(PartialEq, Clone, Debug)]
pub enum TypeOf {
    Num,
    Bool,
    String,
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokenVal {
    Delimiter, //; char
    OpeningPars,
    ClosingPars,
    OpeningBrac,
    ClosingBrac,
    Def,
    Extern,
    NIf,
    NWhile,
    Else,
    Assignment,
    Ident(String),
    Value(Value),
    Operator(String),
    Type(TypeOf),
    Logical(String),
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.value {
            TokenVal::Delimiter => write!(f, "Delimiter"),
            TokenVal::OpeningPars => write!(f, "OpeningPars"),
            TokenVal::ClosingPars => write!(f, "ClosingPars"),
            TokenVal::OpeningBrac => write!(f, "OpeningBrac"),
            TokenVal::ClosingBrac => write!(f, "ClosingBrac"),
            TokenVal::Def => write!(f, "Def"),
            TokenVal::Extern => write!(f, "Extern"),
            TokenVal::NIf => write!(f, "NIf"),
            TokenVal::NWhile => write!(f, "NWhile"),
            TokenVal::Else => write!(f, "Else"),
            TokenVal::Assignment => write!(f, "Assignment"),
            TokenVal::Ident(str) => write!(f, "Ident({})", str),
            TokenVal::Value(val) => write!(f, "Val({})",
                match val {
                    Value::Num(v) => v.to_string(),
                    Value::Bool(bool) => bool.to_string(),
                    Value::String(str) => String::from("\"") + str + "\"",
                }
            ),
            TokenVal::Operator(str) => write!(f, "Op({})", str),
            TokenVal::Type(type_of) => write!(f, "Type({})",
                match type_of {
                    TypeOf::Num => "Num",
                    TypeOf::Bool => "Bool",
                    TypeOf::String => "String",
                }
            ),
            TokenVal::Logical(str) => write!(f, "Logi({})", str),
        }
        //write!(f, "Hi: {}", self.id)
    }
}
