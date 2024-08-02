package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.signatures.Sign
import space.themelon.eia64.signatures.Signature
import space.themelon.eia64.syntax.Token

data class Until(
    val where: Token,
    val expression: Expression, // sig checked
    val body: Expression, // sig checked
) : Expression(where) {

    override fun <R> accept(v: Visitor<R>) = v.until(this)

    override fun sig(): Signature {
        expression.sig()
        body.sig()
        return Sign.ANY
    }
}