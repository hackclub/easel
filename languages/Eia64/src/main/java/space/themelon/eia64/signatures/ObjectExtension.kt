package space.themelon.eia64.signatures

class ObjectExtension(
    val extensionClass: String // could be `Object` or a `Car` (Object extension) or a `Bus`
) : Signature() {
    override fun equals(other: Any?): Boolean {
        return other is ObjectExtension && other.extensionClass == extensionClass
    }

    override fun logName() = "Object<$extensionClass>"

    override fun hashCode() = extensionClass.hashCode()

    override fun toString() = "ObjectExtension<$extensionClass>"
}