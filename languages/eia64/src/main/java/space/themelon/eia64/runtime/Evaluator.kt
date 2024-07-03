package space.themelon.eia64.runtime

import space.themelon.eia64.Expression
import space.themelon.eia64.primitives.*
import space.themelon.eia64.runtime.Entity.Companion.getType
import space.themelon.eia64.runtime.Entity.Companion.unbox
import space.themelon.eia64.syntax.Type
import space.themelon.eia64.syntax.Type.*
import java.util.*
import kotlin.collections.ArrayList
import kotlin.math.pow
import kotlin.random.Random
import kotlin.system.exitProcess

class Evaluator(private val executor: Executor) : Expression.Visitor<Any> {

    private val startupTime = System.currentTimeMillis()

    fun eval(expr: Expression) = expr.accept(this)
    private fun unboxEval(expr: Expression) = unbox(eval(expr))

    private fun safeUnbox(expr: Expression, expectedType: Type, operation: String): Any {
        val result = unboxEval(expr)
        val gotType = getType(result)
        if (gotType != expectedType)
            throw RuntimeException("Expected type $expectedType for [$operation] but got $gotType")
        return result
    }

    private fun booleanExpr(expr: Expression, operation: String) = safeUnbox(expr, E_BOOL, operation) as EBool
    private fun intExpr(expr: Expression, operation: String) = safeUnbox(expr, E_INT, operation) as EInt

    private val memory = Memory()

    override fun genericLiteral(literal: Expression.GenericLiteral) = literal.value
    override fun intLiteral(intLiteral: Expression.IntLiteral) = EInt(intLiteral.value)
    override fun boolLiteral(boolLiteral: Expression.BoolLiteral) = EBool(boolLiteral.value)
    override fun stringLiteral(stringLiteral: Expression.StringLiteral) = EString(stringLiteral.value)
    override fun charLiteral(charLiteral: Expression.CharLiteral) = EChar(charLiteral.value)

    override fun alpha(alpha: Expression.Alpha) = memory.getVar(alpha.index, alpha.value)
    override fun operator(operator: Expression.Operator) = operator.value

    private fun update(scope: Int, name: String, value: Any) {
        (memory.getVar(scope, name) as Entity).update(value)
    }

    override fun variable(variable: Expression.ExplicitVariable): Any {
        val value = unboxEval(variable.expr)
        val valueType = getType(value)
        val def = variable.definition
        val mutable = variable.mutable

        if (def.type != E_ANY && def.type != valueType)
            throw RuntimeException("Variable ${def.name} has type ${def.type}, but got value type of $valueType")
        memory.declareVar(def.name, Entity(def.name, mutable, value, def.type))
        return value
    }

    override fun autoVariable(autoVariable: Expression.AutoVariable): Any {
        val value = unboxEval(autoVariable.expr)
        memory.declareVar(autoVariable.name, Entity(autoVariable.name, true, unbox(value), getType(value)))
        return value
    }

    override fun unaryOperation(expr: Expression.UnaryOperation): Any = when (val type = operator(expr.operator)) {
        NOT -> EBool(!(booleanExpr(expr.expr, "! Not").get()))
        NEGATE -> EInt(Math.negateExact(intExpr(expr.expr, "- Negate").get()))
        INCREMENT, DECREMENT -> {
            val eInt = intExpr(expr.expr, "++ Increment")
            EInt(if (expr.left) {
                if (type == INCREMENT) eInt.incrementAndGet()
                else eInt.decrementAndGet()
            } else {
                if (type == INCREMENT) eInt.getAndIncrement()
                else eInt.getAndDecrement()
            })
        }
        else -> throw RuntimeException("Unknown unary operator $type")
    }

    private fun valueEquals(left: Any, right: Any): Boolean {
        if (getType(left) != getType(right)) return false
        when (left) {
            is EInt,
            is EString,
            is EChar,
            is EBool,
            is EArray -> return left == right
        }
        return false
    }

