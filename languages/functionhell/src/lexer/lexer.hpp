#ifndef LEXER_H
#define LEXER_H

#include <string>
#include <map>
#include <vector>
#include <iostream>
#include <regex>
#include "./tokens/tokens.hpp"



class Lexer {
public:
    Lexer();
    std::vector<Token> tokenize(std::string source);
    std::vector<Token> tokens;
private:
    std::string m_Source;
    std::string m_CurrentToken;
    int m_CurrentIndex;
    int m_Line;
    int m_Column;
};

#endif