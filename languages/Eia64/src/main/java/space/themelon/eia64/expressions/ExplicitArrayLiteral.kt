package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.signatures.ArrayExtension
import space.themelon.eia64.signatures.Matching.matches
import space.themelon.eia64.signatures.Signature
import space.themelon.eia64.syntax.Token

class ExplicitArrayLiteral(
    val where: Token,
    val elementSignature: Signature,
    val elements: List<Expression>,
) : Expression() {

    override fun <R> accept(v: Visitor<R>) = v.explicitArrayLiteral(this)

    override fun sig(): Signature {
        for ((index, expression) in elements.withIndex()) {
            val expressionSignature = expression.sig()
            if (!matches(elementSignature, expression.sig())) {
                where.error<String>(
                    "Array has signature of $elementSignature " +
                            "but contains element of signature $expressionSignature at index $index"
                )
            }
        }

        return ArrayExtension(elementSignature)
    }

    fun elementSignature() = elementSignature
}