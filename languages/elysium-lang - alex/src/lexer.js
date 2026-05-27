class Lexer {
    constructor(input) {
        this.input = input;
        this.position = 0;
        this.currentChar = this.input[this.position];
    }

    advance() {
        this.position++;
        if (this.position < this.input.length) {
            this.currentChar = this.input[this.position];
        } else {
            this.currentChar = null;
        }
    }

    tokenize() {
        const tokens = [];
        while (this.currentChar !== null) {
            if (this.isWhitespace(this.currentChar)) {
                this.advance();
            } else if (this.isLetter(this.currentChar)) {
                tokens.push(this.identifier());
            } else if (this.isDigit(this.currentChar)) {
                tokens.push(this.number());
            } else if (this.currentChar === '=' && this.peek() === '=') {
                tokens.push(this.equalityOperator());
            } else if (this.currentChar === '!' && this.peek() === '=') {
                tokens.push(this.equalityOperator());
            } else if (this.currentChar === '=' || this.currentChar === '<' || this.currentChar === '>' || this.currentChar === '!') {
                tokens.push(this.relationalOperator());
            } else if (this.currentChar === '+') {
                tokens.push({ type: 'OPERATOR', value: '+' });
                this.advance();
            } else if (this.currentChar === '-') {
                tokens.push({ type: 'OPERATOR', value: '-' });
                this.advance();
            } else if (this.currentChar === '*') {
                tokens.push({ type: 'OPERATOR', value: '*' });
                this.advance();
            } else if (this.currentChar === '/') {
                tokens.push({ type: 'OPERATOR', value: '/' });
                this.advance();
            } else if (this.currentChar === '{' || this.currentChar === '}' || this.currentChar === '(' || this.currentChar === ')' || this.currentChar === ',') {
                tokens.push({ type: 'PUNCTUATION', value: this.currentChar });
                this.advance();
            } else {
                throw new Error(`Unexpected character: ${this.currentChar}`);
            }
        }
        return tokens;
    }

    peek() {
        return this.position + 1 < this.input.length ? this.input[this.position + 1] : null;
    }

    isWhitespace(char) {
        return /\s/.test(char);
    }

    isLetter(char) {
        return /[a-zA-Z]/.test(char);
    }

    isDigit(char) {
        return /[0-9]/.test(char);
    }

    identifier() {
        let result = '';
        while (this.currentChar !== null && this.isLetter(this.currentChar)) {
            result += this.currentChar;
            this.advance();
        }
        const type = this.isKeyword(result) ? 'KEYWORD' : 'IDENTIFIER';
        return { type, value: result };
    }

    isKeyword(word) {
        return ['let', 'print', 'while', 'if', 'else', 'function', 'return'].includes(word);
    }

    number() {
        let result = '';
        while (this.currentChar !== null && this.isDigit(this.currentChar)) {
            result += this.currentChar;
            this.advance();
        }
        return { type: 'NUMBER', value: Number(result) };
    }

    relationalOperator() {
        let result = this.currentChar;
        this.advance();
        if (this.currentChar === '=') {
            result += this.currentChar;
            this.advance();
        }
        return { type: 'OPERATOR', value: result };
    }

    equalityOperator() {
        let result = this.currentChar;
        this.advance();
        if (this.currentChar === '=') {
            result += this.currentChar;
            this.advance();
        }
        return { type: 'OPERATOR', value: result };
    }
}

module.exports = Lexer;