    override fun binaryOperation(expr: Expression.BinaryOperation) = when (val type = operator(expr.operator)) {
        PLUS -> {
            val left = unboxEval(expr.left)
            val right = unboxEval(expr.right)

            if (getType(left) == E_INT && getType(right) == E_INT)
                left as EInt + right as EInt
            else EString(left.toString() + right.toString())
        }
        NEGATE -> intExpr(expr.left, "- Subtract") - intExpr(expr.right, "- Subtract")
        TIMES -> intExpr(expr.left, "* Multiply") * intExpr(expr.right, "* Multiply")
        SLASH -> intExpr(expr.left, "/ Divide") / intExpr(expr.right, "/ Divide")
        EQUALS, NOT_EQUALS -> {
            val left = unboxEval(expr.left)
            val right = unboxEval(expr.right)

            EBool(if (type == EQUALS) valueEquals(left, right) else !valueEquals(left, right))
        }
        LOGICAL_AND -> booleanExpr(expr.left, "&& Logical And").and(booleanExpr(expr.right, "&& Logical And"))
        LOGICAL_OR -> booleanExpr(expr.left, "|| Logical Or").or(booleanExpr(expr.right, "|| Logical Or"))
        GREATER_THAN -> EBool(intExpr(expr.left, "> GreaterThan") > intExpr(expr.right, "> GreaterThan"))
        LESSER_THAN -> {
            val left = intExpr(expr.left, "< LesserThan")
            val right = intExpr(expr.right, "< LesserThan")
            EBool(left < right)
        }
        GREATER_THAN_EQUALS -> EBool(intExpr(expr.left, ">= GreaterThanEquals") >= intExpr(expr.right, ">= GreaterThanEquals"))
        LESSER_THAN_EQUALS -> EBool(intExpr(expr.left, "<= LesserThan") <= intExpr(expr.right, "<= LesserThan"))
        ASSIGNMENT -> {
            val toUpdate = expr.left
            val value = unboxEval(expr.right)
            when (toUpdate) {
                is Expression.Alpha -> update(toUpdate.index, toUpdate.value, value)
                is Expression.ElementAccess -> {
                    val array = unboxEval(toUpdate.expr)
                    val index = intExpr(toUpdate.index, "[] ArraySet").get()

                    @Suppress("UNCHECKED_CAST")
                    when (getType(array)) {
                        E_ARRAY -> (array as ArrayOperable<Any>).setAt(index, value)
                        E_STRING -> {
                            if (value !is EChar) throw RuntimeException("string[index] requires a Char")
                            (array as EString).setAt(index, value)
                        }
                        else -> throw RuntimeException("Unknown element access of {$array}")
                    }
                }
                else -> throw RuntimeException("Unknown left operand for [= Assignment]: $toUpdate")
            }
            value
        }
        ADDITIVE_ASSIGNMENT -> {
            val element = unboxEval(expr.left)
            when (element) {
                is EString -> element.append(unboxEval(expr.right))
                is EInt -> element += intExpr(expr.right, "+=")
                else -> throw RuntimeException("Cannot apply += operator on element $element")
            }
            element
        }
        DEDUCTIVE_ASSIGNMENT -> {
            val element = unboxEval(expr.left)
            when (element) {
                is EInt -> element -= intExpr(expr.right, "-=")
                else -> throw RuntimeException("Cannot apply -= operator on element $element")
            }
            element
        }
        MULTIPLICATIVE_ASSIGNMENT -> {
            val element = unboxEval(expr.left)
            when (element) {
                is EInt -> element *= intExpr(expr.right, "*=")
                else -> throw RuntimeException("Cannot apply *= operator on element $element")
            }
            element
        }
        POWER -> {
            val left = intExpr(expr.left, "** Power")
            val right = intExpr(expr.right, "** Power")
            EString(left.get().toDouble().pow(right.get().toDouble()).toString())
        }
        DIVIDIVE_ASSIGNMENT -> {
            val element = unboxEval(expr.left)
            when (element) {
                is EInt -> element /= intExpr(expr.right, "/=")
                else -> throw RuntimeException("Cannot apply /= operator on element $element")
            }
            element
        }
        BITWISE_AND -> intExpr(expr.left, "& BitwiseAnd").and(intExpr(expr.right, "& BitwiseAnd"))
        BITWISE_OR -> intExpr(expr.left, "| BitwiseOr").or(intExpr(expr.right, "| BitwiseOr"))
        else -> throw RuntimeException("Unknown binary operator $type")
    }

