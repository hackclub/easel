package space.themelon.eia64.runtime

import space.themelon.eia64.analysis.ParserX
import space.themelon.eia64.syntax.Lexer
import space.themelon.eia64.syntax.Token
import java.io.File
import kotlin.system.exitProcess

class Executor {

    companion object {
        var DEBUG = false
        // where runtime logs are displayed
        var LOGS_PIPE_PATH = "/tmp/pipe1"

        var STD_LIB = "" // will be set
        var EXECUTION_DIRECTORY: String = File(System.getProperty("user.dir")).absolutePath

        // This unit could be overridden to replace default exitProcess() behaviour
        // When you are demonstrating Eia for e.g., in a server, you shouldn't to allow a random
        // dude to shut down your whole server by doing exit(n) in Eia
        var EIA_SHUTDOWN: (Int) -> Unit = { exitCode -> exitProcess(exitCode) }
    }

    init {
        if (STD_LIB.isBlank()) throw RuntimeException("STD_LIB is not set")
    }


    // why do we do this? sometimes while we are developing demonstrable
    // APIs for Eia64, we would want the output to be captured in memory and
    // sent somewhere else
    var standardOutput = System.out
    var standardInput = System.`in`

    private val externalExecutors = HashMap<String, Evaluator>()
    private val mainEvaluator = Evaluator("Main", this)

    private val externalParsers = HashMap<String, ParserX>()
    private val mainParser = ParserX(this)

    fun loadMainFile(sourceFile: String) {
        try {
            mainEvaluator.mainEval(mainParser.parse(getTokens(sourceFile)))
        } catch (e: ShutdownException) {
            standardOutput.println("Executor was shutdown")
        }
    }

    fun loadMainSource(source: String): Any {
        try {
            val tokens = mainParser.parse(Lexer(source).tokens)
            return mainEvaluator.eval(tokens)
        } catch (e: ShutdownException) {
            throw RuntimeException("Executor was shutdown")
        }
    }

    fun loadMainTokens(tokens: List<Token>): Any {
        try {
            return mainEvaluator.eval(mainParser.parse(tokens))
        } catch (e: ShutdownException) {
            throw RuntimeException("Executor was shutdown")
        }
    }

    // this can be used to enforce restriction on the execution time
    // of the program, while in demonstration environments

    fun shutdownEvaluator() {
        mainEvaluator.shutdown()
    }

    // maybe for internal testing only
    private fun clearMemories() {
        mainEvaluator.clearMemory()
        externalExecutors.values.forEach {
            it.clearMemory()
        }
    }

    // called by parsers, parse the included module
    fun addModule(sourceFile: String, name: String): Boolean {
        if (externalParsers[name] != null) return false
        externalParsers[name] = ParserX(this).also { it.parse(getTokens(sourceFile)) }
        return true
    }

    fun getModule(name: String) = externalParsers[name] ?: throw RuntimeException("Could not find module '$name'")

    // loads the included module and executes it
    fun executeModule(name: String): Evaluator {
        val evaluator = newEvaluator(name)
        externalExecutors[name] = evaluator
        return evaluator
    }

    fun newEvaluator(name: String) = Evaluator(name, this).also {
        it.eval((externalParsers[name] ?: throw RuntimeException("Static module '$name') not found")).parsed)
    }

    fun getEvaluator(name: String) = externalExecutors[name]

    private fun getTokens(sourceFile: String) = Lexer(File(sourceFile).readText()).tokens
}