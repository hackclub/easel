package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.signatures.Sign
import space.themelon.eia64.signatures.Signature
import space.themelon.eia64.syntax.Token

data class ForEach(
    val where: Token,
    val name: String,
    val entity: Expression, // sig checked
    val body: Expression, // sig checked
) : Expression(where) {

    override fun <R> accept(v: Visitor<R>) = v.forEach(this)

    override fun sig(): Signature {
        entity.sig()
        body.sig()
        return Sign.NONE
    }
}