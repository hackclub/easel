#include "typeChecker.hpp"

// Similar types
bool areSimilar(DataType* d1, DataType* d2, bool f = false) {
    if (d1->toString() == d2->toString()) {
        return true;
    }
    if (d1->toString() == "float" && d2->toString() == "int") {
        return true;
    }
    if (d1->toString() == "any" || d2->toString() == "any") {
        return true;
    }
    if (util_isType<ListType>(d1) && util_isType<ListType>(d2)) {
        ListType* l1 = dynamic_cast<ListType*>(d1);
        ListType* l2 = dynamic_cast<ListType*>(d2);
        return areSimilar(l1->subType, l2->subType);
    }
    return !f ? areSimilar(d2, d1, true) : false;
}


TypeChecker::TypeChecker() {
    m_globalScope = new Scope();
    m_addSTD(m_globalScope);
}

std::string scopeToString(Scope* scope) {
    std::string o = "Scope {";
    for (const std::string& v : scope->var_names) {
        o += "\n   " + v + " -> " + scope->variables[v]->toString();
    }
    o += "\n}";
    return o;
}
void TypeChecker::printScope(Scope* scope) {
    std::cout << scopeToString(scope) << std::endl;
}

void TypeChecker::m_addSTD(Scope* scope) {
    scope->variables["log"] = new VoidType();
    scope->variables["fileRead"] = new StringType();
    scope->variables["fileWrite"] = new VoidType();
    scope->variables["varErr"] = new VoidType();
    scope->variables["at"] = new AnyType();
    scope->variables["sin"] = new FloatType();
    scope->variables["cos"] = new FloatType();
    scope->variables["tan"] = new FloatType();
    scope->variables["abs"] = new FloatType();
    scope->variables["parseint"] = new IntegerType();
    scope->variables["parsefloat"] = new FloatType();
    scope->variables["len"] = new IntegerType();
    scope->variables["str"] = new StringType();
    scope->variables["sqrt"] = new FloatType();
    scope->variables["push"] = new ListType(new AnyType());
    scope->variables["setAt"] = new ListType(new AnyType());
    scope->variables["listOf"] = new ListType(new AnyType());
    scope->variables["str"] = new StringType();
    scope->variables["len"] = new IntegerType();
    // Hack!
    scope->variables["stringAt"] = new StringType();
    scope->variables["intAt"] = new IntegerType();
    scope->variables["floatAt"] = new FloatType();
    scope->variables["boolAt"] = new BoolType();

    scope->variables["PI"] = new FloatType();
}

DataType* TypeChecker::m_findInImmediateScope(const std::string& name, Scope* scope) {
    if (scope->variables.find(name) != scope->variables.end()) {
        return scope->variables[name];
    }
    return nullptr;
}

DataType* TypeChecker::m_findInHigherScope(const std::string& name, Scope* scope) {
    if (scope->parent != nullptr) {
        return m_findInScope(name, scope->parent);
    }
    return nullptr;
}

DataType* TypeChecker::m_findInScope(const std::string& name, Scope* scope) {
    DataType* dt = m_findInImmediateScope(name, scope);
    if (dt != nullptr) {
        return dt;
    }
    return m_findInHigherScope(name, scope);
}

int TypeChecker::checkTypes(std::vector<ASTNode*> statements, Scope* scope, DataType* returnType) {
    bool foundReturn = false;
    if (scope == nullptr) {
        scope = m_globalScope;
    }
    for (ASTNode* statement : statements) {
        //std::cout << "Checking: " << statement->toString() << std::endl;
        if (util_isType<Statement>(statement)) {
            //std::cout << "  - Is statement" << std::endl;
            if (util_isType<VariableAssignment>(statement)) {
                //std::cout << "  - Is variable assignment" << std::endl;
                if (int result = m_checkVariableAssignment(statement, scope)) {
                    return result;
                }
            } else if (util_isType<ReturnStatement>(statement)) {
                //std::cout << "  - Is return statement" << std::endl;
                foundReturn = true;
                //std::cout << "  - Found return statement" << std::endl;
                //std::cout << "  - Scope: " << scopeToString(scope) << " Return type: " << returnType->toString() << std::endl;
                int result = m_checkReturnStatement(statement, scope, returnType);
                //std::cout << "  - Result: " << result << std::endl;
                if (result != 0) { 
                    return result;
                }
            } else if (util_isType<VariableDeclaration>(statement)) {
                //std::cout << "  - Is variable declaration" << std::endl;
                int result = m_checkVariableDeclaration(dynamic_cast<VariableDeclaration*>(statement), scope);
                if (result != 0) {
                    return result;
                }
            } else if (util_isType<IfStatement>(statement)){
                //std::cout << "  - Is if statement" << std::endl;
                IfStatement* is = dynamic_cast<IfStatement*>(statement);
                if (int result = checkTypes(is->body, scope, returnType)) {
                    return result;
                }
                if (is->elseBody.size() != 0) {
                    if (int result = checkTypes(is->elseBody, scope, returnType)) {
                        return result;
                    }
                }
            }
            else {
                langError("Unknown statement type: " + statement->toString(), -1, -1);
                return 1;
            }
        }

    }
    if (!foundReturn && returnType != nullptr && returnType->toString() != "void") {
        langError("Non-void function of return type " + returnType->toString() + " does not return a value", -1, -1);
        return 1;
    }
    return 0;
}

