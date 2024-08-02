#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Void,
    String(String),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Number(n) => *n != 0.0,
            Value::Boolean(b) => *b,
            Value::Void => false,
            Value::String(_) => false,
        }
    }

    pub fn format(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Void => "void".to_string(),
            Value::String(s) => s.clone(),
        }
    }

    pub fn from_string(s: &str) -> Value {
        Value::String(s.to_string())
    }

    pub fn from_number(n: f64) -> Value {
        Value::Number(n)
    }

    pub fn from_boolean(b: bool) -> Value {
        Value::Boolean(b)
    }
}
