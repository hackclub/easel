use crate::get_bool;
use crate::nil::grammar::*;
use crate::nil::specialforms::SpecialForms;
use crate::nil::scope::Scope;
use Expression::*;

macro_rules! get_result {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(err) => return Err(err),
        }
    };
}

pub fn eval_ast(ast: Vec<ASTNode>) {
    let specialforms = SpecialForms::new();
    let mut scope = Scope::new();

    for node in ast {
        eval(node, &specialforms, &mut scope);
    }
}

fn eval(node: ASTNode, sp: &SpecialForms, scope: &mut Scope) -> Value {
    match node {
            ASTNode::FunctionNode(fun) => {
                if fun.prototype.name == "" {
                    
                    return match eval_expression(&sp, scope, fun.body) {
                        Ok(val) => val,
                        Err(err) => error(err)
                    }
                } else { //Named protype add to scope
                    scope.funs.insert(fun.prototype.name.clone(), fun);
                }
            },
            ASTNode::ExternNode(_) => {}
    }

    return Value::Num(-1.0);
}
//eval args fn?
fn eval_expression(sp: &SpecialForms, scope: &mut Scope, expr: Expression) -> Result<Value, String> {
    match expr {
        LiteralExpr(val) => Ok(val),
        AssignmentExpr(name, expr, init) => {
            let set = get_result!(eval_expression(sp, scope, *expr));
            if init {
                let _ = scope.set_var_local(name, set);
            } else {
                let _ = scope.set_var(name, set);
            }
            
            Ok(Value::Bool(true))
        },
        VariableExpr(name) => {
            match scope.get_var(&name) {
                Some(val) => Ok(val.clone()), //deref
                None => error(format!("Undefined Varable: {:?}", name))
            } 
        },
        BinaryExpr(op, expr1, expr2) => {//split Binary Ops and built in fn?
            let lhs = get_result!(eval_expression(&sp, scope, *expr1));
            let rhs = get_result!(eval_expression(&sp, scope, *expr2));
            Ok(run(sp, scope, op, vec!(lhs, rhs)))
        },
        ConditionalExpr { cond_expr: cond, then_expr: then, else_expr: else_ep } => {
            if !(get_bool!(get_result!(eval_expression(sp, scope, *cond)))) {//not if
                let mut ret = Value::Bool(false);

                for node in *then {
                    ret = eval(node, sp, scope);
                }
                Ok(ret)
            } else {
                // if else eval_expression
                match else_ep {
                    Some(expr) => eval_expression(sp, scope, *expr),
                    None => Ok(Value::Bool(false))
                }
            }
            
        },
        LoopExpr { cond_expr: cond, then_expr: then} => {
            let mut ret = Value::Bool(false);

            while !get_bool!(get_result!(eval_expression(sp, scope, *cond.clone()))) {//rewirte as ref 
                for node in &*then {
                    ret = eval(node.clone(), sp, scope); //find better sloution
                }
            }
            
            Ok(ret)
        },
        CallExpr(name, args) => {
            let mut args_vals: Vec<Value> = vec![];
            for arg in args {
                match eval_expression(&sp, scope, arg) {
                    Ok(val) => args_vals.push(val),
                    Err(err) => error(err)
                }
            }

            Ok(run(sp, scope, name, args_vals))
        },
        DoExpr(list) => {
            let mut ret = Value::Bool(false);
            for node in list {
                ret = eval(node.clone(), sp, scope);
            }

            Ok(ret)
        }
    }
}

fn run(sp: &SpecialForms, scope: &mut Scope, fn_name: String, args: Vec<Value>) -> Value {
    //println!("run name: {}, args: {:?}", fn_name, args);

    match sp.map.get(&fn_name) {
        Some(fun) => {
            match fun(args) {
                Ok(val) => val,
                Err(mes) => error(mes)
            }
        },
        None => {
            match scope.funs.get(&fn_name) {
                Some(fun) => {
                    let fun_copy = fun.clone();//find better sloution
                    //extend scope
                    let _ = scope.create_depth();
                    //println!("-----S{:?}S-----", scope.var.len());

                    for i in 0..args.len() { //check args count matches
                        let _ = scope.set_var_local(fun_copy.prototype.args[i].to_string(), args[i].clone());
                    }
                    //eval
                    let res = eval(
                        ASTNode::FunctionNode(Function {
                            prototype: Prototype {name: String::from(""), 
                            args: vec![]}, body: fun_copy.body.clone()
                        }),
                        &sp,
                        scope
                    );
                    let _ = scope.remove_depth();
                    //println!("-----E{:?}E-----", scope.var.len());
                    return res;
                },
                None => error(format!("Undefined Function: {}", fn_name))
            }
        }
    }

}

fn error(mes: String) -> ! {
    println!("\x1b[91mError\x1b[0m \n {}", mes);
    panic!();
}
