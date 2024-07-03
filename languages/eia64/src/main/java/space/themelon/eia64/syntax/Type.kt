package space.themelon.eia64.syntax

import java.util.*
import kotlin.collections.HashMap

enum class Type {

    LOGICAL_AND, LOGICAL_OR,
    BITWISE_AND, BITWISE_OR,
    EQUALS, NOT_EQUALS,
    GREATER_THAN, LESSER_THAN,
    GREATER_THAN_EQUALS, LESSER_THAN_EQUALS,
    SLASH, TIMES, POWER,
    PLUS, NEGATE,
    NOT, INCREMENT, DECREMENT,
    USE,
    DOT,
    RIGHT_ARROW,

    COLON,
    ASSIGNMENT,
    ADDITIVE_ASSIGNMENT, DEDUCTIVE_ASSIGNMENT,
    MULTIPLICATIVE_ASSIGNMENT, DIVIDIVE_ASSIGNMENT,
    OPEN_CURVE, CLOSE_CURVE,
    OPEN_SQUARE, CLOSE_SQUARE,
    OPEN_CURLY, CLOSE_CURLY,
    COMMA,

    E_INT, E_BOOL, E_STRING, E_CHAR,
    E_ARRAY, E_ANY, E_UNIT,

    ALPHA,
    E_TRUE, E_FALSE,

    BOOL_CAST, INT_CAST, STRING_CAST,
    TYPE,

    LET, VAR, SHADO, WHEN,
    IF, ELSE,
    ITR, TO, IN, BY,
    FOR, UNTIL,
    FUN,
    INCLUDE, COPY, ARRALLOC, ARRAYOF, TIME, RAND, PRINT, PRINTLN, READ, READLN, LEN, SLEEP, FORMAT, EXIT,
    STDLIB,

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

                it["||"] = StaticToken(LOGICAL_OR, arrayOf(Flag.LOGICAL_OR, Flag.OPERATOR))
                it["&&"] = StaticToken(LOGICAL_AND, arrayOf(Flag.LOGICAL_AND, Flag.OPERATOR))

                it["|"] = StaticToken(BITWISE_OR, arrayOf(Flag.BITWISE_OR, Flag.OPERATOR))
                it["&"] = StaticToken(BITWISE_AND, arrayOf(Flag.BITWISE_AND, Flag.OPERATOR))

                it["=="] = StaticToken(EQUALS, arrayOf(Flag.EQUALITY, Flag.OPERATOR))
                it["!="] = StaticToken(NOT_EQUALS, arrayOf(Flag.EQUALITY, Flag.OPERATOR))

                it[">"] = StaticToken(GREATER_THAN, arrayOf(Flag.RELATIONAL, Flag.OPERATOR))
                it["<"] = StaticToken(LESSER_THAN, arrayOf(Flag.RELATIONAL, Flag.OPERATOR))
                it[">="] = StaticToken(GREATER_THAN_EQUALS, arrayOf(Flag.RELATIONAL, Flag.OPERATOR))
                it["<="] = StaticToken(LESSER_THAN_EQUALS, arrayOf(Flag.RELATIONAL, Flag.OPERATOR))

                it["/"] = StaticToken(SLASH, arrayOf(Flag.BINARY_L2, Flag.PRESERVE_ORDER, Flag.OPERATOR))
                it["*"] = StaticToken(TIMES, arrayOf(Flag.BINARY_L2, Flag.PRESERVE_ORDER, Flag.OPERATOR))
                it["^"] = StaticToken(POWER, arrayOf(Flag.BINARY_L3, Flag.PRESERVE_ORDER, Flag.OPERATOR))

                it["+"] = StaticToken(PLUS, arrayOf(Flag.BINARY, Flag.OPERATOR))
                it["-"] = StaticToken(NEGATE, arrayOf(Flag.BINARY, Flag.UNARY, Flag.OPERATOR))

                it["!"] = StaticToken(NOT, arrayOf(Flag.UNARY))
                it["++"] = StaticToken(INCREMENT, arrayOf(Flag.UNARY, Flag.POSSIBLE_RIGHT_UNARY))
                it["--"] = StaticToken(DECREMENT, arrayOf(Flag.UNARY, Flag.POSSIBLE_RIGHT_UNARY))

                it["."] = StaticToken(DOT)
                it["->"] = StaticToken(RIGHT_ARROW)

                it[":"] = StaticToken(COLON)

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
                it["Int"] = StaticToken(E_INT, arrayOf(Flag.CLASS))
                it["Bool"] = StaticToken(E_BOOL, arrayOf(Flag.CLASS))
                it["String"] = StaticToken(E_STRING, arrayOf(Flag.CLASS))
                it["Char"] = StaticToken(E_CHAR, arrayOf(Flag.CLASS))
                it["Any"] = StaticToken(E_ANY, arrayOf(Flag.CLASS))
                it["Array"] = StaticToken(E_ARRAY, arrayOf(Flag.CLASS))
                it["Unit"] = StaticToken(E_UNIT, arrayOf(Flag.CLASS))

                it["true"] = StaticToken(E_TRUE, arrayOf(Flag.VALUE, Flag.E_BOOL))
                it["false"] = StaticToken(E_FALSE, arrayOf(Flag.VALUE, Flag.E_BOOL))

                it["bool"] = StaticToken(BOOL_CAST, arrayOf(Flag.NATIVE_CALL))
                it["int"] = StaticToken(INT_CAST, arrayOf(Flag.NATIVE_CALL))
                it["str"] = StaticToken(STRING_CAST, arrayOf(Flag.NATIVE_CALL))

                it["type"] = StaticToken(TYPE, arrayOf(Flag.NATIVE_CALL))

                it["include"] = StaticToken(INCLUDE, arrayOf(Flag.NATIVE_CALL))
                it["copy"] = StaticToken(COPY, arrayOf(Flag.NATIVE_CALL))
                it["arralloc"] = StaticToken(ARRALLOC, arrayOf(Flag.NATIVE_CALL))
                it["arrayOf"] = StaticToken(ARRAYOF, arrayOf(Flag.NATIVE_CALL))
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

                it["stdlib"] = StaticToken(STDLIB)

                it["until"] = StaticToken(UNTIL, arrayOf(Flag.LOOP)) // auto scope
                it["itr"] = StaticToken(ITR, arrayOf(Flag.LOOP)) // TODO check this later
                it["to"] = StaticToken(TO)
                it["in"] = StaticToken(IN)
                it["by"] = StaticToken(BY)
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