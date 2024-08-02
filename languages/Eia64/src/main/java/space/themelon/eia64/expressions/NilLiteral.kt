package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.signatures.Sign
import space.themelon.eia64.syntax.Token

data class NilLiteral(val where: Token): Expression(where) {
    override fun <R> accept(v: Visitor<R>) = v.nilLiteral(this)

    override fun sig() = Sign.NIL
}