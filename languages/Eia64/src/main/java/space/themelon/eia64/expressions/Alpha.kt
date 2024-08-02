package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.signatures.Signature
import space.themelon.eia64.signatures.SimpleSignature
import space.themelon.eia64.syntax.Token

data class Alpha(
    val where: Token,
    val index: Int,
    val value: String,
    val sign: Signature,
) : Expression(where) {

    override fun <R> accept(v: Visitor<R>) = v.alpha(this)
    override fun sig() = sign
}