/// Simple test for CURSED collections iterators without ambiguity
use cursed::stdlib::collections::{
    SimpleIterator, SimpleIntoIterator, SimpleIteratorUtils,
    VecIterator, simple_range,
    HashSet, TreeSet, BitSet,
};

#[test]
fn test_simple_range() {
    let result: Vec<i32> = simple_range(0, 5).collect();
    assert_eq!(result, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_vec_simple_iterator() {
    let items = vec![1, 2, 3, 4, 5];
    let iter = VecIterator::new(items);
    let result: Vec<i32> = iter.collect();
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_simple_iterator_adapters() {
    let items = vec![1, 2, 3, 4, 5];
    let iter = VecIterator::new(items);
    
    let doubled: Vec<i32> = iter.map(|x| x * 2).collect();
    assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    
    let range_iter = simple_range(1, 11);
    let evens: Vec<i32> = range_iter.filter(|&x| x % 2 == 0).collect();
    assert_eq!(evens, vec![2, 4, 6, 8, 10]);
}

#[test]
fn test_simple_utilities() {
    let range_iter = simple_range(1, 6);
    assert_eq!(SimpleIteratorUtils::sum(range_iter), 15);
    
    let range_iter = simple_range(1, 6);
    assert_eq!(SimpleIteratorUtils::min(range_iter), Some(1));
    
    let range_iter = simple_range(1, 6);
    assert_eq!(SimpleIteratorUtils::max(range_iter), Some(5));
}

#[test]
fn test_collection_integration() {
    let mut set = HashSet::new();
    let _ = set.insert(1);
    let _ = set.insert(2);
    let _ = set.insert(3);
    
    // Iterator not yet implemented for HashSet, using contains instead
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
    assert!(!set.contains(&4));
    assert_eq!(set.size(), 3);
}

#[test]
fn test_simple_comprehensive_functionality() {
    // Test chaining operations without ambiguity
    let range_iter = simple_range(1, 11);
    let result: Vec<i32> = range_iter
        .filter(|&x| x > 3)
        .map(|x| x * 2)
        .take(3)
        .collect();
    
    assert_eq!(result, vec![8, 10, 12]);
    println!("✅ Simple iterator functionality working correctly!");
}
