use std::collections::HashMap;

use crate::value::Value;

#[derive(Debug, Clone)]
pub enum OpCode {
    Constant(usize),
    Print,
    GetGlobal(String),
    SetGlobal(String),
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Not,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
    JumpIfFalse(usize),
    JumpForwardIfFalse(isize),
    Jump(usize),
    JumpForward(isize),
    ProcedureCall(String),
    Pop,

    // Economy System
    Cost(f64),
    Gamble,
    Loan,
    Repay,
    Work,
    Buy,
    Sell,
    GetStockPrice(String),
}

#[derive(Debug, Clone)]
pub struct VM {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    globals: HashMap<String, Value>,
    pub procedures: HashMap<String, Vec<OpCode>>,
    stack: Vec<Value>,
    ip: usize,

    // Economy System
    balance: f64,
    debt: f64,
    stock_ownership: HashMap<String, u32>,
    stock_prices: HashMap<String, f64>,
    won_last_gamble: bool,
    op_debt_timer: u32,  // Timer for operations, used for forced debt collection
    op_work_timer: u32,  // Timer for operations, used for forced work
    op_stock_timer: u32, // Timer for operations, used for forced stock trading
    can_work: bool,
}

impl VM {
    pub fn new() -> Self {
        VM {
            code: Vec::new(),
            constants: Vec::new(),
            globals: HashMap::new(),
            procedures: HashMap::new(),
            stack: Vec::new(),
            ip: 0,
            balance: 250.0,
            debt: 0.0,
            stock_ownership: HashMap::new(),
            stock_prices: HashMap::new(),
            won_last_gamble: false,
            op_debt_timer: 0,
            op_work_timer: 0,
            op_stock_timer: 0,
            can_work: true,
        }
    }

    pub fn write_op(&mut self, op: OpCode) -> usize {
        self.code.push(op);
        self.code.len() - 1
    }

