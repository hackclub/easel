package space.themelon.eia64.analysis

import space.themelon.eia64.Config
import space.themelon.eia64.Expression
import space.themelon.eia64.syntax.Flag
import space.themelon.eia64.syntax.Token
import space.themelon.eia64.syntax.Type

class Parser {

    private val nameResolver = NameResolver()

    private lateinit var tokens: List<Token>
    private var index = 0
    private var size = 0

    fun parse(tokens: List<Token>): Expression.ExpressionList {
        index = 0
        size = tokens.size
        this.tokens = tokens

        val expressions = ArrayList<Expression>()
        while (!isEOF()) expressions.add(parseNext())
        if (Config.DEBUG) expressions.forEach { println(it) }
        return Expression.ExpressionList(expressions)
    }

    // make sure to update canParseNext() when we add stuff here!
    private fun parseNext(): Expression {
        val token = next()
        if (token.flags.isNotEmpty()) {
            when (token.flags[0]) {
                Flag.LOOP -> return loop(token)
                Flag.V_KEYWORD -> return variableDeclaration(token)
                Flag.INTERRUPTION -> return interruption(token)
                else -> {}
            }
        }
        return when (token.type) {
            Type.IF -> ifDeclaration()
            Type.FUN -> fnDeclaration()
            Type.SHADO -> shadoDeclaration()
            Type.STDLIB -> importStdLib()
            Type.WHEN -> whenStatement(token)
            else -> {
                back()
                return parseExpr(0)
            }
        }
    }

    private fun canParseNext(): Boolean {
        val token = peek()
        if (token.flags.isNotEmpty())
            token.flags[0].let {
                if (it == Flag.LOOP || it == Flag.V_KEYWORD || it == Flag.INTERRUPTION)
                    return true
            }
        return when (token.type) {
            Type.IF, Type.FUN, Type.STDLIB, Type.SHADO, Type.WHEN -> true
            else -> false
        }
    }

    private fun importStdLib(): Expression.ImportStdLib {
        expectType(Type.OPEN_CURVE)
        val imports = ArrayList<String>()
        while (true) {
            val className = readAlpha()
            imports.add(className)
            nameResolver.classes.add(className)
            if (peek().type == Type.CLOSE_CURVE) {
                skip()
                break
            }
            expectType(Type.COMMA)
        }
        return Expression.ImportStdLib(imports)
    }

    private fun whenStatement(token: Token): Expression {
        expectType(Type.OPEN_CURVE)
        val expr = parseNext()
        expectType(Type.CLOSE_CURVE)

        // Scope: Automatic
        fun readStatement(): Expression.Scope {
            expectType(Type.RIGHT_ARROW)
            return autoScopeBody()
        }

        expectType(Type.OPEN_CURLY)
        val matches = ArrayList<Pair<Expression, Expression.Scope>>()
        while (true) {
            val p = peek()
            if (p.type == Type.ELSE) break
            if (p.type == Type.CLOSE_CURLY) {
                token.error<String>("Expected else branch for the when statement")
            }
            val match = parseNext()
            matches.add(match to readStatement())
        }
        expectType(Type.ELSE)
        val elseBranch = readStatement()
        expectType(Type.CLOSE_CURLY)

        if (matches.isEmpty()) {
            token.error<String>("When statement cannot be empty")
        }
        return Expression.When(expr, matches, elseBranch)
    }

