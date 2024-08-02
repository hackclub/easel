package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.signatures.SimpleSignature
import space.themelon.eia64.signatures.Sign
import space.themelon.eia64.signatures.Signature
import space.themelon.eia64.syntax.Type

data class ExpressionList(
    val expressions: List<Expression>,
    var preserveState: Boolean = false,
) : Expression(null) {

    val size = expressions.size
    override fun <R> accept(v: Visitor<R>) = v.expressions(this)

    fun returnSig(): Signature {
        val expressionItr = expressions.iterator()
        while (expressionItr.hasNext()) {
            val expr = expressionItr.next()
            if (expr is Interruption && expr.operator == Type.RETURN) {
                // this ensures there are no more statements after
                // return statement is encountered
                if (expressionItr.hasNext()) {
                    expr.where.error<String>("Cannot have more statements after return")
                }
                return expr.sig()
            }
        }
        return Sign.NONE
    }

    override fun sig(): Signature {
        val expressionItr = expressions.iterator()
        while (expressionItr.hasNext()) {
            val expr = expressionItr.next()
            val signature = expr.sig() // Invoke sig() on ALL of the statements
            if (expr is Interruption && expr.operator == Type.RETURN) {
                if (expressionItr.hasNext()) {
                    expr.where.error<String>("Cannot have more statements after return")
                }
                return signature
            }
        }
        // return last signature of the expression, useful for units
        return expressions.last().sig()
    }
}