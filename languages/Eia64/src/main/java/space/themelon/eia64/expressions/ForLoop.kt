package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.signatures.Sign
import space.themelon.eia64.signatures.Signature
import space.themelon.eia64.syntax.Token

data class ForLoop(
    val where: Token,
    val initializer: Expression?, // sig checked
    val conditional: Expression?, // sig checked
    val operational: Expression?, // sig checked
    val body: Expression,
) : Expression(where) {

    override fun <R> accept(v: Visitor<R>) = v.forLoop(this)

    override fun sig(): Signature {
        initializer?.sig()
        conditional?.sig()
        operational?.sig()
        body.sig()

        return Sign.NONE
    }
}