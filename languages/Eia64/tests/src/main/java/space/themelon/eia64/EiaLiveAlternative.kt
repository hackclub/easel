package space.themelon.eia64

import java.util.*
import java.util.concurrent.Executor

object EiaLiveAlternative {
    @JvmStatic
    fun main(args: Array<String>) {
        val scanner = Scanner(System.`in`)

        var buffer = StringJoiner("\n")
        while (true) {
            print("> ")
            val line = scanner.nextLine()
            if (line == "exit") break
            else if (line == "~~") {
                buffer = StringJoiner("\n")
            }
            else buffer.add(line)
        }
    }
}