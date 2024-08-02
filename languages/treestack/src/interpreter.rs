use crate::error::{Positioned, RangeError};
use crate::lexer::{PointerAction, Token};
use crate::parser::Node;
use crate::tree::TreeNode;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use fehler::throws;
use rand::Rng;
use std::collections::HashMap;
use std::io::{Read, stdout, Write};
use std::ops::{self, Range};
use std::time::Duration;
#[cfg(target_os = "linux")]
use syscalls::{raw_syscall, Sysno};

type Error = RangeError;

#[derive(Default, Clone, Debug)]
struct Pointer {
    pub tree: Vec<usize>,
    pub branch: usize,
}

impl Pointer {
    pub fn open_branch(&mut self, len: usize) {
        self.tree.push(self.branch);
        self.branch = len;
    }

    pub fn close_branch(&mut self) {
        self.branch = self.tree.pop().unwrap(); // Error
    }
}

#[derive(Default)]
pub struct Interpreter {
    stack: TreeNode<i64>,
    functions: HashMap<String, Vec<Positioned<Node>>>,
    pointer: Pointer,
    pointers: HashMap<String, Pointer>,
    debug: bool,
    range: Range<usize>,
    brk: bool,
}

impl Interpreter {
    pub fn new(debug: bool) -> Self {
        Self { debug, ..Default::default() }
    }

    pub fn parse(&mut self, instructions: Vec<Positioned<Node>>) -> Result<(), RangeError> {
        for instruction in instructions.into_iter() {
            self.range = instruction.range;
            let inst = format!("{:?}: ", instruction.inner);
            match instruction.inner {
                Node::Push(u) => self.push_raw(u),
                Node::Return => return Ok(()),
                Node::Break => { self.brk = true; break; }
                Node::Continue => { break; }
                Node::Block(s) => self.push_string(s),
                Node::String(string) => self.push_string(string),
                Node::Operator(op) => self.eval_op(op.clone())?,
                Node::Call(call) => {
                    match self.functions.get(&call) {
                        Some(f) => self.parse(f.clone())?,
                        None => self.call(&call)?,
                    };
                }
                Node::While(expr) => {
                    while self.truthy() {
                        if self.brk { self.brk = false; break; }
                        self.parse(expr.clone())?
                    }
                }
                Node::If(if_expr, else_expr) => {
                    if self.truthy() {
                        self.parse(if_expr.clone())?
                    } else {
                        if let Some(expr) = else_expr {
                            self.parse(expr.clone())?;
                        }
                    }
                }
                Node::Function(name, f) => {
                    self.functions.insert(name, f);
                }
                Node::Pointer(name, action) => self.call_pointer(name, action)?,
            }

            if self.debug {
                let pointer = self.pointer.clone();
                println!("{inst}: {}, {:?}", self.current(), pointer);
            }
        }

        Ok(())
    }

