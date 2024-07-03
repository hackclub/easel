package space.themelon.eia64

import space.themelon.eia64.runtime.Executor
import java.util.*

object EiaLive {
    @JvmStatic
    fun main(args: Array<String>) {
        val scanner = Scanner(System.`in`)
        val executor = Executor()

        var buffer = StringJoiner("\n")
        while (true) {
            print("> ")
            val line = scanner.nextLine()
            if (line == "exit") break
            else if (line == "~~") {
                executor.loadSource(buffer.toString())
                buffer = StringJoiner("\n")
            }
            else buffer.add(line)
        }
    }
}