package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.signatures.SimpleSignature
import space.themelon.eia64.signatures.Sign
import space.themelon.eia64.syntax.Token

data class IntLiteral(
    val where: Token,
    val value: Int
) : Expression(where) {

    override fun <R> accept(v: Visitor<R>) = v.intLiteral(this)

    override fun sig() = Sign.INT
}