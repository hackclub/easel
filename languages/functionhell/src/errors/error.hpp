#ifndef STD_H
#define STD_H
#include <string>
#include <iostream>
#include <vector>
extern std::string m_Source;
extern std::vector<std::string> m_SourceLines;

extern void langError(std::string message, int line, int col);

extern void setSource(std::string src);

#endif