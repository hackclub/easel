package space.themelon.eia64

import space.themelon.eia64.expressions.*
import space.themelon.eia64.expressions.ArrayLiteral
import space.themelon.eia64.expressions.FunctionExpr
import space.themelon.eia64.signatures.Signature
import space.themelon.eia64.syntax.Token

abstract class Expression(
    val marking: Token? = null,
) {

    interface Visitor<R> {
        fun noneExpression(): R
        fun nilLiteral(nil: NilLiteral): R
        fun intLiteral(literal: IntLiteral): R
        fun floatLiteral(literal: FloatLiteral): R
        fun boolLiteral(literal: BoolLiteral): R
        fun stringLiteral(literal: StringLiteral): R
        fun charLiteral(literal: CharLiteral): R
        fun typeLiteral(literal: TypeLiteral): R
        fun alpha(alpha: Alpha): R
        fun array(literal: ArrayLiteral): R
        fun explicitArrayLiteral(arrayCreation: ExplicitArrayLiteral): R
        fun arrayAllocation(arrayAllocation: ArrayAllocation): R
        fun include(include: Include): R
        fun new(new: NewObj): R
        fun throwExpr(throwExpr: ThrowExpr): R
        fun tryCatch(tryCatch: TryCatch): R
        fun variable(variable: ExplicitVariable): R
        fun autoVariable(autoVariable: AutoVariable): R
        fun isStatement(isStatement: IsStatement): R
        fun shado(shadow: Shadow): R
        fun unaryOperation(expr: UnaryOperation): R
        fun binaryOperation(expr: BinaryOperation): R
        fun expressions(list: ExpressionList): R
        fun nativeCall(call: NativeCall): R
        fun cast(cast: Cast): R
        fun scope(scope: Scope): R
        fun methodCall(call: MethodCall): R
        fun classPropertyAccess(propertyAccess: ForeignField): R
        fun classMethodCall(call: ClassMethodCall): R
        fun unitInvoke(shadoInvoke: ShadoInvoke): R
        fun until(until: Until): R
        fun itr(itr: Itr): R
        fun whenExpr(whenExpr: When): R
        fun forEach(forEach: ForEach): R
        fun forLoop(forLoop: ForLoop): R
        fun interruption(interruption: Interruption): R
        fun ifFunction(ifExpr: IfStatement): R
        fun function(function: FunctionExpr): R
        fun arrayAccess(access: ArrayAccess): R
    }

    abstract fun <R> accept(v: Visitor<R>): R
    abstract fun sig(): Signature
}