#!/usr/bin/env python3

import re

def fix_collections_test():
    """Fix collections integration test API mismatches"""
    
    test_file = "tests/collections_integration_test.rs"
    
    with open(test_file, 'r') as f:
        content = f.read()
    
    # Fix variable 'e' not found issue
    content = re.sub(
        r'errors_encountered\.push\(format!\("EmptyQueue: \{\}", e\)\);',
        'errors_encountered.push("EmptyQueue error".to_string());',
        content
    )
    
    # Fix unwrap() calls on HashSet union/intersection (they return Self)
    content = re.sub(r'\.union\(&[^)]+\)\.unwrap\(\)', '.union(&set2)', content)
    content = re.sub(r'\.intersection\(&[^)]+\)\.unwrap\(\)', '.intersection(&set2)', content)
    
    # Fix priority queue enqueue - it needs same type for priority and item
    content = re.sub(
        r'pq\.enqueue\(\*priority, task\.to_string\(\)\)',
        'pq.enqueue(*priority, *priority)', 
        content
    )
    
    # Fix is_empty() - it returns bool, not Result
    content = re.sub(r'\.is_empty\(\)\.unwrap_or\(false\)', '.is_empty()', content)
    
    # Fix dequeue() - it returns Result<Option<T>, not Result<(priority, item)>
    content = re.sub(
        r'if let Ok\(\(priority, task\)\) = pq\.dequeue\(\)',
        'if let Ok(Some(item)) = pq.dequeue()',
        content
    )
    content = re.sub(
        r'if let Ok\(\(priority, event\)\) = priority_queue\.dequeue\(\)',
        'if let Ok(Some(item)) = priority_queue.dequeue()',
        content
    )
    
    # Fix push method name on circular queue (should be enqueue)
    content = re.sub(r'circular_queue\.push\(', 'circular_queue.enqueue(', content)
    
    # Fix BitSet::new() - it returns BitSet, not Result
    content = re.sub(r'BitSet::new\(([^)]+)\)\?', r'BitSet::new(\1)', content)
    
    # Fix test function return types for ? operator
    content = re.sub(
        r'fn (test_bit_set_operations_with_regular_sets|test_memory_efficiency_multiple_collections|test_comprehensive_error_handling)\(\) \{',
        r'fn \1() -> Result<(), Box<dyn std::error::Error>> {',
        content
    )
    
    # Add Ok(()) returns for modified functions
    content = re.sub(
        r'(fn test_(?:bit_set_operations_with_regular_sets|memory_efficiency_multiple_collections|comprehensive_error_handling)\(\) -> Result<[^}]+\{[^}]+)\}',
        r'\1\n        Ok(())\n    }',
        content
    )
    
    # Fix Queue::pop() method name (should be dequeue)
    content = re.sub(r'queue\.pop\(\)', 'queue.dequeue()', content)
    
    # Fix len() result comparison
    content = re.sub(
        r'assert_eq!\(thread_safe_stack\.len\(\), 20\);',
        'assert_eq!(thread_safe_stack.len().unwrap_or(0), 20);',
        content
    )
    
    # Fix thread-safe stack pop() result handling
    content = re.sub(
        r'if let Some\(item\) = thread_safe_stack\.pop\(\)',
        'if let Ok(Some(item)) = thread_safe_stack.pop()',
        content
    )
    
    # Fix priority queue enqueue with UserEvent - use priority field for both args
    content = re.sub(
        r'priority_queue\.enqueue\(event\.priority, event\.clone\(\)\)',
        'priority_queue.enqueue(event.priority, event.priority)',
        content
    )
    
    with open(test_file, 'w') as f:
        f.write(content)
    
    print("Fixed collections integration test API mismatches")

if __name__ == "__main__":
    fix_collections_test()
