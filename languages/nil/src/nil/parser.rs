use std::collections::HashMap;

use crate::nil::errorhandler::Error;
use crate::nil::grammar;
use crate::nil::token::{Token, TokenVal, TypeOf};
use grammar::*;
use ASTNode::*;
use Expression::*;
use TokenVal::*;

macro_rules! get_result {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(err) => return Err(err),
        }
    };
}

macro_rules! next {
    ($toks:expr) => {
        match $toks.first() {
            Some(tk) => tk,
            None => Err(Error::mes("Unexpected End Of Input"))
        }
    } 
}

pub struct ParserSettings {
    operator_precednece: HashMap<String, i32>,
}

impl ParserSettings {
    pub fn default() -> Self {
        let mut op_prec = HashMap::new();
        //op_prec.insert("<".to_string(), 10);
        op_prec.insert("+".to_string(), 20);
        op_prec.insert("-".to_string(), 20);
        op_prec.insert("*".to_string(), 40);
        op_prec.insert("/".to_string(), 40);

        op_prec.insert("%".to_string(), 40);

        op_prec.insert("==".to_string(), 15);
        op_prec.insert("!=".to_string(), 15);
        op_prec.insert(">".to_string(), 15);
        op_prec.insert(">=".to_string(), 15);
        op_prec.insert("<".to_string(), 15);
        op_prec.insert("<=".to_string(), 15);

        op_prec.insert("&&".to_string(), 10);
        op_prec.insert("||".to_string(), 10);
        
        ParserSettings {
            operator_precednece: op_prec,
        }
    }
}

fn error<T>(message: &str) -> Result<T, Error> {
    Err(Error::mes(message))
}

