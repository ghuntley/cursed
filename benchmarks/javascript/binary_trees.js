// Binary trees benchmark adapted from The Computer Language Benchmarks Game

class TreeNode {
    constructor(item) {
        this.left = null;
        this.right = null;
        this.item = item;
    }
    
    // Create a new tree with the given item value at the root
    static createTree(item, depth) {
        const node = new TreeNode(item);
        
        if (depth > 0) {
            node.left = TreeNode.createTree(2 * item - 1, depth - 1);
            node.right = TreeNode.createTree(2 * item, depth - 1);
        }
        
        return node;
    }
    
    // Check the tree and return a checksum
    check() {
        if (!this.left) {
            return this.item;
        }
        return this.item + this.left.check() - this.right.check();
    }
}

function main() {
    const minDepth = 4;
    const maxDepth = 12;
    
    const stretchDepth = maxDepth + 1;
    const startTime = Date.now();
    
    // Allocate and check a big tree
    const bigTree = TreeNode.createTree(0, stretchDepth);
    console.log(`stretch tree of depth ${stretchDepth} check: ${bigTree.check()}`);
    
    // Allow the big tree to be garbage collected
    bigTree = null;
    
    // Allocate a long-lived binary tree
    const longLivedTree = TreeNode.createTree(0, maxDepth);
    
    // Check trees of increasing depth
    for (let depth = minDepth; depth <= maxDepth; depth += 2) {
        const iterations = 1 << (maxDepth - depth + minDepth);
        let result = 0;
        
        for (let i = 0; i < iterations; i++) {
            const a = TreeNode.createTree(i, depth);
            const b = TreeNode.createTree(-i, depth);
            result += a.check() + b.check();
        }
        
        console.log(`${iterations * 2} trees of depth ${depth} check: ${result}`);
    }
    
    // Check the long-lived tree last
    console.log(`long lived tree of depth ${maxDepth} check: ${longLivedTree.check()}`);
    
    const elapsed = Date.now() - startTime;
    console.log(`Time taken: ${elapsed} ms`);
    
    // Get memory stats - approximate since Node.js doesn't provide precise memory usage
    const memoryUsageInMB = process.memoryUsage().heapUsed / 1024 / 1024;
    console.log(`Memory used: ${Math.round(memoryUsageInMB * 1024)} KB`);
}

main();