    pub fn write_constant(&mut self, value: Value) -> usize {
        for (i, constant) in self.constants.iter().enumerate() {
            if constant == &value {
                return i;
            }
        }

        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn write_procedure(&mut self, name: String, code: Vec<OpCode>) {
        self.procedures.insert(name, code);
    }

    pub fn read_constant(&self, index: usize) -> Value {
        self.constants[index].clone()
    }

    pub fn create_stock(&mut self, name: &String) {
        self.stock_prices
            .insert(name.clone(), rand::random::<f64>() * 100.0);
    }

    pub fn execute(&mut self) {
        while self.ip < self.code.len() {
            self.op_debt_timer += 1;
            self.op_debt_timer %= 20000; // Reset timer every 20000 operations

            if self.op_debt_timer == 0 {
                // Force collection of 5% of debt
                let debt_collection = self.debt * 0.05;
                self.debt -= debt_collection;
                self.balance -= debt_collection;
            }

            self.op_work_timer += 1;
            self.op_work_timer %= 225; // Allow work every 225 operations

            if self.op_work_timer == 0 {
                self.can_work = true;
            }

            self.op_stock_timer += 1;
            self.op_stock_timer %= 1000; // Update stock prices every 1000 operations

            if self.op_stock_timer == 0 {
                for (_, price) in self.stock_prices.iter_mut() {
                    let change = rand::random::<f64>() * 0.1 - 0.05;
                    *price += change;
                }
            }

            // Print the instruction pointer, instruction, and stack
            // println!(
            //     "ip: {}, instruction: {:?}, balance: {:?}, stack: {:?}",
            //     self.ip, self.code[self.ip], self.balance, self.stack
            // );

            // Print the code
            // for (i, instruction) in self.code.iter().enumerate() {
            //     if i == self.ip {
            //         print!("-> ");
            //     } else {
            //         print!("   ");
            //     }

            //     println!("{:?}", instruction);
            // }

            match &self.code[self.ip] {
                OpCode::Constant(index) => {
                    self.stack.push(self.read_constant(*index));
                }
                OpCode::Print => {
                    let value = self.stack.pop().unwrap();
                    println!("{}", value.format());
                }
                OpCode::GetGlobal(name) => match name as &str {
                    "@balance" => {
                        self.stack.push(Value::Number(self.balance));
                    }
                    "@debt" => {
                        self.stack.push(Value::Number(self.debt));
                    }
                    "@won" => {
                        self.stack.push(Value::Boolean(self.won_last_gamble));
                    }
                    "@can_work" => {
                        self.stack.push(Value::Boolean(self.can_work));
                    }
                    _ => {
                        let value = self.globals.get(name);

                        if let Some(value) = value {
                            self.stack.push(value.clone());
                        } else {
                            panic!("Accessing undefined variable '{}'", name);
                        }
                    }
                },
                OpCode::SetGlobal(name) => {
                    let value = self.stack.pop().unwrap();
                    self.globals.insert(name.to_string(), value);
                }
                OpCode::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    if let (Value::Number(a), Value::Number(b)) = (&a, &b) {
                        self.stack.push(Value::Number(a + b));
                    } else if let (Value::String(a), Value::String(b)) = (&a, &b) {
                        self.stack.push(Value::String(format!("{}{}", a, b)));
                    } else if let (Value::String(a), Value::Number(b)) = (&a, &b) {
                        self.stack.push(Value::String(format!("{}{}", a, b)));
                    } else if let (Value::Number(a), Value::String(b)) = (&a, &b) {
                        self.stack.push(Value::String(format!("{}{}", a, b)));
                    } else {
                        if a.is_truthy() {
                            self.stack.push(a);
                        } else {
                            self.stack.push(b);
                        }
                        panic!("Operands must be numbers or strings");
                    }
                }
                OpCode::Subtract => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a - b));
                    } else {
                        panic!("Operands must be numbers");
                    }
                }
                OpCode::Multiply => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a * b));
                    } else {
                        panic!("Operands must be numbers");
                    }
                }
                OpCode::Divide => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a / b));
                    } else {
                        panic!("Operands must be numbers");
                    }
                }
                OpCode::Negate => {
                    let a = self.stack.pop().unwrap();

                    if let Value::Number(a) = a {
                        self.stack.push(Value::Number(-a));
                    } else {
                        panic!("Operand must be a number");
                    }
                }
                OpCode::Not => {
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::from_boolean(!a.is_truthy()));
                }
                OpCode::Equal => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::from_boolean(a == b));
                }
                OpCode::NotEqual => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::from_boolean(a != b));
                }
                OpCode::Greater => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::from_boolean(a > b));
                    } else {
                        panic!("Operands must be numbers");
                    }
                }
                OpCode::GreaterEqual => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::from_boolean(a >= b));
                    } else {
                        panic!("Operands must be numbers");
                    }
                }
                OpCode::Less => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::from_boolean(a < b));
                    } else {
                        panic!("Operands must be numbers");
                    }
                }
                OpCode::LessEqual => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::from_boolean(a <= b));
                    } else {
                        panic!("Operands must be numbers");
                    }
                }
                OpCode::And => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    self.stack
                        .push(Value::from_boolean(a.is_truthy() && b.is_truthy()));
                }
                OpCode::Or => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    self.stack
                        .push(Value::from_boolean(a.is_truthy() || b.is_truthy()));
                }
                OpCode::JumpIfFalse(offset) => {
                    let condition = self.stack.pop().unwrap();

                    if !condition.is_truthy() {
                        self.ip = *offset;
                        continue;
                    }
                }
                OpCode::Jump(offset) => {
                    self.ip = *offset;
                    continue;
                }
                OpCode::JumpForwardIfFalse(offset) => {
                    let condition = self.stack.pop().unwrap();

                    if !condition.is_truthy() {
                        self.ip = (self.ip as isize + *offset) as usize;
                        continue;
                    }
                }
                OpCode::JumpForward(offset) => {
                    self.ip = (self.ip as isize + *offset) as usize;
                    continue;
                }
                OpCode::ProcedureCall(name) => {
                    // Replace the procedure call with the procedure's code
                    let mut procedure = self.procedures.get(name).unwrap().clone();

                    // Modify all the jumps in the procedure to be relative to the current IP
                    for op in procedure.iter_mut() {
                        match op {
                            OpCode::Jump(offset) => {
                                *offset = (*offset as isize + self.ip as isize) as usize;
                            }
                            OpCode::JumpIfFalse(offset) => {
                                *offset = (*offset as isize + self.ip as isize) as usize;
                            }
                            _ => {}
                        }
                    }

                    // Modify all future jumps (jumps pointing to IPs past the call) in the code to be +(procedure length - 1)
                    // This is because the procedure call is replaced with the procedure's code and pushes the IP forward
                    for op in self.code.iter_mut() {
                        match op {
                            OpCode::Jump(offset) => {
                                if *offset > self.ip {
                                    *offset =
                                        (*offset as isize + procedure.len() as isize - 1) as usize;
                                }
                            }
                            OpCode::JumpIfFalse(offset) => {
                                if *offset > self.ip {
                                    *offset =
                                        (*offset as isize + procedure.len() as isize - 1) as usize;
                                }
                            }
                            _ => {}
                        }
                    }

                    self.code
                        .splice(self.ip..self.ip + 1, procedure.iter().cloned());

                    self.ip -= 1;
                }
                OpCode::Pop => {
                    self.stack.pop();
                }
                OpCode::Cost(amount) => {
                    self.balance -= amount;
                }
                OpCode::Gamble => {
                    let amount = self.stack.pop().unwrap();

                    if let Value::Number(amount) = amount {
                        if amount > self.balance {
                            panic!("Insufficient funds to gamble!");
                        }

                        let random = rand::random::<f64>();

                        if random < 0.5 {
                            self.balance += amount;
                            self.won_last_gamble = true;
                        } else {
                            self.balance -= amount;
                            self.won_last_gamble = false;
                        }
                    } else {
                        panic!("Operand must be a number");
                    }
                }
                OpCode::Loan => {
                    let amount = self.stack.pop().unwrap();

                    // A loan can only be taken out for up to (balance - debt) * 5
                    // This is to prevent users from taking out absurd loans

                    let max_loan = (self.balance - self.debt) * 5.0;

                    if let Value::Number(amount) = amount {
                        if amount > max_loan {
                            panic!("Loan amount exceeds maximum loan amount");
                        }

                        self.debt += amount;
                        self.balance += amount;
                    } else {
                        panic!("Operand must be a number");
                    }
                }
                OpCode::Repay => {
                    let amount = self.stack.pop().unwrap();

                    if let Value::Number(amount) = amount {
                        if amount > self.balance {
                            panic!("Insufficient funds to repay loan!");
                        }

                        if amount > self.debt {
                            panic!("Repayment amount exceeds debt");
                        }

                        self.debt -= amount;
                        self.balance -= amount;
                    } else {
                        panic!("Operand must be a number");
                    }
                }
                OpCode::Work => {
                    if self.can_work {
                        self.balance += (self.balance * 0.001).max(100.0); // 0.1% of balance or 100, whichever is greater
                        self.can_work = false;

                        // Sleep for 300ms to simulate work
                        std::thread::sleep(std::time::Duration::from_millis(300));
                    } else {
                        panic!("You are on a work cooldown!");
                    }
                }
                OpCode::Buy => {
                    let name = self.stack.pop().unwrap();
                    let amount = self.stack.pop().unwrap();

                    if let (Value::String(name), Value::Number(amount)) = (name, amount) {
                        // If the stock doesn't exist, create it with a random price
                        if !self.stock_prices.contains_key(&name) {
                            self.create_stock(&name);
                        }

                        let price = self.stock_prices.get(&name).unwrap();

                        if amount * price > self.balance {
                            panic!("Insufficient funds to buy stock!");
                        }

                        self.balance -= amount * price;
                        self.stock_ownership
                            .entry(name.clone())
                            .and_modify(|owned| *owned += amount as u32)
                            .or_insert(amount as u32);
                    }
                }
                OpCode::Sell => {
                    let name = self.stack.pop().unwrap();
                    let amount = self.stack.pop().unwrap();

                    if let (Value::String(name), Value::Number(amount)) = (name, amount) {
                        if !self.stock_prices.contains_key(&name) {
                            panic!("Stock does not exist!");
                        }

                        let price = self.stock_prices.get(&name).unwrap();

                        if !self.stock_ownership.contains_key(&name) {
                            panic!("You do not own any of this stock!");
                        }

                        let owned = self.stock_ownership.get(&name).unwrap();

                        if amount > *owned as f64 {
                            panic!("You do not own enough of this stock!");
                        }

                        self.balance += amount * price;
                        self.stock_ownership
                            .entry(name.clone())
                            .and_modify(|owned| *owned -= amount as u32);
                    }
                }
                OpCode::GetStockPrice(name) => {
                    let name = &name.clone(); // Fixes borrow checker issue

                    if !self.stock_prices.contains_key(name) {
                        self.create_stock(name);
                    }

                    let price = self.stock_prices.get(name).unwrap();
                    self.stack.push(Value::Number(*price));
                }
            }

            if self.balance <= 0.0 {
                panic!("Insufficient funds!");
            }

            self.ip += 1;
        }
    }
}
