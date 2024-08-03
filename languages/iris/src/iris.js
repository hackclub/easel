import { Lexer } from "./lexer.js";
import { Parser } from "./parser.js";
import { CodeGen } from "./codegen.js";

import * as fs from "node:fs";

export class Iris {
  constructor(src) {
    this.src = src;
  }

  run(filename) {
    const lexer = new Lexer(this.src);
    lexer.scan();

    const parser = new Parser(lexer.tokens);
    parser.parse();

    const codegen = new CodeGen(parser.ast);
    if (!parser.errored) {
      codegen.generate();
      try {
        fs.writeFileSync(`./${filename}.html`, codegen.html);
      } catch (err) {
        console.error(err);
      }
    }
  }
}

for (let i = 2; i < process.argv.length; i++) {
  try {
    const src = fs.readFileSync(process.argv[i], "utf-8");
    const iris = new Iris(src);

    var filename = process.argv[i].replace(/^.*[\\/]/, '');
    iris.run(filename);
  } catch (err) {
    console.error(err);
  }
}
