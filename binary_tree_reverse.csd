vibe main
yeet "vibez"

fr fr Binary tree node structure
be_like TreeNode squad {
    value normie
    left @TreeNode
    right @TreeNode
}

fr fr Function to create a new tree node
slay newNode(value normie) @TreeNode {
    sus node @TreeNode = &TreeNode{
        value: value,
        left: nah,
        right: nah,
    }
    damn node
}

fr fr Function to reverse/invert a binary tree
slay reverseTree(node @TreeNode) @TreeNode {
    fr fr Base case: if node is null, return null
    ready node == nah {
        damn nah
    }
    
    fr fr Recursively reverse left and right subtrees
    sus leftReversed @TreeNode = reverseTree(node.left)
    sus rightReversed @TreeNode = reverseTree(node.right)
    
    fr fr Swap the left and right children
    node.left = rightReversed
    node.right = leftReversed
    
    damn node
}

fr fr Function to print tree in-order (for testing purposes)
slay printInOrder(node @TreeNode) {
    ready node != nah {
        printInOrder(node.left)
        vibez.spill(node.value)
        printInOrder(node.right)
    }
}

fr fr Main function demonstrating binary tree reversal
slay main_character() {
    vibez.spill("Creating binary tree...")
    
    fr fr Create a sample binary tree:
    fr fr       1
    fr fr      / \
    fr fr     2   3
    fr fr    / \
    fr fr   4   5
    
    sus root @TreeNode = newNode(1)
    root.left = newNode(2)
    root.right = newNode(3)
    root.left.left = newNode(4)
    root.left.right = newNode(5)
    
    vibez.spill("Original tree (in-order):")
    printInOrder(root)
    
    fr fr Reverse the binary tree
    sus reversedRoot @TreeNode = reverseTree(root)
    
    vibez.spill("Reversed tree (in-order):")
    printInOrder(reversedRoot)
    
    vibez.spill("Binary tree reversal complete!")
}
