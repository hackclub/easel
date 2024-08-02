package space.themelon.eia64.primitives

class ENil: Primitive<ENil> {
    override fun set(value: Any) {
        throw UnsupportedOperationException()
    }

    override fun get(): Any {
        throw UnsupportedOperationException()
    }

    override fun stdlibName(): String {
        throw UnsupportedOperationException()
    }

    override fun isCopyable() = true

    override fun copy(): ENil = this

    override fun equals(other: Any?) = other is ENil
    override fun hashCode() = 0

    override fun toString() = "ENil"
}