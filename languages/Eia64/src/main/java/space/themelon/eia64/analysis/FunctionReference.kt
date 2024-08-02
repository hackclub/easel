package space.themelon.eia64.analysis

import space.themelon.eia64.expressions.FunctionExpr
import space.themelon.eia64.signatures.Signature
import space.themelon.eia64.syntax.Token

data class FunctionReference(
    val where: Token,
    val name: String,
    var fnExpression: FunctionExpr? = null,
    val parameters: List<Pair<String, Signature>>, // List < < ParameterName, Signature >
    val argsSize: Int,
    val returnSignature: Signature,
    val isVoid: Boolean,
    val public: Boolean,
    val tokenIndex: Int
) {
    // it is important to override toString() or else it may cause recursive StackOverFlow error
    override fun toString() = "<${fnExpression?.name ?: "UnsetFunctionReference"}()>"
}