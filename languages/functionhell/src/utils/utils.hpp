#ifndef UTILS_H
#define UTILS_H

#include <string>
#include <vector>
#include <map>
#include <iostream>

#include "../parser/ast/ASTNodes.hpp"

template <typename T>
bool util_isType(Node* node) {
    return dynamic_cast<T*>(node) != nullptr;
}

#endif