    override fun expressions(list: Expression.ExpressionList): Any {
        if (list.preserveState)
            // it is being stored somewhere, like in a variable, etc.
            //   that's why we shouldn't evaluate it
            return list
        for (expression in list.expressions) {
            val result = eval(expression)
            if (result is Entity) {
                // flow interruption is just forwarded
                when (result.type) {
                    RETURN, BREAK, CONTINUE, USE -> return result
                    else -> { }
                }
            }
        }
        return EInt(list.size)
    }

    override fun importStdLib(stdLib: Expression.ImportStdLib): Any {
        stdLib.names.forEach { executor.loadExternal("${Executor.STD_LIB}/$it.eia", it) }
        return EBool(true)
    }

    override fun nativeCall(call: Expression.NativeCall): Any {
        val argsSize = call.arguments.size
        when (val type = call.type) {
            PRINT, PRINTLN -> {
                var printCount = 0
                call.arguments.expressions.forEach {
                    var printable = unboxEval(it)
                    printable = if (printable is Array<*>) printable.contentDeepToString() else printable.toString()

                    printCount += printable.length
                    print(printable)
                }
                if (type == PRINTLN) print('\n')
                return EInt(printCount)
            }

            READ, READLN -> {
                if (argsSize != 0) reportWrongArguments("read/readln", 0, argsSize)
                return EString(Scanner(System.`in`).let { if (type == READ) it.next() else it.nextLine() })
            }

            SLEEP -> {
                if (argsSize != 1) reportWrongArguments("sleep", 1, argsSize)
                val millis = intExpr(call.arguments.expressions[0], "sleep()")
                Thread.sleep(millis.get().toLong())
                return millis
            }

            LEN -> {
                if (argsSize != 1) reportWrongArguments("len", 1, argsSize)
                return EInt(when (val data = unboxEval(call.arguments.expressions[0])) {
                    is EString -> data.length
                    is EArray -> data.size
                    is Expression.ExpressionList -> data.size
                    else -> throw RuntimeException("Unknown measurable data type $data")
                })
            }

            FORMAT -> {
                val exprs = call.arguments.expressions
                val string = unboxEval(exprs[0])
                if (getType(string) != E_STRING)
                    throw RuntimeException("format() requires a string argument")
                string as EString
                if (exprs.size > 1) {
                    val values = arrayOfNulls<Any>(exprs.size - 1)
                    for (i in 1 until exprs.size) {
                        val value = unboxEval(exprs[i])
                        values[i - 1] = if (value is Primitive<*>) value.get() else value
                    }
                    return EString(String.format(string.get(), *values))
                }
                return string
            }

            INT_CAST -> {
                if (argsSize != 1) reportWrongArguments("int", 1, argsSize)
                val obj = unboxEval(call.arguments.expressions[0])
                if (getType(obj) == E_INT) return obj
                return EInt(Integer.parseInt(obj.toString()))
            }

            STRING_CAST -> {
                if (argsSize != 1) reportWrongArguments("str", 1, argsSize)
                val obj = unboxEval(call.arguments.expressions[0])
                if (getType(obj) == E_STRING) return obj
                return EString(obj.toString())
            }

            BOOL_CAST -> {
                if (argsSize != 1) reportWrongArguments("bool", 1, argsSize)
                val obj = unboxEval(call.arguments.expressions[0])
                if (getType(obj) == E_BOOL) return obj
                return EBool(when (obj) {
                    "true" -> true
                    "false" -> false
                    else -> throw RuntimeException("Cannot parse boolean value: $obj")
                })
            }

            TYPE -> {
                if (argsSize != 1) reportWrongArguments("type", 1, argsSize)
                val obj = unboxEval(call.arguments.expressions[0])
                return EString(getType(obj).toString())
            }

            INCLUDE -> {
                if (argsSize != 1) reportWrongArguments("include", 1, argsSize)
                val obj = unboxEval(call.arguments.expressions[0])
                if (obj !is EString)
                    throw RuntimeException("Expected a string argument for include() but got $obj")
                val parts = obj.get().split(":")
                if (parts.size != 2)
                    throw RuntimeException("include() received invalid argument: $obj")
                var group = parts[0]
                if (group.isEmpty()) group = Executor.STD_LIB

                val name = parts[1]
                executor.loadExternal("$group/$name.eia", name)
                return EBool(true)
            }

            COPY -> {
                if (argsSize != 1) reportWrongArguments("include", 1, argsSize)
                val obj = unboxEval(call.arguments.expressions[0])
                if (obj !is Primitive<*> || !obj.isCopyable())
                    throw RuntimeException("Cannot apply copy() on object type ${getType(obj)} = $obj")
                return obj.copy()!!
            }

            ARRALLOC -> {
                if (argsSize != 1) reportWrongArguments("arralloc", 1, argsSize)
                val size = intExpr(call.arguments.expressions[0], "arralloc()")
                return EArray(Array(size.get()) { EInt(0) })
            }

            ARRAYOF -> {
                val arguments = call.arguments.expressions
                val evaluated = arrayOfNulls<Any>(arguments.size)
                for ((index, aExpr) in arguments.withIndex())
                    evaluated[index] = unboxEval(aExpr)
                evaluated as Array<Any>
                return EArray(evaluated)
            }

            TIME -> return EInt((System.currentTimeMillis() - startupTime).toInt())

            RAND -> {
                if (argsSize != 2) reportWrongArguments("rand", 2, argsSize)
                val from = intExpr(call.arguments.expressions[0], "rand()")
                val to = intExpr(call.arguments.expressions[1], "rand()")
                return EInt(Random.nextInt(from.get(), to.get()))
            }

            EXIT -> {
                if (argsSize != 1) reportWrongArguments("exit", 1, argsSize)
                val exitCode = intExpr(call.arguments.expressions[0], "exit()")
                exitProcess(exitCode.get())
            }
            else -> throw RuntimeException("Unknown native call operation: '$type'")
        }
    }

