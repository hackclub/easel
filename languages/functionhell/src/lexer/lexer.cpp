#include "lexer.hpp"

Lexer::Lexer() {
    m_CurrentIndex = 0;
    m_Line = 1;
    m_Column = 1;
}

int stringFind(std::string str, const char* tofind){
    int fn = str.find(tofind);
    return fn <= str.length() ? fn : -1;
}

bool isIDChar(char test){
    return isalnum(test) || test == '_';
}

std::vector<Token> Lexer::tokenize(std::string source) {
    m_Source = source;
    tokens.clear();
    std::smatch match;

    while (m_CurrentIndex < m_Source.size()) {
        bool matched = false;

        for (TOKENTYPE tokenType : token_priorities) {
            std::regex regex(token_regexes[tokenType]);
            std::string substring = m_Source.substr(m_CurrentIndex);

            if (std::regex_search(substring, match, regex, std::regex_constants::match_continuous)) {
                if (tokenType != WHITESPACE && tokenType != COMMENT) { // Skip whitespaces and comments
                    if (stringFind(token_strings[tokenType], "KEYWORD") != -1 || 
                        stringFind(token_strings[tokenType], "TYPE") != -1){
                        // Got a keyword now we need to check if the next char is not alnum
                        if (isIDChar(m_Source[m_CurrentIndex + match.length()])){
                            // Redo with next
                            continue;
                        }
                    }
                    tokens.emplace_back(Token(tokenType, match.str(), m_Line, m_Column));
                    if(tokenType == NEWLINE) {
                        m_Line += 1;
                        m_Column = 1;
                    }
                }

                m_CurrentIndex += match.length();
                m_Column += match.length();
                matched = true;
                break;
            }
        }

        if (!matched) {
            std::cerr << "Unexpected character at line " << m_Line << ", column " << m_Column << ": " << m_Source[m_CurrentIndex] << std::endl;
            ++m_CurrentIndex;
            ++m_Column;
        }
    }

    tokens.emplace_back(Token(END_OF_FILE, "", m_Line, m_Column));
    return tokens;
}