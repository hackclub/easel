#ifndef TRANSPILER_H
#define TRANSPILER_H

#include <string>
#include <vector>
#include <fstream>
#include "../parser/ast/ASTNodes.hpp"

class Transpiler {
public:
    Transpiler();
    std::string transpile(std::vector<ASTNode*> statements);
private:

    std::string m_output;
    std::string m_genericTranspile(ASTNode* node);
    std::string m_tanspileStatement(Statement* statement);
    std::string m_transpileExpression(Expression* expression);
    void m_addStd();
};

#endif