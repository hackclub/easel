class Interpreter {
    constructor() {
        this.environment = {};
        this.functions = {};
    }

    evaluate(node) {
        switch (node.type) {
            case 'Declaration':
                this.environment[node.identifier] = this.evaluate(node.value);
                break;
            case 'Assignment':
                if (!(node.identifier in this.environment)) {
                    throw new Error(`Undefined variable ${node.identifier}`);
                }
                this.environment[node.identifier] = this.evaluate(node.value);
                break;
            case 'BinaryExpression':
                return this.evaluateBinaryExpression(node);
            case 'Identifier':
                if (!(node.name in this.environment)) {
                    throw new Error(`Undefined variable ${node.name}`);
                }
                return this.environment[node.name];
            case 'Literal':
                return node.value;
            case 'PrintStatement':
                const value = this.evaluate(node.value);
                console.log(value);
                return value;
            case 'WhileStatement':
                return this.evaluateWhileStatement(node);
            case 'IfStatement':
                return this.evaluateIfStatement(node);
            case 'FunctionDeclaration':
                this.functions[node.name] = node;
                break;
            case 'ReturnStatement':
                return this.evaluate(node.value);
            default:
                throw new Error(`Unknown node type: ${node.type}`);
        }
    }

    evaluateBinaryExpression(node) {
        const left = this.evaluate(node.left);
        const right = this.evaluate(node.right);
        switch (node.operator) {
            case '+':
                return left + right;
            case '-':
                return left - right;
            case '*':
                return left * right;
            case '/':
                return left / right;
            case '<':
                return left < right;
            case '>':
                return left > right;
            case '==':
                return left == right;
            case '<=':
                return left <= right;
            case '>=':
                return left >= right;
            case '!=':
                return left != right;
            default:
                throw new Error(`Unknown operator: ${node.operator}`);
        }
    }

    evaluateWhileStatement(node) {
        while (this.evaluate(node.condition)) {
            for (const statement of node.body) {
                this.evaluate(statement);
            }
        }
    }

    evaluateIfStatement(node) {
        if (this.evaluate(node.condition)) {
            for (const statement of node.consequent) {
                this.evaluate(statement);
            }
        } else if (node.alternate) {
            for (const statement of node.alternate) {
                this.evaluate(statement);
            }
        }
    }

    execute(ast) {
        ast.forEach(node => {
            console.log("Executing node:", node);
            this.evaluate(node);
        });
        console.log("Final environment:", this.environment);
        console.log("Values of variables:", this.environment);
    }
}

module.exports = Interpreter;
