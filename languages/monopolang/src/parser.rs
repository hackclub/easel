use crate::{
    ast::{BinaryOperator, Declaration, Expression, LogicalOperator, Statement, UnaryOperator},
    lexer::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::new();

        while !self.is_at_end() {
            declarations.push(self.declaration());
        }

        declarations
    }

    pub fn declaration(&mut self) -> Declaration {
        if self.match_token(TokenType::Procedure) {
            self.procedure_declaration()
        } else {
            Declaration::Statement(self.statement())
        }
    }

    pub fn procedure_declaration(&mut self) -> Declaration {
        let name = self
            .consume(TokenType::Identifier, "Expected procedure name")
            .lexeme;

        self.consume(TokenType::Do, "Expected 'do' after procedure name");

        let code = self.block();

        Declaration::Procedure(name, code)
    }

    pub fn statement(&mut self) -> Statement {
        match self.peek().kind {
            TokenType::Set => self.variable_assignment_statement(),
            TokenType::Print => self.print_statement(),
            TokenType::If => self.if_statement(),
            TokenType::While => self.while_statement(),
            TokenType::Range => self.range_statement(),
            TokenType::Call => self.procedure_call_statement(),
            TokenType::Gamble => self.gamble_statement(),
            TokenType::Buy => self.buy_statement(),
            TokenType::Sell => self.sell_statement(),
            TokenType::Loan => self.loan_statement(),
            TokenType::Repay => self.pay_statement(),
            TokenType::Work => self.work_statement(),
            _ => Statement::Expression(self.expression()),
        }
    }

    pub fn variable_assignment_statement(&mut self) -> Statement {
        self.advance();

        // If the next token is an at, it's a readonly variable and we should use a specialized error message
        if self.check(TokenType::At) {
            self.error("Cannot declare readonly variable");
        }

        let name = self
            .consume(TokenType::Identifier, "Expected variable name")
            .lexeme;
        self.consume(TokenType::Arrow, "Expected '->' after variable name");

        let initializer = self.expression();

        Statement::VariableAssignment(name, initializer)
    }

    pub fn print_statement(&mut self) -> Statement {
        self.advance();
        let value = self.expression();

        Statement::Print(value)
    }

    pub fn if_statement(&mut self) -> Statement {
        self.advance();

        let condition = self.expression();

        self.consume(TokenType::Then, "Expected 'then' after if condition");

        let then_branch = Box::new(Statement::Block(self.if_block()));
        let mut else_branch: Option<Box<Statement>> = None;

        // If previous token was an 'else', we have an else branch
        // Previous token and not current because block consumes the 'else' token
        if self.previous().kind == TokenType::Else {
            else_branch = Some(Box::new(Statement::Block(self.block())))
        }

        Statement::If(condition, then_branch, else_branch)
    }

    pub fn while_statement(&mut self) -> Statement {
        self.advance();

        let condition = self.expression();

        self.consume(TokenType::Do, "Expected 'do' after while condition");

        Statement::While(condition, Box::new(Statement::Block(self.block())))
    }

    pub fn range_statement(&mut self) -> Statement {
        self.advance();

        let name = self
            .consume(TokenType::Identifier, "Expected variable name")
            .lexeme;

        self.consume(TokenType::From, "Expected 'from' after variable name");

        let start = self.expression();

        self.consume(TokenType::To, "Expected 'to' after range start");

        let end = self.expression();

        let step = if self.match_token(TokenType::By) {
            self.expression()
        } else {
            Expression::Number(1.0)
        };

        self.consume(TokenType::Do, "Expected 'do' after range");

        Statement::Range(
            name,
            start,
            end,
            step,
            Box::new(Statement::Block(self.block())),
        )
    }

    pub fn procedure_call_statement(&mut self) -> Statement {
        self.advance();
        let name = self
            .consume(TokenType::Identifier, "Expected procedure name")
            .lexeme;

        Statement::ProcedureCall(name)
    }

    pub fn gamble_statement(&mut self) -> Statement {
        self.advance();
        let value = self.expression();

        Statement::Gamble(value)
    }

    pub fn buy_statement(&mut self) -> Statement {
        self.advance();
        let stock = self.expression();
        let amount = self.expression();

        Statement::Buy(stock, amount)
    }

    pub fn sell_statement(&mut self) -> Statement {
        self.advance();
        let stock = self.expression();
        let amount = self.expression();

        Statement::Sell(stock, amount)
    }

    pub fn loan_statement(&mut self) -> Statement {
        self.advance();
        let amount = self.expression();

        Statement::Loan(amount)
    }

    pub fn pay_statement(&mut self) -> Statement {
        self.advance();
        let amount = self.expression();

        Statement::Pay(amount)
    }

    pub fn work_statement(&mut self) -> Statement {
        self.advance();

        Statement::Work
    }

    pub fn block(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while !self.check(TokenType::End) && !self.is_at_end() {
            statements.push(self.statement());
        }

        self.consume(TokenType::End, "Expected 'end' after block");

        statements
    }

    pub fn if_block(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while !self.check(TokenType::End) && !self.check(TokenType::Else) && !self.is_at_end() {
            statements.push(self.statement());
        }

        if !self.check(TokenType::Else) && !self.check(TokenType::End) {
            self.error("Expected 'else' or 'end' after if block");
        }

        self.advance();

        statements
    }

    pub fn expression(&mut self) -> Expression {
        self.or_expression()
    }

    pub fn or_expression(&mut self) -> Expression {
        let mut expr = self.and_expression();

        while self.match_token(TokenType::Or) {
            let operator = self.previous().kind;
            let right = self.and_expression();
            expr = Expression::Logical(
                LogicalOperator::from_tokentype(operator),
                Box::new(expr),
                Box::new(right),
            );
        }

        expr
    }

    pub fn and_expression(&mut self) -> Expression {
        let mut expr = self.equality();

        while self.match_token(TokenType::And) {
            let operator = self.previous().kind;
            let right = self.equality();
            expr = Expression::Logical(
                LogicalOperator::from_tokentype(operator),
                Box::new(expr),
                Box::new(right),
            );
        }

        expr
    }

    pub fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();

        while self.match_token(TokenType::Equal) || self.match_token(TokenType::BangEqual) {
            let operator = self.previous().kind;
            let right = self.comparison();
            expr = Expression::Binary(
                BinaryOperator::from_tokentype(operator),
                Box::new(expr),
                Box::new(right),
            );
        }

        expr
    }

    pub fn comparison(&mut self) -> Expression {
        let mut expr = self.term();

        while self.match_token(TokenType::Greater)
            || self.match_token(TokenType::GreaterEqual)
            || self.match_token(TokenType::Less)
            || self.match_token(TokenType::LessEqual)
        {
            let operator = self.previous().kind;
            let right = self.term();
            expr = Expression::Binary(
                BinaryOperator::from_tokentype(operator),
                Box::new(expr),
                Box::new(right),
            );
        }

        expr
    }

    pub fn term(&mut self) -> Expression {
        let mut expr = self.factor();

        while self.match_token(TokenType::Minus) || self.match_token(TokenType::Plus) {
            let operator = self.previous().kind;
            let right = self.factor();
            expr = Expression::Binary(
                BinaryOperator::from_tokentype(operator),
                Box::new(expr),
                Box::new(right),
            );
        }

        expr
    }

    pub fn factor(&mut self) -> Expression {
        let mut expr = self.unary();

        while self.match_token(TokenType::Slash) || self.match_token(TokenType::Star) {
            let operator = self.previous().kind;
            let right = self.unary();
            expr = Expression::Binary(
                BinaryOperator::from_tokentype(operator),
                Box::new(expr),
                Box::new(right),
            );
        }

        expr
    }

    pub fn unary(&mut self) -> Expression {
        if self.match_token(TokenType::Bang) || self.match_token(TokenType::Minus) {
            let operator = self.previous().kind;
            let right = self.unary();
            Expression::Unary(UnaryOperator::from_tokentype(operator), Box::new(right))
        } else {
            self.primary()
        }
    }

    pub fn primary(&mut self) -> Expression {
        if self.match_token(TokenType::False) {
            Expression::Boolean(false)
        } else if self.match_token(TokenType::True) {
            Expression::Boolean(true)
        } else if self.match_token(TokenType::Number) {
            Expression::Number(self.previous().lexeme.parse().unwrap())
        } else if self.match_token(TokenType::String) {
            Expression::String(self.previous().lexeme.clone())
        } else if self.match_token(TokenType::Identifier) {
            Expression::Variable(self.previous().lexeme.clone())
        } else if self.match_token(TokenType::At) {
            let name = self
                .consume(TokenType::Identifier, "Expected identifier after '@'")
                .lexeme;
            Expression::ReadonlyVariable("@".to_string() + &name)
        } else if self.match_token(TokenType::Dollar) {
            let name = self
                .consume(TokenType::Identifier, "Expected identifier after '$'")
                .lexeme;
            Expression::StockPrice(name)
        } else if self.match_token(TokenType::LeftParen) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expected ')' after expression");
            expr
        } else {
            self.error("Expected expression");
        }
    }

    pub fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    pub fn check(&self, kind: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.tokens[self.current].kind == kind
        }
    }

    pub fn consume(&mut self, kind: TokenType, message: &str) -> Token {
        if self.check(kind) {
            self.advance()
        } else {
            self.error(message);
        }
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn match_token(&mut self, kind: TokenType) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.tokens[self.current - 1].clone()
    }

    pub fn is_at_end(&self) -> bool {
        self.tokens[self.current].kind == TokenType::Eof
    }

    pub fn error(&self, message: &str) -> ! {
        let token = &self.tokens[self.current];

        if token.kind == TokenType::Eof {
            panic!("Error at end: {}", message);
        } else {
            panic!(
                "Error at <{}:{}> | '{}': {}",
                token.line, token.column, token.lexeme, message
            );
        }
    }
}
