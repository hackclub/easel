package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.signatures.Sign
import space.themelon.eia64.signatures.Signature
import space.themelon.eia64.syntax.Token

data class Itr(
    val where: Token,
    val name: String,
    val from: Expression, // sig checked
    val to: Expression, // sig checked
    val by: Expression?, // sig checked
    val body: Expression, // sig checked
) : Expression(where) {

    override fun <R> accept(v: Visitor<R>) = v.itr(this)

    override fun sig(): Signature {
        from.sig()
        to.sig()
        by?.sig()
        body.sig()
        return Sign.ANY
    }
}