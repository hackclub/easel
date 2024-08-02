package space.themelon.eia64.runtime

import space.themelon.eia64.EiaTrace
import space.themelon.eia64.expressions.FunctionExpr
import space.themelon.eia64.runtime.Entity.Companion.getSignature
import space.themelon.eia64.runtime.Entity.Companion.unbox
import java.util.*
import kotlin.collections.ArrayList

class Memory(private val trace: EiaTrace?) {

    data class Frame(var fSuper: Frame? = null) {
        var values = ArrayList<Pair<String, Any>>()
        var functions = ArrayList<Pair<String, FunctionExpr>>()

        fun searchVr(index: Int, name: String): Any {
            if (values.size > index) {
                val get = values[index]
                if (get.first == name) return get.second
            }
            return fSuper?.searchVr(index, name) ?: throw RuntimeException("Unable to find variable '$name'")
        }

        fun searchFn(name: String): FunctionExpr? {
            for (function in functions) if (function.first == name) return function.second
            return fSuper?.searchFn(name)
        }

        fun reset(newSuper: Frame?) {
            fSuper = newSuper
            functions.clear()
            values.clear()
        }
    }

    //private val tracer = EiaTrace()

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
        //tracer.enterScope()
        trace?.enterScope() // forward calls
    }

    fun leaveScope() {
        // We move backwards
        val reusable = currentFrame
        currentFrame = reusable.fSuper ?: throw RuntimeException("Already reached super scope")

        frameStack.pop()
        recycle(reusable)
        //tracer.leaveScope()
        trace?.leaveScope() // forward calls
    }

    fun declareVar(name: String, value: Any) {
        currentFrame.values.add(Pair(name, value))
        // set mutable to false while at runtime to avoid complexity
        //tracer.declareVariable(false, name, getSignature(value))
    }

    fun declareFn(name: String, value: FunctionExpr) {
        // we need to define it so that external classes can access it
        currentFrame.functions.add(Pair(name, value))
        //tracer.declareFn(name, value.arguments)
    }

    fun getVar(index: Int, name: String): Any {
        val value = currentFrame.searchVr(index, name)
        val unboxed = unbox(value)
        trace?.getVariableRuntime(name, getSignature(unboxed), unboxed)
        return value
    }

    fun dynamicFnSearch(name: String): FunctionExpr? {
        if (frameStack.size != 1)
            throw RuntimeException("Dynamic search can only be requested from master scope")
        return currentFrame.searchFn(name)
    }

    fun clearMemory() {
        // move fully backwards
        while (currentFrame.fSuper != null) {
            leaveScope()
        }
        // we also need to reset the head node
        currentFrame.reset(null)
    }
}