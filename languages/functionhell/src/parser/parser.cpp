#include "parser.hpp"
#include "../errors/error.hpp"
#include <algorithm>

std::map<TOKENTYPE, int> ORDER_OF_OPERATIONS = {
    {PLUS, 10},
    {MINUS, 10},
    {STAR, 20},
    {SLASH, 20},
    {PERCENT, 20},
    {EQUAL_EQUAL, 5},
    {BANG_EQUAL, 5},
    {LESS, 5},
    {LESS_EQUAL, 5},
    {GREATER, 5},
    {GREATER_EQUAL, 5},
    {AND_KEYWORD, 3},
    {OR_KEYWORD, 2},
    {BANG, 1}
};


Parser::Parser() {
    m_CurrentIndex = 0;
    m_functionMap = {};
}

Program* Parser::parse(std::vector<Token> tokens) {
    m_Tokens = tokens;
    m_CurrentToken = m_Tokens[m_CurrentIndex];
    return m_parseProgram();
}

void Parser::m_advance() {
    m_CurrentIndex += 1;
    if (m_CurrentIndex < (int)m_Tokens.size()) {
        m_CurrentToken = m_Tokens[m_CurrentIndex];
    }
}

Token Parser::m_peek(){
    if (m_CurrentIndex+1 < (int)m_Tokens.size()) {
        return m_Tokens[m_CurrentIndex+1];
    }
    langError("Peeked out of scope", 0, 0);
    return Token();
}

void Parser::m_skipWhitespace() {
    while (m_CurrentToken.type == NEWLINE){
        m_eat(NEWLINE);
    }
}

Token Parser::m_eat(TOKENTYPE type) {
    if (m_CurrentToken.type == type) {
        Token eatenToken = m_CurrentToken;
        m_advance();
        return eatenToken;
    } else {
        langError("Unexpected token in EAT: " + m_CurrentToken.toString() + " | Expected: " + token_strings[type], m_CurrentToken.line, m_CurrentToken.col);
    }
    return Token();
}

void Parser::m_mightEat(TOKENTYPE type){
    if(m_CurrentToken.type == type){
        m_eat(type);
    }
}


Token Parser::m_eatAny(std::vector<TOKENTYPE> types){
    for (int i = 0; i < (signed int)types.size(); i++) {
        TOKENTYPE type = types[i];
        //std::cout << "CHECKING " << token_strings[type] << " AGAINST " << token_strings[m_CurrentToken.type] << std::endl; 
        if (m_CurrentToken.type == type) {
            Token eatenToken = m_CurrentToken;
            m_advance();
            return eatenToken;
        }
    }
    std::string t_string = "";
    for (TOKENTYPE t : types){
        t_string += token_strings[t] + ", ";
    }
    langError("Unexpected token in EATANY: " + m_CurrentToken.toString() + " | Expected: {" + t_string + "}", m_CurrentToken.line, m_CurrentToken.col);
    return Token();
}

Program* Parser::m_parseProgram() {
    std::vector<ASTNode*> statements;
    while (m_CurrentToken.type != END_OF_FILE) {
        ASTNode* node = m_parseNode();
        if (node != NULL) statements.push_back(node);
        if (m_CurrentToken.type == NEWLINE) m_eat(NEWLINE);
    }
    return new Program(statements);
}

std::vector<FunctionParameter*> Parser::m_parseFunctionParams() {
    std::vector<FunctionParameter*> params = {};

    while (m_CurrentToken.type != GREATER) {
        DataType* dataType = m_parseDataType();
        std::string name = m_eat(IDENTIFIER).value; 
        params.push_back(new FunctionParameter(name, dataType));
        m_mightEat(COMMA);
    }
    return params;
}

Function* Parser::m_parseFunctionLiteral() {
    DataType* dataType = m_parseDataType();
    m_eat(LESS);
    std::vector<FunctionParameter*> params = m_parseFunctionParams();
    m_eat(GREATER);
    m_functionCaptures.push_back({});
    std::vector<ASTNode*> body = m_parseBlock();
    Function* fn = new Function(params, dataType, body);
    fn->captures = m_functionCaptures[m_functionCaptures.size()-1];
    for(VariableCaptureAccess* vcc : fn->captures){
        //std::cout << "CAPTURED: " << vcc->toString();
    }
    //std::cout << std::endl;
    m_functionCaptures.pop_back();
    return fn;
}

