package space.themelon.eia64.primitives

class EBool(initialValue: Boolean): Primitive<EBool> {

    private var boolValue = initialValue

    override fun set(value: Any) {
        if (value !is EBool)
            throw RuntimeException("EBool.set() value is not a Bool")
        boolValue = value.boolValue
    }

    override fun get() = boolValue
    override fun stdlibName(): String {
        throw UnsupportedOperationException()
    }

    override fun isCopyable() = true
    override fun copy() = EBool(boolValue)

    // they must NOT be used as logical operators, we need to always
    // apply manual and() and or() while performing logical operations
    fun and(other: EBool) = EBool(boolValue && other.boolValue)
    fun or(other: EBool) = EBool(boolValue || other.boolValue)

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (other !is EBool) return false
        return boolValue == other.boolValue
    }

    override fun toString() = boolValue.toString()
    override fun hashCode() = boolValue.hashCode()
}