    private fun loop(token: Token): Expression {
        when (token.type) {
            Type.UNTIL -> {
                expectType(Type.OPEN_CURVE)
                val expr = parseNext()
                expectType(Type.CLOSE_CURVE)
                // Scope: Automatic
                return Expression.Until(expr, autoBodyExpr())
            }

            Type.FOR -> {
                // we cannot expose initializers outside the for loop
                nameResolver.enterScope()
                expectType(Type.OPEN_CURVE)
                val initializer = if (isNext(Type.COMMA)) null else parseNext()
                expectType(Type.COMMA)
                val conditional = if (isNext(Type.COMMA)) null else parseNext()
                expectType(Type.COMMA)
                val operational = if (isNext(Type.CLOSE_CURVE)) null else parseNext()
                expectType(Type.CLOSE_CURVE)
                // double layer scope wrapping
                val body = autoBodyExpr() // Scope: Automatic
                nameResolver.leaveScope()
                return Expression.ForLoop(
                    initializer,
                    conditional,
                    operational,
                    body)
            }

            Type.ITR -> {
                expectType(Type.OPEN_CURVE)
                val iName = readAlpha()
                if (isNext(Type.COLON)) {
                    skip()
                    val from = parseNext()
                    expectType(Type.TO)
                    val to = parseNext()

                    var by: Expression? = null
                    if (isNext(Type.BY)) {
                        skip()
                        by = parseNext()
                    }
                    expectType(Type.CLOSE_CURVE)
                    nameResolver.enterScope()
                    nameResolver.defineVr(iName)
                    // Manual Scopped!
                    val body = unscoppedBodyExpr()
                    nameResolver.leaveScope()
                    return Expression.Itr(iName, from, to, by, body)
                } else {
                    expectType(Type.IN)
                    val entity = parseNext()
                    expectType(Type.CLOSE_CURVE)
                    nameResolver.enterScope()
                    nameResolver.defineVr(iName)
                    // Manual Scopped!
                    val body = unscoppedBodyExpr()
                    nameResolver.leaveScope()
                    return Expression.ForEach(iName, entity, body)
                }
            }

            else -> return token.error("Unknown loop type symbol")
        }
    }

    private fun interruption(token: Token) = Expression.Interruption(
        Expression.Operator(token.type),
        when (token.type) {
            Type.RETURN -> parseNext()
            Type.USE -> parseNext()
            else -> null
        }
    )

    private fun fnDeclaration(): Expression {
        val name = readAlpha()

        // create a wrapper object, that can be set to actual value later
        val fnElement = FnElement()
        nameResolver.defineFn(name, fnElement)
        nameResolver.enterScope()

        expectType(Type.OPEN_CURVE)
        val requiredArgs = ArrayList<Expression.DefinitionType>()
        while (!isEOF() && peek().type != Type.CLOSE_CURVE) {
            val parameterName = readAlpha()
            nameResolver.defineVr(parameterName)
            expectType(Type.COLON)
            val clazz = readClassType()

            requiredArgs.add(Expression.DefinitionType(parameterName, clazz))
            if (!isNext(Type.COMMA)) break
            skip()
        }
        fnElement.argsSize = requiredArgs.size
        expectType(Type.CLOSE_CURVE)
        val returnType = if (isNext(Type.COLON)) {
            skip()
            readClassType()
        } else Type.E_ANY

        val body = unitBody() // Fully Manual Scopped
        nameResolver.leaveScope()
        val fnExpr = Expression.Function(name, requiredArgs, returnType, body)
        fnElement.fnExpression = fnExpr
        return fnExpr
    }

    private fun shadoDeclaration(): Expression.Shadow {
        val names = ArrayList<String>()

        nameResolver.enterScope()
        expectType(Type.OPEN_CURVE)
        while (!isEOF() && peek().type != Type.CLOSE_CURVE) {
            val name = readAlpha()
            nameResolver.defineVr(name)
            names.add(name)
            if (!isNext(Type.COMMA)) break
            skip()
        }
        expectType(Type.CLOSE_CURVE)
        val body = unitBody() // Fully Manual Scopped
        nameResolver.leaveScope()
        return Expression.Shadow(names, body)
    }

    private fun unitBody() = if (isNext(Type.ASSIGNMENT)) {
        skip()
        parseNext()
    } else expressions()

    private fun ifDeclaration(): Expression {
        expectType(Type.OPEN_CURVE)
        val logicalExpr = parseNext()
        expectType(Type.CLOSE_CURVE)
        val ifBody = autoBodyExpr()

        // All is Auto Scopped!
        if (isEOF() || peek().type != Type.ELSE)
            return Expression.If(logicalExpr, ifBody)
        skip()

        val elseBranch = when (peek().type) {
            Type.IF -> {
                skip()
                ifDeclaration()
            }
            else -> autoBodyExpr()
        }
        return Expression.If(logicalExpr, ifBody, elseBranch)
    }

    private fun autoBodyExpr(): Expression.Scope {
        // used everywhere where there is no manual scope management is required,
        //  e.g., IfExpr, Until, For
        if (peek().type == Type.OPEN_CURLY) return autoScopeBody()
        nameResolver.enterScope()
        return Expression.Scope(parseNext(), nameResolver.leaveScope())
    }

