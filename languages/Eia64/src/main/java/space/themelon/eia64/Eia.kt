package space.themelon.eia64

import space.themelon.eia64.runtime.Executor
import java.io.File

object Eia {

    private val directory = File(System.getProperty("user.dir"))

    @JvmStatic
    fun main(args: Array<String>) {
        setStdLibPath()

        val iterator = args.iterator()
        val live: Boolean // true => live mode, else a file
        var sourceFile = ""

        if (args.isEmpty()) {
            // defaults to live mode
            live = true
        } else {
            val type = iterator.next()
            if (type == "live") live = true
            else {
                live = false
                sourceFile = type
            }
        }
        val props = HashMap<String, String>(3)
        while (iterator.hasNext()) {
            iterator.next().split("=").let {
                if (it.size == 2) props[it[0]] = it[1]
            }
        }
        props["debug"]?.let { Executor.DEBUG = it == "true" }
        props["pipe"]?.let { Executor.LOGS_PIPE_PATH = it }
        props["stdlib"]?.let { Executor.STD_LIB = it }

        if (live) {
            EiaLive(System.`in`, System.out)
        } else {
            val executor = Executor()
            if (!sourceFile.startsWith('/')) {
                sourceFile = directory.absolutePath + "/" + sourceFile
            }
            val file = File(sourceFile)
            if (!file.isFile || !file.exists()) {
                println("Cannot find source file '$file', make sure it is a full valid path")
                return
            }
            executor.loadMainFile(file.absolutePath)
        }
    }

    private fun setStdLibPath() {
        val stdlib = File("$directory/stdlib")
        if (!stdlib.isDirectory || !stdlib.exists()) {
            // In the future, it would be convenient to pack stdlib/ in the JAR itself
            // This is not to be an error, stdlib could also be set using stdlib=flag
            println("Cannot find stdlib/ in the local directory")
            return
        }
        Executor.STD_LIB = stdlib.absolutePath
    }
}