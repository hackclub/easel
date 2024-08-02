package space.themelon.eia64.signatures

abstract class Signature {
    fun isInt() = this == Sign.INT
    fun isFloat() = this == Sign.FLOAT

    fun isNumeric() = this == Sign.NUM || this == Sign.INT || this == Sign.FLOAT
    fun isNumericOrChar() = isNumeric() || this == Sign.CHAR

    abstract fun logName(): String
}