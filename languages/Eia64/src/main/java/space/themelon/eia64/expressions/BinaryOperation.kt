package space.themelon.eia64.expressions

import space.themelon.eia64.Expression
import space.themelon.eia64.signatures.Matching.matches
import space.themelon.eia64.signatures.Matching.numericOrChar
import space.themelon.eia64.signatures.Sign
import space.themelon.eia64.signatures.Signature
import space.themelon.eia64.syntax.Token
import space.themelon.eia64.syntax.Type

data class BinaryOperation(
    val where: Token,
    val left: Expression, // sig checked
    val right: Expression, // sig checked
    val operator: Type
) : Expression(where) {

    override fun <R> accept(v: Visitor<R>) = v.binaryOperation(this)

    override fun sig(): Signature {
        val leftExprSign = left.sig()
        val rightExprSign = right.sig()

        val leftLogName = leftExprSign.logName()
        val rightLogName = rightExprSign.logName()

        var resultSign = leftExprSign
        when (operator) {
            Type.PLUS -> if (!leftExprSign.isNumeric() && !rightExprSign.isNumeric()) resultSign = Sign.STRING

            Type.NEGATE -> if (!leftExprSign.isNumeric() || !rightExprSign.isNumeric())
                applyError("arithmetic", "Numeric", "-")

            Type.TIMES -> if (!leftExprSign.isNumeric() || !rightExprSign.isNumeric())
                applyError("arithmetic", "Numeric", "*")

            Type.SLASH -> if (!leftExprSign.isNumeric() || !rightExprSign.isNumeric())
                applyError("arithmetic", "Numeric", "/")

            Type.REMAINDER -> if (!leftExprSign.isNumeric() || !rightExprSign.isNumeric())
                applyError("arithmetic", "Remainder", "%")

            Type.BITWISE_AND -> if (!leftExprSign.isNumeric() || !rightExprSign.isNumeric())
                applyError("bitwise", "Numeric", "&")

            Type.BITWISE_OR -> if (!leftExprSign.isNumeric() || !rightExprSign.isNumeric())
                applyError("bitwise", "Numeric", "|")

            Type.EQUALS, Type.NOT_EQUALS -> resultSign = Sign.BOOL

            Type.LOGICAL_AND -> if (leftExprSign != Sign.BOOL || rightExprSign != Sign.BOOL)
                applyError("logical", "Numeric", "&&")

            Type.LOGICAL_OR -> if (leftExprSign != Sign.BOOL || rightExprSign != Sign.BOOL) {
                applyError("logical", "Numeric", "||")
            } else resultSign = Sign.BOOL

            Type.RIGHT_DIAMOND -> if (!numericOrChar(left, right)) {
                where.error<String>("Cannot apply logical operator on non [Numeric/Char] expressions: ($leftLogName > $rightLogName)")
            } else resultSign = Sign.BOOL

            Type.LEFT_DIAMOND -> if (!numericOrChar(left, right)) {
                where.error<String>("Cannot apply logical operator on non [Numeric/Char] expressions: ($leftLogName < $rightLogName)")
            } else resultSign = Sign.BOOL

            Type.GREATER_THAN_EQUALS -> if (!numericOrChar(left, right)) {
                where.error<String>("Cannot apply logical operator on non [Numeric/Char] expressions: ($leftLogName >= $rightLogName)")
                resultSign = Sign.BOOL
            } else resultSign = Sign.BOOL

            Type.LESSER_THAN_EQUALS -> if (!numericOrChar(left, right)) {
                where.error<String>("Cannot apply logical operator on [Numeric/Char] expressions: ($leftLogName <= $rightLogName)")
            } else resultSign = Sign.BOOL

            Type.ASSIGNMENT -> {
                if (left is ArrayAccess) {
                    // Array Assignment Safety Check
                    // Can we really assign *this type* to that *array type*?
                    if (!matches(expect = leftExprSign, got = rightExprSign)) {
                        where.error<String>("Cannot assign type $rightExprSign to an array of type $leftExprSign")
                    }
                }
                resultSign = rightExprSign
            }

            Type.ADDITIVE_ASSIGNMENT -> when (rightExprSign) {
                Sign.STRING, Sign.CHAR -> resultSign = Sign.STRING
                Sign.INT -> resultSign = Sign.INT
                Sign.FLOAT -> resultSign = Sign.FLOAT
                else -> where.error("Unknown expression signature for operator (+= Additive Assignment): $rightExprSign")
            }

            Type.POWER -> if (!leftExprSign.isInt() || !rightExprSign.isInt()) applyError("arithmetic", "Numeric", "**")

            Type.DEDUCTIVE_ASSIGNMENT -> if (!rightExprSign.isNumeric()) simpleApplyError("Numeric", "-=")
            Type.MULTIPLICATIVE_ASSIGNMENT -> if (!rightExprSign.isNumeric()) simpleApplyError("Numeric", "*=")
            Type.DIVIDIVE_ASSIGNMENT -> if (!rightExprSign.isNumeric()) simpleApplyError("Numeric", "/=")
            Type.REMAINDER_ASSIGNMENT -> if (!rightExprSign.isNumeric()) simpleApplyError("Numeric", "%=")

            else -> where.error("Unknown Binary Operator $operator")
        }
        return resultSign
    }

    private fun applyError(group: String, type: String, operator: String) {
        where.error<String>("Cannot apply $group operator on non $type expressions: " +
                "(${left.sig().logName()} $operator ${right.sig().logName()})")
    }

    private fun simpleApplyError(type: String, operator: String) {
        where.error<String>("Expected $type expression for ($operator) but got ${right.sig().logName()}")
    }
}