    pub fn call(&mut self, call: &str) -> Result<(), RangeError> {
        match call {
            "swap" => {
                let first = self.pop()?;
                let second = self.pop()?;
                self.push(first);
                self.push(second);
            }
            "dup" => {
                let first = self.on()?.clone();
                self.push(first);
            }
            "read" => {
                let file = self.pop_string()?;
                let contents = std::fs::read_to_string(file).unwrap();
                self.push_string(contents);
            }
            "write" => {
                let file = self.pop_string()?;
                let to_write = self.pop_string()?;
                let error = |e: std::io::Error| {
                    self.error::<()>(&format!("Writing to file failed {e}")).unwrap_err()
                };
                std::fs::write(file, to_write).map_err(|e| error(e))?;
            }
            "syscall" => {
                let call = self.pop()?.val;
                self.push_raw(syscall(call));
            }
            "shear" => self.on()?.children.clear(),
            "empty" => {
                self.current().children.clear();
                self.pointer.branch = 0;
            }
            "flush" => stdout().flush().unwrap(),
            "drop" => {
                self.pop()?;
            }
            "abs" => {
                let val = self.on()?.val;
                self.on()?.val = val.abs();
            }
            "over" => {
                let second = self.before()?;
                self.push(second);
            }
            "concat" => {
                let first = self.pop()?;
                let second = self.pop()?;
                self.push(TreeNode {
                    val: first.val + second.val,
                    children: [first.children, second.children].concat(),
                })
            }
            "map" => {
                let program = self.pop_string()?; // change to
                let ast = crate::compile_ast(program, self.debug)?;
                let start_pointer = self.pointer.clone();
                let mut current_offset = 0;

                while self.pointer.branch > 0 {
                    self.parse(ast.clone())?;
                    current_offset += 1;
                    self.pointer = start_pointer.clone();
                    self.pointer.branch -= current_offset;
                }

                self.pointer = start_pointer.clone();
            }
            "filter" => {
                let program = self.pop_string()?; // change to
                let ast = crate::compile_ast(program, self.debug)?;
                let start_pointer = self.pointer.clone();
                let mut current_offset = 0;
                let mut popped = 0;

                while self.pointer.branch > 0 {
                    let current = self.on()?.clone();
                    self.push(current);
                    self.parse(ast.clone())?;
                    let truthy = self.truthy();
                    self.pop()?;

                    current_offset += 1;

                    if !truthy {
                        popped += 1;
                        self.pop()?;
                    }

                    self.pointer = start_pointer.clone();
                    self.pointer.branch -= current_offset;
                }

                self.pointer = start_pointer.clone();
                self.pointer.branch -= popped;
            }
            "ifthen" => {

            }
            "dowhile" => {
                let while_expr = self.pop_string()?;
                let while_ast = crate::compile_ast(while_expr, self.debug)?;
                let do_expr = self.pop_string()?;
                let do_ast = crate::compile_ast(do_expr, self.debug)?;

                self.parse(while_ast.clone())?;
                while self.truthy() {
                    self.parse(do_ast.clone())?;
                    self.parse(while_ast.clone())?;
                }
            }
            "match" => {

            }
            "recmap" => {}
            "range" => {
                let max = self.pop()?;
                let min = self.pop()?;

                for i in min.val..=max.val {
                    self.push_raw(i);
                }
            }
            "print" => print!("{}", self.pop_string()?),
            "group" => {
                let length = self.pop()?.val;
                let children: Result<Vec<TreeNode<i64>>, Error> =
                    (0..length).map(|_| self.pop()).collect();
                let children = children?.into_iter().rev().collect();
                self.push(TreeNode { val: length, children })
            }
            "flatten" => {
                let current = self.current().clone();
                let old_branch = current.len();
                let flattened = current.flatten();
                self.current().children = flattened;
                let new_len = self.current().len();
                self.pointer.branch += new_len - old_branch;
            }
            "left" => {
                let left = self.pointer.branch;
                self.push_raw(left as i64);
            } 
            "size" => {
                let size = self.current().len();
                self.push_raw(size as i64);
            } 
            "rotate" => {
                let amount = self.pop()?.val as usize;
                let vec = self.current().children.clone();
                self.current().children = rotate_vec_slice(vec, amount)
            }
            "in" => {
                self.pointer.branch = self.current().len();
            }
            "rev" => {
                let rev_children = self.current().children.clone().into_iter().rev().collect();
                self.current().children = rev_children;
            }
            "eval" => {
                let program = self.pop_string()?;
                let ast = crate::compile_ast(program, self.debug)?;
                self.parse(ast)?;
            }
            "random" => {
                let max = self.pop()?.val;
                let min = self.pop()?.val;
                let random_no: i64 = rand::thread_rng().gen_range(min..max);
                self.push_raw(random_no)
            }
            "true" => self.push_raw(1),
            "false" => self.push_raw(1),
            "sleep" => std::thread::sleep(Duration::from_millis(self.pop()?.val as u64)),
            "rawmode" => {
                if self.truthy() {
                    enable_raw_mode()
                        .map_err(|_| self.error::<()>("Failed to enter raw mode").unwrap_err())?;
                } else {
                    disable_raw_mode()
                        .map_err(|_| self.error::<()>("Failed to exit raw mode").unwrap_err())?;
                }
            }
            _ => self.error("Function not found")?,
        };

        Ok(())
    }

    #[throws]
    pub fn pop_string(&mut self) -> String {
        let children = self.pop()?.children;
        let string: Option<String> =
            children.iter().map(|i| char::from_u32(i.val as u32)).collect();
        string.ok_or(self.error::<String>("Failed to parse string").unwrap_err())?
    }

    #[throws]
    fn call_pointer(&mut self, name: String, action: PointerAction) {
        let error = self.error::<()>(&format!("No pointer named {name}")).unwrap_err();
        match action {
            PointerAction::Jump => {
                self.pointer = self.pointers.get(&name).ok_or_else(|| error)?.clone();
            }
            PointerAction::Create => {
                self.pointers.insert(name, self.pointer.clone());
            }
            PointerAction::Push => {
                let pointer = self.pointers.get(&name).ok_or_else(|| error)?.clone();
                if !self.is_pointer_valid(&pointer) {
                    return;
                } // Error
                let value = self.at_pointer(pointer.clone()).children[pointer.branch - 1].clone();
                self.push(value)
            }
        }
    }

    fn is_pointer_valid(&mut self, pointer: &Pointer) -> bool {
        if pointer.branch == 0 {
            return false;
        }
        if self.current().children.len() < pointer.branch {
            return false;
        }
        true
    }

    fn current(&mut self) -> &mut TreeNode<i64> {
        self.at_pointer(self.pointer.clone())
    }

    fn at_pointer(&mut self, pointer: Pointer) -> &mut TreeNode<i64> {
        let mut head = &mut self.stack;
        for pointer in &pointer.tree {
            // add valid pointer checks
            head = &mut head.children[*pointer - 1];
        }
        head
    }

    fn truthy(&mut self) -> bool {
        let branch = self.pointer.branch;
        if branch == 0 || branch > self.current().len() {
            return false;
        }
        self.current()[branch - 1].val > 0
    }

