package space.themelon.eia64.runtime

import space.themelon.eia64.EiaTrace
import space.themelon.eia64.Expression
import space.themelon.eia64.expressions.*
import space.themelon.eia64.expressions.FunctionExpr
import space.themelon.eia64.primitives.*
import space.themelon.eia64.runtime.Entity.Companion.getSignature
import space.themelon.eia64.runtime.Entity.Companion.unbox
import space.themelon.eia64.signatures.ArrayExtension
import space.themelon.eia64.signatures.Matching.matches
import space.themelon.eia64.signatures.ObjectExtension
import space.themelon.eia64.signatures.Sign
import space.themelon.eia64.signatures.Signature
import space.themelon.eia64.syntax.Type.*
import java.io.FileOutputStream
import java.io.PrintStream
import java.util.Scanner
import kotlin.collections.ArrayList
import kotlin.math.pow
import kotlin.random.Random

class Evaluator(
    val className: String,
    private val executor: Executor
) : Expression.Visitor<Any> {

    private val startupTime = System.currentTimeMillis()

    private var evaluator: Expression.Visitor<Any> = this

    // in the future, we need to give options to enable/ disable:
    //  how about enabling it through eia code?
    private val tracer = if (Executor.DEBUG) EiaTrace(PrintStream(FileOutputStream(Executor.LOGS_PIPE_PATH))) else null

    fun shutdown() {
        // Reroute all the traffic to Void, which would raise ShutdownException.
        // We use this strategy to cause an efficient shutdown than checking fields each time
        evaluator = VoidEvaluator()
    }

    fun mainEval(expr: Expression): Any {
        val normalEvaluated = eval(expr)
        val mainEvaluated = dynamicFnCall(
            "main",
            emptyArray(),
            true, "")
        if (mainEvaluated == null || mainEvaluated == "") return normalEvaluated
        return mainEvaluated
    }

    fun eval(expr: Expression) = expr.accept(evaluator)

    private fun unboxEval(expr: Expression) = unbox(eval(expr))

    private fun booleanExpr(expr: Expression) = unboxEval(expr) as EBool

    private fun intExpr(expr: Expression) = when (val result = unboxEval(expr)) {
        is EChar -> EInt(result.get().code)
        else -> result as EInt
    }

    // Plan future: My opinion would be that this should NOT happen, these types of
    // Runtime Checks Hinder performance. There should be a common wrapper to all
    //  the numeric applicable types. Runtime checking should be avoided
    private fun numericExpr(expr: Expression): Numeric = when (val result = unboxEval(expr)) {
        is EChar -> EInt(result.get().code)
        is EInt -> result
        else -> result as EFloat
    }

    // Supply tracer to memory, so that it calls enterScope() and leaveScope()
    // on tracer on behalf of us
    private val memory = Memory(tracer)

    fun clearMemory() {
        memory.clearMemory()
    }

    override fun noneExpression() = Nothing.INSTANCE
    override fun nilLiteral(nil: NilLiteral) = ENil()
    override fun intLiteral(literal: IntLiteral) = EInt(literal.value)
    override fun floatLiteral(literal: FloatLiteral) = EFloat(literal.value)

    override fun boolLiteral(literal: BoolLiteral) = EBool(literal.value)
    override fun stringLiteral(literal: StringLiteral) = EString(literal.value)
    override fun charLiteral(literal: CharLiteral) = EChar(literal.value)
    override fun typeLiteral(literal: TypeLiteral) = EType(literal.signature)

    override fun alpha(alpha: Alpha) = memory.getVar(alpha.index, alpha.value)

    private fun prepareArrayOf(
        arguments: List<Expression>,
        elementSignature: Signature
    ): EArray {
        val evaluated = arrayOfNulls<Any>(arguments.size)
        for ((index, aExpr) in arguments.withIndex())
            evaluated[index] = unboxEval(aExpr)
        evaluated as Array<Any>
        return EArray(elementSignature, evaluated)
    }

    override fun array(literal: ArrayLiteral) = prepareArrayOf(literal.elements, literal.elementSignature())

    override fun explicitArrayLiteral(arrayCreation: ExplicitArrayLiteral) =
        prepareArrayOf(arrayCreation.elements, arrayCreation.elementSignature)

    override fun arrayAllocation(arrayAllocation: ArrayAllocation): Any {
        val size = intExpr(arrayAllocation.size)
        val defaultValue = unboxEval(arrayAllocation.defaultValue)
        return EArray(getSignature(defaultValue), Array(size.get()) { defaultValue })
    }

    private fun update(index: Int,
                       name: String,
                       value: Any) {
        (memory.getVar(index, name) as Entity).update(value)
        tracer?.updateVariableRuntime(name, getSignature(value), value)
    }

    private fun update(aMemory: Memory,
                       index: Int,
                       name: String,
                       value: Any) {
        (aMemory.getVar(index, name) as Entity).update(value)
        tracer?.updateVariableRuntime(name, getSignature(value), value)
    }

    override fun variable(variable: ExplicitVariable): Any {
        val name = variable.name
        val signature = variable.explicitSignature
        val value = unboxEval(variable.expr)
        val mutable = variable.mutable

        memory.declareVar(name, Entity(name, mutable, value, signature))
        tracer?.declareVariableRuntime(
            mutable,
            name,
            signature,
            value)
        return value
    }

    override fun autoVariable(autoVariable: AutoVariable): Any {
        val name = autoVariable.name
        val value = unboxEval(autoVariable.expr)
        val signature = getSignature(value)
        memory.declareVar(
            name,
            Entity(
                name,
                true,
                unbox(value),
                signature
            )
        )
        tracer?.declareVariableRuntime(
            true,
            autoVariable.name,
            signature,
            value)
        return value
    }

    override fun unaryOperation(expr: UnaryOperation): Any = when (val type = expr.operator) {
        NOT -> EBool(!(booleanExpr(expr.expr).get()))
        NEGATE -> {
            // first, we need to check the type to ensure we negate Float
            // and Int separately and properly
            val value = numericExpr(expr.expr).get()
            if (expr.sig().isFloat()) EFloat(value.toFloat() * -1)
            else EInt(value.toInt() * -1)
        }
        INCREMENT, DECREMENT -> {
            val numeric = numericExpr(expr.expr)
            val value = if (expr.towardsLeft) {
                if (type == INCREMENT) numeric.incrementAndGet()
                else numeric.decrementAndGet()
            } else {
                if (type == INCREMENT) numeric.getAndIncrement()
                else numeric.getAndDecrement()
            }
            if (value is Int) EInt(value)
            else EFloat(value as Float)
        }
        else -> throw RuntimeException("Unknown unary operator $type")
    }

    private fun valueEquals(left: Any, right: Any) = when (left) {
        is Numeric,
        is EString,
        is EChar,
        is EBool,
        is ENil,
        is EType,
        is EArray -> left == right
        else -> false
    }

    override fun binaryOperation(expr: BinaryOperation) = when (val type = expr.operator) {
        PLUS -> {
            val left = unboxEval(expr.left)
            val right = unboxEval(expr.right)

            if (left is Numeric && right is Numeric) left + right
            else EString(left.toString() + right.toString())
        }
        NEGATE -> numericExpr(expr.left) - numericExpr(expr.right)
        TIMES -> numericExpr(expr.left) * numericExpr(expr.right)
        SLASH -> numericExpr(expr.left) / numericExpr(expr.right)
        EQUALS, NOT_EQUALS -> {
            val left = unboxEval(expr.left)
            val right = unboxEval(expr.right)
            EBool(if (type == EQUALS) valueEquals(left, right) else !valueEquals(left, right))
        }
        LOGICAL_AND -> booleanExpr(expr.left).and(booleanExpr(expr.right))
        LOGICAL_OR -> booleanExpr(expr.left).or(booleanExpr(expr.right))
        RIGHT_DIAMOND -> EBool(numericExpr(expr.left) > numericExpr(expr.right))
        LEFT_DIAMOND -> EBool(numericExpr(expr.left) < numericExpr(expr.right))
        GREATER_THAN_EQUALS -> EBool(intExpr(expr.left) >= intExpr(expr.right))
        LESSER_THAN_EQUALS -> EBool(intExpr(expr.left) <= intExpr(expr.right))
        ASSIGNMENT -> {
            val toUpdate = expr.left
            val value = unboxEval(expr.right)
            when (toUpdate) {
                is Alpha -> update(toUpdate.index, toUpdate.value, value)
                is ArrayAccess -> updateArrayElement(toUpdate, value)
                is ForeignField -> updateForeignField(toUpdate, value)
                else -> throw RuntimeException("Unknown left operand for [= Assignment]: $toUpdate")
            }
            value
        }
        ADDITIVE_ASSIGNMENT -> {
            val element = unboxEval(expr.left)
            when (element) {
                is EString -> element.append(unboxEval(expr.right))
                is Numeric -> element.plusAssign(numericExpr(expr.right))
                else -> throw RuntimeException("Cannot apply += operator on element $element")
            }
            element
        }
        DEDUCTIVE_ASSIGNMENT -> {
            val element = unboxEval(expr.left)
            when (element) {
                is Numeric -> element.minusAssign(numericExpr(expr.right))
                else -> throw RuntimeException("Cannot apply -= operator on element $element")
            }
            element
        }
        MULTIPLICATIVE_ASSIGNMENT -> {
            val element = unboxEval(expr.left)
            when (element) {
                is Numeric -> element.timesAssign(numericExpr(expr.right))
                else -> throw RuntimeException("Cannot apply *= operator on element $element")
            }
            element
        }
        DIVIDIVE_ASSIGNMENT -> {
            val element = unboxEval(expr.left)
            when (element) {
                is Numeric -> element.divAssign(intExpr(expr.right))
                else -> throw RuntimeException("Cannot apply /= operator on element $element")
            }
            element
        }
        POWER -> {
            val left = numericExpr(expr.left)
            val right = numericExpr(expr.right)
            EString(left.get().toDouble().pow(right.get().toDouble()).toString())
        }
        BITWISE_AND -> numericExpr(expr.left).and(numericExpr(expr.right))
        BITWISE_OR -> numericExpr(expr.left).or(numericExpr(expr.right))
        else -> throw RuntimeException("Unknown binary operator $type")
    }

    private fun updateArrayElement(access: ArrayAccess, value: Any) {
        val array = unboxEval(access.expr)
        val index = intExpr(access.index).get()

        @Suppress("UNCHECKED_CAST")
        when (getSignature(array)) {
            // TODO:
            //  we need to look here later, it could also be an array extension
            Sign.ARRAY, is ArrayExtension -> (array as ArrayOperable<Any>).setAt(index, value)
            Sign.STRING -> {
                if (value !is EChar) throw RuntimeException("string[index] requires a Char")
                (array as EString).setAt(index, value)
            }

            else -> throw RuntimeException("Unknown element access of {$array}")
        }
    }

    override fun isStatement(isStatement: IsStatement) =
        EBool(matches(isStatement.signature, getSignature(unboxEval(isStatement.expression))))

    private fun updateForeignField(field: ForeignField, value: Any) {
        val evaluator = getEvaluatorForField(field)
        val uniqueVariable = field.uniqueVariable
        update(
            aMemory = evaluator.memory,
            index = uniqueVariable.index,
            name = field.property,
            value = value
        )
    }

    override fun expressions(list: ExpressionList): Any {
        if (list.preserveState)
            // it is being stored somewhere, like in a variable, etc.
            //   that's why we shouldn't evaluate it
            return list
        var result: Any? = null
        for (expression in list.expressions) {
            result = eval(expression)
            if (result is Entity) {
                // flow interruption is just forwarded

                // TODO:
                //  We need to verify that these things work
                //when (result.type) {
                    //RETURN, BREAK, CONTINUE, USE -> return result
                    //else -> { }
                //}
                when (result.interruption) {
                    InterruptionType.RETURN,
                    InterruptionType.BREAK,
                    InterruptionType.CONTINUE,
                    InterruptionType.USE -> return result
                    else -> { }
                }
            }
        }
        return result!!
    }

    override fun include(include: Include): Any {
        include.names.forEach { executor.executeModule(it) }
        return EBool(true)
    }

    override fun new(new: NewObj): Evaluator {
        val evaluator = executor.newEvaluator(new.name)
        fnInvoke(new.reference.fnExpression!!, evaluateArgs(new.arguments))
        tracer?.runtimeObjectCreation(new.name, evaluator)
        return evaluator
    }

    // try to call a string() method located in local class if available
    @Override
    override fun toString(): String {
        val result = dynamicFnCall(
            "string",
            emptyArray(),
            true,
            "Class<$className>")
        if (result is String) return result
        if (result is EString) return result.get()
        throw RuntimeException("string() returned a non string $result")
    }

    override fun cast(cast: Cast): Any {
        val result = unboxEval(cast.expr)
        val promisedSignature = cast.expectSignature
        val gotSignature = getSignature(result)

        if (promisedSignature is ObjectExtension) {
            val promisedClass = promisedSignature.extensionClass
            if (result !is Evaluator) {
                cast.where.error<String>("${getSignature(result)} cannot be cast into class $promisedClass")
                throw RuntimeException()
            }
            val gotClass = result.className
            if (promisedClass != gotClass) {
                cast.where.error<String>("Class $gotClass cannot be cast into $promisedClass")
                throw RuntimeException()
            }
        } else if (promisedSignature is ArrayExtension) {
            // Cast into explicit type declaration
            if (gotSignature == Sign.ARRAY) return promisedSignature
            if (gotSignature !is ArrayExtension) {
                cast.where.error<String>("Cannot cast $result into array type $promisedSignature")
                throw RuntimeException()
            }
            val castArrayType = promisedSignature.elementSignature
            val currentArrayType = gotSignature.elementSignature
            // TODO:
            //  Here we would need to verify all the element types and reassign signature
            //  If cast is being from Array<Int> we need to ensure all elements of Array are of Int
            //  before renaming signature to Array<Int>
            if (castArrayType != currentArrayType) {
                cast.where.error<String>("Cannot cast array element type $currentArrayType into $castArrayType")
            }
        } else if (promisedSignature == Sign.ARRAY) {
            if (!(gotSignature is ArrayExtension || gotSignature == Sign.ARRAY)) {
                cast.where.error<String>("Cannot cast $result to $promisedSignature")
            }
        }
        return result
    }

    override fun nativeCall(call: NativeCall): Any {
        when (val type = call.call) {
            PRINT, PRINTLN -> {
                var printCount = 0
                call.arguments.forEach {
                    var printable = unboxEval(it)
                    printable = if (printable is Array<*>) printable.contentDeepToString() else printable.toString()

                    printCount += printable.length
                    executor.standardOutput.print(printable)
                }
                if (type == PRINTLN) executor.standardOutput.print('\n')
                return Nothing.INSTANCE
            }

            READ, READLN -> {
                return EString(Scanner(executor.standardInput).let { if (type == READ) it.next() else it.nextLine() })
            }

            SLEEP -> {
                Thread.sleep(intExpr(call.arguments[0]).get().toLong())
                return Nothing.INSTANCE
            }

            LEN -> {
                return EInt(when (val data = unboxEval(call.arguments[0])) {
                    is EString -> data.length
                    is EArray -> data.size
                    is ExpressionList -> data.size
                    is ENil -> 0
                    else -> throw RuntimeException("Unknown measurable data type $data")
                })
            }

            FORMAT -> {
                val exprs = call.arguments
                val string = unboxEval(exprs[0])
                if (getSignature(string) != Sign.STRING)
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
                val obj = unboxEval(call.arguments[0])

                return when (val objType = getSignature(obj)) {
                    Sign.INT -> obj
                    Sign.CHAR -> EInt((obj as EChar).get().code)
                    Sign.STRING -> EInt(obj.toString().toInt())
                    Sign.FLOAT -> EInt((obj as EFloat).get().toInt())
                    else -> throw RuntimeException("Unknown type for int() cast $objType")
                }
            }

            FLOAT_CAST -> {
                val obj = unboxEval(call.arguments[0])

                return when (val objType = getSignature(obj)) {
                    Sign.INT -> (obj as EInt).get().toFloat()
                    Sign.FLOAT -> obj
                    Sign.CHAR -> EFloat((obj as EChar).get().code.toFloat())
                    Sign.STRING -> EFloat(obj.toString().toFloat())
                    else -> throw RuntimeException("Unknown type for int() cast $objType")
                }
            }

            CHAR_CAST -> {
                val obj = unboxEval(call.arguments[0])
                return when (val objType = getSignature(obj)) {
                    Sign.CHAR -> objType
                    Sign.INT -> EChar((obj as EInt).get().toChar())
                    else -> throw RuntimeException("Unknown type for char() cast $objType")
                }
            }

            STRING_CAST -> {
                val obj = unboxEval(call.arguments[0])
                if (getSignature(obj) == Sign.STRING) return obj
                return EString(obj.toString())
            }

            BOOL_CAST -> {
                val obj = unboxEval(call.arguments[0])
                if (getSignature(obj) == Sign.BOOL) return obj
                return EBool(when (obj) {
                    "true" -> true
                    "false" -> false
                    else -> throw RuntimeException("Cannot parse boolean value: $obj")
                })
            }

            TYPE_OF -> return EType(getSignature(unboxEval(call.arguments[0])))

            INCLUDE -> {
                val obj = unboxEval(call.arguments[0])
                if (obj !is EString)
                    throw RuntimeException("Expected a string argument for include() but got $obj")
                val parts = obj.get().split(":")
                if (parts.size != 2)
                    throw RuntimeException("include() received invalid argument: $obj")
                var group = parts[0]
                if (group.isEmpty()) group = Executor.STD_LIB

                val name = parts[1]
                executor.addModule("$group/$name.eia", name)
                return Nothing.INSTANCE
            }

            COPY -> {
                val obj = unboxEval(call.arguments[0])
                if (obj !is Primitive<*> || !obj.isCopyable())
                    throw RuntimeException("Cannot apply copy() on object type ${getSignature(obj)} = $obj")
                return obj.copy()!!
            }

            TIME -> return EInt((System.currentTimeMillis() - startupTime).toInt())

            RAND -> {
                val from = intExpr(call.arguments[0])
                val to = intExpr(call.arguments[1])
                return EInt(Random.nextInt(from.get(), to.get()))
            }

            // don't do a direct exitProcess(n), Eia could be running in a server
            // you don't need the entire server to shut down
            EXIT -> {
                Executor.EIA_SHUTDOWN(intExpr(call.arguments[0]).get())
                return EBool(true) // never reached (hopefully?)
            }

            MEM_CLEAR -> {
                // for clearing memory of the current class
                memory.clearMemory()
                return Nothing.INSTANCE
            }
            else -> throw RuntimeException("Unknown native call operation: '$type'")
        }
    }

    override fun throwExpr(throwExpr: ThrowExpr): Any {
        val message = throwExpr.where.prepareError(unboxEval(throwExpr.error).toString())
        throw EiaRuntimeException(message)
    }

    override fun tryCatch(tryCatch: TryCatch): Any {
        try {
            return unboxEval(tryCatch.tryBlock)
        } catch (e: EiaRuntimeException) {
            // manual scope handling begins
            memory.enterScope()
            memory.declareVar(tryCatch.catchIdentifier, EString(e.message))
            val result = unboxEval(tryCatch.catchBlock)
            memory.leaveScope()
            return result
        }
    }

    override fun scope(scope: Scope): Any {
        if (scope.imaginary) return eval(scope.expr)
        memory.enterScope()
        val result = eval(scope.expr)
        memory.leaveScope()
        return result
    }

    override fun classPropertyAccess(propertyAccess: ForeignField): Any {
        val evaluator = getEvaluatorForField(propertyAccess)
        val uniqueVariable = propertyAccess.uniqueVariable
        return evaluator.memory.getVar(
            uniqueVariable.index,
            propertyAccess.property
        )
    }

    // finds associated evaluator for a foreign field (gVariable)
    // that is being accessed
    private fun getEvaluatorForField(propertyAccess: ForeignField): Evaluator {
        val property = propertyAccess.property
        val moduleName = propertyAccess.moduleInfo.name

        var evaluator: Evaluator? = null
        if (propertyAccess.static) {
            evaluator = executor.getEvaluator(moduleName)
        } else {
            when (val evaluatedObject = unboxEval(propertyAccess.objectExpression)) {
                is Evaluator -> evaluator = evaluatedObject
                is Primitive<*> -> executor.getEvaluator(moduleName)
                else -> throw RuntimeException("Could not find property $property of object $evaluatedObject")
            }
        }
        return evaluator ?: throw RuntimeException("Could not find module $moduleName")
    }

    override fun methodCall(call: MethodCall)
        = fnInvoke(call.reference.fnExpression!!, evaluateArgs(call.arguments))

    override fun classMethodCall(call: ClassMethodCall): Any {
        val obj = call.objectExpression
        val methodName = call.method
        val args: Array<Any>

        var evaluator: Evaluator? = null
        // we may need to do a recursive alpha parse
        if (call.static) {
            // static invocation of an included class
            args = evaluateArgs(call.arguments)
        } else {
            val evaluatedObj = unboxEval(obj)
            call.arguments as ArrayList
            args = when (evaluatedObj) {
                is Primitive<*> -> {
                    val evaluatedArgs = arrayOfNulls<Any>(call.arguments.size + 1)
                    for ((index, expression) in call.arguments.withIndex())
                        evaluatedArgs[index + 1] = unboxEval(expression)
                    // NOTE: we never should directly modify the original expression list
                    evaluatedArgs[0] = evaluatedObj
                    @Suppress("UNCHECKED_CAST")
                    evaluatedArgs as Array<Any>
                    evaluatedArgs
                }
                is Evaluator -> {
                    evaluator = evaluatedObj
                    evaluateArgs(call.arguments)
                }
                else -> throw RuntimeException("Could not find method '$methodName' of object $evaluatedObj")
            }
        }
        val moduleName = call.moduleInfo.name
        val finalEvaluator = evaluator ?: executor.getEvaluator(moduleName)
            ?: throw RuntimeException("Could not find module $moduleName")
        return finalEvaluator.fnInvoke(call.reference.fnExpression!!, args)
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
        args: Array<Any>,
        discardIfNotFound: Boolean,
        defaultValue: Any? = null
    ): Any? {
        val fn = memory.dynamicFnSearch(name)
        if (discardIfNotFound && fn == null) return defaultValue
        if (fn == null) throw RuntimeException("Unable to find function '$name()' in class $className")
        return fnInvoke(fn, args)
    }

    private fun fnInvoke(fn: FunctionExpr, callArgs: Array<Any>): Any {
        // Fully Manual Scopped!
        val fnName = fn.name

        val sigArgsSize = fn.arguments.size
        val callArgsSize = callArgs.size

        if (sigArgsSize != callArgsSize)
            reportWrongArguments(fnName, sigArgsSize, callArgsSize)
        val parameters = fn.arguments.iterator()
        val callExpressions = callArgs.iterator()

        val argValues = ArrayList<Pair<String, Any>>() // used for logging only

        val callValues = ArrayList<Pair<Pair<String, Signature>, Any>>()
        while (parameters.hasNext()) {
            val definedParameter = parameters.next()
            val callValue = callExpressions.next()

            callValues += Pair(definedParameter, callValue)
            argValues += Pair(definedParameter.first, callValue)
        }
        tracer?.runtimeFnCall(fnName, argValues)
        memory.enterScope()
        callValues.forEach {
            val definedParameter = it.first
            val value = it.second
            memory.declareVar(definedParameter.first,
                Entity(definedParameter.first, true, value, definedParameter.second))
        }
        val result = unboxEval(fn.body)
        memory.leaveScope()
        // Return the function itself as a unit
        if (fn.isVoid) return fn
        return result
    }

    override fun unitInvoke(shadoInvoke: ShadoInvoke): Any {
        var operand: Any = shadoInvoke.expr

        // Fully Manual Scopped
        if (operand !is Shadow)
            operand = unboxEval(operand as Expression)

        if (operand !is Shadow)
            throw RuntimeException("Expected shadow element for call, but got $operand")

        val expectedArgs = operand.names.size
        val gotArgs = shadoInvoke.arguments.size

        if (expectedArgs != gotArgs) {
            reportWrongArguments("AnonShado", expectedArgs, gotArgs, "Shado")
        }

        val argIterator = operand.names.iterator()
        val exprIterator = evaluateArgs(shadoInvoke.arguments).iterator()

        memory.enterScope()
        while (exprIterator.hasNext()) {
            memory.declareVar(argIterator.next(), exprIterator.next())
        }

        val result = eval(operand.body)
        memory.leaveScope()

        if (result is Entity) {
            when (result.interruption) {
                InterruptionType.RETURN,
                InterruptionType.USE -> return result
                else -> { }
            }
        }
        return result
    }

    private fun reportWrongArguments(name: String, expectedArgs: Int, gotArgs: Int, type: String = "Fn") {
        throw RuntimeException("$type [$name()] expected $expectedArgs but got $gotArgs")
    }

    override fun until(until: Until): Any {
        // Auto Scopped
        var numIterations = 0
        while (booleanExpr(until.expression).get()) {
            numIterations++
            tracer?.runtimeUntil(numIterations)
            val result = eval(until.body)
            if (result is Entity) {
                when (result.interruption) {
                    InterruptionType.BREAK -> break
                    InterruptionType.CONTINUE -> continue
                    InterruptionType.RETURN -> return result
                    InterruptionType.USE -> result.value
                    else -> { }
                }
            }
        }
        return EInt(numIterations)
    }

    override fun forEach(forEach: ForEach): Any {
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
            memory.declareVar(named, Entity(named, false, element, getSignature(element)))
            tracer?.runtimeForEach(numIterations, iterable, element)
            val result = eval(body)
            memory.leaveScope()
            if (result is Entity) {
                when (result.interruption) {
                    InterruptionType.BREAK -> break
                    InterruptionType.CONTINUE -> continue
                    InterruptionType.RETURN -> return result
                    InterruptionType.USE -> result.value
                    else -> { }
                }
            }
        }
        return EInt(numIterations)
    }

    override fun itr(itr: Itr): Any {
        val named = itr.name
        var from = intExpr(itr.from)
        val to = intExpr(itr.to)
        val by = if (itr.by == null) EInt(1) else intExpr(itr.by)

        val reverse = from > to
        if (reverse) by.set(EInt(-by.get()))

        var numIterations = 0
        while (if (reverse) from >= to else from <= to) {
            numIterations++
            // Manual Scopped
            memory.enterScope()
            memory.declareVar(named, Entity(named, true, from, Sign.INT))
            val result = eval(itr.body)
            memory.leaveScope()
            if (result is Entity) {
                when (result.interruption) {
                    InterruptionType.BREAK -> break
                    InterruptionType.CONTINUE -> {
                        from = from + by
                        continue
                    }
                    InterruptionType.RETURN -> return result
                    InterruptionType.USE -> return result.value
                    else -> { }
                }
            }
            from = from + by
        }
        return EInt(numIterations)
    }

    override fun forLoop(forLoop: ForLoop): Any {
        memory.enterScope()
        forLoop.initializer?.let { eval(it) }

        val conditional = forLoop.conditional

        var numIterations = 0
        fun evalOperational() = forLoop.operational?.let { eval(it) }

        while (if (conditional == null) true else booleanExpr(conditional).get()) {
            numIterations++
            // Auto Scopped
            tracer?.runtimeFor(numIterations)
            val result = eval(forLoop.body)
            // Scope -> Memory -> Array
            if (result is Entity) {
                when (result.interruption) {
                    InterruptionType.BREAK -> break
                    InterruptionType.CONTINUE -> {
                        evalOperational()
                        continue
                    }
                    InterruptionType.RETURN -> {
                        memory.leaveScope()
                        return result
                    }
                    InterruptionType.USE -> {
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

    override fun interruption(interruption: Interruption) = when (val type = interruption.operator) {
        // wrap it as a normal entity, this will be naturally unboxed when called unbox()
        RETURN -> {
            // could be of a void type, so it could be null
            val expr = if (interruption.expr == null) 0 else unboxEval(interruption.expr)
            Entity("FlowReturn",
                false,
                expr,
                Sign.NONE,
                InterruptionType.RETURN)
        }
        USE -> Entity("FlowUse",
            false,
            unboxEval(interruption.expr!!),
            Sign.NONE,
            InterruptionType.USE)
        BREAK -> Entity("FlowBreak",
            false,
            0,
            Sign.NONE,
            InterruptionType.BREAK)
        CONTINUE -> Entity("FlowContinue",
            false,
            0,
            Sign.NONE,
            InterruptionType.CONTINUE)
        else -> throw RuntimeException("Unknown interruption type $type")
    }

    override fun whenExpr(whenExpr: When): Any {
        // Fully Auto Scopped
        val matchExpr = unboxEval(whenExpr.expr)
        for (match in whenExpr.matches)
            if (valueEquals(matchExpr, unboxEval(match.first)))
                return unboxEval(match.second)
        return unboxEval(whenExpr.defaultBranch)
    }

    override fun ifFunction(ifExpr: IfStatement): Any {
        val conditionSuccess = booleanExpr(ifExpr.condition).get()
        // Here it would be best if we could add a fallback NONE value that
        // would prevent us from doing a lot of if checks at runtime
        return eval(if (conditionSuccess) ifExpr.thenBody else ifExpr.elseBody)
    }

    override fun function(function: FunctionExpr): Any {
        memory.declareFn(function.name, function)
        tracer?.declareFn(function.name, function.arguments)
        return EBool(true)
    }

    override fun shado(shadow: Shadow) = shadow

    override fun arrayAccess(access: ArrayAccess): Any {
        val entity = unboxEval(access.expr)
        val index = intExpr(access.index).get()

        if (entity !is ArrayOperable<*>)
            throw RuntimeException("Unknown non-array operable element access of $entity")
        return entity.getAt(index)!!
    }
}
