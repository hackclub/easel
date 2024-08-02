package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.signatures.Matching.matches
import space.themelon.eia64.signatures.Signature
import space.themelon.eia64.syntax.Token

data class ExplicitVariable(
    val where: Token,
    val mutable: Boolean,
    val name: String,
    val expr: Expression,
    val explicitSignature: Signature
) : Expression(where) {

    init {
        sig()
    }

    override fun <R> accept(v: Visitor<R>) = v.variable(this)

    override fun sig(): Signature {
        val exprSig = expr.sig()
        if (!matches(expect = explicitSignature, got = exprSig)) {
            where.error<String>("Variable '$name' expected signature $explicitSignature but got $exprSig")
            throw RuntimeException()
        }
        return explicitSignature
    }
}