    fn push_raw(&mut self, val: i64) {
        self.push(TreeNode { val, children: Vec::new() });
    }

    fn push_string(&mut self, string: String) {
        let length = string.len() as i64;
        let children = string.chars().map(|c| TreeNode::new(c as i64)).collect();
        self.push(TreeNode { val: length, children });
    }

    pub fn push(&mut self, node: TreeNode<i64>) {
        let branch = self.pointer.branch;
        if branch <= self.current().len() {
            self.current().insert(branch, node);
        } else {
            self.current().push(node)
        }
        self.pointer.branch += 1;
    }

    #[throws]
    pub fn eval_op(&mut self, op: Token) {
        use Token::*;
        match &op {
            Period => print!("{}", self.pop()?),
            Comma => print!("{}", char::from_u32(self.pop()?.val as u32).unwrap()),
            OpenBracket => {
                let branch = self.pointer.branch;
                if branch == 0 || branch > self.current().len() {
                    self.error("Tried opening non-existant branch")?;
                }
                let len = self.current()[branch - 1].len();
                self.pointer.open_branch(len);
            }
            CloseBracket => self.pointer.close_branch(),
            OpenParen => {
                if self.pointer.branch == 0 {
                    self.error("Cannot move further inwards on branch")?;
                }
                self.pointer.branch -= 1;
            }
            CloseParen => {
                if self.pointer.branch >= self.current().len() {
                    self.error("Pointer fell off branch")?;
                }
                self.pointer.branch += 1;
            }
            PlusPlus => {
                self.on()?.val += 1;
            }
            MinusMinus => {
                self.on()?.val -= 1;
            }
            Not => {
                let mut item = self.pop()?;
                item.val = (item.val == 0) as i64;
                self.push(item);
            },
            Grave => {
                self.pop()?;
            }
            Question => {
                let mut chars = [0; 1];
                let _ = std::io::stdin().read(&mut chars);
                self.push_raw(chars[0] as i64);
            }
            _ => {
                let rhs = self.pop()?;
                let lhs = self.pop()?;
                let func = op.func();
                self.push(lhs.eval(rhs, func));
            }
        }
    }

    pub fn pop(&mut self) -> Result<TreeNode<i64>, Error> {
        let branch = self.pointer.branch;
        if self.current().children.is_empty() || branch == 0 {
            return self.error("Stack underflow");
        }
        if self.current().len() < branch {
            return self.error("Cannot pop due to reasons sorry </3");
        }
        let value = self.current().remove(branch - 1);
        self.pointer.branch -= 1;
        Ok(value)
    }

    pub fn on(&mut self) -> Result<&mut TreeNode<i64>, Error> {
        let branch = self.pointer.branch;
        self.get_child(branch)
    }

    pub fn get_child(&mut self, position: usize) -> Result<&mut TreeNode<i64>, Error> {
        if self.current().children.is_empty() || position == 0 {
            return self.error("Stack underflow");
        }
        if self.current().len() < position {
            return self.error("Tried to reach item outside stack");
        }
        let value = &mut self.current()[position - 1];
        Ok(value)
    }

    pub fn before(&mut self) -> Result<TreeNode<i64>, Error> {
        let branch = self.pointer.branch;
        Ok(self.get_child(branch - 1)?.clone())
    }

    pub fn error<T>(&self, msg: &str) -> Result<T, Error> {
        Err(RangeError { message: msg.to_string(), range: self.range.clone() })
    }
}

impl Token {
    pub fn func(&self) -> fn(i64, i64) -> i64 {
        use Token::*;
        match self {
            Plus => ops::Add::add,
            Asterisk => ops::Mul::mul,
            Minus => ops::Sub::sub,
            Slash => ops::Div::div,
            Percent => ops::Rem::rem,
            And => |l, r| (l > 0 && r > 0) as i64,
            Or => |l, r| (l > 0 || r > 0) as i64,
            Equals => |l, r| (l == r) as i64,
            Greater => |l, r| (l > r) as i64,
            Lesser => |l, r| (l < r) as i64,
            GreaterThan => |l, r| (l >= r) as i64,
            LesserThan => |l, r| (l <= r) as i64,
            _ => panic!("Operator not implemented {:?}", self),
        }
    }
}

fn rotate_vec_slice(mut vec: Vec<TreeNode<i64>>, amount: usize) -> Vec<TreeNode<i64>> {
    let len = vec.len();
    let new_amount = amount % len; // Handle rotations greater than vector length
    let first_part = vec.drain(..new_amount).collect::<Vec<TreeNode<i64>>>();
    let second_part = vec;
    [second_part, first_part].concat()
}

#[cfg(target_os = "linux")]
fn syscall(call: i64) -> i64 {
    unsafe {
        let result = raw_syscall!(Sysno::from(call as i32));
        result as i64
    }
}

#[cfg(not(target_os = "linux"))]
fn syscall(call: i64) -> i64 {
    -1
}
