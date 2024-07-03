use crate::lexer::TokenType;

#[derive(Debug, Copy, Clone)]
pub enum UnaryOperator {
    Negate,
    Not,
}

#[derive(Debug, Copy, Clone)]
pub enum BinaryOperator {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,

    // Boolean
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug, Copy, Clone)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Statement(Statement),
    Procedure(String, Vec<Statement>), // Name, Code
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableAssignment(String, Expression), // Name, Value
    Expression(Expression),
    Print(Expression),
    Block(Vec<Statement>),
    If(Expression, Box<Statement>, Option<Box<Statement>>), // Condition, Then, Else
    ProcedureCall(String),                                  // Name
    Gamble(Expression),                                     // Amount to gamble
    Buy(Expression, Expression),                            // Stock, Amount
    Sell(Expression, Expression),                           // Stock, Amount
    Loan(Expression),                                       // Take out loan for amount
    Pay(Expression),                                        // Amount to pay back loan
    While(Expression, Box<Statement>),                      // Condition, Body
    Range(String, Expression, Expression, Expression, Box<Statement>), // Variable name, Start, End, Step, Body
    Work,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    Boolean(bool),
    Void,
    String(String),
    Variable(String),
    ReadonlyVariable(String), // Used internally for economy variables, like @balance
    StockPrice(String),       // Used to access stock prices from inside the program
    Unary(UnaryOperator, Box<Expression>), // Operator, Operand
    Binary(BinaryOperator, Box<Expression>, Box<Expression>), // Operator, Left, Right
    Logical(LogicalOperator, Box<Expression>, Box<Expression>), // Operator, Left, Right
}

impl UnaryOperator {
    pub fn from_tokentype(kind: TokenType) -> Self {
        match kind {
            TokenType::Bang => Self::Not,
            TokenType::Minus => Self::Negate,
            _ => unreachable!("Invalid unary operator"),
        }
    }
}

impl BinaryOperator {
    pub fn from_tokentype(kind: TokenType) -> Self {
        match kind {
            TokenType::Plus => Self::Add,
            TokenType::Minus => Self::Subtract,
            TokenType::Star => Self::Multiply,
            TokenType::Slash => Self::Divide,
            TokenType::Equal => Self::Equal,
            TokenType::BangEqual => Self::NotEqual,
            TokenType::Greater => Self::Greater,
            TokenType::GreaterEqual => Self::GreaterEqual,
            TokenType::Less => Self::Less,
            TokenType::LessEqual => Self::LessEqual,
            _ => unreachable!("Invalid binary operator"),
        }
    }
}

impl LogicalOperator {
    pub fn from_tokentype(kind: TokenType) -> Self {
        match kind {
            TokenType::And => Self::And,
            TokenType::Or => Self::Or,
            _ => unreachable!("Invalid logical operator"),
        }
    }
}
