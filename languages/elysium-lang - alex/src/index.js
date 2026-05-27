const fs = require('fs');
const Lexer = require('./lexer');
const Parser = require('./parser');
const Interpreter = require('./interpreter');

// Read the input file
const input = fs.readFileSync('./test/test.ely', 'utf8');

// Tokenize the input
try {
    const lexer = new Lexer(input);
    const tokens = lexer.tokenize();
    console.log("Tokens:", tokens);

    // Parse the tokens
    const parser = new Parser(tokens);
    const ast = parser.parse();
    console.log("AST:", JSON.stringify(ast, null, 2));

    // Interpret the AST
    const interpreter = new Interpreter();
    interpreter.execute(ast);
} catch (error) {
    console.error("Error:", error.message);
    console.error(error.stack);
}
