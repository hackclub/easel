#include <iostream>
#include <fstream>
#include "./lexer/lexer.hpp"
#include "./parser/parser.hpp"
#include "./errors/error.hpp"
#include "./transpiler/transpiler.hpp"
#include "./typeChecker/typeChecker.hpp"
//#include "./LLVMCompiler/compiler.hpp"

std::string readFile(const char* filename)
{
    std::ifstream file(filename);
    if (!file.is_open()) {
        std::cerr << "Could not open file: " << filename << std::endl;
        exit(1);
    }
    std::string source((std::istreambuf_iterator<char>(file)), std::istreambuf_iterator<char>());
    file.close();
    return source;
}


void printHelp(char** argv){
    std::cout << "Usage: " << argv[0] << " <filename>" << " -o" << " <output_file>" << std::endl;
    
}

void runCMD(const char* cmd){
    std::cout << cmd << std::endl;
    system(cmd);
}
std::vector<Token> runTokenizer(std::string source){
    Lexer lexer;
    return lexer.tokenize(source);
}
Program* runParser(std::vector<Token> tokens){
    Parser parser;
    Program* program = parser.parse(tokens);
    return program;
}

void transpileAST(Program* program, const char* output_file, bool runOutput){
    Transpiler transpiler;
    std::cout << "Transpiling, output file = " << output_file << std::endl;
    std::string output = transpiler.transpile(program->statements);

    std::ofstream file(output_file);
    file << output;
    file.close();
    if (runOutput ){
        runCMD(("node " + std::string(output_file)).c_str());
    } 
}

//void compileAST(Program* program, const char* output_file){
//    LLVMCompiler compiler;
//    compiler.compile(program);
//    compiler.printModule();
//    compiler.writeToFile(output_file);
//}

int main(int argc, char** argv) {
    char* output_file = "output.js";
    bool runOutput = false;
    bool doTranspile = true;
    bool doCompile = !doTranspile;
    bool printTokens = false;
    bool printAST = false;
    bool doTypeCheck = true;
    bool doLog = false;
    if (argc < 2 || argc == 0) { printHelp(argv); exit(1); }
    for(int i = 0; i < argc; i++){
        if (strcmp(argv[i], "-h") == 0 || strcmp(argv[i], "--help") == 0){
            printHelp(argv);
            exit(0);
        }
        else if(strcmp(argv[i], "-o") == 0){
            output_file = argv[i+1];
            
        }
        else if(strcmp(argv[i], "-r") == 0){
            runOutput = true;
        }
        else if (strcmp(argv[i], "--noTranspile") == 0){
            doTranspile = false;
            std::cout << "Transpilation disabled" << std::endl;
        }
        else if (strcmp(argv[i], "--noCompile") == 0){
            doCompile = false;
            std::cout << "Compilation disabled" << std::endl;
        }
        else if (strcmp(argv[i], "--printTokens") == 0){
            printTokens = true;
        }
        else if (strcmp(argv[i], "--printAST") == 0){
            printAST = true;
        }
        else if (strcmp(argv[i], "--noTypeCheck") == 0){
            doTypeCheck = false;
        }
        else if (strcmp(argv[i], "--log") == 0){
            doLog = true;
        }
    }
    std::string source = readFile(argv[1]);
    setSource(source);
    std::vector<Token> tokens = runTokenizer(source);
    if (printTokens){
        for (Token t : tokens){
            std::cout << t.toString() << std::endl;
        }
    }
    if (doLog) {
        std::cout << "Got " << tokens.size() << " tokens" << "\n" << "Parsing..." << std::endl;
    }
    
    Program* program = runParser(tokens);
    if (printAST){
        for (ASTNode* node : program->statements){
            std::cout << node->toString() << std::endl;
        }
    }
    if (doLog) std::cout << "Parsed " << program->statements.size() << " statements" << "\n" << "Running type checker..." << std::endl;
    if (doTypeCheck){
        TypeChecker typeChecker;
        int r = typeChecker.checkTypes(program->statements, nullptr, nullptr);
        if (r != 0) {
            std::cout << "Type checking failed..." << std::endl;
            return 1;
        }
    }


    std::cout << "Type checking passed" << "\n" << "Transpiling..." << std::endl;
    if (doTranspile) transpileAST(program, output_file, runOutput);
    //if (doCompile) compileAST(program, output_file);


    delete program;
    return 0;
}


