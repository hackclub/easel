package space.themelon.eia64.runtime

import space.themelon.eia64.Expression
import space.themelon.eia64.expressions.*

class VoidEvaluator() : Expression.Visitor<Any> {
    val className
        get() = "VoidEvaluator"

    override fun noneExpression(): Any {
        throw ShutdownException()
    }

    override fun nilLiteral(nil: NilLiteral): Any {
        throw ShutdownException()
    }

    override fun intLiteral(literal: IntLiteral): Any {
        throw ShutdownException()
    }

    override fun floatLiteral(literal: FloatLiteral): Any {
        throw ShutdownException()
    }

    override fun boolLiteral(literal: BoolLiteral): Any {
        throw ShutdownException()
    }

    override fun stringLiteral(literal: StringLiteral): Any {
        throw ShutdownException()
    }

    override fun charLiteral(literal: CharLiteral): Any {
        throw ShutdownException()
    }

    override fun typeLiteral(literal: TypeLiteral): Any {
        throw ShutdownException()
    }

    override fun alpha(alpha: Alpha): Any {
        throw ShutdownException()
    }

    override fun explicitArrayLiteral(arrayCreation: ExplicitArrayLiteral): Any {
        throw ShutdownException()
    }

    override fun arrayAllocation(arrayAllocation: ArrayAllocation): Any {
        throw ShutdownException()
    }

    override fun array(literal: ArrayLiteral): Any {
        throw ShutdownException()
    }

    override fun include(include: Include): Any {
        throw ShutdownException()
    }

    override fun new(new: NewObj): Any {
        throw ShutdownException()
    }

    override fun throwExpr(throwExpr: ThrowExpr): Any {
        throw ShutdownException()
    }

    override fun tryCatch(tryCatch: TryCatch): Any {
        throw ShutdownException()
    }

    override fun variable(variable: ExplicitVariable): Any {
        throw ShutdownException()
    }

    override fun autoVariable(autoVariable: AutoVariable): Any {
        throw ShutdownException()
    }

    override fun isStatement(isStatement: IsStatement): Any {
        throw ShutdownException()
    }

    override fun shado(shadow: Shadow): Any {
        throw ShutdownException()
    }

    override fun unaryOperation(expr: UnaryOperation): Any {
        throw ShutdownException()
    }

    override fun binaryOperation(expr: BinaryOperation): Any {
        throw ShutdownException()
    }

    override fun expressions(list: ExpressionList): Any {
        throw ShutdownException()
    }

    override fun nativeCall(call: NativeCall): Any {
        throw ShutdownException()
    }

    override fun cast(cast: Cast): Any {
        throw ShutdownException()
    }

    override fun scope(scope: Scope): Any {
        throw ShutdownException()
    }

    override fun methodCall(call: MethodCall): Any {
        throw ShutdownException()
    }

    override fun classPropertyAccess(propertyAccess: ForeignField): Any {
        throw ShutdownException()
    }

    override fun classMethodCall(call: ClassMethodCall): Any {
        throw ShutdownException()
    }

    override fun unitInvoke(shadoInvoke: ShadoInvoke): Any {
        throw ShutdownException()
    }

    override fun until(until: Until): Any {
        throw ShutdownException()
    }

    override fun itr(itr: Itr): Any {
        throw ShutdownException()
    }

    override fun whenExpr(whenExpr: When): Any {
        throw ShutdownException()
    }

    override fun forEach(forEach: ForEach): Any {
        throw ShutdownException()
    }

    override fun forLoop(forLoop: ForLoop): Any {
        throw ShutdownException()
    }

    override fun interruption(interruption: Interruption): Any {
        throw ShutdownException()
    }

    override fun ifFunction(ifExpr: IfStatement): Any {
        throw ShutdownException()
    }

    override fun function(function: FunctionExpr): Any {
        throw ShutdownException()
    }

    override fun arrayAccess(access: ArrayAccess): Any {
        throw ShutdownException()
    }

}
