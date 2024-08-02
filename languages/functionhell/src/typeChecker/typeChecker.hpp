#ifndef TYPECHECKER_H
#define TYPECHECKER_H

#include <string>
#include <vector>
#include <map>
#include <iostream>
#include <unordered_set>
#include <unordered_map>
#include "../parser/ast/ASTNodes.hpp"

#include "../errors/error.hpp"
#include "../utils/utils.hpp"


struct Scope {
    std::vector<std::string> var_names;
    std::map<std::string, DataType*> variables;
    Scope* parent;
};

class TypeChecker {
public:
    TypeChecker();
    int checkTypes(std::vector<ASTNode*> statements, Scope* scope, DataType* returnType); // Returns 0 if no errors, 1 if errors
    void printScope(Scope* scope);
    ~TypeChecker();
private:
    void m_addSTD(Scope* scope);
    DataType* m_findInScope(const std::string& name, Scope* scope);
    DataType* m_findInImmediateScope(const std::string& name, Scope* scope);
    DataType* m_findInHigherScope(const std::string& name, Scope* scope);
    int m_checkVariableAssignment(VariableAssignment* node, Scope* scope);
    int m_checkReturnStatement(ASTNode* statement, Scope* scope, DataType* returnType);
    int m_checkVariableAssignment(ASTNode* statement, Scope* scope);
    int m_checkExpression(ASTNode* expr, Scope* scope, DataType* expectedType);
    int m_checkVariableAccess(VariableAccess* va, DataType* expectedType, Scope* scope);
    int m_checkFunction(Function* f, DataType* expectedType, Scope* scope);
    int m_checkVariableDeclaration(VariableDeclaration* vd, Scope* scope);
    int m_checkStatement(ASTNode* statement, Scope* scope, DataType* returnType);
    DataType* m_evalOperation(DataType* d1, DataType* d2, const std::string& op);
    Scope* m_globalScope;
    DataType* m_getExpression(ASTNode* node, Scope* scope);
    DataType* m_checkBinaryExpression(BinaryExpression* node, Scope* scope);
    DataType* m_literalType(ASTNode* node, Scope* scope);
    std::vector<std::tuple<std::string, DataType*>> getCaptures(std::vector<ASTNode*> statements, Scope* scope);
};

#endif