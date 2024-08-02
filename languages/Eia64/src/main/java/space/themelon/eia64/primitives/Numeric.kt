package space.themelon.eia64.primitives

interface Numeric {

    fun get(): Number

    fun getAndIncrement(): Number { throw NotImplementedError() }
    fun incrementAndGet(): Number { throw NotImplementedError() }

    fun getAndDecrement(): Number { throw NotImplementedError() }
    fun decrementAndGet(): Number { throw NotImplementedError() }

    operator fun plus(number: Numeric): Numeric { throw NotImplementedError() }
    operator fun plusAssign(number: Numeric) { throw NotImplementedError() }

    operator fun minus(number: Numeric): Numeric { throw NotImplementedError() }
    operator fun minusAssign(number: Numeric) { throw NotImplementedError() }

    operator fun times(number: Numeric): Numeric { throw NotImplementedError() }
    operator fun timesAssign(number: Numeric) { throw NotImplementedError() }

    operator fun div(number: Numeric): Numeric { throw NotImplementedError() }
    operator fun divAssign(number: Numeric) { throw NotImplementedError() }

    fun and(number: Numeric): Numeric { throw NotImplementedError() }
    fun or(number: Numeric): Numeric { throw NotImplementedError() }

    operator fun compareTo(number: Numeric): Int { throw NotImplementedError() }
}