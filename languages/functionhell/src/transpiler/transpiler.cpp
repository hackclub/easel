#include "transpiler.hpp"

Transpiler::Transpiler() {
    m_output = "";
}

void Transpiler::m_addStd(){
    m_output +=
    "const __fs__ = require('fs');\n"
    "var fileRead = (path) => __fs__.readFileSync(path, 'utf8');\n"
    "var fileWrite = (path, data) => { try { __fs__.writeFileSync(path(), data(), 'utf8'); } catch (err) { console.error('Error writing to file:', err); }};\n"
    "var fileAppend = (path, data) => { try { __fs__.appendFileSync(path(), data(), 'utf8'); } catch (err) { console.error('Error writing to file:', err); }};\n"
    "var varErr = () => console.error('Variable is not a function');\n"
    "var log = (...args) => {for(let i = 0; i < args.length; i ++){process.stdout.write(String(args[i]()))}}\n"
    "var at = (x, index) => x()[index()];\n"
    "var push = (x, val) => { let l = x(); l.push(val()); return l; };\n"
    "var remove = (x, index) => x().splice(index(), 1);\n"
    "var listOf = (val, length) => Array(length()).fill(val());\n"
    "var setAt = (x, index, val) => {let y = x(); y[index()] = val(); return y;};\n"
    "var stringAt = at;\n"
    "var intAt = at;\n"
    "var floatAt = at;\n"
    "var boolAt = at;\n"
    "var sin = (x) => Math.sin(x());\n"
    "var cos = (x) => Math.cos(x());\n"
    "var tan = (x) => Math.tan(x());\n"
    "var abs = (x) => x() > 0 ? x() : -x();\n"
    "var parseint = (x) => parseInt(x());\n"
    "var parsefloat = (x) => parseFloat(x());\n"
    "var len = (x) => x().length;\n"
    "var str = (x) => x().toString();\n"
    "var sqrt = (x) => Math.sqrt(x());\n"
    "var PI = () => Math.PI;\n"
    ;
}


std::string Transpiler::transpile(std::vector<ASTNode*> statements) {
    m_addStd();
    for (ASTNode* statement : statements) {
        m_output += m_genericTranspile(statement);
        if ( m_output[m_output.size()-1] != ';') {
            m_output += ';';
        }
    }
    return m_output;
}

std::string Transpiler::m_genericTranspile(ASTNode* node) {
    // Check if node is expression or statement
    if (dynamic_cast<Expression*>(node) != nullptr) {
        return m_transpileExpression(dynamic_cast<Expression*>(node));
    } else if (dynamic_cast<Statement*>(node) != nullptr) {
        return m_tanspileStatement(dynamic_cast<Statement*>(node));
    } else {
        return "Unknown node type";
    }
}


std::string Transpiler::m_tanspileStatement(Statement* statement) {
    return statement->toJS();
}

std::string Transpiler::m_transpileExpression(Expression* expression) {
    return expression->toJS();
}

