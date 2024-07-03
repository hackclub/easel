package space.themelon.eia64.primitives

class EChar(initialValue: Char): Primitive<EChar> {

    private var charValue = initialValue

    override fun set(value: Any) {
        if (value !is EChar)
            throw IllegalArgumentException("EChar.set() value is not a Char")
        charValue = value.charValue
    }

    override fun get() = charValue
    override fun stdlibName() = "char"

    override fun isCopyable() = true
    override fun copy() = EChar(charValue)

    override fun toString() = charValue.toString()

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (other !is EChar) return false
        return charValue == other.charValue
    }

    override fun hashCode() = charValue.hashCode()
}