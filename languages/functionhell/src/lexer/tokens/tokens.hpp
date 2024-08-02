#ifndef TOKENS_H
#define TOKENS_H

#include <string>
#include <map>
#include <vector>

enum TOKENTYPE {
    NEWLINE,
    WHITESPACE,
    END_OF_FILE,
    IDENTIFIER,
    INTEGER,
    INTEGER_TYPE,
    FLOAT,
    FLOAT_TYPE,
    STRING,
    STRING_TYPE,
    BOOL,
    BOOL_TYPE,
    VOID_TYPE,
    NONE_TYPE,
    LIST_TYPE,
    IF_KEYWORD,
    ELSE_KEYWORD,
    LBRACE,
    RBRACE,
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,
    SEMICOLON,
    COMMA,
    EQUAL,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    PERCENT,
    BANG,
    WITH_KEYWORD,
    AND_KEYWORD,
    OR_KEYWORD,
    COMMENT,
    EQUAL_EQUAL,
    BANG_EQUAL,
    LESS,
    LESS_EQUAL,
    GREATER,
    GREATER_EQUAL,
    VAR_KEYWORD,
    RETURN_KEYWORD,
    CAPTURED_KEYWORD,
    FUNCTION_TYPE,
    CARAT,
};

extern std::map<TOKENTYPE, std::string> token_strings;

extern std::map<TOKENTYPE, std::string> token_regexes;

extern std::vector<TOKENTYPE> token_priorities;



class Token {
public:
    TOKENTYPE type;
    std::string value;
    unsigned int line;
    unsigned int col;
    Token(TOKENTYPE type, std::string value, unsigned int line, unsigned int col);
    Token();
    std::string toString();
};

extern std::vector<TOKENTYPE> DATA_TYPES;


extern std::vector<TOKENTYPE> ATOMS;

#endif // TOKENS_H
