// Binary trees benchmark for Kotlin

import java.util.*
import kotlin.system.measureTimeMillis

class TreeNode(val item: Int, var left: TreeNode? = null, var right: TreeNode? = null)

// Create a tree of given depth
fun makeTree(depth: Int, item: Int = 0): TreeNode {
    return if (depth <= 0) {
        TreeNode(item)
    } else {
        TreeNode(
            item = item,
            left = makeTree(depth - 1, 2 * item - 1),
            right = makeTree(depth - 1, 2 * item)
        )
    }
}

// Calculate checksum of tree
fun checkTree(node: TreeNode?): Int {
    if (node == null) return 0
    if (node.left == null) return node.item
    return node.item + checkTree(node.left) - checkTree(node.right)
}

fun main() {
    val minDepth = 4
    val maxDepth = 12
    val stretchDepth = maxDepth + 1
    
    val totalTime = measureTimeMillis {
        // Stretch tree
        val stretchTree = makeTree(stretchDepth)
        println("stretch tree of depth $stretchDepth\t check: ${checkTree(stretchTree)}")
        
        // Long-lived tree
        val longLivedTree = makeTree(maxDepth)
        
        // Process trees of increasing depths
        for (depth in minDepth..maxDepth step 2) {
            val iterations = 1 shl (maxDepth - depth + minDepth)
            var check = 0
            
            for (i in 0 until iterations) {
                val a = makeTree(depth, i)
                val b = makeTree(depth, -i)
                check += checkTree(a) + checkTree(b)
            }
            
            println("${iterations * 2}\t trees of depth $depth\t check: $check")
        }
        
        // Check long-lived tree at the end
        println("long lived tree of depth $maxDepth\t check: ${checkTree(longLivedTree)}")
    }
    
    println("Time taken: $totalTime ms")
}