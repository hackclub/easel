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

        if (args.size != 1) throw RuntimeException("eia <source_path> or eia live")
        if (args[0] == "live") {
            EiaLive.main(emptyArray<String>())
        } else {
            val executor = Executor()
            val startTime = System.nanoTime()

            var sourceFile = args[0]
            if (!sourceFile.startsWith('/')) {
                sourceFile = directory.absolutePath + "/" + sourceFile
            }
            val file = File(sourceFile)
            if (!file.isFile || !file.exists()) {
                println("Cannot find source file '$file', make sure it is a full valid path")
                return
            }
            executor.loadFile(file.absolutePath)
            println("Took " + (System.nanoTime() - startTime) + " ns")
        }
    }
}