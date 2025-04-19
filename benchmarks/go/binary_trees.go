// Binary trees benchmark adapted from The Computer Language Benchmarks Game

package main

import (
	"fmt"
	"runtime"
	"time"
)

// A TreeNode structure
type TreeNode struct {
	left  *TreeNode
	right *TreeNode
	item  int
}

// Create a new tree with the given item value at the root
func NewTree(item, depth int) *TreeNode {
	if depth <= 0 {
		return &TreeNode{nil, nil, item}
	}
	return &TreeNode{
		NewTree(2*item-1, depth-1),
		NewTree(2*item, depth-1),
		item,
	}
}

// Check the tree and return a checksum
func CheckTree(node *TreeNode) int {
	if node == nil {
		return 0
	}
	if node.left == nil {
		return node.item
	}
	return node.item + CheckTree(node.left) - CheckTree(node.right)
}

func main() {
	minDepth := 4
	maxDepth := 12

	stretchDepth := maxDepth + 1
	startTime := time.Now()

	// Allocate and check a big tree
	bigTree := NewTree(0, stretchDepth)
	fmt.Printf("stretch tree of depth %d check: %d\n", stretchDepth, CheckTree(bigTree))

	// Allow the big tree to be garbage collected
	bigTree = nil

	// Allocate a long-lived binary tree
	longLivedTree := NewTree(0, maxDepth)

	// Check trees of increasing depth
	for depth := minDepth; depth <= maxDepth; depth += 2 {
		iterations := 1 << (maxDepth - depth + minDepth)
		result := 0

		for i := 0; i < iterations; i++ {
			a := NewTree(i, depth)
			b := NewTree(-i, depth)
			result += CheckTree(a) + CheckTree(b)
		}

		fmt.Printf("%d trees of depth %d check: %d\n", iterations*2, depth, result)
	}

	// Check the long-lived tree last
	fmt.Printf("long lived tree of depth %d check: %d\n", maxDepth, CheckTree(longLivedTree))

	elapsed := time.Since(startTime).Milliseconds()
	fmt.Printf("Time taken: %d ms\n", elapsed)

	// Get memory stats
	var mem runtime.MemStats
	runtime.ReadMemStats(&mem)
	fmt.Printf("Memory used: %d KB\n", mem.Alloc/1024)
}