package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.signatures.Sign
import space.themelon.eia64.signatures.Signature
import space.themelon.eia64.syntax.Token

data class ThrowExpr(
    val where: Token,
    val error: Expression // sig checked
) : Expression(where) {

    override fun <R> accept(v: Visitor<R>) = v.throwExpr(this)

    override fun sig(): Signature {
        error.sig() // necessary
        return Sign.NONE
    }
}