    override fun scope(scope: Expression.Scope): Any {
        if (scope.imaginary) return eval(scope.expr)
        memory.enterScope()
        val result = eval(scope.expr)
        memory.leaveScope()
        return result
    }

    override fun methodCall(call: Expression.MethodCall) = fnInvoke(call.fnExpr.fnExpression!!, evaluateArgs(call.arguments))

    override fun classMethodCall(call: Expression.ClassMethodCall): Any {
        val obj = call.obj
        val methodName = call.method
        val args: Array<Any>
        val className: String

        // we may need to do a recursive alpha parse
        if (call.static) {
            // static invocation of an included class
            className = (obj as Expression.Alpha).value
            args = evaluateArgs(call.arguments)
        } else {
           val evaluatedObj = unboxEval(obj)
           if (evaluatedObj !is Primitive<*>)
               throw RuntimeException("Could not find method '$methodName' of object $evaluatedObj")
           className = evaluatedObj.stdlibName()
           call.arguments as ArrayList
            val evaluatedArgs = arrayOfNulls<Any>(call.arguments.size + 1)
            for ((index, expression) in call.arguments.withIndex())
                evaluatedArgs[index + 1] = unboxEval(expression)
            // NOTE: we never should directly modify the original expression list
            evaluatedArgs[0] = evaluatedObj
            @Suppress("UNCHECKED_CAST")
            evaluatedArgs as Array<Any>
            args = evaluatedArgs
        }
        val executor = executor.getExternalExecutor(className) ?: throw RuntimeException("Could not find class (for) $className")
        return executor.dynamicFnCall(methodName, args)
    }

    private fun evaluateArgs(args: List<Expression>): Array<Any> {
        val evaluatedArgs = arrayOfNulls<Any>(args.size)
        for ((index, expression) in args.withIndex())
            evaluatedArgs[index] = unboxEval(expression)
        @Suppress("UNCHECKED_CAST")
        evaluatedArgs as Array<Any>
        return evaluatedArgs
    }

