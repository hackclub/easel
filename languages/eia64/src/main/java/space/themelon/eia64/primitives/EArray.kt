package space.themelon.eia64.primitives

class EArray(initialValue: Array<Any>): Primitive<EArray>, ArrayOperable<Any> {

    private var arrayValue = initialValue

    val size: Int
        get() = arrayValue.size

    override fun set(value: Any) {
        if (!(value is Array<*> && value.isArrayOf<Any>()))
            throw RuntimeException("EArray.set() value is not an Array")
        @Suppress("UNCHECKED_CAST")
        arrayValue = value as Array<Any>
    }

    override fun get() = arrayValue

    override fun getAt(index: Int): Any = arrayValue[index]

    override fun setAt(index: Int, value: Any) {
        arrayValue[index] = value
    }

    override fun stdlibName() = "array"

    override fun isCopyable() = false
    override fun copy(): EArray {
        throw RuntimeException("Cannot apply copy() on EArray")
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (other !is EArray) return false
        return arrayValue.contentEquals(other.arrayValue)
    }

    override fun hashCode() = arrayValue.contentHashCode()
    override fun toString() = "EArray(${arrayValue.contentToString()})"
}