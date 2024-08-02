package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.signatures.Sign
import space.themelon.eia64.signatures.Signature

data class IsStatement(
    val expression: Expression, // sig checked
    val signature: Signature,
): Expression() {

    override fun <R> accept(v: Visitor<R>) = v.isStatement(this)

    override fun sig(): Signature {
        expression.sig() // necessary
        return Sign.BOOL
    }
}