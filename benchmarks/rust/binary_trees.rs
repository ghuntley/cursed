// Binary trees benchmark adapted from The Computer Language Benchmarks Game

use std::rc::Rc;
use std::cell::RefCell;
use std::time::Instant;

// A TreeNode structure
struct TreeNode {
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
    item: i32,
}

impl TreeNode {
    // Create a new tree with the given item value at the root
    fn new(item: i32, depth: i32) -> Rc<RefCell<TreeNode>> {
        let mut node = TreeNode { left: None, right: None, item };
        
        if depth > 0 {
            node.left = Some(TreeNode::new(2 * item - 1, depth - 1));
            node.right = Some(TreeNode::new(2 * item, depth - 1));
        }
        
        Rc::new(RefCell::new(node))
    }
    
    // Check the tree and return a checksum
    fn check(&self) -> i32 {
        match (&self.left, &self.right) {
            (None, None) => self.item,
            (Some(left), Some(right)) => {
                self.item + left.borrow().check() - right.borrow().check()
            },
            _ => unreachable!("Tree should always have either both children or no children"),
        }
    }
}

fn main() {
    let min_depth = 4;
    let max_depth = 12;
    
    let stretch_depth = max_depth + 1;
    let start_time = Instant::now();
    
    // Allocate and check a big tree
    let big_tree = TreeNode::new(0, stretch_depth);
    println!("stretch tree of depth {} check: {}", stretch_depth, big_tree.borrow().check());
    
    // Allow the big tree to be garbage collected
    drop(big_tree);
    
    // Allocate a long-lived binary tree
    let long_lived_tree = TreeNode::new(0, max_depth);
    
    // Check trees of increasing depth
    for depth in (min_depth..=max_depth).step_by(2) {
        let iterations = 1 << (max_depth - depth + min_depth);
        let mut result = 0;
        
        for i in 0..iterations {
            let a = TreeNode::new(i, depth);
            let b = TreeNode::new(-i, depth);
            result += a.borrow().check() + b.borrow().check();
        }
        
        println!("{} trees of depth {} check: {}", iterations*2, depth, result);
    }
    
    // Check the long-lived tree last
    println!("long lived tree of depth {} check: {}", max_depth, long_lived_tree.borrow().check());
    
    let elapsed = start_time.elapsed();
    println!("Time taken: {} ms", elapsed.as_millis());
    
    // Get approximate memory usage - note this isn't as accurate as in other languages
    let memory_usage = std::mem::size_of::<TreeNode>() * (1 << (max_depth + 1)) / 1024;
    println!("Memory used: {} KB", memory_usage);
}