package space.themelon.eia64.primitives

class EString(
    initialValue: String
): ArrayOperable<EChar>, Primitive<EString> {

    private var string = initialValue

    val length: Int get() = string.length

    fun append(value: Any) {
        string += value.toString()
    }

    override fun getAt(index: Int) = EChar(string[index])

    override fun setAt(index: Int, value: EChar) {
        string = string.replaceRange(index, index + 1, value.get().toString())
    }

    override fun set(value: Any) {
        if (value !is EString)
            throw IllegalArgumentException("EString.set() value is not a EString")
        string = value.string
    }

    override fun get(): String = string
    override fun stdlibName() = "string"

    override fun isCopyable() = true
    override fun copy() = EString(string)

    override fun toString() = string

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (other !is EString) return false
        return string == other.string
    }

    override fun hashCode() = string.hashCode()
}