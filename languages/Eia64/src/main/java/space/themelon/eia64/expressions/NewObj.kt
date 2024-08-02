package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.analysis.FunctionReference
import space.themelon.eia64.signatures.Matching.matches
import space.themelon.eia64.signatures.ObjectExtension
import space.themelon.eia64.signatures.Signature
import space.themelon.eia64.syntax.Token

data class NewObj(
    val where: Token,
    val name: String,
    val arguments: List<Expression>, // sig checked
    val reference: FunctionReference // of init() function
) : Expression(where) {

    override fun <R> accept(v: Visitor<R>) = v.new(this)

    override fun sig(): Signature {
//        if (reference == null) {
//            if (arguments.isNotEmpty()) {
//                where.error<String>("init() expects no arguments but ${arguments.size} were provided")
//                throw RuntimeException()
//            }
//            return ObjectExtension(name)
//        }
        val argSigns = reference.parameters

        val expectedArgsSize = argSigns.size
        val suppliedArgsSize = arguments.size

        if (expectedArgsSize != suppliedArgsSize) {
            where.error<String>("init() expected $expectedArgsSize arguments but got $suppliedArgsSize")
        }

        val signIterator = argSigns.iterator()
        val argIterator = arguments.iterator()

        while (signIterator.hasNext()) {
            val argInfo = signIterator.next() // <ParameterName, Sign>

            val argName = argInfo.first
            val expectedArgSign = argInfo.second
            val suppliedArgSign = argIterator.next().sig()

            if (!matches(expectedArgSign, suppliedArgSign)) {
                where.error<String>("init() expected $expectedArgSign for argument $argName but got $suppliedArgSign")
            }
        }
        return ObjectExtension(name)
    }
}