    private fun autoScopeBody(): Expression.Scope {
        nameResolver.enterScope()
        return Expression.Scope(expressions(), nameResolver.leaveScope())
    }

    private fun unscoppedBodyExpr(): Expression {
        if (peek().type == Type.OPEN_CURLY) return expressions()
        return parseNext()
    }

    private fun expressions(): Expression {
        expectType(Type.OPEN_CURLY)
        val expressions = ArrayList<Expression>()
        if (peek().type == Type.CLOSE_CURLY)
            return Expression.ExpressionList(expressions)
        while (!isEOF() && peek().type != Type.CLOSE_CURLY)
            expressions.add(parseNext())
        expectType(Type.CLOSE_CURLY)
        if (expressions.size == 1) return expressions[0]
        return Expression.ExpressionList(expressions)
    }

    private fun variableDeclaration(token: Token): Expression {
        val name = readAlpha()
        nameResolver.defineVr(name)
        if (!isNext(Type.COLON)) {
            return Expression.AutoVariable(name, readVariableExpr())
        }
        skip()
        val definition = Expression.DefinitionType(name, readClassType())
        return Expression.ExplicitVariable(token.type == Type.VAR, definition, readVariableExpr())
    }

    private fun readClassType(): Type {
        val next = next()
        if (!next.hasFlag(Flag.CLASS))
            next.error<String>("Expected class type token")
        return next.type
    }

    private fun readVariableExpr(): Expression {
        val nextToken = peek()
        return when (nextToken.type) {
            Type.ASSIGNMENT -> {
                skip()
                parseNext()
            }

            Type.OPEN_CURVE -> shadoDeclaration()
            Type.OPEN_CURLY -> parseNext()
            else -> nextToken.error("Unexpected variable expression")
        }
    }

    private fun parseExpr(minPrecedence: Int): Expression {
        var left = parseElement()
        // a[x][y]
        // {{a, x}, y}
        while (!isEOF()) {
            val nextOp = peek()
            if (nextOp.type != Type.DOT
                && nextOp.type != Type.OPEN_CURVE &&
                nextOp.type != Type.OPEN_SQUARE
            ) break

            when (nextOp.type) {
                // calling shadow funcs
                Type.OPEN_CURVE -> left = unitCall(left)
                Type.OPEN_SQUARE -> {
                    // array access
                    skip()
                    val expr = parseNext()
                    expectType(Type.CLOSE_SQUARE)
                    left = Expression.ElementAccess(left, expr)
                }

                else -> {
                    // calling a method in another class
                    // string.contains("melon")
                    skip()

                    val method = readAlpha()
                    expectType(Type.OPEN_CURVE)
                    val arguments = parseArguments()
                    expectType(Type.CLOSE_CURVE)
                    var static = false
                    if (left is Expression.Alpha)
                        static = nameResolver.classes.contains(left.value)
                    left = Expression.ClassMethodCall(static, left, method, arguments)
                }
            }
        }
        if (!isEOF() && peek().hasFlag(Flag.POSSIBLE_RIGHT_UNARY)) {
            left = Expression.UnaryOperation(Expression.Operator(next().type), left, false)
        }
        while (!isEOF()) {
            val opToken = peek()
            if (!opToken.hasFlag(Flag.OPERATOR)) return left

            val precedence = operatorPrecedence(opToken.flags[0])
            if (precedence == -1) return left

            if (precedence >= minPrecedence) {
                skip() // operator token
                val right =
                    if (opToken.hasFlag(Flag.PRESERVE_ORDER)) parseElement()
                    else parseExpr(precedence)
                left = Expression.BinaryOperation(
                    left,
                    right,
                    Expression.Operator(opToken.type)
                )
            } else return left
        }
        return left
    }

    private fun operatorPrecedence(type: Flag) = when (type) {
        Flag.ASSIGNMENT_TYPE -> 1
        Flag.LOGICAL_OR -> 2
        Flag.LOGICAL_AND -> 3
        Flag.BITWISE_OR -> 4
        Flag.BITWISE_AND -> 5
        Flag.EQUALITY -> 6
        Flag.RELATIONAL -> 7
        Flag.BINARY -> 8
        Flag.BINARY_L2 -> 9
        Flag.BINARY_L3 -> 10
        else -> -1
    }