pub fn parser(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> Result<Vec<ASTNode>, Error> {
    let mut ast: Vec<ASTNode> = vec![];
    let mut hold: Vec<Token> = vec![]; //keeps tokens of the current line

    loop {
        if tokens.len() == 0 {
            break;
        }

        let cur_token = &tokens[0];

        let result = match cur_token.value {
            Def => parse_function(tokens, settings),
            Extern => parse_extern(tokens, settings),
            Delimiter => {
                hold = vec![];
                tokens.remove(0);
                continue;
            }
            _ => {parse_expression(tokens, &hold, settings)},
        };

        ast.push(get_result!(result));

        if tokens.len() == 0 {
            break;
        }

        hold.push(tokens.remove(0));
    }

    Ok(ast)
}

fn parse_function(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> Result<ASTNode, Error> {
    tokens.remove(0); //Removes Def
    
    let mut raw_body = vec![];
    let mut pars = 1;
    let line = tokens[0].pos;

    tokens.remove(0); //removes '('

    loop {
        if tokens.len() == 0 {
            return Err(Error::at("Expected ')' nil statment starting", line)) //fix
        }

        match tokens[0].value {
            OpeningPars => pars += 1,
            ClosingPars => pars -= 1,
            _ => {}
        }
        if pars == 0 {
            break
        } else {
            raw_body.push(tokens.remove(0))
        }
    }
    tokens.remove(0);

    let body = DoExpr(get_result!(parser(&mut raw_body, settings)));
    let prototype = parse_prototype(tokens, settings);

    Ok(FunctionNode(Function {
        prototype: get_result!(prototype),
        body: body,
    }))
}

fn parse_extern(tokens: &mut Vec<Token>, settings: &mut ParserSettings) -> Result<ASTNode, Error> {
    tokens.remove(0); //Removes Def token
    let prototype = parse_prototype(tokens, settings);

    Ok(ExternNode(get_result!(prototype)))
}
fn parse_expression(
    tokens: &mut Vec<Token>,
    hold: &Vec<Token>,
    settings: &mut ParserSettings,
) -> Result<ASTNode, Error> {
    let expression = parse_expr(tokens, settings, hold);
    let prototype = Prototype {
        name: "".to_string(),
        args: vec![],
    };
    let lambda = Function {
        prototype: prototype,
        body: get_result!(expression),
    };

    Ok(FunctionNode(lambda))
}

fn parse_prototype(
    tokens: &mut Vec<Token>,
    _settings: &mut ParserSettings,
) -> Result<Prototype, Error> {
    if tokens[0].value != OpeningBrac {
        return Err(Error::at_pt("Expected '{", tokens[0].pos));
    }
    tokens.remove(0);

    let mut args = vec![];

    let line = tokens[0].pos;

    loop {
        match tokens.remove(0).value {
            Ident(arg) => args.push(arg),
            ClosingBrac => break,
            _ => return Err(Error::at("Expected '}", tokens[0].pos)),
        }
    }

    let name: String;
    match tokens.remove(0).value {
        Ident(val) => name = val,
        _ => return Err(Error::at("Expected function name", line)),
    }

    Ok(Prototype {
        name: name,
        args: args,
    })
}

fn parse_primary_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> Result<Expression, Error> {
    if &tokens[0].value == &Delimiter {
        tokens.remove(0);
    }
    //println!("PPE: {:?}", tokens);
    let expr = match &tokens[0].value {
        Ident(name) => {
            //Only variable start with Ident
            let id_name = name.to_owned();
            tokens.remove(0);
            Ok(VariableExpr(id_name.to_string()))
        }
        Value(_) => parse_literal_expr(tokens, settings),
        NIf => parse_conditional_expr(tokens, settings),
        NWhile => parse_loop_expr(tokens, settings),
        OpeningBrac => parse_call_expr(tokens, settings),
        OpeningPars => parse_parenthesis_expr(tokens, settings),
        _ => Err(Error::at_pt("Error Parsing Stantment", tokens[0].pos))
    };

    Ok(get_result!(expr))
}

fn parse_loop_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    ) -> Result<Expression, Error> {
    tokens.remove(0); //removes NIf

    let mut raw_cond = vec![];
    let mut line = tokens[0].pos;

    loop {
        if tokens.len() == 0 {
            return Err(Error::at("Expected '(' in nil statment", line)) //fix
        }

        match tokens[0].value {
            OpeningPars => break,
            _ => raw_cond.push(tokens.remove(0)),
        }

    }


    let mut raw_then = vec![];
    let mut pars = 1;
    line = tokens[0].pos;

    tokens.remove(0); //removes '('

    loop {
        if tokens.len() == 0 {
            return Err(Error::at("Expected ')' nil statment starting", line)) //fix
        }

        match tokens[0].value {
            OpeningPars => pars += 1,
            ClosingPars => pars -= 1,
            _ => {}
        }
        if pars == 0 {
            break
        } else {
            raw_then.push(tokens.remove(0))
        }
    }
    tokens.remove(0); // Remove ')'
    //pop else token parse conditinal, 

    let cond = get_result!(parse_expr(&mut raw_cond, settings, &Vec::new()));
    let then = get_result!(parser(&mut raw_then, settings));

    Ok(LoopExpr{
        cond_expr: Box::new(cond), 
        then_expr: Box::new(then)
    })
}

fn parse_conditional_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    ) -> Result<Expression, Error> {
    tokens.remove(0); //removes NIf
    let mut raw_cond = vec![];
    let mut line = tokens[0].pos;
        
    loop {
        if tokens.len() == 0 {
            return Err(Error::at("Expected '(' in nil statment", line)) //fix
        }
        
        match tokens[0].value {
            OpeningPars => break,
            _ => raw_cond.push(tokens.remove(0)),
        }

    }
    
    
    let mut raw_then = vec![];
    let mut pars = 1;

    line = tokens[0].pos;
    tokens.remove(0); //removes '('

    loop {
        if tokens.len() == 0 {
            return Err(Error::at("Expected ')' nil statment starting", line)) //fix
        }

        match tokens[0].value {
            OpeningPars => pars += 1,
            ClosingPars => pars -= 1,
            _ => {}
        }
        if pars == 0 {
            break
        } else {
            raw_then.push(tokens.remove(0))
        }

    }
    tokens.remove(0); // Remove ')'
    //pop else token parse conditinal, 

    let else_expr = match tokens.first() {
        Some(val) => match val.value {
            Else => {
                tokens.remove(0); //removes Else
                Some(Box::new(get_result!(parse_conditional_expr(tokens, settings))))}
                ,
            _ => None
        }
        _ => None
    };
    let cond = get_result!(parse_expr(&mut raw_cond, settings, &Vec::new()));
    let then = get_result!(parser(&mut raw_then, settings));

    Ok(ConditionalExpr{
        cond_expr: Box::new(cond), 
        then_expr: Box::new(then), 
        else_expr: else_expr
    })
}

