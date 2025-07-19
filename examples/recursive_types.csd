fr fr Example demonstrating recursive type definitions in CURSED

fr fr Simple linked list node
be_like Node squad {
    value normie
    next *Node
}

fr fr Binary tree node 
be_like TreeNode squad {
    value normie
    left *TreeNode
    right *TreeNode
}

fr fr Mutually recursive types for a graph structure
be_like GraphNode squad {
    id normie
    edges []*GraphEdge
}

be_like GraphEdge squad {
    from *GraphNode
    to *GraphNode
    weight normie
}

fr fr Generic recursive list
be_like List[T] squad {
    head *ListNode[T]
    size normie
}

be_like ListNode[T] squad {
    value T
    next *ListNode[T]
}

fr fr Complex recursive structure for an expression tree
be_like Expr squad {
    type tea
}

be_like BinaryExpr squad {
    left *Expr
    right *Expr
    operator tea
}

be_like UnaryExpr squad {
    operand *Expr
    operator tea
}

fr fr Recursive function to traverse linked list
slay traverse_list(head *Node) normie {
    if head == nil {
        return 0
    }
    return 1 + traverse_list(head.next)
}

fr fr Function to create a simple linked list
slay create_list() *Node {
    first := Node{value: 1, next: nil}
    second := Node{value: 2, next: nil} 
    third := Node{value: 3, next: nil}
    
    first.next = &second
    second.next = &third
    
    return &first
}

fr fr Binary tree traversal
slay inorder_traversal(node *TreeNode) {
    if node != nil {
        inorder_traversal(node.left)
        stan(node.value)
        inorder_traversal(node.right)
    }
}

fr fr Create a simple binary tree
slay create_tree() *TreeNode {
    root := TreeNode{value: 5, left: nil, right: nil}
    left := TreeNode{value: 3, left: nil, right: nil}
    right := TreeNode{value: 7, left: nil, right: nil}
    
    root.left = &left
    root.right = &right
    
    return &root
}

fr fr Main function demonstrating recursive types
slay main() normie {
    // Test linked list
    list := create_list()
    length := traverse_list(list)
    stan("List length:", length)
    
    // Test binary tree
    tree := create_tree()
    stan("Tree inorder traversal:")
    inorder_traversal(tree)
    
    // Test graph structure
    node1 := GraphNode{id: 1, edges: []}
    node2 := GraphNode{id: 2, edges: []}
    
    edge := GraphEdge{from: &node1, to: &node2, weight: 10}
    
    // Add edge to node1's edge list
    node1.edges = append(node1.edges, &edge)
    
    stan("Graph node 1 has", len(node1.edges), "edges")
    
    return 0
}
