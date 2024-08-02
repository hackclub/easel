package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.signatures.SimpleSignature
import space.themelon.eia64.signatures.Sign

data class Include(
    val names: List<String>
) : Expression(null) {

    override fun <R> accept(v: Visitor<R>): R = v.include(this)

    override fun sig() = Sign.NONE
}