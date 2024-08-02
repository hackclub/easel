package space.themelon.eia64

import space.themelon.eia64.runtime.Executor
import java.io.File

object Main {
    @JvmStatic
    fun main(args: Array<String>) {
        val directory = File(System.getProperty("user.dir"))
        val stdlib = File("$directory/stdlib")
        if (!stdlib.isDirectory || !stdlib.exists()) {
            println("Cannot find stdlib/ in the local directory")
            return
        }
        Executor.STD_LIB = stdlib.absolutePath
        if (args.isNotEmpty()) {
            val iterator = args.iterator()
            if (args[0] == "debug") {
                Executor.DEBUG = true
                iterator.next()
            }
            startProcess(iterator, directory)
        } else startProcess(args.iterator(), directory) // args is empty
    }

    private fun startProcess(argsIterator: Iterator<String>, directory: File) {
        val next = if (argsIterator.hasNext()) argsIterator.next() else null
        if (next == null || next == "live") {
            EiaLive(System.`in`, System.out)
        } else {
            val executor = Executor()
            val startTime = System.nanoTime()

            var sourceFile = next
            if (!sourceFile.startsWith('/')) sourceFile = directory.absolutePath + "/" + sourceFile
            val file = File(sourceFile)
            if (!file.isFile || !file.exists()) {
                println("Cannot find source file '$file', make sure it is a full valid path")
                return
            }
            executor.loadMainFile(file.absolutePath)
            println("Took " + (System.nanoTime() - startTime) + " ns")
        }
    }
}