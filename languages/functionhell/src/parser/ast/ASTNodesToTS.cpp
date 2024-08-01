#include "ASTNodes.hpp"
#include <iostream>

std::string ASTNode::toJS() {
    return "null";
}

std::string Statement::toJS() {
    return "null";
}

std::string Expression::toJS() {
    return "null";
}

std::string Program::toJS() {
    std::string result = "";
    for (ASTNode* statement : statements) {
        result += statement->toJS() + ";\n";
    }
    return result;
}

std::string ListLiteral::toJS(){
    std::string result = "[";
    for (Expression* e: elements){
        result += e->toJS() + ", ";
    }
    return result + "]";
}

std::string IntegerLiteral::toJS() {
    return std::to_string(value);
}

std::string FloatLiteral::toJS() {
    //std::cout << "TRANSPILED FLOAT TO " << std::to_string(value) << std::endl;
    return std::to_string(value);
}

std::string StringLiteral::toJS() {
    return "\"" + value + "\"";
}

std::string BooleanLiteral::toJS() {
    return this->toString();
}

std::string VariableDeclaration::toJS() {
    std::string args = "";
    std::string valJs = "";
    if (value != nullptr){
        if (dynamic_cast<Function*>(value) != nullptr){
            std::vector<FunctionParameter*> params = dynamic_cast<Function*>(value)->params;

            args = "(";
            for(FunctionParameter* param: params){
                std::string a = param->toJS() + ", ";
                args += a;
            }
            args += ")";

            valJs = value->toJS();
        }
        else{
            if(dynamic_cast<VariableAccess*>(value) == nullptr)
                valJs = value->toJS();
            else{
                VariableAccess* va = dynamic_cast<VariableAccess*>(value);
                valJs = va->name;
                args = "()";
            }
        }
    }
    return "let " + name/* + ": " + args + " => " + type->toJS()*/ + " = "  + valJs + ";\n";
}

std::string Function::toJS(){
    std::string o = "(function ";
    o += recurseName + " ";
    o += "(";
    for(FunctionParameter* param: params){
        o += param->toJS() + ", ";
    }
    o += ") {\n";
    for(ASTNode* node: body){
        o += node->toJS() + ";\n";
    }
    o += "})";
    return o;
}

std::string FunctionParameter::toJS() {
    return name;
}


std::string VariableAssignment::toJS() {
    std::string valJs = "";
    valJs = value->toJS();

    return name + " = " + valJs + ";\n";
}


std::string ReturnStatement::toJS() {
    return "return " + value->toJS();
}

std::string IfStatement::toJS() {
    std::string o = "if(" + condition->toJS() + ") {";
    for(ASTNode* node: body){
        o += node->toJS() + ";\n";
    }
    if (elseBody.size() == 0){
        o += "}";
        return o;
    }
    o += "} else {";
    for(ASTNode* node: elseBody){
        o += node->toJS() + ";\n";
    }
    o += "}";
    return o;
}

std::string VariableCaptureAccess::toJS() {
    return access->toJS();
}

std::string VariableAccess::toJS() {
    std::string o = name + "(";
    for(Expression* arg: args){
        o += arg->toJS() + ", ";
    }
    o += ")";
    return o;
}

std::string BinaryExpression::toJS() {
    bool useParens = true;
    if (op[0] == '=' || op == "!=" || op[0] == '>' || op[0] == '<')
        useParens = false;
    if (op == "="){
        return left->toJS() + " = " + right->toJS();
    }
    return (useParens ? "(" : "") + left->toJS() + " " + op + " " + right->toJS() + (useParens ? ")" : "");
}

std::string DataType::toJS() {
    return "null";
}

std::string FloatType::toJS(){
    return "number";
}

std::string FunctionType::toJS(){
    return "() => any";
}

std::string StringType::toJS(){
    return "string";
}

std::string BoolType::toJS(){
    return "boolean";
}

std::string VoidType::toJS(){
    return "void";
}

std::string ListType::toJS(){
    return subType->toJS() + "[]";
}

std::string AnyType::toJS(){
    return "any";
}