package space.themelon.eia64.runtime

import space.themelon.eia64.Expression
import java.util.*
import kotlin.collections.ArrayList

class Memory {

    data class Frame(var fSuper: Frame? = null) {
        var values = ArrayList<Pair<String, Any>>()
        var functions = ArrayList<Pair<String, Expression.Function>>()

        fun searchVr(index: Int, name: String): Any {
            if (values.size > index) {
                val get = values[index]
                if (get.first == name) return get.second
            }
            return fSuper?.searchVr(index, name) ?: throw RuntimeException("Unable to find variable '$name'")
        }

        fun searchFn(name: String): Expression.Function {
            for (function in functions) if (function.first == name) return function.second
            return fSuper?.searchFn(name) ?: throw RuntimeException("Unable to find function '$name'")
        }

        fun reset(newSuper: Frame) {
            fSuper = newSuper
            functions.clear()
            values.clear()
        }
    }

    private var recyclePool: Frame? = null

    private val frameStack = Stack<Frame>()
    private var currentFrame = Frame().also { frameStack.add(it) }

    private fun createFrame() = if (recyclePool == null) {
        Frame(currentFrame)
    } else {
        val tail = recyclePool
        recyclePool = tail?.fSuper

        tail!!.reset(currentFrame)
        tail
    }

    private fun recycle(reusable: Frame) {
        reusable.fSuper = recyclePool
        recyclePool = reusable
    }

    fun enterScope() {
        currentFrame = createFrame()
        frameStack.push(currentFrame)
    }

    fun leaveScope() {
        val reusable = currentFrame
        currentFrame = reusable.fSuper ?: throw RuntimeException("Already reached super scope")

        frameStack.pop()
        recycle(reusable)
    }

    fun declareVar(name: String, value: Any) {
        currentFrame.values.add(Pair(name, value))
    }

    fun declareFn(name: String, value: Expression.Function) {
        currentFrame.functions.add(Pair(name, value))
    }

    fun getVar(index: Int, name: String) = currentFrame.searchVr(index, name)

    fun dynamicFnSearch(name: String): Expression.Function {
        if (frameStack.size != 1)
            throw RuntimeException("Dynamic search can only be requested from master scope")
        return currentFrame.searchFn(name)
    }
}