int TypeChecker::m_checkStatement(ASTNode* statement, Scope* scope, DataType* returnType) {
    Statement* s = dynamic_cast<Statement*>(statement);
    if (util_isType<VariableDeclaration>(s)) {
        //std::cout << "  - Is variable declaration" << std::endl;
        VariableDeclaration* vd = dynamic_cast<VariableDeclaration*>(s);
        int result = m_checkVariableDeclaration(vd, scope);

    }
    return 0;
}

int TypeChecker::m_checkVariableDeclaration(VariableDeclaration* vd, Scope* scope) {
    if (m_findInScope(vd->name, scope) != nullptr) {
        langError("Variable " + vd->name + " already declared in this scope", -1, -1);
        return 1;
    } else {
        //std::cout << "    - Variable " << vd->name << " not found in scope" << std::endl;
        scope->variables[vd->name] = vd->type;
        scope->var_names.push_back(vd->name);
        //std::cout << "    - Added variable to scope: " << vd->name << " -> " << vd->type->toString() << std::endl;
        if (util_isType<Function>(vd->value)) {
            //std::cout << "    - Value is function" << std::endl;
            //std::cout << "   - Scope: " << scopeToString(scope) << std::endl;
            Function* f = dynamic_cast<Function*>(vd->value);
            f->recurseName = vd->name;
            return m_checkFunction(f, vd->type, scope);
        } else if (util_isType<VariableAccess>(vd->value)) {
            //std::cout << "    - Value is variable access" << std::endl;
            return m_checkVariableAccess(dynamic_cast<VariableAccess*>(vd->value), vd->type, scope);
        } else {
            langError("Variable value cannot be non-function", -1,-1);
        }
    }
    return 0;
}

int TypeChecker::m_checkFunction(Function* f, DataType* expectedType, Scope* scope) {
    if (f->returnType != nullptr && !areSimilar(f->returnType, expectedType)) {
        langError("Function return type: " + f->returnType->toString() + " does not match variable type: " + expectedType->toString() + " in " + f->toString(), -1, -1);
        return 1;
    }
    Scope* funcScope = new Scope();
    funcScope->parent = scope;
    for (FunctionParameter* fp : f->params) {
        funcScope->variables[fp->name] = fp->type;
        funcScope->var_names.push_back(fp->name);
    }
    //printScope(funcScope);
    return checkTypes(f->body, funcScope, f->returnType);
}

int TypeChecker::m_checkVariableAccess(VariableAccess* va, DataType* expectedType, Scope* scope) {
    DataType* found = m_findInScope(va->name, scope);
    if (found == nullptr) {
        langError("Variable " + va->name + " not declared in this scope (access)", -1, -1);
        return 1;
    }
    if (found->toString() != expectedType->toString() && expectedType->toString() != "any" && found->toString() != "any"){
        langError("Variable " + va->name + " type does not match variable type", -1, -1);
        return 1;
    }
    if (va->args.size() > 0){
        // TODO: Impliment function call arg checking
    }
    return 0;
}

