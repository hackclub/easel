#ifndef PARSER_H
#define PARSER_H
#include <string>
#include <vector>
#include <iostream>
#include <map>
#include "../lexer/tokens/tokens.hpp"
#include "../lexer/lexer.hpp"
#include "./ast/ASTNodes.hpp"

typedef struct {
    DataType* returnType;
    std::vector<DataType*> paramTypes;
} FunctionData;

class Parser {
public:
    Parser();
    Program* parse(std::vector<Token> tokens);
private:
    void m_advance();
    void m_mightEat(TOKENTYPE type);
    void m_skipWhitespace();
    
    Token m_eat(TOKENTYPE type);
    Token m_eatAny(std::vector<TOKENTYPE> types);
    
    Token m_peek();
    
    Program* m_parseProgram();

    // Parse any
    ASTNode* m_genericParse();
    ASTNode* m_parseNode();

    // Multi-handlers
    ASTNode* m_handleName();

    // Intermediate
    DataType* m_parseDataType();
    DataType* m_rawParseDataType(TOKENTYPE type);
    std::vector<FunctionParameter*> m_parseFunctionParams();
    std::vector<Expression*> m_parseFunctionCallArgs();
    std::vector<ASTNode*> m_parseBlock();

    
    // Literals
    ListLiteral* m_parseListLiteral();
    Function* m_parseFunctionLiteral();

    // Expressions
    Expression* m_parseExpression(int precedence = 0);
    Expression* m_parseAtom();
    Expression* m_handleDataTypeAtom();
    
    // Specific expressions
    BinaryExpression* m_parseBinaryExpression(Expression* left, int precedence);
    VariableAccess* m_parseVariableAccess();

    // Statements     
    VariableDeclaration* m_parseVariableDeclaration();
    IfStatement* m_parseIfStatement();
    VariableAssignment* m_parseVariableAssignment();
    ReturnStatement* m_parseReturnStatement();

    // Member variables
    std::map<std::string, ASTNode*> m_functionMap;
    std::map<std::string, ASTNode*> m_variableMap;
    Token m_CurrentToken;
    int m_CurrentIndex;
    std::vector<Token> m_Tokens;
    std::vector<std::vector<VariableCaptureAccess*>> m_functionCaptures;
};




#endif