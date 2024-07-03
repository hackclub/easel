use crate::ast::{self};

pub fn traverse_print(decl: &ast::Declaration) {
    traverse_print_decl(decl, 0);
}

pub fn traverse_print_decl(decl: &ast::Declaration, indent: usize) {
    match decl {
        ast::Declaration::Statement(stmt) => {
            println!("{}Statement:", " ".repeat(indent));
            traverse_print_stmt(stmt, indent + 1);
        }
        ast::Declaration::Procedure(name, code) => {
            println!("{}Procedure: {}", " ".repeat(indent), name);
            println!("{}Code:", " ".repeat(indent));
            for stmt in code {
                traverse_print_stmt(stmt, indent + 1);
            }
        }
    }
}

pub fn traverse_print_stmt(stmt: &ast::Statement, indent: usize) {
    match stmt {
        ast::Statement::VariableAssignment(name, initializer) => {
            println!("{}Variable: {}", " ".repeat(indent), name);
            println!("{}Value:", " ".repeat(indent));
            traverse_print_expr(initializer, indent + 1);
        }
        ast::Statement::Expression(expr) => {
            println!("{}Expression:", " ".repeat(indent));
            traverse_print_expr(expr, indent + 1);
        }
        ast::Statement::Print(expr) => {
            println!("{}Print:", " ".repeat(indent));
            traverse_print_expr(expr, indent + 1);
        }
        ast::Statement::Block(stmts) => {
            println!("{}Block:", " ".repeat(indent));
            for stmt in stmts {
                traverse_print_stmt(stmt, indent + 1);
            }
        }
        ast::Statement::If(condition, then_branch, else_branch) => {
            println!("{}If:", " ".repeat(indent));
            println!("{}Condition:", " ".repeat(indent + 1));
            traverse_print_expr(condition, indent + 2);
            println!("{}Then:", " ".repeat(indent + 1));
            traverse_print_stmt(then_branch, indent + 2);
            if let Some(else_branch) = else_branch {
                println!("{}Else:", " ".repeat(indent + 1));
                traverse_print_stmt(else_branch, indent + 2);
            }
        }
        ast::Statement::While(condition, body) => {
            println!("{}While:", " ".repeat(indent));
            println!("{}Condition:", " ".repeat(indent + 1));
            traverse_print_expr(condition, indent + 2);
            println!("{}Body:", " ".repeat(indent + 1));
            traverse_print_stmt(body, indent + 2);
        }
        ast::Statement::Range(name, start, end, step, body) => {
            println!("{}Range: {}", " ".repeat(indent), name);
            println!("{}Start:", " ".repeat(indent + 1));
            traverse_print_expr(start, indent + 2);
            println!("{}End:", " ".repeat(indent + 1));
            traverse_print_expr(end, indent + 2);
            println!("{}Step:", " ".repeat(indent + 1));
            traverse_print_expr(step, indent + 2);
            println!("{}Body:", " ".repeat(indent + 1));
            traverse_print_stmt(body, indent + 2);
        }
        ast::Statement::ProcedureCall(name) => {
            println!("{}ProcedureCall: {}", " ".repeat(indent), name);
        }
        ast::Statement::Gamble(expr) => {
            println!("{}Gamble:", " ".repeat(indent));
            traverse_print_expr(expr, indent + 1);
        }
        ast::Statement::Buy(stock, amount) => {
            println!("{}Buy:", " ".repeat(indent));
            println!("{}Stock:", " ".repeat(indent + 1));
            traverse_print_expr(stock, indent + 2);
            println!("{}Amount:", " ".repeat(indent + 1));
            traverse_print_expr(amount, indent + 2);
        }
        ast::Statement::Sell(stock, amount) => {
            println!("{}Sell:", " ".repeat(indent));
            println!("{}Stock:", " ".repeat(indent + 1));
            traverse_print_expr(stock, indent + 2);
            println!("{}Amount:", " ".repeat(indent + 1));
            traverse_print_expr(amount, indent + 2);
        }
        ast::Statement::Loan(amount) => {
            println!("{}Loan:", " ".repeat(indent));
            traverse_print_expr(amount, indent + 1);
        }
        ast::Statement::Pay(amount) => {
            println!("{}Pay:", " ".repeat(indent));
            traverse_print_expr(amount, indent + 1);
        }
        ast::Statement::Work => {
            println!("{}Work", " ".repeat(indent));
        }
    }
}

pub fn traverse_print_expr(expr: &ast::Expression, indent: usize) {
    match expr {
        ast::Expression::Number(value) => {
            println!("{}Number: {}", " ".repeat(indent), value);
        }
        ast::Expression::String(value) => {
            println!("{}String: {}", " ".repeat(indent), value);
        }
        ast::Expression::Boolean(value) => {
            println!("{}Boolean: {}", " ".repeat(indent), value);
        }
        ast::Expression::Void => {
            println!("{}Void", " ".repeat(indent));
        }
        ast::Expression::Variable(name) => {
            println!("{}Variable: {}", " ".repeat(indent), name);
        }
        ast::Expression::ReadonlyVariable(name) => {
            println!("{}ReadonlyVariable: {}", " ".repeat(indent), name);
        }
        ast::Expression::StockPrice(name) => {
            println!("{}StockPrice: {}", " ".repeat(indent), name);
        }
        ast::Expression::Unary(operator, right) => {
            println!("{}Unary: {:?}", " ".repeat(indent), operator);
            traverse_print_expr(right, indent + 1);
        }
        ast::Expression::Binary(operator, left, right) => {
            println!("{}Binary: {:?}", " ".repeat(indent), operator);
            traverse_print_expr(left, indent + 1);
            traverse_print_expr(right, indent + 1);
        }
        ast::Expression::Logical(operator, left, right) => {
            println!("{}Logical: {:?}", " ".repeat(indent), operator);
            traverse_print_expr(left, indent + 1);
            traverse_print_expr(right, indent + 1);
        }
    }
}
