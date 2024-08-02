package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.signatures.Sign
import space.themelon.eia64.signatures.Signature
import space.themelon.eia64.syntax.Token
import space.themelon.eia64.syntax.Type

data class Interruption(
    val where: Token,
    val operator: Type,
    val expr: Expression? = null // sig checked
) : Expression(where) {

    override fun <R> accept(v: Visitor<R>) = v.interruption(this)

    override fun sig(): Signature {
        expr?.sig() // necessary

        if (operator == Type.RETURN || operator == Type.USE) {
            if (expr == null) {
                // `return` may not have an expression, but `use` must.
                if (operator == Type.USE) {
                    where.error<String>("No expression for operator $operator provided")
                } else return Sign.ANY
            }
            return expr!!.sig()
        }
        return Sign.ANY
    }
}