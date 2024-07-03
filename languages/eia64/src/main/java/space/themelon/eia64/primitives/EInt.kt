package space.themelon.eia64.primitives

class EInt(initialValue: Int): Primitive<EInt>, Comparable<EInt> {

    private var intValue = initialValue

    override fun set(value: Any) {
        if (value !is EInt)
            throw RuntimeException("EInt.set() value is not an EInt")
        intValue = value.intValue
    }

    override fun get() = intValue
    override fun stdlibName() = "int"

    override fun isCopyable() = true
    override fun copy() = EInt(intValue)

    fun getAndIncrement() = intValue++
    fun incrementAndGet() = ++intValue

    fun getAndDecrement() = intValue--
    fun decrementAndGet() = --intValue

    operator fun plus(other: EInt) = EInt(intValue + other.intValue)
    operator fun plusAssign(other: EInt) {
        intValue += other.intValue
    }

    operator fun minus(other: EInt) = EInt(intValue - other.intValue)
    operator fun minusAssign(other: EInt) {
        intValue -= other.intValue
    }

    operator fun times(other: EInt) = EInt(intValue * other.intValue)
    operator fun timesAssign(other: EInt) {
        intValue *= other.intValue
    }

    operator fun div(other: EInt) = EInt(intValue / other.intValue)
    operator fun divAssign(other: EInt) {
        intValue /= other.intValue
    }

    fun and(other: EInt) = EInt(intValue and other.intValue)
    fun or(other: EInt) = EInt(intValue or other.intValue)

    override fun compareTo(other: EInt) = intValue - other.intValue

    override fun toString() = intValue.toString()

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (other !is EInt) return false
        return intValue == other.intValue
    }

    override fun hashCode() = intValue
}