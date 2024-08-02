package space.themelon.eia64.expressions

import space.themelon.eia64.Expression

data class Scope(
    val expr: Expression, // sig checked
    val imaginary: Boolean
) : Expression(null) {

    override fun <R> accept(v: Visitor<R>) = v.scope(this)

    override fun sig() = expr.sig()
}