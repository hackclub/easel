package space.themelon.eia64.syntax

import java.util.*
import kotlin.collections.HashMap

enum class Type {

    LOGICAL_AND, LOGICAL_OR,
    BITWISE_AND, BITWISE_OR,
    EQUALS, NOT_EQUALS,
    RIGHT_DIAMOND, LEFT_DIAMOND,
    GREATER_THAN_EQUALS, LESSER_THAN_EQUALS,
    SLASH, TIMES, REMAINDER, POWER,
    PLUS, NEGATE,
    NOT, INCREMENT, DECREMENT,
    USE,
    DOT,
    RIGHT_ARROW,

    COLON, DOUBLE_COLON,
    ASSIGNMENT,
    ADDITIVE_ASSIGNMENT, DEDUCTIVE_ASSIGNMENT,
    MULTIPLICATIVE_ASSIGNMENT, DIVIDIVE_ASSIGNMENT, REMAINDER_ASSIGNMENT,
    OPEN_CURVE, CLOSE_CURVE,
    OPEN_SQUARE, CLOSE_SQUARE,
    OPEN_CURLY, CLOSE_CURLY,
    COMMA,

    IS,

    E_NIL,
    E_NUMBER, E_INT, E_FLOAT, E_BOOL, E_STRING, E_CHAR,
    E_ARRAY, E_ANY, E_UNIT, E_OBJECT, E_TYPE,

    ALPHA,
    E_TRUE, E_FALSE, CLASS_VALUE,
    NIL,

    VISIBLE, INVISIBLE,

    BOOL_CAST, INT_CAST, FLOAT_CAST, CHAR_CAST, STRING_CAST,
    TYPE_OF,

    LET, VAR, SHADO, WHEN,
    IF, ELSE,
    EACH, TO, IN, BY, AS,
    FOR, UNTIL,
    FUN,
    ARRAY_OF, MAKE_ARRAY,
    COPY, TIME, RAND, PRINT, PRINTLN, READ, READLN, LEN, SLEEP, FORMAT, EXIT, MEM_CLEAR,
    INCLUDE, STD, STATIC, NEW,
    THROW, TRY, CATCH,

    RETURN, BREAK, CONTINUE,
    ;

    override fun toString() = name.lowercase(Locale.getDefault())