//def parseExpression(self, precedence=0) -> Expression:
//        left = self.parseAtom()
//        while self.curToken().type in ORDER_OF_OPERATIONS and ORDER_OF_OPERATIONS[self.curToken().type][1] >= precedence:
//            op = self.curToken().type
//            self.advance()
//            right = self.parseExpression(ORDER_OF_OPERATIONS[op][1])
//            left = BinaryOperation(left, TOKEN_TO_OPERATOR[op], right)
//        return left
Expression* Parser::m_parseExpression(int precedence) {
    Expression* left = m_parseAtom();
    while (m_CurrentToken.type != NEWLINE && ORDER_OF_OPERATIONS.find(m_CurrentToken.type) != ORDER_OF_OPERATIONS.end() && ORDER_OF_OPERATIONS[m_CurrentToken.type] >= precedence) {
        Token op = m_CurrentToken;
        m_advance();
        Expression* right = m_parseExpression(ORDER_OF_OPERATIONS[op.type]);
        left = new BinaryExpression(left, right, op.value);
    }
    return left;
}

ASTNode* Parser::m_handleName() {
    return nullptr;
}

ASTNode* Parser::m_parseNode() {
    if (m_CurrentToken.type == VAR_KEYWORD) {
        ASTNode* n = m_parseVariableDeclaration();
        m_eat(NEWLINE);
        return n;
    }
    else if(m_CurrentToken.type == IF_KEYWORD) {
        ASTNode* n = m_parseIfStatement();
        return n;
    }
    else if (m_CurrentToken.type == IDENTIFIER){
        return m_parseExpression();
    }
    else if (std::find(ATOMS.begin(), ATOMS.end(), m_CurrentToken.type) != ATOMS.end()){
        return m_parseExpression();
    }
    else if (m_CurrentToken.type == RETURN_KEYWORD){
        ASTNode* n = m_parseReturnStatement();
        //m_eat(NEWLINE);
        return n;
    }
    else if (m_CurrentToken.type == NEWLINE){
        m_eat(NEWLINE);
        return NULL;
    }
    else {
        langError("Unexpected token in parseNode: " + m_CurrentToken.toString(), m_CurrentToken.line, m_CurrentToken.col);
    }
    return nullptr;
}

VariableAssignment* Parser::m_parseVariableAssignment() {
    std::string name = m_eat(IDENTIFIER).value;
    m_eat(EQUAL);
    Expression* value = m_parseExpression();
    return new VariableAssignment(name, value);
}
DataType* Parser::m_rawParseDataType(TOKENTYPE type){
    switch(type){
        case INTEGER_TYPE:
            return new IntegerType();
        case FLOAT_TYPE:
            //std::cout << "FLOAT TYPE" << std::endl;
            return new FloatType();
        case STRING_TYPE:
            return new StringType();
        case BOOL_TYPE:
            return new BoolType();
        case VOID_TYPE:
            return new VoidType();
        case FUNCTION_TYPE:
            return new FunctionType();
        default:
            return nullptr;
    }
}
DataType* Parser::m_parseDataType() {
    // Special case for lists
    if (m_peek().type == LBRACKET){
        //List type, form: type[]
        DataType* dt = m_rawParseDataType(m_eatAny(DATA_TYPES).type);
        m_eat(LBRACKET);
        m_eat(RBRACKET);
        return new ListType(dt);
    }

    Token t = m_eatAny(DATA_TYPES);
    TOKENTYPE type = t.type;

    DataType* d = m_rawParseDataType(type);
    if (d == nullptr) langError("Unimplemented data type: " + token_strings[type], t.line, t.col);
    return d;
}

ReturnStatement* Parser::m_parseReturnStatement() {
    m_eat(RETURN_KEYWORD);
    Expression* value = m_parseExpression();
    return new ReturnStatement(value);
}

std::vector<Expression*> Parser::m_parseFunctionCallArgs() {
    std::vector<Expression*> args = {};
    m_eat(LPAREN);
    while (m_CurrentToken.type != RPAREN) {
        args.push_back(m_parseExpression());
        m_skipWhitespace();
        m_mightEat(COMMA);
    
    }
    m_eat(RPAREN);
    return args;
}

VariableAccess* Parser::m_parseVariableAccess() {
    std::string name = m_eat(IDENTIFIER).value;
    std::vector<Expression*> args = {};
    //std::cout << "VAR ACCESS CURRENT TOKEN: " << m_CurrentToken.toString() << std::endl;
    if (m_CurrentToken.type == WITH_KEYWORD){
        m_eat(WITH_KEYWORD);
        args = m_parseFunctionCallArgs();
    }
    return new VariableAccess(name, args);
}

ListLiteral* Parser::m_parseListLiteral(){
    DataType* dataType = m_parseDataType();
    std::vector<Expression*> elements = {};
    m_eat(LBRACE);
    while (m_CurrentToken.type != RBRACE) {
        m_skipWhitespace();
        Expression* e = m_parseExpression();
        elements.push_back(e);
        if(m_CurrentToken.type != RBRACE) m_eat(COMMA);
        m_skipWhitespace();
    }
    m_eat(RBRACE);
    return new ListLiteral(elements, dataType);
}