    private fun dynamicFnCall(
        name: String,
        args: Array<Any>
    ) = fnInvoke(memory.dynamicFnSearch(name), args)

    private fun fnInvoke(fn: Expression.Function, callArgs: Array<Any>): Any {
        // Fully Manual Scopped!
        val fnName = fn.name

        val sigArgsSize = fn.arguments.size
        val callArgsSize = callArgs.size

        if (sigArgsSize != callArgsSize)
            reportWrongArguments(fnName, sigArgsSize, callArgsSize)
        val parameters = fn.arguments.iterator()
        val callExpressions = callArgs.iterator()

        val callValues = ArrayList<Pair<Expression.DefinitionType, Any>>()
        while (parameters.hasNext()) {
            val definedParameter = parameters.next()
            val typeSignature = definedParameter.type

            val callValue = callExpressions.next()
            val gotTypeSignature = getType(callValue)

            if (typeSignature != E_ANY && typeSignature != gotTypeSignature)
                throw RuntimeException("Expected type $typeSignature for arg '${definedParameter.name}' for function $fnName but got $gotTypeSignature")
            callValues.add(Pair(definedParameter, callValue))
        }
        memory.enterScope()
        callValues.forEach {
            val definedParameter = it.first
            val value = it.second
            memory.declareVar(definedParameter.name,
                Entity(definedParameter.name, true, value, definedParameter.type))
        }
        // do not handle return calls here, let it naturally unbox itself
        val result = eval(fn.body)
        memory.leaveScope()

        val returnSignature = fn.returnType
        val gotReturnSignature = getType(result)

        if (returnSignature != E_ANY && returnSignature != gotReturnSignature)
            throw RuntimeException("Expected return type $returnSignature for function $fnName but got $gotReturnSignature")

        return result
    }

    override fun unitInvoke(shadoInvoke: Expression.ShadoInvoke): Any {
        var operand: Any = shadoInvoke.expr

        // Fully Manual Scopped
        if (operand !is Expression.Shadow)
            operand = unboxEval(operand as Expression)

        if (operand !is Expression.Shadow)
            throw RuntimeException("Expected shadow element for call, but got $operand")

        val expectedArgs = operand.names.size
        val gotArgs = shadoInvoke.arguments.size
        if (expectedArgs != gotArgs) {
            reportWrongArguments("AnonShado", expectedArgs, gotArgs, "Shado")
        }

        val namesIterator = operand.names.iterator()
        val exprIterator = evaluateArgs(shadoInvoke.arguments).iterator()

        memory.enterScope()
        while (exprIterator.hasNext()) memory.declareVar(namesIterator.next(), exprIterator.next())

        val result = eval(operand.body)
        memory.leaveScope()

        if (result is Entity) {
            when (result.type) {
                RETURN, USE -> return result
                else -> { }
            }
        }
        return result
    }

    private fun reportWrongArguments(name: String, expectedArgs: Int, gotArgs: Int, type: String = "Fn") {
        throw RuntimeException("$type [$name()] expected $expectedArgs but got $gotArgs")
    }

    override fun until(until: Expression.Until): Any {
        // Auto Scopped
        var numIterations = 0
        while (booleanExpr(until.expression, "Until Condition").get()) {
            numIterations++
            val result = eval(until.body)
            if (result is Entity) {
                when (result.type) {
                    BREAK -> break
                    CONTINUE -> continue
                    RETURN -> return result
                    USE -> result.value
                    else -> { }
                }
            }
        }
        return EInt(numIterations)
    }

    override fun forEach(forEach: Expression.ForEach): Any {
        val iterable = unboxEval(forEach.entity)

        var index = 0
        val size:Int

        val getNext: () -> Any
        when (iterable) {
            is EString -> {
                size = iterable.length
                getNext = { iterable.getAt(index++) }
            }

            is EArray -> {
                size = iterable.size
                getNext = { iterable.getAt(index++) }
            }

            else -> throw RuntimeException("Unknown non-iterable element $iterable")
        }

        val named = forEach.name
        val body = forEach.body

        var numIterations = 0
        while (index < size) {
            numIterations++
            // Manual Scopped
            memory.enterScope()
            val element = getNext()
            memory.declareVar(named, Entity(named, false, element, getType(element)))
            val result = eval(body)
            memory.leaveScope()
            if (result is Entity) {
                when (result.type) {
                    BREAK -> break
                    CONTINUE -> continue
                    RETURN -> return result
                    USE -> return result.value
                    else -> { }
                }
            }
        }
        return EInt(numIterations)
    }

