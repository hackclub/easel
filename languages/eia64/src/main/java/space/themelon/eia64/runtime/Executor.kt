package space.themelon.eia64.runtime

import space.themelon.eia64.analysis.Parser
import space.themelon.eia64.syntax.Lexer
import java.io.File

class Executor {

    companion object {
        var STD_LIB = ""
    }

    init {
        if (STD_LIB.isBlank())
            throw RuntimeException("STD_LIB is not set")
    }

    private val externalEvaluator = HashMap<String, Evaluator>()

    private val mainParser = Parser()

    private val mainEvaluator = Evaluator(this)

    fun loadFile(sourceFile: String) {
        mainEvaluator.eval(mainParser.parse(getTokens(sourceFile)))
    }

    fun loadSource(source: String) {
        mainEvaluator.eval(mainParser.parse(Lexer(source).tokens))
    }

    fun loadExternal(sourceFile: String, name: String) {
        if (externalEvaluator[name] != null) return
        Evaluator(this).apply {
            externalEvaluator[name] = this
            eval(Parser().parse(getTokens(sourceFile)))
        }
    }

    fun getExternalExecutor(name: String) = externalEvaluator[name]

    private fun getTokens(sourceFile: String) = Lexer(File(sourceFile).readText()).tokens
}