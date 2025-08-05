fr fr Binary trees benchmark adapted from The Computer Language Benchmarks Game

yeet "fmt"

be_like TreeNode squad {
    left @TreeNode
    right @TreeNode
    item normie
}

slay new_tree(item normie, depth normie) @TreeNode {
    sus node @TreeNode = @TreeNode{left: cap, right: cap, item: item}
    
    lowkey depth > 0 {
        node.left = new_tree(2 * item - 1, depth - 1)
        node.right = new_tree(2 * item, depth - 1)
    }
    
    damn node
}

slay check_tree(node @TreeNode) normie {
    lowkey node == cap {
        damn 0
    }
    
    lowkey node.left == cap {
        damn node.item
    }
    
    damn node.item + check_tree(node.left) - check_tree(node.right)
}

slay main() {
    sus min_depth normie = 4
    sus max_depth normie = 12
    
    sus stretch_depth normie = max_depth + 1
    sus start_ts thicc = timez.now()
    
    fr fr Allocate and check a big tree
    sus big_tree @TreeNode = new_tree(0, stretch_depth)
    fmt.Println("stretch tree of depth", stretch_depth, "check:", check_tree(big_tree))
    
    fr fr Allow the big tree to be garbage collected
    big_tree = cap
    
    fr fr Allocate a long-lived binary tree
    sus long_lived_tree @TreeNode = new_tree(0, max_depth)
    
    sus iterations normie
    sus result normie
    sus depth normie
    
    fr fr Check trees of increasing depth
    bestie depth = min_depth; depth <= max_depth; depth += 2 {
        iterations = 1 << (max_depth - depth + min_depth)
        result = 0
        
        bestie i := 0; i < iterations; i++ {
            sus a @TreeNode = new_tree(i, depth)
            sus b @TreeNode = new_tree(-i, depth)
            result += check_tree(a) + check_tree(b)
        }
        
        fmt.Println(iterations * 2, "trees of depth", depth, "check:", result)
    }
    
    fmt.Println("long lived tree of depth", max_depth, "check:", check_tree(long_lived_tree))
    
    sus elapsed thicc = timez.now() - start_ts
    fmt.Println("Time taken:", elapsed, "ms")
    fmt.Println("Memory used:", stats.heap_memory(), "KB")
}