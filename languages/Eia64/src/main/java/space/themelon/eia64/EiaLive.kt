package space.themelon.eia64

import space.themelon.eia64.TerminalColors.BLUE
import space.themelon.eia64.TerminalColors.BOLD
import space.themelon.eia64.TerminalColors.RED
import space.themelon.eia64.TerminalColors.RESET
import space.themelon.eia64.TerminalColors.YELLOW
import space.themelon.eia64.runtime.Executor
import java.io.InputStream
import java.io.OutputStream
import java.io.PrintStream
import java.util.*
import java.util.concurrent.atomic.AtomicReference

class EiaLive(
    private val input: InputStream,
    private val output: OutputStream
) {

    companion object {
        private val INTRO = """
            Eia64 Dev 2.1
            Type "debug" to toggle debug mode
            
            
        """.trimIndent().encodeToByteArray()
        private val SHELL_STYLE = "${YELLOW}eia>$RESET ".toByteArray()
        private val PENDING_SHELL_STYLE = "> ".toByteArray()
        private val OUTPUT_STYLE = "$BLUE$BOLD".encodeToByteArray()
        private val ERROR_OUTPUT_STYLE = "$RED$BOLD".encodeToByteArray()
    }

    init {
        serve()
    }

    private fun serve() {
        val executor = AtomicReference(Executor())

        output.write(INTRO)
        output.write(SHELL_STYLE)

        executor.get().apply {
            standardInput = input
            standardOutput = PrintStream(output)
        }

        val helper = CompletionHelper(
            ready = { tokens ->
                if (tokens.isNotEmpty()) {
                    output.write(OUTPUT_STYLE)
                    runSafely(output) {
                        executor.get().loadMainTokens(tokens)
                    }
                }
                output.write(SHELL_STYLE)
            },
            syntaxError = { error ->
                output.write(ERROR_OUTPUT_STYLE)
                output.write("$error\n".encodeToByteArray())
                output.write(SHELL_STYLE)
            }
        )

        val scanner = Scanner(input)
        while (true) {
            val line = scanner.nextLine()
            if (line == "debug") {
                Executor.DEBUG = !Executor.DEBUG
                output.write(OUTPUT_STYLE)
                println("Debug mode ${if (Executor.DEBUG) "Enabled" else "Disabled"}")
                output.write(SHELL_STYLE)
            } else if (!helper.addLine(line)) {
                // There's some more code that needs to be typed in
                output.write(PENDING_SHELL_STYLE)
            }
        }
    }

    private fun runSafely(
        output: OutputStream,
        block: () -> Unit
    ) {
        try {
            block()
        } catch (e: Exception) {
            output.write(ERROR_OUTPUT_STYLE)
            output.write("${e.message.toString()}\n".encodeToByteArray())
        }
    }
}