int TypeChecker::m_checkReturnStatement(ASTNode* statement, Scope* scope, DataType* returnType) {
    //std::cout << "    - Casting return statement" << std::endl;
    ReturnStatement* rs = dynamic_cast<ReturnStatement*>(statement);
    if (rs == nullptr) {
        langError("Statement is not a ReturnStatement",-1,-1);
        return 1; // or handle the error appropriately
    }

    //std::cout << "    - Checking return statement: " << rs->toString() << std::endl;

    if (rs->value != nullptr) {
        //std::cout << "    - Return value is not null" << std::endl;
        DataType* rs_dt = m_getExpression(rs->value, scope);
        if (rs_dt == nullptr) {
            langError("Return value type is null: " + statement->toString(),-1,-1);
            return 1;
        }
        //std::cout << "    - Return value type: " << rs_dt->toString() << std::endl;

        if (!areSimilar(rs_dt, returnType)) {
            langError("Return type: " + rs_dt->toString() + " does not match expected type: " + returnType->toString(), -1, -1);
            return 1;
        }
    } else {
        //std::cout << "    - Return value is null" << std::endl;
    }

    //std::cout << "    - Return statement passed" << std::endl;
    return 0;
}

int TypeChecker::m_checkVariableAssignment(ASTNode* statement, Scope* scope) {
    VariableAssignment* va = dynamic_cast<VariableAssignment*>(statement);
    DataType* va_dt = m_findInScope(va->name, scope);
    if (va_dt == nullptr) {
        langError("Variable " + va->name + " not declared in this scope (assignment)", -1, -1);
        return 1;
    }
    if (util_isType<Function>(va->value)) {
        //std::cout << "    - Value is function" << std::endl;
        return m_checkFunction(dynamic_cast<Function*>(va->value), va_dt, scope);
    } else if (util_isType<VariableAccess>(va->value)) {
        //std::cout << "    - Value is variable access" << std::endl;
        return m_checkVariableAccess(dynamic_cast<VariableAccess*>(va->value), va_dt, scope);
    } else {
        //std::cout << "    - Value is expression" << std::endl;
        return m_checkExpression(va->value, scope, va_dt);
    }
}

int TypeChecker::m_checkExpression(ASTNode* expr, Scope* scope, DataType* expectedType) {
    DataType* dt = m_getExpression(expr, scope);
    if (dt == nullptr) {
        return 1;
    }
    if (expectedType != nullptr && dt->toString() != expectedType->toString()) {
        langError("Expression type does not match expected type", -1, -1);
        return 1;
    }
    return 0;
}

std::vector<std::tuple<std::string, DataType*>> TypeChecker::getCaptures(std::vector<ASTNode*> statements, Scope* scope) {
    std::vector<std::tuple<std::string, DataType*>> captures;
    for (ASTNode* statement : statements) {
        if (util_isType<VariableAccess>(statement)) {
            VariableAccess* va = dynamic_cast<VariableAccess*>(statement);
            if (m_findInScope(va->name, scope) == nullptr) {
                captures.push_back(std::make_tuple(va->name, nullptr));
            }
        } else if (util_isType<VariableAssignment>(statement)) {
            VariableAssignment* va = dynamic_cast<VariableAssignment*>(statement);
            if (m_findInScope(va->name, scope) == nullptr) {
                captures.push_back(std::make_tuple(va->name, nullptr));
            }
        } else if (util_isType<Function>(statement)) {
            Function* f = dynamic_cast<Function*>(statement);
            Scope* funcScope = new Scope();
            funcScope->parent = scope;
            for (FunctionParameter* fp : f->params) {
                funcScope->variables[fp->name] = fp->type;
                funcScope->var_names.push_back(fp->name);
            }
            //printScope(funcScope);
            std::vector<std::tuple<std::string, DataType*>> a = getCaptures(f->body, funcScope);
            captures.insert(captures.end(), a.begin(), a.end());
        }
    }
    return captures;
}

DataType* TypeChecker::m_checkBinaryExpression(BinaryExpression* node, Scope* scope) {
    DataType* d1 = m_getExpression(node->left, scope);
    DataType* d2 = m_getExpression(node->right, scope);
    if (d1 == nullptr || d2 == nullptr) {
        langError("Binary expression type is null: " + node->toString(), -1, -1);
        return nullptr;
    }
    DataType* o = m_evalOperation(d1, d2, node->op);
    if (o == nullptr) {
        if (d1->toString() == "any" || d2->toString() == "any") {
            return new AnyType();
        }
        langError("Operation " + node->toString() + " not valid for types " + d1->toString() + " and " + d2->toString(), -1, -1);
        return nullptr;
    }
    return o;
}

