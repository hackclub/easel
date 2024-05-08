AST_TYPE = {
    "Var": "Var",
    "Func": "Func",
    "Class": "Class",
    "Array": "Array",
    "Dict": "Dict",
    "Number": "Number",
    "String": "String",
    "Bool": "Bool",
    "BinOp": "BinOp",
    "Return": "Return",
    "If": "If",
    "Else": "Else",
    "While": "While",
    "For": "For",
    "Attr": "Attr",
    "Call": "Call",
    "Chain": "Chain",
}


def new_var(name, value=""):
    return {"type": AST_TYPE["Var"], "name": name, "value": value}


def new_func(name, args, body):
    return {"type": AST_TYPE["Func"], "name": name, "args": args, "body": body}


def new_class(name, methods):
    return {
        "type": AST_TYPE["Class"],
        "name": name,
        "methods": methods,
    }


def new_dict(items):
    return {"type": AST_TYPE["Dict"], "items": items}


def new_array(items):
    return {"type": AST_TYPE["Array"], "value": items}


def new_return(value):
    return {"type": AST_TYPE["Return"], "value": value}


def new_if(condition, body, otherwise=None):
    return {
        "type": AST_TYPE["If"],
        "condition": condition,
        "body": body,
        "otherwise": otherwise,
    }


def new_else(body):
    return {"type": AST_TYPE["Else"], "body": body}


def new_while(condition, body):
    return {"type": AST_TYPE["While"], "condition": condition, "body": body}


def new_for(var, through, body):
    return {"type": AST_TYPE["For"], "var": var, "range": through, "body": body}


def new_number(val):
    return {"type": AST_TYPE["Number"], "value": val}


def new_string(val):
    return {"type": AST_TYPE["String"], "value": val}


def new_bool(val):
    return {"type": AST_TYPE["Bool"], "value": val}


def new_binop(left, right, op, wrapped=False):
    return {
        "type": AST_TYPE["BinOp"],
        "left": left,
        "right": right,
        "op": op,
        "wrapped": wrapped,
    }


def new_call(args):
    return {"type": AST_TYPE["Call"], "args": args}


def new_attr(attr):
    return {"type": AST_TYPE["Attr"], "name": attr}


def new_chain(name, chain):
    return {"type": AST_TYPE["Chain"], "name": name, "chain": chain}
