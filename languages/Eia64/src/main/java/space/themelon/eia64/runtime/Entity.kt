package space.themelon.eia64.runtime

import space.themelon.eia64.Expression
import space.themelon.eia64.primitives.*
import space.themelon.eia64.signatures.ArrayExtension
import space.themelon.eia64.signatures.Matching.matches
import space.themelon.eia64.signatures.ObjectExtension
import space.themelon.eia64.signatures.Sign
import space.themelon.eia64.signatures.Signature

open class Entity(
    private val name: String,
    private val mutable: Boolean,
    var value: Any,
    val signature: Signature,
    val interruption: InterruptionType = InterruptionType.NONE,
) {

    open fun update(another: Any) {
        if (!mutable) throw RuntimeException("Entity $name is immutable")
        if (signature == Sign.ANY) value = another
        else {
            val otherSignature = getSignature(another)
            if (!matches(signature, otherSignature)) throw RuntimeException("Entity $name cannot change type $signature to $otherSignature")
            value = unbox(another)
        }
    }

    companion object {
        fun unbox(value: Any): Any {
            return if (value is Entity) {
                // break that return boxing
                if (value.interruption != InterruptionType.NONE) unbox(value.value)
                else value.value
            } else value
        }

        fun getSignature(value: Any): Signature = when (value) {
            is Entity -> {
                // repeatedly break that repeat unboxing
                //  to retrieve the underlying value
                if (value.interruption != InterruptionType.NONE) getSignature(value.value)
                else value.signature
            }
            is ENil -> Sign.NIL
            is EInt -> Sign.INT
            is EFloat -> Sign.FLOAT
            is EString -> Sign.STRING
            is EBool -> Sign.BOOL
            is EChar -> Sign.CHAR
            is EArray -> ArrayExtension(value.elementSignature)
            is EType -> Sign.TYPE
            is Expression -> Sign.UNIT
            is Evaluator -> ObjectExtension(value.className)
            else -> throw RuntimeException("Unknown type of value $value")
        }
    }
}