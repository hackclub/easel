use crate::{
    ast::*,
    value::Value,
    vm::{OpCode, VM},
};

#[derive(Debug, Clone)]
pub struct Compiler {
    ast: Vec<Declaration>,
    pub vm: VM,
    pub parent: Option<Box<Compiler>>,
}

impl Compiler {
    pub fn new(ast: Vec<Declaration>, vm: VM) -> Self {
        Compiler {
            ast,
            vm,
            parent: None,
        }
    }

    pub fn compile(&mut self) {
        for decl in self.ast.clone() {
            self.declaration(decl);
        }
    }

    fn declaration(&mut self, decl: Declaration) {
        match decl {
            Declaration::Statement(stmt) => self.statement(stmt),
            Declaration::Procedure(name, code) => self.procedure(name, code),
        }
    }

    fn procedure(&mut self, name: String, code: Vec<Statement>) {
        // Create a new compiler for the procedure, with the current compiler as the parent
        let mut compiler = Compiler {
            ast: code
                .into_iter()
                .map(|stmt| Declaration::Statement(stmt))
                .collect(),
            vm: VM::new(),
            parent: Some(Box::new(self.clone())),
        };

        compiler.compile();

        self.vm.constants = compiler.parent.unwrap().vm.constants.clone();
        self.vm.procedures.insert(name, compiler.vm.code);
    }

    fn statement(&mut self, stmt: Statement) {
        match stmt {
            Statement::Print(expr) => {
                self.op_cost(1);
                self.expression(expr);
                self.vm.write_op(OpCode::Print);
            }
            Statement::Expression(expr) => {
                self.expression(expr);
                self.vm.write_op(OpCode::Pop);
            }
            Statement::VariableAssignment(name, expr) => {
                self.op_cost(2);
                self.expression(expr);
                self.vm.write_op(OpCode::SetGlobal(name));
            }
            Statement::Block(stmts) => {
                for stmt in stmts {
                    self.statement(stmt);
                }
            }
            Statement::ProcedureCall(name) => {
                self.op_cost(5);
                self.vm.write_op(OpCode::ProcedureCall(name));
            }
            Statement::If(cond, then_branch, else_branch) => {
                self.op_cost(3);
                self.expression(cond);
                let jump_forward = self.vm.write_op(OpCode::JumpIfFalse(0));
                self.statement(*then_branch);
                // If there is an else branch, we need to jump over it
                // Patch jump_forward to jump to the end of the then branch
                let current_idx = self.vm.code.len();
                self.vm.code[jump_forward] = OpCode::JumpIfFalse(current_idx);
                // If there is an else branch, we need to jump over it
                if let Some(else_branch) = else_branch {
                    // +1 to the jump offset to skip the jump over the else branch
                    match self.vm.code[jump_forward] {
                        OpCode::JumpIfFalse(ref mut offset) => *offset += 1,
                        _ => unreachable!(),
                    }

                    let skip_else = self.vm.write_op(OpCode::Jump(0));
                    self.statement(*else_branch);
                    let current_idx = self.vm.code.len();
                    self.vm.code[skip_else] = OpCode::Jump(current_idx);
                }
            }
            Statement::While(cond, body) => {
                self.op_cost(5);
                let loop_start = self.vm.code.len();
                self.expression(cond);
                let jump_forward = self.vm.write_op(OpCode::JumpIfFalse(0));
                self.statement(*body);
                self.vm.write_op(OpCode::Jump(loop_start));
                let current_idx = self.vm.code.len();
                self.vm.code[jump_forward] = OpCode::JumpIfFalse(current_idx);
            }
            Statement::Range(variable, start, end, step, body) => {
                // Convert the range to a while loop
                // No op_cost here because it transforms to a while loop
                self.expression(start);
                self.vm.write_op(OpCode::SetGlobal(variable.clone()));

                self.statement(Statement::While(
                    Expression::Binary(
                        BinaryOperator::Less,
                        Box::new(Expression::Variable(variable.clone())),
                        Box::new(end),
                    ),
                    Box::new(Statement::Block(vec![
                        *body.clone(),
                        Statement::VariableAssignment(
                            variable.clone(),
                            Expression::Binary(
                                BinaryOperator::Add,
                                Box::new(Expression::Variable(variable)),
                                Box::new(step),
                            ),
                        ),
                    ])),
                ));
            }
            // TODO: Economic statements
            Statement::Buy(name, amount) => {
                self.expression(amount);
                self.expression(name);
                self.vm.write_op(OpCode::Buy);
            }
            Statement::Sell(name, amount) => {
                self.expression(amount);
                self.expression(name);
                self.vm.write_op(OpCode::Sell);
            }
            Statement::Loan(expr) => {
                self.expression(expr);
                self.vm.write_op(OpCode::Loan);
            }
            Statement::Pay(expr) => {
                self.expression(expr);
                self.vm.write_op(OpCode::Repay);
            }
            Statement::Gamble(expr) => {
                self.expression(expr);
                self.vm.write_op(OpCode::Gamble);
            }
            Statement::Work => {
                self.vm.write_op(OpCode::Work);
            }
        }
    }

