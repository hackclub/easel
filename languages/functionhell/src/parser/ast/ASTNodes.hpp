#ifndef ASTNODES_H
#define ASTNODES_H

#include <string>
#include <vector>
#include <iostream>
#include <algorithm>


class Node{
public:
    virtual ~Node() = default;
    virtual std::string toString() = 0;
    virtual std::string toJS() = 0;
};
class ASTNode : public Node{
public:
    ASTNode();
    virtual ~ASTNode(); // Virtual destructor for proper cleanup
    virtual std::string toString() = 0;
    virtual std::string toJS();
};

class DataType : public Node {
public:
    virtual ~DataType(); // Virtual destructor for proper cleanup
    virtual std::string toString() = 0;
    virtual std::string toJS();
};

class Statement : public ASTNode {
public:
    virtual ~Statement(); // Virtual destructor for proper cleanup
    virtual std::string toString() = 0;
    virtual std::string toJS();
};

class Expression : public ASTNode {
public:
    virtual ~Expression(); // Virtual destructor for proper cleanup
    virtual std::string toString() = 0;
    virtual std::string toJS();
};

class Program : public ASTNode {
public:
    std::vector<ASTNode*> statements;
    Program(std::vector<ASTNode*> statements) : statements(statements) {}
    ~Program(); // Destructor to delete statements
    void print();
    std::string toString();
    std::string toJS();
};

class ListLiteral : public Expression {
public:
    std::vector<Expression*> elements;
    DataType* dataType;
    ListLiteral(std::vector<Expression*> elements, DataType* dataType);
    ~ListLiteral(); // Destructor to delete elements and dataType
    std::string toString();
    std::string toJS();
};

// Literal nodes
class IntegerLiteral : public Expression {
public:
    int value;
    IntegerLiteral(int value);
    ~IntegerLiteral(); // Destructor
    std::string toString();
    std::string toJS();
};

class FloatLiteral : public Expression {
public:
    float value;
    FloatLiteral(float value);
    ~FloatLiteral(); // Destructor
    std::string toString();
    std::string toJS();
};

class StringLiteral : public Expression {
public:
    std::string value;
    StringLiteral(std::string value);
    ~StringLiteral(); // Destructor
    std::string toString();
    std::string toJS();
};

class BooleanLiteral : public Expression {
public:
    bool value;
    BooleanLiteral(bool value);
    ~BooleanLiteral(); // Destructor
    std::string toString();
    std::string toJS();
};

// Identifier node
class VariableDeclaration : public Statement {
public:
    std::string name;
    Expression* value;
    DataType* type;
    VariableDeclaration(std::string name, DataType* type, Expression* value);
    ~VariableDeclaration(); // Destructor to delete type and value
    std::string toString();
    std::string toJS();
};

class IfStatement : public Statement {
public:
    Expression* condition;
    std::vector<ASTNode*> body;
    std::vector<ASTNode*> elseBody;
    IfStatement(Expression* condition, std::vector<ASTNode*> body, std::vector<ASTNode*> elseBlock);
    ~IfStatement(); // Destructor to delete condition, body, and elseBody
    std::string toString();
    std::string toJS();
};

class VariableAssignment : public Expression {
public:
    std::string name;
    Expression* value;
    VariableAssignment(std::string name, Expression* value);
    ~VariableAssignment(); // Destructor to delete value
    std::string toString();
    std::string toJS();
};

class ReturnStatement : public Statement {
public:
    Expression* value;
    ReturnStatement(Expression* value);
    ~ReturnStatement(); // Destructor to delete value
    std::string toString();
    std::string toJS();
};

class FunctionParameter : public ASTNode {
public:
    std::string name;
    DataType* type;
    FunctionParameter(std::string name, DataType* type);
    ~FunctionParameter(); // Destructor to delete type
    std::string toString();
    std::string toJS();
};

class VariableAccess : public Expression {
public:
    std::string name;
    std::vector<Expression*> args;
    VariableAccess(std::string name, std::vector<Expression*> args);
    ~VariableAccess(); // Destructor to delete args
    std::string toString();
    std::string toJS();
};

class VariableCaptureAccess : public Expression {
public:
    VariableAccess* access;
    DataType* type;
    VariableCaptureAccess(VariableAccess* access);
    ~VariableCaptureAccess(); // Destructor to delete access
    std::string toString();
    std::string toJS();
};

class BinaryExpression : public Expression {
public:
    Expression* left;
    Expression* right;
    std::string op;
    BinaryExpression(Expression* left, Expression* right, std::string op);
    ~BinaryExpression(); // Destructor to delete left and right
    std::string toString();
    std::string toJS();
};

class Function : public Expression {
public:
    std::vector<FunctionParameter*> params;
    std::vector<ASTNode*> body;
    std::string recurseName = "__temp__";
    DataType* returnType;
    std::vector<VariableCaptureAccess*> captures;
    Function(std::vector<FunctionParameter*> params, DataType* returnType, std::vector<ASTNode*> body);
    ~Function(); // Destructor to delete params, body, returnType, and captures
    std::string toString();
    std::string toJS();
};

class UnaryExpression : public Expression {
public:
    Expression* expr;
    std::string op;
    UnaryExpression(Expression* expr, std::string op);
    ~UnaryExpression(); // Destructor to delete expr
    std::string toString();
    std::string toJS();
};

class IntegerType : public DataType {
public:
    IntegerType();
    ~IntegerType(); // Destructor
    std::string toString();
    std::string toJS();
};

class FloatType : public DataType {
public:
    FloatType();
    ~FloatType(); // Destructor
    std::string toString();
    std::string toJS();
};

class FunctionType : public DataType {
public:
    FunctionType();
    ~FunctionType(); // Destructor
    std::string toString();
    std::string toJS();
};

class StringType : public DataType {
public:
    StringType();
    ~StringType(); // Destructor
    std::string toString();
    std::string toJS();
};

class BoolType : public DataType {
public:
    BoolType();
    ~BoolType(); // Destructor
    std::string toString();
    std::string toJS();
};

class VoidType : public DataType {
public:
    VoidType();
    ~VoidType(); // Destructor
    std::string toString();
    std::string toJS();
};

class ListType : public DataType {
public:
    DataType* subType;
    ListType(DataType* subType);
    ~ListType(); // Destructor to delete subType
    std::string toString();
    std::string toJS();
};

class AnyType : public DataType {
public:
    AnyType();
    ~AnyType(); // Destructor
    std::string toString();
    std::string toJS();
};

#endif
