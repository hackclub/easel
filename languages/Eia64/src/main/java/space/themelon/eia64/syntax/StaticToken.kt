package space.themelon.eia64.syntax

data class StaticToken(val type: Type, val flags: Array<Flag> = emptyArray()) {

    fun normalToken(lineCount: Int) = Token(lineCount, type, flags)

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as StaticToken

        if (type != other.type) return false
        if (!flags.contentEquals(other.flags)) return false

        return true
    }

    override fun hashCode(): Int {
        var result = type.hashCode()
        result = 31 * result + flags.contentHashCode()
        return result
    }
}