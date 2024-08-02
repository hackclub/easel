package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.analysis.ModuleInfo
import space.themelon.eia64.analysis.UniqueVariable
import space.themelon.eia64.signatures.Signature
import space.themelon.eia64.syntax.Token

data class ForeignField(
    val where: Token,
    val static: Boolean,
    val objectExpression: Expression,
    val property: String,
    val uniqueVariable: UniqueVariable,
    val moduleInfo: ModuleInfo,
): Expression(where) {

    override fun <R> accept(v: Visitor<R>) = v.classPropertyAccess(this)

    override fun sig(): Signature {
        objectExpression.sig()
        return uniqueVariable.signature
    }
}