package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.signatures.*
import space.themelon.eia64.signatures.Matching.matches
import space.themelon.eia64.syntax.Token

class ArrayAllocation(
    val where: Token,
    private val elementSignature: Signature,
    val size: Expression,
    val defaultValue: Expression,
) : Expression(where) {

    override fun <R> accept(v: Visitor<R>) = v.arrayAllocation(this)

    override fun sig(): Signature {
        val gotSig = size.sig()
        if (!matches(Sign.INT, gotSig)) {
            where.error<String>("Array allocation expects an Int for array length, but got $gotSig")
        }
        if (!matches(elementSignature, defaultValue.sig())) {
            where.error<String>("arralloc() element signature and default value does not match (type mismatch)")
        }
        return ArrayExtension(elementSignature)
    }
}