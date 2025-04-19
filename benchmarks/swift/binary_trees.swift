// BinaryTrees benchmark for Swift

import Foundation

// Tree node class
class TreeNode {
    var left: TreeNode?
    var right: TreeNode?
    var item: Int
    
    init(item: Int, left: TreeNode? = nil, right: TreeNode? = nil) {
        self.item = item
        self.left = left
        self.right = right
    }
}

// Create a binary tree of specified depth
func makeTree(depth: Int, item: Int = 0) -> TreeNode {
    if depth <= 0 {
        return TreeNode(item: item)
    } else {
        return TreeNode(
            item: item,
            left: makeTree(depth: depth - 1, item: 2 * item - 1),
            right: makeTree(depth: depth - 1, item: 2 * item)
        )
    }
}

// Check a tree by calculating its sum
func checkTree(_ node: TreeNode?) -> Int {
    guard let node = node else { return 0 }
    return node.item + checkTree(node.left) - checkTree(node.right)
}

func main() {
    let minDepth = 4
    let maxDepth = 12
    let stretchDepth = maxDepth + 1
    
    let startTime = Date()
    
    // Create and check stretch tree
    let stretchTree = makeTree(depth: stretchDepth)
    print("stretch tree of depth \(stretchDepth)\t check: \(checkTree(stretchTree))")
    
    // Create long-lived tree
    let longLivedTree = makeTree(depth: maxDepth)
    
    // Process trees of increasing depth
    for depth in stride(from: minDepth, through: maxDepth, by: 2) {
        let iterations = 1 << (maxDepth - depth + minDepth)
        var check = 0
        
        for i in 0..<iterations {
            let a = makeTree(depth: depth, item: i)
            let b = makeTree(depth: depth, item: -i)
            check += checkTree(a) + checkTree(b)
        }
        
        print("\(iterations * 2)\t trees of depth \(depth)\t check: \(check)")
    }
    
    // Check long-lived tree
    print("long lived tree of depth \(maxDepth)\t check: \(checkTree(longLivedTree))")
    
    let elapsedTime = -startTime.timeIntervalSinceNow * 1000
    print("Time taken: \(elapsedTime) ms")
}

main()