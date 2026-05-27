class Parser {
    constructor(tokens) {
        this.tokens = tokens;
        this.position = 0;
    }

    parse() {
        console.log("Parsing tokens");
        const ast = [];
        while (this.position < this.tokens.length) {
            console.log("Current token position:", this.position);
            ast.push(this.parseStatement());
        }
        console.log("AST:", JSON.stringify(ast, null, 2));
        return ast;
    }

    parseStatement() {
        const token = this.tokens[this.position];
        console.log("Parsing statement:", token);
        if (token.type === 'KEYWORD' && token.value === 'let') {
            return this.parseDeclaration();
        } else if (token.type === 'IDENTIFIER') {
            return this.parseAssignment();
        } else if (token.type === 'KEYWORD' && token.value === 'print') {
            return this.parsePrintStatement();
        } else if (token.type === 'KEYWORD' && token.value === 'while') {
            return this.parseWhileStatement();
        } else if (token.type === 'KEYWORD' && token.value === 'if') {
            return this.parseIfStatement();
        } else if (token.type === 'KEYWORD' && token.value === 'function') {
            return this.parseFunctionDeclaration();
        }
        throw new Error(`Unexpected token type at position ${this.position}: ${token.type}`);
    }

    parseDeclaration() {
        console.log("Parsing declaration");
        this.position++; // Consume 'let'
        const identifierToken = this.tokens[this.position++];
        if (identifierToken.type !== 'IDENTIFIER') {
            throw new Error(`Expected identifier after 'let', but got ${identifierToken.type}`);
        }
        const identifier = identifierToken.value;
        console.log("Parsed declaration identifier:", identifier);

        const equalToken = this.tokens[this.position++];
        if (equalToken.type !== 'OPERATOR' || equalToken.value !== '=') {
            throw new Error("Expected '=' after identifier in declaration");
        }

        const value = this.parseExpression();
        console.log("Parsed declaration value:", value);
        return { type: 'Declaration', identifier, value };
    }

    parseAssignment() {
        console.log("Parsing assignment");
        const identifier = this.tokens[this.position++].value;
        console.log("Parsed identifier:", identifier);

        const equalToken = this.tokens[this.position++];
        if (equalToken.type !== 'OPERATOR' || equalToken.value !== '=') {
            throw new Error("Expected '=' after identifier");
        }

        const value = this.parseExpression();
        console.log("Parsed assignment value:", value);
        return { type: 'Assignment', identifier, value };
    }

    parsePrintStatement() {
        console.log("Parsing print statement");
        this.position++; // Consume 'print'
        const value = this.parseExpression();
        console.log("Parsed print value:", value);
        return { type: 'PrintStatement', value };
    }

    parseWhileStatement() {
        console.log("Parsing while statement");
        this.position++; // Consume 'while'
        const condition = this.parseExpression();
        this.expectPunctuation('{');
        const body = [];
        while (this.tokens[this.position].value !== '}') {
            body.push(this.parseStatement());
        }
        this.expectPunctuation('}');
        console.log("Parsed while statement condition:", condition);
        console.log("Parsed while statement body:", body);
        return { type: 'WhileStatement', condition, body };
    }

    parseIfStatement() {
        console.log("Parsing if statement");
        this.position++; // Consume 'if'
        const condition = this.parseExpression();
        this.expectPunctuation('{');
        const consequent = [];
        while (this.tokens[this.position].value !== '}') {
            consequent.push(this.parseStatement());
        }
        this.expectPunctuation('}');
        let alternate = null;
        if (this.tokens[this.position] && this.tokens[this.position].value === 'else') {
            alternate = this.parseElseStatement();
        }
        console.log("Parsed if statement condition:", condition);
        console.log("Parsed if statement consequent:", consequent);
        console.log("Parsed if statement alternate:", alternate);
        return { type: 'IfStatement', condition, consequent, alternate };
    }

    parseElseStatement() {
        console.log("Parsing else statement");
        this.position++; // Consume 'else'
        this.expectPunctuation('{');
        const alternate = [];
        while (this.tokens[this.position].value !== '}') {
            alternate.push(this.parseStatement());
        }
        this.expectPunctuation('}');
        console.log("Parsed else statement body:", alternate);
        return alternate;
    }

    parseFunctionDeclaration() {
        console.log("Parsing function declaration");
        this.position++; // Consume 'function'
        const nameToken = this.tokens[this.position++];
        if (nameToken.type !== 'IDENTIFIER') {
            throw new Error('Expected function name');
        }
        const name = nameToken.value;
        console.log("Parsed function name:", name);
        this.expectPunctuation('(');
        const parameters = [];
        while (this.tokens[this.position].type !== 'PUNCTUATION' || this.tokens[this.position].value !== ')') {
            const paramToken = this.tokens[this.position++];
            if (paramToken.type !== 'IDENTIFIER') {
                throw new Error('Expected parameter name');
            }
            parameters.push(paramToken.value);
            if (this.tokens[this.position].type === 'PUNCTUATION' && this.tokens[this.position].value === ',') {
                this.position++;
            }
        }
        this.expectPunctuation(')');
        this.expectPunctuation('{');
        const body = [];
        while (this.tokens[this.position].type !== 'PUNCTUATION' || this.tokens[this.position].value !== '}') {
            body.push(this.parseStatement());
        }
        this.expectPunctuation('}');
        console.log("Parsed function parameters:", parameters);
        console.log("Parsed function body:", body);
        return { type: 'FunctionDeclaration', name, parameters, body };
    }

    expectPunctuation(char) {
        const token = this.tokens[this.position++];
        if (token.type !== 'PUNCTUATION' || token.value !== char) {
            throw new Error(`Expected punctuation: '${char}', but got '${token.value}'`);
        }
    }

    parseExpression() {
        let left = this.parseTerm();
        while (this.tokens[this.position] && this.tokens[this.position].type === 'OPERATOR' && ['+', '-', '<', '>', '<=', '>=', '==', '!='].includes(this.tokens[this.position].value)) {
            const operator = this.tokens[this.position].value;
            this.position++;
            const right = this.parseTerm();
            left = { type: 'BinaryExpression', left, operator, right };
            console.log("Parsed binary expression:", left);
        }
        return left;
    }

    parseTerm() {
        let left = this.parseFactor();
        while (this.tokens[this.position] && this.tokens[this.position].type === 'OPERATOR' && ['*', '/'].includes(this.tokens[this.position].value)) {
            const operator = this.tokens[this.position].value;
            this.position++;
            const right = this.parseFactor();
            left = { type: 'BinaryExpression', left, operator, right };
            console.log("Parsed binary expression:", left);
        }
        return left;
    }

    parseFactor() {
        const token = this.tokens[this.position++];
        console.log("Parsing factor:", token);
        if (token.type === 'NUMBER') {
            return { type: 'Literal', value: token.value };
        } else if (token.type === 'IDENTIFIER') {
            return { type: 'Identifier', name: token.value };
        } else if (token.type === 'PUNCTUATION' && token.value === '(') {
            const expression = this.parseExpression();
            this.expectPunctuation(')');
            return expression;
        }
        throw new Error(`Unexpected token type at position ${this.position - 1}: ${token.type}`);
    }
}

module.exports = Parser;