    private fun parseElement(): Expression {
        when (peek().type) {
            Type.OPEN_CURVE -> {
                skip()
                val expr = parseNext()
                expectType(Type.CLOSE_CURVE)
                return expr
            }

            Type.OPEN_CURLY -> Expression.Shadow(emptyList(), autoScopeBody().expr)

            else -> {}
        }
        val token = next()
        if (token.hasFlag(Flag.VALUE)) {
            val alpha = parseValue(token)
            if (!isEOF() && peek().type == Type.OPEN_CURVE)
                return unitCall(alpha)
            return alpha
        } else if (token.hasFlag(Flag.UNARY)) {
            return Expression.UnaryOperation(Expression.Operator(token.type), parseElement(), true)
        } else if (token.hasFlag(Flag.NATIVE_CALL)) {
            expectType(Type.OPEN_CURVE)
            val arguments = parseArguments()
            expectType(Type.CLOSE_CURVE)
            return Expression.NativeCall(token.type, Expression.ExpressionList(arguments))
        }
        back()
        if (canParseNext()) return parseNext()
        return token.error("Unexpected token")
    }

    private fun parseValue(token: Token): Expression {
        return when (token.type) {
            Type.E_TRUE, Type.E_FALSE -> Expression.BoolLiteral(token.type == Type.E_TRUE)
            Type.E_INT -> Expression.IntLiteral(token.optionalData.toString().toInt())
            Type.E_STRING -> Expression.StringLiteral(token.optionalData as String)
            Type.E_CHAR -> Expression.CharLiteral(token.optionalData as Char)
            Type.ALPHA -> {
                val name = readAlpha(token)
                val vrIndex = nameResolver.resolveVr(name)
                if (vrIndex == -1) {
                    // could be a function call or static invocation
                    if (nameResolver.resolveFn(name) != null || nameResolver.classes.contains(name))
                        Expression.Alpha(-2, name)
                    else token.error<Expression>("Could not resolve name $name")
                } else {
                    Expression.Alpha(vrIndex, name)
                }
            }

            Type.OPEN_CURVE -> {
                val expr = parseNext()
                expectType(Type.CLOSE_CURVE)
                expr
            }

            else -> token.error("Unknown token type")
        }
    }

    private fun unitCall(unitExpr: Expression): Expression {
        val at = expectType(Type.OPEN_CURVE)
        val arguments = parseArguments()
        expectType(Type.CLOSE_CURVE)

        if (unitExpr is Expression.Alpha) {
            val name = unitExpr.value
            val fnExpr = nameResolver.resolveFn(name)
            if (fnExpr != null) {
                if (fnExpr.argsSize == -1)
                    throw RuntimeException("[Internal] Function args size is not yet set")
                if (fnExpr.argsSize != arguments.size)
                    at.error<String>("Fn [$name] expected ${fnExpr.argsSize} but got ${arguments.size}")
                return Expression.MethodCall(fnExpr, arguments)
            }
        }
        return Expression.ShadoInvoke(unitExpr, arguments)
    }

    private fun parseArguments(): List<Expression> {
        val expressions = ArrayList<Expression>()
        if (!isEOF() && peek().type == Type.CLOSE_CURVE)
            return expressions
        while (!isEOF()) {
            expressions.add(parseNext())
            if (!isNext(Type.COMMA)) break
            skip()
        }
        return expressions
    }

    private fun readAlpha(): String {
        val token = next()
        if (token.type != Type.ALPHA) return token.error("Expected alpha token got $token")
        return token.optionalData as String
    }

    private fun readAlpha(token: Token) =
        if (token.type == Type.ALPHA) token.optionalData as String
        else token.error("Was expecting an alpha token")

    private fun expectType(type: Type): Token {
        val next = next()
        if (next.type != type)
            next.error<String>("Expected token type $type but got $next")
        return next
    }

    private fun isNext(type: Type) = peek().type == type

    private fun back() {
        index--
    }

    private fun skip() {
        index++
    }

    private fun next(): Token {
        if (isEOF()) throw RuntimeException("Early EOF")
        return tokens[index++]
    }

    private fun peek(): Token {
        if (isEOF()) throw RuntimeException("Early EOF")
        return tokens[index]
    }

    private fun isEOF() = index == size
}