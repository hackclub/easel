package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.signatures.Sign
import space.themelon.eia64.signatures.Signature
import space.themelon.eia64.syntax.Token

data class TryCatch(
    val where: Token,
    val tryBlock: Expression, // sig checked
    val catchIdentifier: String,
    val catchBlock: Expression, // sig checked
): Expression(where) {

    override fun <R> accept(v: Visitor<R>) = v.tryCatch(this)

    override fun sig(): Signature {
        val trySignature = tryBlock.sig()
        val catchSignature = catchBlock.sig()

        if (trySignature == catchSignature) return trySignature
        return Sign.ANY
    }
}