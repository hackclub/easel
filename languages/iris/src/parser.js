import { Tokens } from "./lexer.js";

export const Types = {
  Text: "Text",
  Section: "Section",
  Choice: "Choice",
  Diversion: "Diversion",
  Var: "Var",
  If: "If",
};

export class Text {
  constructor(content) {
    this.type = Types["Text"];
    this.content = content;
  }
}

export class Choice {
  constructor(content, body) {
    this.type = Types["Choice"];
    this.content = content;
    this.body = body;
  }
}

export class Section {
  constructor(name, body) {
    this.type = Types["Section"];
    this.name = name;
    this.body = body;
  }
}

export class Diversion {
  constructor(section) {
    this.type = Types["Diversion"];
    this.section = section;
  }
}

export class Var {
  constructor(name, value) {
    this.type = Types["Var"];
    this.name = name;
    this.value = value;
  }
}

export class If {
  constructor(condition, body) {
    this.type = Types["If"];
    this.condition = condition;
    this.body = body;
  }
}

export class Parser {
  constructor(tokens) {
    this.tokens = tokens;
    this.ast = [];
    this.current = 0;

    this.errored = false;
  }

  error(msg, line) {
    console.error(`Error: ${msg} at the starting of line ${line}.\n`);
    this.errored = true;
  }

  peek() {
    if (this.current >= this.tokens.length) return "\0";
    return this.tokens[this.current];
  }

  advance() {
    if (this.current >= this.tokens.length) return "\0";
    return this.tokens[this.current++];
  }

  textStatement(token, inline) {
    let content = token.content;

    let line = token.line;
    while (this.peek().type === Tokens["Eol"] && !inline) {
      if (this.current < this.tokens.length) {
        this.advance();
        if (this.tokens[this.current].type === Tokens["Text"]) {
          if (this.tokens[this.current].line - line <= 1) {
            content += " " + this.tokens[this.current].content;
            line = this.tokens[this.current].line;
            this.advance();
          } else {
            break;
          }
        }
      }
    }

    return new Text(content);
  }

  choiceStatement(token) {
    let content = "";
    let body = [];

    if (this.peek().type === Tokens["Text"]) {
      content = this.textStatement(this.advance(), true).content;
    } else
      this.error(`Expected 'Text' but got '${this.peek().type}'`, token.line);

    while (
      this.peek().type !== Tokens["Eof"] &&
      this.peek().type !== Tokens["Section"] &&
      this.peek().type !== Tokens["Choice"]
    ) {
      if (this.peek().type === Tokens["Text"]) {
        body.push(this.textStatement(this.advance(), false));
      } else if (this.peek().type === Tokens["Choice"]) {
        body.push(this.choiceStatement(this.advance()));
      } else if (this.peek().type === Tokens["Diversion"]) {
        body.push(this.diversionStatement(this.advance()));
      } else if (this.peek().type === Tokens["Var"]) {
        body.push(this.varStatement(this.advance()));
      } else if (this.peek().type === Tokens["If"]) {
        body.push(this.ifStatement(this.advance()));
      } else this.advance();
    }

    return new Choice(content, body);
  }

  sectionStatement(token) {
    let name = "";
    let body = [];

    if (this.peek().type === Tokens["Text"])
      name = this.textStatement(this.advance(), true).content;
    else
      this.error(`Expected 'Text' but got '${this.peek().type}'`, token.line);

    while (
      this.peek().type !== Tokens["Eof"] &&
      this.peek().type !== Tokens["Section"]
    ) {
      if (this.peek().type === Tokens["Text"]) {
        body.push(this.textStatement(this.advance(), false));
      } else if (this.peek().type === Tokens["Choice"]) {
        body.push(this.choiceStatement(this.advance()));
      } else if (this.peek().type === Tokens["Diversion"]) {
        body.push(this.diversionStatement(this.advance()));
      } else if (this.peek().type === Tokens["Var"]) {
        body.push(this.varStatement(this.advance()));
      } else if (this.peek().type === Tokens["If"]) {
        body.push(this.ifStatement(this.advance()));
      } else this.advance();
    }

    function isValidSectionName(name) {
      if (!name || typeof name !== "string") return false;

      const firstCharRegex = /^[a-zA-Z_$]/;
      if (!firstCharRegex.test(name[0])) return false;

      const allowedCharsRegex = /^[a-zA-Z0-9_$]*$/;
      return allowedCharsRegex.test(name);
    }

    if (isValidSectionName(name)) return new Section(name, body);
    else this.error(`Invalid section name`, token.line);
  }

