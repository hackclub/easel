package space.themelon.eia64.primitives

class EInt(initialValue: Int): Primitive<EInt>, Comparable<EInt>, Numeric {

    private var intValue = initialValue

    override fun set(value: Any) {
        if (value !is EInt)
            throw RuntimeException("EInt.set() value is not an EInt")
        intValue = value.intValue
    }

    override fun get() = intValue
    override fun stdlibName() = "eint"

    override fun isCopyable() = true
    override fun copy() = EInt(intValue)

    override fun getAndIncrement() = intValue++
    override fun incrementAndGet() = ++intValue

    override fun getAndDecrement() = intValue--
    override fun decrementAndGet() = --intValue

    override operator fun plus(number: Numeric) = EInt(intValue + number.get().toInt())
    override operator fun plusAssign(number: Numeric) {
        intValue += number.get().toInt()
    }

    override operator fun minus(number: Numeric) = EInt(intValue - number.get().toInt())
    override operator fun minusAssign(number: Numeric) {
        intValue -= number.get().toInt()
    }

    override operator fun times(number: Numeric) = EInt(intValue * number.get().toInt())
    override operator fun timesAssign(number: Numeric) {
        intValue *= number.get().toInt()
    }

    override operator fun div(number: Numeric) = EInt(intValue / number.get().toInt())
    override operator fun divAssign(number: Numeric) {
        intValue /= number.get().toInt()
    }

    override operator fun rem(number: Numeric) = EInt(intValue % number.get().toInt())
    override operator fun remAssign(number: Numeric) {
        intValue %= number.get().toInt()
    }

    override fun and(number: Numeric) = EInt(intValue and number.get().toInt())
    override fun or(number: Numeric) = EInt(intValue or number.get().toInt())

    override fun compareTo(other: EInt) = intValue - other.intValue
    override fun compareTo(number: Numeric) = intValue - number.get().toInt()

    override fun toString() = intValue.toString()

    override fun equals(other: Any?): Boolean {
        if (other !is Numeric) return false
        return intValue == other.get().toInt()
    }

    override fun hashCode() = intValue
}