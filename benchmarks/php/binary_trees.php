#!/usr/bin/env php
<?php
// Binary trees benchmark adapted from The Computer Language Benchmarks Game

class TreeNode {
    public $left;
    public $right;
    public $item;
    
    public function __construct($item) {
        $this->item = $item;
    }
    
    // Create a new tree with the given item value at the root
    public static function createTree($item, $depth) {
        $node = new TreeNode($item);
        
        if ($depth > 0) {
            $node->left = self::createTree(2 * $item - 1, $depth - 1);
            $node->right = self::createTree(2 * $item, $depth - 1);
        }
        
        return $node;
    }
    
    // Check the tree and return a checksum
    public function check() {
        if ($this->left === null) {
            return $this->item;
        }
        return $this->item + $this->left->check() - $this->right->check();
    }
}

function main() {
    $minDepth = 4;
    $maxDepth = 12;
    
    $stretchDepth = $maxDepth + 1;
    $startTime = microtime(true);
    
    // Allocate and check a big tree
    $bigTree = TreeNode::createTree(0, $stretchDepth);
    echo "stretch tree of depth $stretchDepth check: " . $bigTree->check() . "\n";
    
    // Allow the big tree to be garbage collected
    $bigTree = null;
    
    // Allocate a long-lived binary tree
    $longLivedTree = TreeNode::createTree(0, $maxDepth);
    
    // Check trees of increasing depth
    for ($depth = $minDepth; $depth <= $maxDepth; $depth += 2) {
        $iterations = 1 << ($maxDepth - $depth + $minDepth);
        $result = 0;
        
        for ($i = 0; $i < $iterations; $i++) {
            $a = TreeNode::createTree($i, $depth);
            $b = TreeNode::createTree(-$i, $depth);
            $result += $a->check() + $b->check();
        }
        
        echo "$iterations*2 trees of depth $depth check: $result\n";
    }
    
    // Check the long-lived tree last
    echo "long lived tree of depth $maxDepth check: " . $longLivedTree->check() . "\n";
    
    $elapsed = (microtime(true) - $startTime) * 1000;
    echo "Time taken: $elapsed ms\n";
    
    // Get memory stats
    $memoryUsed = memory_get_peak_usage(true) / 1024;
    echo "Memory used: $memoryUsed KB\n";
}

main();