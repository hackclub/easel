package space.themelon.eia64.analysis

class NameResolver {

    class Scope(val before: Scope? = null) {
        val names = ArrayList<String>()
        val functions = ArrayList<String>()

        val funcObjs = HashMap<String, FnElement>()

        fun resolveFn(name: String, travelDepth: Int): FnElement? {
            functions.indexOf(name).let {
                if (it != -1) return funcObjs[name]!!
                if (before != null) return before.resolveFn(name, travelDepth + 1)
                return null
            }
        }

        fun resolveVr(name: String): Int {
            names.indexOf(name).let { if (it != -1) return it }
            if (before != null) return before.resolveVr(name)
            return -1
        }
    }

    val classes = ArrayList<String>()

    private var currentScope = Scope()

    fun enterScope() {
        val newScope = Scope(currentScope)
        currentScope = newScope
    }

    fun leaveScope(): Boolean {
        // imaginary scope is a scope where you don't have to actually create a new scope
        // you could run without it, consider this situation:
        // let x = 5
        // if (x) { println("Hello, "World") }
        // here you don't require creating a new scope to evaluate it
        val imaginaryScope = currentScope.names.isEmpty() && currentScope.functions.isEmpty()
        currentScope.before.let {
            if (it == null)
                throw RuntimeException("Reached super scope")
            currentScope = it
        }
        return imaginaryScope
    }

    fun defineFn(name: String, fnExpression: FnElement) {
        if (name in currentScope.functions)
            throw RuntimeException("Function $name is already defined in the current scope")
        currentScope.functions += name
        currentScope.funcObjs[name] = fnExpression
    }

    fun defineVr(name: String) {
        if (name in currentScope.names)
            throw RuntimeException("Variable $name is already defined in the current scope")
        currentScope.names += name
    }

    fun resolveFn(name: String) = currentScope.resolveFn(name, 0)

    fun resolveVr(name: String) = currentScope.resolveVr(name)
}