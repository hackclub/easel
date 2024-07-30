export const Tokens = {
  Text: "Text",
  Choice: "Choice",
  Section: "Section",
  Diversion: "Diversion",

  Var: "Var",
  Equal: "Equal",

  If: "If",
  LeftBracket: "LeftBracket",
  RightBracket: "RightBracket",

  Eol: "Eol",
  Eof: "Eof",
};

export class Token {
  constructor(type, content, line) {
    this.type = type;
    this.content = content;
    this.line = line;
  }
}

export class Lexer {
  constructor(source = "") {
    this.source = source;
    this.tokens = [];
    this.current = 0;
    this.line = 1;
  }

  peek() {
    if (this.current >= this.source.length) return "\0";
    return this.source[this.current];
  }

  advance() {
    if (this.current >= this.source.length) return "\0";
    return this.source[this.current++];
  }

  scan() {
    const text = (char) => {
      let result = char; // Starting char
      while (this.peek() !== "\0" && this.peek() !== "\n") {
        result += this.advance();
      }
      return result;
    };

    while (this.peek() !== "\0") {
      let char = this.advance();

      switch (char) {
        case "\n": {
          this.tokens.push(new Token(Tokens["Eol"], "\n", this.line));
          this.line++;
          break;
        }
        case " ":
          break;
        case "\t":
          break;
        case "#": {
          while (this.peek() !== "\n" && this.peek() !== "\0") {
            this.advance();
          }
          break;
        }
        case "+": {
          this.tokens.push(new Token(Tokens["Choice"], "+", this.line));
          break;
        }
        case "-": {
          this.tokens.push(new Token(Tokens["Section"], "-", this.line));
          break;
        }
        case ">": {
          this.tokens.push(new Token(Tokens["Diversion"], ">", this.line));
          break;
        }
        case "~": {
          this.tokens.push(new Token(Tokens["Var"], "~", this.line));
          break;
        }
        case "=": {
          this.tokens.push(new Token(Tokens["Equal"], "=", this.line));
          break;
        }
        case "?": {
          this.tokens.push(new Token(Tokens["If"], "?", this.line));
          break;
        }
        case "{": {
          this.tokens.push(new Token(Tokens["LeftBracket"], "{", this.line));
          break;
        }
        case "}": {
          this.tokens.push(new Token(Tokens["RightBracket"], "}", this.line));
          break;
        }
        default: {
          let c = text(char);
          this.tokens.push(new Token(Tokens["Text"], c, this.line));
        }
      }
    }

    this.tokens.push(new Token(Tokens["Eof"], "\0", this.line));
  }
}
