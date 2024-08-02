package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.signatures.Sign

class NoneExpression: Expression() {

    companion object {
        val INSTANCE = NoneExpression()
    }

    override fun <R> accept(v: Visitor<R>) = v.noneExpression()
    override fun sig() = Sign.NONE
}