fn parse_call_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> Result<Expression, Error> {
    tokens.remove(0); //removes OpeningBrac

    let name: String;
    match tokens.remove(0).value {
        Ident(val) => name = val,
        _ => return error("expected function name in prototype"),
    }

    if tokens.remove(0).value != ClosingBrac {
        return error("expected function name in prototype");
    }

    let mut args = vec![];

    loop {
        if tokens.len() == 0 {
            break;
        }
        match &tokens[0].value {
            Delimiter => break, //Next line(end of args) starts with ';'
            Assignment => break,
            _ => args.push(get_result!(parse_primary_expr(tokens, settings)))//args.push(get_result!(parse_expr(tokens, settings, &Vec::new()))),
        }
    }

    //println!("args: {:?}", args);

    Ok(CallExpr(name, args))
}

fn parse_literal_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> Result<Expression, Error> { 
    
    match tokens.remove(0).value {
        Value(val) => Ok(LiteralExpr(val)),
        _ => error("literal expected"),
    }
}

fn parse_parenthesis_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> Result<Expression, Error> {
    tokens.remove(0); //removes '('
    let expr = parse_expr(tokens, settings, &Vec::new());

    if tokens.remove(0).value != ClosingPars {
        return error("expected ')'");
    }

    Ok(get_result!(expr))
}

fn parse_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    _hold: &Vec<Token>,
) -> Result<Expression, Error> {
    let mut lhs = parse_primary_expr(tokens, settings);

    if tokens.len() != 0 {
    match &tokens[0].value {
        Type(type_of) => {
            if let VariableExpr(name) = get_result!(lhs) {
                match type_of {
                    TypeOf::Num => lhs = Ok(AssignmentExpr(name, Box::new(LiteralExpr(grammar::Value::Num(0.0))), true)),
                    TypeOf::String => lhs = Ok(AssignmentExpr(name, Box::new(LiteralExpr(grammar::Value::String(String::from("")))), true)),
                    TypeOf::Bool => lhs = Ok(AssignmentExpr(name, Box::new(LiteralExpr(grammar::Value::Bool(false))), true)),
                    }
            } else {
                return error("Error parsing Variable Init")
            }
            
        },
        _ => {}
    };
    }
    
    let expr = parse_binary_expr(tokens, settings, 0, &(get_result!(lhs)));
    Ok(get_result!(expr))
}

fn parse_binary_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    expr_precednce: i32,
    lhs: &Expression,
) -> Result<Expression, Error> {
    let mut result = lhs.clone();
    
    loop {
        if tokens.len() == 0 {
            break;
        }
        let (operator, precednce) = match &tokens[0].value {
            &Operator(ref op) | &Logical(ref op) => {
                
                match settings.operator_precednece.get(op) {
                    //checks hashmap for op
                    Some(pr) if *pr >= expr_precednce => (op.clone(), *pr),
                    None => return error(&format!("unkonwn operator: {}", op)),
                    _ => break,
                }
            },
            &Assignment => {
                tokens.remove(0);
                let Ident(name) = tokens.remove(0).value else {return Err(Error::at_pt("Expected Variable Name", tokens[0].pos))};
                return Ok(AssignmentExpr(name, Box::new(result), false));
            },
            
            _ => break,
        };
        tokens.remove(0);

        let mut rhs = parse_primary_expr(tokens, settings);

        loop {
            if tokens.len() == 0 {
                break;
            }
            let binary_rhs = match &tokens[0].value {
                &Operator(ref op) | &Logical(ref op) => match settings.operator_precednece.get(op) {
                    Some(pr) if *pr > precednce => {
                        parse_binary_expr(tokens, settings, *pr, &(get_result!(rhs)))
                    }
                    None => return error(&format!("unkonwn operator: {}", op)),
                    _ => break,
                },
                &Assignment => {
                    let result = BinaryExpr(operator, Box::new(result), Box::new(get_result!(rhs)));
                    tokens.remove(0);//removes '='
                    let name = match &tokens[0].value {
                        Ident(val) => val.to_owned(),
                        _ => return error("Varable name is improper")
                    };
                    tokens.remove(0);//removes name
                    return Ok(AssignmentExpr(name, Box::new(result), false));
                }
                _ => break,
            };

            rhs = binary_rhs;
        }

        result = BinaryExpr(operator, Box::new(result), Box::new(get_result!(rhs)));
    }

    Ok(result)
}
