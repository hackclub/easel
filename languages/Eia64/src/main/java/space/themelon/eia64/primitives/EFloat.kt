package space.themelon.eia64.primitives

class EFloat(initialValue: Float): Primitive<EFloat>, Comparable<EFloat>, Numeric {

    private var floatValue = initialValue

    override fun set(value: Any) {
        if (value !is EFloat)
            throw RuntimeException("EFloat.set() value is not an EFloat")
        floatValue = value.floatValue
    }

    override fun get() = floatValue

    override fun stdlibName() = "float"

    override fun isCopyable() = true
    override fun copy() = EFloat(floatValue)

    override fun compareTo(other: EFloat) = floatValue.compareTo(other.floatValue)
    override fun compareTo(number: Numeric) = floatValue.compareTo(number.get().toFloat())

    override fun toString() = floatValue.toString()

    override fun equals(other: Any?): Boolean {
        if (other !is Numeric) return false
        return floatValue == other.get().toFloat()
    }

    override fun hashCode() = floatValue.hashCode()

    override fun getAndIncrement() = floatValue++
    override fun incrementAndGet() = ++floatValue

    override fun getAndDecrement() = floatValue--
    override fun decrementAndGet() = --floatValue

    override operator fun plus(number: Numeric) = EFloat(floatValue + number.get().toFloat())
    override operator fun plusAssign(number: Numeric) {
        floatValue += number.get().toFloat()
    }

    override operator fun minus(number: Numeric) = EFloat(floatValue - number.get().toFloat())
    override operator fun minusAssign(number: Numeric) {
        floatValue -= number.get().toFloat()
    }

    override operator fun times(number: Numeric) = EFloat(floatValue * number.get().toFloat())
    override operator fun timesAssign(number: Numeric) {
        floatValue *= number.get().toFloat()
    }

    override operator fun div(number: Numeric) = EFloat(floatValue / number.get().toFloat())
    override operator fun divAssign(number: Numeric) {
        floatValue /= number.get().toFloat()
    }

    override fun and(number: Numeric) =
        EFloat(Float.fromBits(floatValue.toRawBits() and number.get().toFloat().toRawBits()))

    override fun or(number: Numeric) =
        EFloat(Float.fromBits(floatValue.toRawBits() or number.get().toFloat().toRawBits()))
}