  diversionStatement(token) {
    if (this.peek().type === Tokens["Text"]) {
      return new Diversion(this.textStatement(this.advance(), true).content);
    } else
      this.error(`Expected 'Text' but got '${this.peek().type}'`, token.line);

    return new Diversion("");
  }

  varStatement(token) {
    let name, value;
    if (this.peek().content.split("=").length == 2) {
      name = this.peek().content.split("=")[0].trim();
      value = this.peek().content.split("=")[1].trim();
    } else this.error("Variable declarment error", token.line);
    this.advance();

    function isValidVarName(name) {
      if (!name || typeof name !== "string") return false;

      const firstCharRegex = /^[a-zA-Z_$]/;
      if (!firstCharRegex.test(name[0])) return false;

      const allowedCharsRegex = /^[a-zA-Z0-9_$]*$/;
      return allowedCharsRegex.test(name);
    }

    if (isValidVarName(name)) return new Var(name, value);
    else this.error(`Invalid variable name`, token.line);
  }

  ifStatement(token) {
    let condition = this.peek().content;
    this.advance();

    let body = [];

    let hadLeftBracket = false;
    hadLeftBracket = condition.trim()[condition.trim().length - 1] === "{";

    while (this.peek().type === Tokens["Eol"]) this.advance();

    if (this.peek().type === Tokens["LeftBracket"] || hadLeftBracket) {
      while (this.peek().type !== Tokens["RightBracket"]) {
        if (
          this.peek().type === Tokens["Section"] ||
          this.peek().type === Tokens["Eof"]
        ) {
          this.error(`Missing close bracket for if statement`, token.line);
          break;
        }

        if (this.peek().type === Tokens["Text"]) {
          body.push(this.textStatement(this.advance(), false));
        } else if (this.peek().type === Tokens["Choice"]) {
          body.push(this.choiceStatement(this.advance()));
        } else if (this.peek().type === Tokens["Diversion"]) {
          body.push(this.diversionStatement(this.advance()));
        } else if (this.peek().type === Tokens["Var"]) {
          body.push(this.varStatement(this.advance()));
        } else if (this.peek().type === Tokens["If"]) {
          body.push(this.ifStatement(this.advance()));
        } else this.advance();
      }
    } else this.error(`Missing open bracket for if statement`, token.line);
    this.advance();

    return new If(condition, body);
  }

  parse() {
    while (
      this.peek().type !== Tokens["Eof"] &&
      this.peek().type !== undefined
    ) {
      let token = this.advance();

      switch (token.type) {
        case Tokens["Text"]: {
          this.ast.push(this.textStatement(token, false));
          break;
        }
        case Tokens["Choice"]: {
          this.ast.push(this.choiceStatement(token));
          break;
        }
        case Tokens["Section"]: {
          this.ast.push(this.sectionStatement(token));
          break;
        }
        case Tokens["Diversion"]: {
          this.ast.push(this.diversionStatement(token));
          break;
        }
        case Tokens["Var"]: {
          this.ast.push(this.varStatement(token));
          break;
        }
        case Tokens["If"]: {
          console.log("!!!");
          this.ast.push(this.ifStatement(token));
          break;
        }
      }
    }
  }
}
