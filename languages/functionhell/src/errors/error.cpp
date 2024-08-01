#include "error.hpp"

std::string m_Source = "";
std::vector<std::string> m_SourceLines;

void langError(std::string message, int line, int col) {
    if (line == -1 && col == -1) {
        std::cerr << "\e[1;31m" << "Error: " << message << "\e[0m" << std::endl;
        exit(1);
    }
    if (m_Source != "") {
        std::cerr << "\e[1;31m" << "Error: " << message << " at line " << line << " column " << col << std::endl;
        std::cerr << m_SourceLines[line-1] << std::endl;
        std::cerr << std::string(col-1, ' ') << "^" << "\e[0m" << std::endl;
    } else{
        std::cerr << "\e[1;31m" << "Error: " << message << " at line " << line << " column " << col << "\e[0m" << std::endl;
    }
    exit(1);
}


void setSource(std::string src) {
    m_Source = src;
    m_SourceLines.clear();
    std::string line = "";
    for (char c : m_Source) {
        if (c == '\n') {
            m_SourceLines.push_back(line);
            line = "";
        } else {
            line += c;
        }
    }
    m_SourceLines.push_back(line);
}