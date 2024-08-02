#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    Num(f64),
    Bool(bool),
    String(String)
}

#[derive(PartialEq, Clone, Debug)]
pub enum ASTNode {
    ExternNode(Prototype),
    FunctionNode(Function)
}

#[derive(PartialEq, Clone, Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub body: Expression
}

#[derive(PartialEq, Clone, Debug)]
pub struct Prototype {
    pub name: String,
    pub args: Vec<String>
}

#[macro_export]
macro_rules! get_num {
    ( $arg:expr ) => {
        match $arg {
            Value::Num(num) => num,
            _ => return Err(String::from("improper type of args for num"))//make clearer
        }
    };
}

#[macro_export]
macro_rules! get_bool {
    ( $arg:expr ) => {
        match $arg {
            Value::Bool(bool) => bool,
            _ => return Err(String::from("improper type of args for bool"))//make clearer
        }
    };
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    LiteralExpr(Value),
    VariableExpr(String),
    BinaryExpr(String, Box<Expression>, Box<Expression>),
    //cond, then, else
    ConditionalExpr{cond_expr: Box<Expression>, then_expr: Box<Vec<ASTNode>>, else_expr: Option<Box<Expression>>},
    LoopExpr{cond_expr: Box<Expression>, then_expr: Box<Vec<ASTNode>>},
    AssignmentExpr(String, Box<Expression>, bool),
    CallExpr(String, Vec<Expression>),
    DoExpr(Vec<ASTNode>)
}