    fn expression(&mut self, expr: Expression) {
        match expr {
            Expression::Number(n) => {
                let idx = self.write_constant(Value::from_number(n));
                self.vm.write_op(OpCode::Constant(idx));
            }
            Expression::String(s) => {
                let idx = self.write_constant(Value::from_string(s.as_str()));
                self.vm.write_op(OpCode::Constant(idx));
            }
            Expression::Boolean(b) => {
                let idx = self.write_constant(Value::from_boolean(b));
                self.vm.write_op(OpCode::Constant(idx));
            }
            Expression::Variable(name) => {
                self.vm.write_op(OpCode::GetGlobal(name));
            }
            Expression::ReadonlyVariable(name) => {
                self.vm.write_op(OpCode::GetGlobal(name));
            }
            Expression::StockPrice(name) => {
                self.vm.write_op(OpCode::GetStockPrice(name));
            }
            Expression::Unary(op, expr) => {
                self.expression(*expr);
                match op {
                    UnaryOperator::Negate => self.vm.write_op(OpCode::Negate),
                    UnaryOperator::Not => self.vm.write_op(OpCode::Not),
                };
            }
            Expression::Binary(op, left, right) => {
                self.expression(*left);
                self.expression(*right);
                match op {
                    BinaryOperator::Add => self.vm.write_op(OpCode::Add),
                    BinaryOperator::Subtract => self.vm.write_op(OpCode::Subtract),
                    BinaryOperator::Multiply => self.vm.write_op(OpCode::Multiply),
                    BinaryOperator::Divide => self.vm.write_op(OpCode::Divide),
                    BinaryOperator::Equal => self.vm.write_op(OpCode::Equal),
                    BinaryOperator::NotEqual => self.vm.write_op(OpCode::NotEqual),
                    BinaryOperator::Less => self.vm.write_op(OpCode::Less),
                    BinaryOperator::LessEqual => self.vm.write_op(OpCode::LessEqual),
                    BinaryOperator::Greater => self.vm.write_op(OpCode::Greater),
                    BinaryOperator::GreaterEqual => self.vm.write_op(OpCode::GreaterEqual),
                };
            }
            Expression::Logical(op, left, right) => {
                self.expression(*left);
                self.expression(*right);
                match op {
                    LogicalOperator::And => self.vm.write_op(OpCode::And),
                    LogicalOperator::Or => self.vm.write_op(OpCode::Or),
                };
            }
            Expression::Void => {} // TODO: Implement void expression
        }
    }

    pub fn write_constant(&mut self, value: Value) -> usize {
        // Write to parent if exists
        if let Some(parent) = &mut self.parent {
            return parent.write_constant(value);
        } else {
            self.vm.write_constant(value)
        }
    }

    pub fn op_cost(&mut self, cost: i32) {
        self.vm.write_op(OpCode::Cost(cost as f64));
    }
}