Expression* Parser::m_handleDataTypeAtom(){
    // Save current index and token for later restoration
    int t_index = m_CurrentIndex;   

    // Parse datatype (to be able to determine if it's a list or function literal)
    m_parseDataType();
    ListLiteral* (Parser::*listFunc)() = nullptr;
    Function* (Parser::*funcFunc)() = nullptr;
    
    if (m_CurrentToken.type == LBRACE) {
        listFunc = &Parser::m_parseListLiteral;
    } else {
        funcFunc = &Parser::m_parseFunctionLiteral;
    }
    
    // Restore
    m_CurrentIndex = t_index;
    m_CurrentToken = m_Tokens[m_CurrentIndex];
    if (listFunc) {
        return (this->*listFunc)();
    } else if (funcFunc) {
        return (this->*funcFunc)();
    }

    langError("Unexpected token in HANDLEDATATYPEATOM: " + m_CurrentToken.toString(), m_CurrentToken.line, m_CurrentToken.col);
    return nullptr;
}

Expression* Parser::m_parseAtom() {
    if (m_CurrentToken.type == INTEGER){
        return new IntegerLiteral(std::stoi(m_eat(INTEGER).value));
    } 
    else if (m_CurrentToken.type == FLOAT){
        //std::cout << "FLOAT LITERAL GOT" << std::endl;
        return new FloatLiteral(std::stof(m_eat(FLOAT).value));
    }
    else if (m_CurrentToken.type == NEWLINE){
        m_eat(NEWLINE);
        return m_parseAtom();
    }
    else if (m_CurrentToken.type == CARAT){
        // Variable capture
        m_eat(CARAT);
        VariableAccess* acc = m_parseVariableAccess();
        VariableCaptureAccess* vcc = new VariableCaptureAccess(acc);
        m_functionCaptures[m_functionCaptures.size()-1].push_back(vcc);
        return vcc;
    }

    else if (m_CurrentToken.type == STRING){
        return new StringLiteral(m_eat(STRING).value);
    }
    else if (m_CurrentToken.type == BOOL){
        return new BooleanLiteral(m_eat(BOOL).value == "true");
    }
    else if (m_CurrentToken.type == IDENTIFIER){
        if (m_peek().type == EQUAL){
            return m_parseVariableAssignment();
        }
        return m_parseVariableAccess();
    }
    else if (std::find(DATA_TYPES.begin(), DATA_TYPES.end(), m_CurrentToken.type) != DATA_TYPES.end()){
        return m_handleDataTypeAtom();
    }
    else if (m_CurrentToken.type == LPAREN){
        m_advance();
        Expression* expr = m_parseExpression();
        m_eat(RPAREN);
        return expr;
    }
    else{
        langError("Unexpected token in PARSEATOM: " + m_CurrentToken.toString(), m_CurrentToken.line, m_CurrentToken.col);
    }
    return nullptr;
}

std::vector<ASTNode*> Parser::m_parseBlock() {
    std::vector<ASTNode*> statements;
    m_eat(LBRACE);
    m_skipWhitespace();
    while (m_CurrentToken.type != RBRACE) {
        ASTNode* node = m_parseNode();
        if (node != NULL) statements.push_back(node);
        m_skipWhitespace();
    }
    m_eat(RBRACE);
    return statements;
}

VariableDeclaration* Parser::m_parseVariableDeclaration() {
    m_eat(VAR_KEYWORD);
    DataType* data_type = m_parseDataType();
    std::string name = m_eat(IDENTIFIER).value;
    m_eat(EQUAL);
    Expression* value = m_parseExpression();
    m_functionMap[name] = value;
    return new VariableDeclaration(name, data_type, value);
}

IfStatement* Parser::m_parseIfStatement() {
    m_eat(IF_KEYWORD);
    m_eat(LPAREN);
    Expression* condition = m_parseExpression();
    m_eat(RPAREN);
    std::vector<ASTNode*> body = m_parseBlock();
    std::vector<ASTNode*> elseBody;
    m_skipWhitespace();
    //std::cout << "END OF IF: " << m_CurrentToken.toString() << std::endl;
    if (m_CurrentToken.type == ELSE_KEYWORD) {
        m_eat(ELSE_KEYWORD);
        if (m_CurrentToken.type == IF_KEYWORD){
            elseBody = {
                m_parseIfStatement()
            };
        }else{
            elseBody = m_parseBlock();
        }
    }
    return new IfStatement(condition, body, elseBody);
}