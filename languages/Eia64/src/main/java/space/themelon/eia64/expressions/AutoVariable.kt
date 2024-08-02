package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.syntax.Token

data class AutoVariable(
    val where: Token,
    val name: String,
    val expr: Expression
) : Expression(where) {

    override fun <R> accept(v: Visitor<R>) = v.autoVariable(this)

    override fun sig() = expr.sig()
}