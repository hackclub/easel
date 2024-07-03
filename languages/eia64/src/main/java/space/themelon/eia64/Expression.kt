package space.themelon.eia64

import space.themelon.eia64.analysis.FnElement
import space.themelon.eia64.syntax.Type

abstract class Expression {

    interface Visitor<R> {
        fun genericLiteral(literal: GenericLiteral): R
        fun intLiteral(intLiteral: IntLiteral): R
        fun boolLiteral(boolLiteral: BoolLiteral): R
        fun stringLiteral(stringLiteral: StringLiteral): R
        fun charLiteral(charLiteral: CharLiteral): R
        fun alpha(alpha: Alpha): R
        fun operator(operator: Operator): R
        fun importStdLib(stdLib: ImportStdLib): R
        fun variable(variable: ExplicitVariable): R
        fun autoVariable(autoVariable: AutoVariable): R
        fun shado(shadow: Shadow): R
        fun unaryOperation(expr: UnaryOperation): R
        fun binaryOperation(expr: BinaryOperation): R
        fun expressions(list: ExpressionList): R
        fun nativeCall(call: NativeCall): R
        fun scope(scope: Scope): R
        fun methodCall(call: MethodCall): R
        fun classMethodCall(call: ClassMethodCall): R
        fun unitInvoke(shadoInvoke: ShadoInvoke): R
        fun until(until: Until): R
        fun itr(itr: Itr): R
        fun whenExpr(whenExpr: When): R
        fun forEach(forEach: ForEach): R
        fun forLoop(forLoop: ForLoop): R
        fun interruption(interruption: Interruption): R
        fun ifFunction(ifExpr: If): R
        fun function(function: Function): R
        fun elementAccess(access: ElementAccess): R
    }

    abstract fun <R> accept(v: Visitor<R>): R

    // for internal evaluation use
    data class GenericLiteral(val value: Any): Expression() {
        override fun <R> accept(v: Visitor<R>) = v.genericLiteral(this)
    }

    data class IntLiteral(val value: Int): Expression() {
        override fun <R> accept(v: Visitor<R>) = v.intLiteral(this)
    }

    data class BoolLiteral(val value: Boolean): Expression() {
        override fun <R> accept(v: Visitor<R>) = v.boolLiteral(this)
    }

    data class StringLiteral(val value: String): Expression() {
        override fun <R> accept(v: Visitor<R>) = v.stringLiteral(this)
    }

    data class CharLiteral(val value: Char): Expression() {
        override fun <R> accept(v: Visitor<R>) = v.charLiteral(this)
    }

    data class Alpha(val index: Int, val value: String) : Expression() {
        override fun <R> accept(v: Visitor<R>) = v.alpha(this)
    }

    data class Operator(val value: Type) : Expression() {
        override fun <R> accept(v: Visitor<R>) = v.operator(this)
    }

    open class ImportStdLib(val names: List<String>): Expression() {
        override fun <R> accept(v: Visitor<R>): R = v.importStdLib(this)
    }

    data class UnaryOperation(
        val operator: Operator,
        val expr: Expression,
        val left: Boolean
    ) : Expression() {
        override fun <R> accept(v: Visitor<R>) = v.unaryOperation(this)
    }

    data class BinaryOperation(
        val left: Expression,
        val right: Expression,
        val operator: Operator) : Expression() {
        override fun <R> accept(v: Visitor<R>) = v.binaryOperation(this)
    }

    data class DefinitionType(
        val name: String,
        val type: Type
    )

    data class ExplicitVariable(
        val mutable: Boolean,
        val definition: DefinitionType,
        val expr: Expression
    ) : Expression() {
        override fun <R> accept(v: Visitor<R>) = v.variable(this)
    }

    data class AutoVariable(
        val name: String,
        val expr: Expression
    ) : Expression() {
        override fun <R> accept(v: Visitor<R>) = v.autoVariable(this)
    }

    data class NativeCall(
        val type: Type,
        val arguments: ExpressionList,
    ) : Expression() {
        override fun <R> accept(v: Visitor<R>) = v.nativeCall(this)
    }

    data class Scope(
        val expr: Expression,
        val imaginary: Boolean
    ): Expression() {
        override fun <R> accept(v: Visitor<R>) = v.scope(this)
    }

    data class MethodCall(
        val fnExpr: FnElement,
        val arguments: List<Expression>,
    ) : Expression() {
        override fun <R> accept(v: Visitor<R>) = v.methodCall(this)
    }

    data class ClassMethodCall(
        val static: Boolean,
        val obj: Expression,
        val method: String,
        val arguments: List<Expression>
    ): Expression() {
        override fun <R> accept(v: Visitor<R>) = v.classMethodCall(this)
    }

    data class ShadoInvoke(
        val expr: Expression,
        val arguments: List<Expression>
    ): Expression() {
        override fun <R> accept(v: Visitor<R>) = v.unitInvoke(this)
    }

    data class ExpressionList(
        val expressions: List<Expression>,
        var preserveState: Boolean = false,
    ) : Expression() {
        val size = expressions.size
        override fun <R> accept(v: Visitor<R>) = v.expressions(this)
    }

    data class ForEach(
        val name: String,
        val entity: Expression,
        val body: Expression,
    ): Expression() {
        override fun <R> accept(v: Visitor<R>) = v.forEach(this)
    }

    data class ForLoop(
        val initializer: Expression?,
        val conditional: Expression?,
        val operational: Expression?,
        val body: Expression,
    ) : Expression() {
        override fun <R> accept(v: Visitor<R>) = v.forLoop(this)
    }

    data class Itr(
        val name: String,
        val from: Expression,
        val to: Expression,
        val by: Expression?,
        val body: Expression,
    ) : Expression() {
        override fun <R> accept(v: Visitor<R>) = v.itr(this)
    }

    data class When(
        val expr: Expression,
        val matches: List<Pair<Expression, Expression>>,
        val defaultBranch: Expression,
    ): Expression() {
        override fun <R> accept(v: Visitor<R>) = v.whenExpr(this)
    }

    data class Until(
        val expression: Expression,
        val body: Expression,
    ) : Expression() {
        override fun <R> accept(v: Visitor<R>) = v.until(this)
    }

    data class If(
        val condition: Expression,
        val thenBody: Expression,
        val elseBody: Expression? = null,
    ) : Expression() {
        override fun <R> accept(v: Visitor<R>) = v.ifFunction(this)
    }

    data class Interruption(val type: Operator, val expr: Expression? = null)
        : Expression() {
        override fun <R> accept(v: Visitor<R>) = v.interruption(this)
    }

    data class Function(
        val name: String,
        val arguments: List<DefinitionType>,
        val returnType: Type,
        val body: Expression
    ) : Expression() {
        override fun <R> accept(v: Visitor<R>) = v.function(this)
    }

    data class Shadow(
        val names: List<String>,
        val body: Expression
    ): Expression() {
        override fun <R> accept(v: Visitor<R>) = v.shado(this)
    }

    data class ElementAccess(
        val expr: Expression,
        val index: Expression
    ) : Expression() {
        override fun <R> accept(v: Visitor<R>) = v.elementAccess(this)
    }
}