    companion object {
        val SYMBOLS = HashMap<String, StaticToken>()
        val KEYWORDS = HashMap<String, StaticToken>()

        init {
            SYMBOLS.let {
                // Binary operators arranged from the lowest precedence to highest

                it["="] = StaticToken(ASSIGNMENT, arrayOf(Flag.ASSIGNMENT_TYPE, Flag.OPERATOR, Flag.NONE))
                it["+="] = StaticToken(ADDITIVE_ASSIGNMENT, arrayOf(Flag.ASSIGNMENT_TYPE, Flag.OPERATOR, Flag.NONE))
                it["-="] = StaticToken(DEDUCTIVE_ASSIGNMENT, arrayOf(Flag.ASSIGNMENT_TYPE, Flag.OPERATOR, Flag.NONE))
                it["*="] = StaticToken(MULTIPLICATIVE_ASSIGNMENT, arrayOf(Flag.ASSIGNMENT_TYPE, Flag.OPERATOR, Flag.NONE))
                it["/="] = StaticToken(DIVIDIVE_ASSIGNMENT, arrayOf(Flag.ASSIGNMENT_TYPE, Flag.OPERATOR, Flag.NONE))
                it["%="] = StaticToken(REMAINDER_ASSIGNMENT, arrayOf(Flag.ASSIGNMENT_TYPE, Flag.OPERATOR, Flag.NONE))

                it["||"] = StaticToken(LOGICAL_OR, arrayOf(Flag.LOGICAL_OR, Flag.OPERATOR))
                it["&&"] = StaticToken(LOGICAL_AND, arrayOf(Flag.LOGICAL_AND, Flag.OPERATOR))

                it["|"] = StaticToken(BITWISE_OR, arrayOf(Flag.BITWISE_OR, Flag.OPERATOR))
                it["&"] = StaticToken(BITWISE_AND, arrayOf(Flag.BITWISE_AND, Flag.OPERATOR))

                it["=="] = StaticToken(EQUALS, arrayOf(Flag.EQUALITY, Flag.OPERATOR))
                it["!="] = StaticToken(NOT_EQUALS, arrayOf(Flag.EQUALITY, Flag.OPERATOR))

                it[">"] = StaticToken(RIGHT_DIAMOND, arrayOf(Flag.RELATIONAL, Flag.OPERATOR))
                it["<"] = StaticToken(LEFT_DIAMOND, arrayOf(Flag.RELATIONAL, Flag.OPERATOR))
                it[">="] = StaticToken(GREATER_THAN_EQUALS, arrayOf(Flag.RELATIONAL, Flag.OPERATOR))
                it["<="] = StaticToken(LESSER_THAN_EQUALS, arrayOf(Flag.RELATIONAL, Flag.OPERATOR))

                it["/"] = StaticToken(SLASH, arrayOf(Flag.BINARY_L2, Flag.PRESERVE_ORDER, Flag.OPERATOR))
                it["*"] = StaticToken(TIMES, arrayOf(Flag.BINARY_L2, Flag.PRESERVE_ORDER, Flag.OPERATOR))
                it["%"] = StaticToken(REMAINDER, arrayOf(Flag.BINARY_L2, Flag.PRESERVE_ORDER, Flag.OPERATOR))

                it["^"] = StaticToken(POWER, arrayOf(Flag.BINARY_L3, Flag.PRESERVE_ORDER, Flag.OPERATOR))

                it["+"] = StaticToken(PLUS, arrayOf(Flag.BINARY, Flag.OPERATOR))
                it["-"] = StaticToken(NEGATE, arrayOf(Flag.BINARY, Flag.UNARY, Flag.OPERATOR))

                it["!"] = StaticToken(NOT, arrayOf(Flag.UNARY))
                it["++"] = StaticToken(INCREMENT, arrayOf(Flag.UNARY, Flag.POSSIBLE_RIGHT_UNARY))
                it["--"] = StaticToken(DECREMENT, arrayOf(Flag.UNARY, Flag.POSSIBLE_RIGHT_UNARY))

                it["."] = StaticToken(DOT)
                it["->"] = StaticToken(RIGHT_ARROW)

                it[":"] = StaticToken(COLON)
                it["::"] = StaticToken(DOUBLE_COLON)

                it["["] = StaticToken(OPEN_SQUARE, arrayOf(Flag.NONE))
                it["]"] = StaticToken(CLOSE_SQUARE, arrayOf(Flag.NONE))

                it["("] = StaticToken(OPEN_CURVE, arrayOf(Flag.NONE))
                it[")"] = StaticToken(CLOSE_CURVE, arrayOf(Flag.NONE))
                it["{"] = StaticToken(OPEN_CURLY, arrayOf(Flag.NONE))
                it["}"] = StaticToken(CLOSE_CURLY, arrayOf(Flag.NONE))
                it[","] = StaticToken(COMMA, arrayOf(Flag.NONE))

                it[":="] = StaticToken(USE, arrayOf(Flag.INTERRUPTION))
            }

            KEYWORDS.let {
                it["Nil"] = StaticToken(E_NIL, arrayOf(Flag.CLASS))
                it["Number"] = StaticToken(E_NUMBER, arrayOf(Flag.CLASS))
                it["Int"] = StaticToken(E_INT, arrayOf(Flag.CLASS))
                it["Float"] = StaticToken(E_FLOAT, arrayOf(Flag.CLASS))
                it["Bool"] = StaticToken(E_BOOL, arrayOf(Flag.CLASS))
                it["String"] = StaticToken(E_STRING, arrayOf(Flag.CLASS))
                it["Char"] = StaticToken(E_CHAR, arrayOf(Flag.CLASS))
                it["Any"] = StaticToken(E_ANY, arrayOf(Flag.CLASS))
                it["Array"] = StaticToken(E_ARRAY, arrayOf(Flag.CLASS))
                it["Unit"] = StaticToken(E_UNIT, arrayOf(Flag.CLASS))
                it["Object"] = StaticToken(E_OBJECT, arrayOf(Flag.CLASS))
                it["Type"] = StaticToken(E_TYPE, arrayOf(Flag.CLASS))

                it["nil"] = StaticToken(NIL, arrayOf(Flag.VALUE))
                it["true"] = StaticToken(E_TRUE, arrayOf(Flag.VALUE, Flag.E_BOOL))
                it["false"] = StaticToken(E_FALSE, arrayOf(Flag.VALUE, Flag.E_BOOL))
                it["type"] = StaticToken(CLASS_VALUE, arrayOf(Flag.VALUE))

                it["visible"] = StaticToken(VISIBLE, arrayOf(Flag.MODIFIER))
                it["private"] = StaticToken(INVISIBLE, arrayOf(Flag.MODIFIER))

                it["bool"] = StaticToken(BOOL_CAST, arrayOf(Flag.NATIVE_CALL))
                it["int"] = StaticToken(INT_CAST, arrayOf(Flag.NATIVE_CALL))
                it["float"] = StaticToken(FLOAT_CAST, arrayOf(Flag.NATIVE_CALL))
                it["char"] = StaticToken(CHAR_CAST, arrayOf(Flag.NATIVE_CALL))
                it["str"] = StaticToken(STRING_CAST, arrayOf(Flag.NATIVE_CALL))

                it["is"] = StaticToken(IS, arrayOf(Flag.IS, Flag.OPERATOR))

                it["typeOf"] = StaticToken(TYPE_OF, arrayOf(Flag.NATIVE_CALL))
                it["copy"] = StaticToken(COPY, arrayOf(Flag.NATIVE_CALL))

                it["arrayOf"] = StaticToken(ARRAY_OF)
                it["makeArray"] = StaticToken(MAKE_ARRAY)

                it["time"] = StaticToken(TIME, arrayOf(Flag.NATIVE_CALL))
                it["rand"] = StaticToken(RAND, arrayOf(Flag.NATIVE_CALL))
                it["print"] = StaticToken(PRINT, arrayOf(Flag.NATIVE_CALL))
                it["println"] = StaticToken(PRINTLN, arrayOf(Flag.NATIVE_CALL))
                it["read"] = StaticToken(READ, arrayOf(Flag.NATIVE_CALL))
                it["readln"] = StaticToken(READLN, arrayOf(Flag.NATIVE_CALL))
                it["sleep"] = StaticToken(SLEEP, arrayOf(Flag.NATIVE_CALL))
                it["len"] = StaticToken(LEN, arrayOf(Flag.NATIVE_CALL))
                it["format"] = StaticToken(FORMAT, arrayOf(Flag.NATIVE_CALL))
                it["exit"] = StaticToken(EXIT, arrayOf(Flag.NATIVE_CALL))
                it["memclear"] = StaticToken(MEM_CLEAR, arrayOf(Flag.NATIVE_CALL))

                it["std"] = StaticToken(STD)
                it["static"] = StaticToken(STATIC)
                it["include"] = StaticToken(INCLUDE)
                it["new"] = StaticToken(NEW)
                it["throw"] = StaticToken(THROW)
                it["try"] = StaticToken(TRY)
                it["catch"] = StaticToken(CATCH)

                it["until"] = StaticToken(UNTIL, arrayOf(Flag.LOOP)) // auto scope
                it["each"] = StaticToken(EACH, arrayOf(Flag.LOOP)) // TODO check this later
                it["to"] = StaticToken(TO)
                it["in"] = StaticToken(IN)
                it["by"] = StaticToken(BY)
                it["as"] = StaticToken(AS)
                it["for"] = StaticToken(FOR, arrayOf(Flag.LOOP)) // auto scope

                it["let"] = StaticToken(LET, arrayOf(Flag.V_KEYWORD))
                it["var"] = StaticToken(VAR, arrayOf(Flag.V_KEYWORD))

                it["if"] = StaticToken(IF, arrayOf(Flag.NONE)) // auto scope
                it["else"] = StaticToken(ELSE, arrayOf(Flag.NONE))

                it["fn"] = StaticToken(FUN, arrayOf(Flag.NONE)) // manual scope
                it["shado"] = StaticToken(SHADO) // manual scope
                it["when"] = StaticToken(WHEN) // auto scope

                it["return"] = StaticToken(RETURN, arrayOf(Flag.INTERRUPTION))
                it["break"] = StaticToken(BREAK, arrayOf(Flag.INTERRUPTION))
                it["continue"] = StaticToken(CONTINUE, arrayOf(Flag.INTERRUPTION))
            }
        }
    }
}