DataType* TypeChecker::m_getExpression(ASTNode* expr, Scope* scope) {
    if (util_isType<BinaryExpression>(expr)) {
        return m_checkBinaryExpression(dynamic_cast<BinaryExpression*>(expr), scope);
    } else if (util_isType<VariableAccess>(expr)) {
        VariableAccess* va = dynamic_cast<VariableAccess*>(expr);
        DataType* dt = m_findInImmediateScope(va->name, scope);
        if (dt == nullptr) {
            langError("Variable " + va->name + " not declared in this scope", -1, -1);
            return nullptr;
        }
        return dt;
    } else if (util_isType<VariableCaptureAccess>(expr)) {
        VariableCaptureAccess* vca = dynamic_cast<VariableCaptureAccess*>(expr);
        DataType* dt = m_findInScope(vca->access->name, scope);
        vca->type = dt;
        if (dt == nullptr) {
            langError("Variable capture " + vca->access->name + " not declared in scope", -1, -1);
            return nullptr;
        }
        return dt;
    } else if (util_isType<ListLiteral>(expr)){
        ListLiteral* ll = dynamic_cast<ListLiteral*>(expr);
        DataType* dt = new ListType(ll->dataType);
        return dt;
    } 
    else {
        DataType* dt = m_literalType(expr, scope);
        if (dt == nullptr) {
            langError("Unknown expression type: " + expr->toString(), -1, -1);
            return nullptr;
        }
        return dt;
    }
}

DataType* TypeChecker::m_evalOperation(DataType* d1, DataType* d2, const std::string& op) {
    static std::map<std::string, DataType*> ops = {
        {"int + int", new IntegerType()},
        {"int + float", new FloatType()},
        {"float + int", new FloatType()},
        {"float + float", new FloatType()},
        {"string + string", new StringType()},
        {"int - int", new IntegerType()},
        {"int - float", new FloatType()},
        {"float - int", new FloatType()},
        {"float - float", new FloatType()},
        {"int * int", new IntegerType()},
        {"int * float", new FloatType()},
        {"float * int", new FloatType()},
        {"float * float", new FloatType()},
        {"int / int", new IntegerType()},
        {"int / float", new FloatType()},
        {"float / int", new FloatType()},
        {"float / float", new FloatType()},
        {"int % int", new IntegerType()},
        {"int % float", new FloatType()},
        {"float % int", new FloatType()},
        {"float % float", new FloatType()},
        {"int == int", new BoolType()},
        {"int == float", new BoolType()},
        {"float == int", new BoolType()},
        {"float == float", new BoolType()},
        {"string == string", new BoolType()},
        {"int != int", new BoolType()},
        {"int != float", new BoolType()},
        {"float != int", new BoolType()},
        {"float != float", new BoolType()},
        {"string != string", new BoolType()},
        {"int < int", new BoolType()},
        {"int < float", new BoolType()},
        {"float < int", new BoolType()},
        {"float < float", new BoolType()},
        {"int > int", new BoolType()},
        {"int > float", new BoolType()},
        {"float > int", new BoolType()},
        {"float > float", new BoolType()},
        {"int <= int", new BoolType()},
        {"int <= float", new BoolType()},
        {"float <= int", new BoolType()},
        {"float <= float", new BoolType()},
        {"int >= int", new BoolType()},
        {"int >= float", new BoolType()},
        {"float >= int", new BoolType()},
        {"float >= float", new BoolType()},
        {"bool == bool", new BoolType()},
        {"bool != bool", new BoolType()},
        {"bool && bool", new BoolType()},
        {"bool || bool", new BoolType()},
    };

    std::string s = d1->toString() + " " + op + " " + d2->toString();
    if (ops.find(s) != ops.end()) {
        return ops[s];
    }
    return nullptr;
}

DataType* TypeChecker::m_literalType(ASTNode* node, Scope* scope) {
    if (util_isType<IntegerLiteral>(node)) {
        return new IntegerType();
    } else if (util_isType<FloatLiteral>(node)) {
        return new FloatType();
    } else if (util_isType<StringLiteral>(node)) {
        return new StringType();
    } else if (util_isType<BooleanLiteral>(node)) {
        return new BoolType();
    } else if (util_isType<Function>(node)) {
        return new FunctionType();
    }
    return nullptr;
}

TypeChecker::~TypeChecker() {
    delete m_globalScope;
}