    override fun itr(itr: Expression.Itr): Any {
        val named = itr.name
        var from = intExpr(itr.from, "Itr from")
        val to = intExpr(itr.to, "Itr to")
        val by = if (itr.by == null) EInt(1) else intExpr(itr.by, "Itr by")

        val reverse = from > to
        if (reverse) by.set(-by.get())

        var numIterations = 0
        while (if (reverse) from >= to else from <= to) {
            numIterations++
            // Manual Scopped
            memory.enterScope()
            memory.declareVar(named, Entity(named, true, from, E_INT))
            val result = eval(itr.body)
            memory.leaveScope()
            if (result is Entity) {
                when (result.type) {
                    BREAK -> break
                    CONTINUE -> {
                        from = from + by
                        continue
                    }
                    RETURN -> return result
                    USE -> return result.value
                    else -> { }
                }
            }
            from = from + by
        }
        return EInt(numIterations)
    }

    override fun forLoop(forLoop: Expression.ForLoop): Any {
        memory.enterScope()
        forLoop.initializer?.let { eval(it) }

        val conditional = forLoop.conditional

        var numIterations = 0
        fun evalOperational() = forLoop.operational?.let { eval(it) }

        while (if (conditional == null) true else booleanExpr(conditional, "ForLoop").get()) {
            numIterations++
            // Auto Scopped
            val result = eval(forLoop.body)
            if (result is Entity) {
                when (result.type) {
                    BREAK -> break
                    CONTINUE -> {
                        evalOperational()
                        continue
                    }
                    RETURN -> {
                        memory.leaveScope()
                        return result
                    }
                    USE -> {
                        memory.leaveScope()
                        return result.value
                    }
                    else -> { }
                }
            }
            evalOperational()
        }
        memory.leaveScope()
        return EInt(numIterations)
    }

    override fun interruption(interruption: Expression.Interruption) = when (val type = eval(interruption.type)) {
        // wrap it as a normal entity, this will be naturally unboxed when called unbox()
        RETURN -> Entity("FlowReturn", false, unboxEval(interruption.expr!!), RETURN)
        BREAK -> Entity("FlowBreak", false, 0, BREAK)
        CONTINUE -> Entity("FlowContinue", false, 0, CONTINUE)
        USE -> Entity("FlowUse", false, unboxEval(interruption.expr!!), USE)
        else -> throw RuntimeException("Unknown interruption type $type")
    }

    override fun whenExpr(whenExpr: Expression.When): Any {
        // Fully Auto Scopped
        val matchExpr = unboxEval(whenExpr.expr)
        for (match in whenExpr.matches)
            if (valueEquals(matchExpr, unboxEval(match.first)))
                return unboxEval(match.second)
        return unboxEval(whenExpr.defaultBranch)
    }

    override fun ifFunction(ifExpr: Expression.If): Any {
        val conditionSuccess = booleanExpr(ifExpr.condition, "If Condition").get()
        val body = if (conditionSuccess) ifExpr.thenBody else ifExpr.elseBody
        // Auto Scopped
        if (body != null) return eval(body)
        return EBool(conditionSuccess)
    }

    override fun function(function: Expression.Function): Any {
        memory.declareFn(function.name, function)
        return EBool(true)
    }

    override fun shado(shadow: Expression.Shadow) = shadow

    override fun elementAccess(access: Expression.ElementAccess): Any {
        val entity = unboxEval(access.expr)
        val index = intExpr(access.index, "[] ArrayAccess").get()

        if (entity !is ArrayOperable<*>)
            throw RuntimeException("Unknown non-array operable element access of $entity")
        return entity.getAt(index)!!
    }
}