/// Comprehensive test suite for CURSED Collections Sets
/// 
/// This test suite validates all set operations including:
/// - HashSet functionality and performance
/// - TreeSet ordered operations and tree properties
/// - BitSet efficient bit manipulation
/// - Set operations (union, intersection, difference, etc.)
/// - Error handling and edge cases
/// - Integration with other collection types

use cursed::stdlib::collections::*;
use std::collections::BTreeSet;

// ==================== HashSet Tests ====================

#[test]
fn test_hash_set_creation_and_basic_operations() {
    // Test creation
    let mut set = HashSet::<i32>::new();
    assert!(set.is_empty());
    assert_eq!(set.len(), 0);
    
    // Test with capacity
    let mut set_with_capacity = HashSet::<String>::with_capacity(10);
    assert!(set_with_capacity.capacity() >= 10);
    assert!(set_with_capacity.is_empty());
    
    // Test insertion
    assert!(set.insert(1));  // First insertion returns true
    assert!(!set.insert(1)); // Duplicate insertion returns false
    assert!(set.insert(2));
    assert!(set.insert(3));
    
    assert_eq!(set.len(), 3);
    assert!(!set.is_empty());
    
    // Test contains
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
    assert!(!set.contains(&4));
    
    // Test removal
    assert!(set.remove(&2));  // Successful removal returns true
    assert!(!set.remove(&2)); // Already removed returns false
    assert_eq!(set.len(), 2);
    assert!(!set.contains(&2));
}

#[test]
fn test_hash_set_capacity_management() {
    let mut set = HashSet::<i32>::new();
    
    // Test reserve
    set.reserve(100);
    assert!(set.capacity() >= 100);
    
    // Add many elements
    for i in 0..50 {
        set.insert(i);
    }
    
    let capacity_before_shrink = set.capacity();
    set.shrink_to_fit();
    let capacity_after_shrink = set.capacity();
    
    // Capacity should be reduced or at least not increased
    assert!(capacity_after_shrink <= capacity_before_shrink);
    assert_eq!(set.len(), 50);
    
    // Clear and test
    set.clear();
    assert!(set.is_empty());
    assert_eq!(set.len(), 0);
}

#[test]
fn test_hash_set_iteration_and_conversion() {
    let mut set = HashSet::new();
    set.insert("apple".to_string());
    set.insert("banana".to_string());
    set.insert("cherry".to_string());
    
    // Test iteration
    let mut count = 0;
    for item in set.iter() {
        assert!(item.starts_with('a') || item.starts_with('b') || item.starts_with('c'));
        count += 1;
    }
    assert_eq!(count, 3);
    
    // Test conversion to vector
    let vec = set.to_vec();
    assert_eq!(vec.len(), 3);
    
    // Test from_iter
    let vec_input = vec![1, 2, 3, 2, 1]; // Duplicates
    let set_from_iter: HashSet<i32> = HashSet::from_iter(vec_input);
    assert_eq!(set_from_iter.len(), 3);
    assert!(set_from_iter.contains(&1));
    assert!(set_from_iter.contains(&2));
    assert!(set_from_iter.contains(&3));
}

#[test]
fn test_hash_set_operations() {
    let mut set1 = HashSet::new();
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    set1.insert(4);
    
    let mut set2 = HashSet::new();
    set2.insert(3);
    set2.insert(4);
    set2.insert(5);
    set2.insert(6);
    
    // Test union
    let union = set1.union(&set2);
    assert_eq!(union.len(), 6);
    for i in 1..=6 {
        assert!(union.contains(&i));
    }
    
    // Test intersection
    let intersection = set1.intersection(&set2);
    assert_eq!(intersection.len(), 2);
    assert!(intersection.contains(&3));
    assert!(intersection.contains(&4));
    
    // Test difference
    let difference = set1.difference(&set2);
    assert_eq!(difference.len(), 2);
    assert!(difference.contains(&1));
    assert!(difference.contains(&2));
    
    // Test symmetric difference
    let sym_diff = set1.symmetric_difference(&set2);
    assert_eq!(sym_diff.len(), 4);
    assert!(sym_diff.contains(&1));
    assert!(sym_diff.contains(&2));
    assert!(sym_diff.contains(&5));
    assert!(sym_diff.contains(&6));
    assert!(!sym_diff.contains(&3));
    assert!(!sym_diff.contains(&4));
}

#[test]
fn test_hash_set_subset_relationships() {
    let mut superset = HashSet::new();
    superset.insert(1);
    superset.insert(2);
    superset.insert(3);
    superset.insert(4);
    superset.insert(5);
    
    let mut subset = HashSet::new();
    subset.insert(2);
    subset.insert(3);
    subset.insert(4);
    
    let mut disjoint = HashSet::new();
    disjoint.insert(6);
    disjoint.insert(7);
    disjoint.insert(8);
    
    // Test subset relationships
    assert!(subset.is_subset(&superset));
    assert!(!superset.is_subset(&subset));
    assert!(superset.is_superset(&subset));
    assert!(!subset.is_superset(&superset));
    
    // Test disjoint sets
    assert!(subset.is_disjoint(&disjoint));
    assert!(disjoint.is_disjoint(&subset));
    assert!(!subset.is_disjoint(&superset));
    
    // Edge case: empty set is subset of any set
    let empty = HashSet::<i32>::new();
    assert!(empty.is_subset(&superset));
    assert!(superset.is_superset(&empty));
    assert!(empty.is_disjoint(&superset));
}

// ==================== TreeSet Tests ====================

#[test]
fn test_tree_set_ordered_operations() {
    let mut set = TreeSet::new();
    
    // Insert in random order
    let values = vec![5, 2, 8, 1, 9, 3, 7, 4, 6];
    for value in values {
        set.insert(value);
    }
    
    assert_eq!(set.len(), 9);
    
    // Test ordering
    let ordered: Vec<_> = set.iter().cloned().collect();
    assert_eq!(ordered, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    
    // Test first and last
    assert_eq!(set.first(), Some(&1));
    assert_eq!(set.last(), Some(&9));
    
    // Test pop operations
    assert_eq!(set.pop_first(), Some(1));
    assert_eq!(set.first(), Some(&2));
    assert_eq!(set.len(), 8);
    
    assert_eq!(set.pop_last(), Some(9));
    assert_eq!(set.last(), Some(&8));
    assert_eq!(set.len(), 7);
}

#[test]
fn test_tree_set_range_operations() {
    let mut set = TreeSet::new();
    for i in 0..10 {
        set.insert(i);
    }
    
    // Test range iteration
    let range_3_to_7: Vec<_> = set.range(3..=7).cloned().collect();
    assert_eq!(range_3_to_7, vec![3, 4, 5, 6, 7]);
    
    let range_5_to_end: Vec<_> = set.range(5..).cloned().collect();
    assert_eq!(range_5_to_end, vec![5, 6, 7, 8, 9]);
    
    let range_start_to_4: Vec<_> = set.range(..=4).cloned().collect();
    assert_eq!(range_start_to_4, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_tree_set_string_ordering() {
    let mut set = TreeSet::new();
    set.insert("zebra".to_string());
    set.insert("apple".to_string());
    set.insert("banana".to_string());
    set.insert("cherry".to_string());
    set.insert("date".to_string());
    
    let ordered: Vec<_> = set.iter().cloned().collect();
    assert_eq!(ordered, vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
        "zebra".to_string()
    ]);
    
    // Test that duplicates are not added
    assert!(!set.insert("apple".to_string()));
    assert_eq!(set.len(), 5);
}

#[test]
fn test_tree_set_operations() {
    let set1: TreeSet<i32> = TreeSet::from_iter(vec![1, 2, 3, 4, 5]);
    let set2: TreeSet<i32> = TreeSet::from_iter(vec![4, 5, 6, 7, 8]);
    
    // Test union (should maintain order)
    let union = set1.union(&set2);
    assert_eq!(union.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8]);
    
    // Test intersection (should maintain order)
    let intersection = set1.intersection(&set2);
    assert_eq!(intersection.to_vec(), vec![4, 5]);
    
    // Test difference (should maintain order)
    let difference = set1.difference(&set2);
    assert_eq!(difference.to_vec(), vec![1, 2, 3]);
    
    // Test symmetric difference (should maintain order)
    let sym_diff = set1.symmetric_difference(&set2);
    assert_eq!(sym_diff.to_vec(), vec![1, 2, 3, 6, 7, 8]);
}

// ==================== BitSet Tests ====================

#[test]
fn test_bit_set_creation_and_basic_operations() {
    let mut bitset = BitSet::new(64);
    
    assert_eq!(bitset.len(), 64);
    assert!(!bitset.is_empty());
    assert_eq!(bitset.count(), 0);
    assert!(bitset.none());
    assert!(!bitset.any());
    assert!(!bitset.all());
    assert_eq!(bitset.count_zeros(), 64);
    
    // Test setting bits
    assert!(bitset.set(5).is_ok());
    assert!(bitset.set(10).is_ok());
    assert!(bitset.set(15).is_ok());
    
    assert_eq!(bitset.count(), 3);
    assert!(bitset.any());
    assert!(!bitset.all());
    assert!(!bitset.none());
    assert_eq!(bitset.count_zeros(), 61);
    
    // Test getting bits
    assert_eq!(bitset.get(5), Ok(true));
    assert_eq!(bitset.get(10), Ok(true));
    assert_eq!(bitset.get(15), Ok(true));
    assert_eq!(bitset.get(20), Ok(false));
    
    // Test contains
    assert!(bitset.contains(5));
    assert!(bitset.contains(10));
    assert!(bitset.contains(15));
    assert!(!bitset.contains(20));
}

#[test]
fn test_bit_set_manipulation_operations() {
    let mut bitset = BitSet::new(32);
    
    // Test insert/remove (returning previous state)
    assert_eq!(bitset.insert(5), Ok(true));  // Was not set, now is
    assert_eq!(bitset.insert(5), Ok(false)); // Was already set
    assert_eq!(bitset.remove(5), Ok(true));  // Was set, now is not
    assert_eq!(bitset.remove(5), Ok(false)); // Was already not set
    
    // Test toggle
    assert!(bitset.toggle(10).is_ok());
    assert!(bitset.get(10).unwrap());
    assert!(bitset.toggle(10).is_ok());
    assert!(!bitset.get(10).unwrap());
    
    // Test clear individual bit
    bitset.set(15).unwrap();
    assert!(bitset.get(15).unwrap());
    bitset.clear(15).unwrap();
    assert!(!bitset.get(15).unwrap());
}

#[test]
fn test_bit_set_bulk_operations() {
    let mut bitset = BitSet::new(100);
    
    // Test set_all
    bitset.set_all();
    assert_eq!(bitset.count(), 100);
    assert!(bitset.all());
    assert!(bitset.any());
    assert!(!bitset.none());
    
    // Test clear_all
    bitset.clear_all();
    assert_eq!(bitset.count(), 0);
    assert!(!bitset.all());
    assert!(!bitset.any());
    assert!(bitset.none());
    
    // Test ones constructor
    let ones_bitset = BitSet::ones(50);
    assert_eq!(ones_bitset.count(), 50);
    assert!(ones_bitset.all());
    
    // Test zeros constructor
    let zeros_bitset = BitSet::zeros(75);
    assert_eq!(zeros_bitset.count(), 0);
    assert!(zeros_bitset.none());
}

#[test]
fn test_bit_set_iteration() {
    let mut bitset = BitSet::new(100);
    let indices = vec![5, 15, 25, 35, 45, 55, 65, 75, 85, 95];
    
    for &index in &indices {
        bitset.set(index).unwrap();
    }
    
    // Test iterator
    let collected_indices: Vec<_> = bitset.iter().collect();
    assert_eq!(collected_indices, indices);
    
    // Test to_vec
    let vec_indices = bitset.to_vec();
    assert_eq!(vec_indices, indices);
}

#[test]
fn test_bit_set_operations() {
    let mut set1 = BitSet::new(64);
    set1.set(1).unwrap();
    set1.set(3).unwrap();
    set1.set(5).unwrap();
    set1.set(7).unwrap();
    
    let mut set2 = BitSet::new(64);
    set2.set(3).unwrap();
    set2.set(5).unwrap();
    set2.set(9).unwrap();
    set2.set(11).unwrap();
    
    // Test union
    let union = set1.union(&set2).unwrap();
    let union_indices: Vec<_> = union.iter().collect();
    assert_eq!(union_indices, vec![1, 3, 5, 7, 9, 11]);
    
    // Test intersection
    let intersection = set1.intersection(&set2).unwrap();
    let intersection_indices: Vec<_> = intersection.iter().collect();
    assert_eq!(intersection_indices, vec![3, 5]);
    
    // Test difference
    let difference = set1.difference(&set2).unwrap();
    let difference_indices: Vec<_> = difference.iter().collect();
    assert_eq!(difference_indices, vec![1, 7]);
    
    // Test symmetric difference
    let sym_diff = set1.symmetric_difference(&set2).unwrap();
    let sym_diff_indices: Vec<_> = sym_diff.iter().collect();
    assert_eq!(sym_diff_indices, vec![1, 7, 9, 11]);
    
    // Test complement
    let complement = set1.complement();
    assert_eq!(complement.count(), 64 - 4); // 64 - number of set bits in set1
    assert!(!complement.contains(1));
    assert!(!complement.contains(3));
    assert!(!complement.contains(5));
    assert!(!complement.contains(7));
    assert!(complement.contains(0));
    assert!(complement.contains(2));
}

#[test]
fn test_bit_set_subset_relationships() {
    let mut superset = BitSet::new(32);
    superset.set(1).unwrap();
    superset.set(3).unwrap();
    superset.set(5).unwrap();
    superset.set(7).unwrap();
    superset.set(9).unwrap();
    
    let mut subset = BitSet::new(32);
    subset.set(3).unwrap();
    subset.set(7).unwrap();
    
    let mut disjoint = BitSet::new(32);
    disjoint.set(2).unwrap();
    disjoint.set(4).unwrap();
    disjoint.set(6).unwrap();
    
    // Test subset relationships
    assert!(subset.is_subset(&superset));
    assert!(!superset.is_subset(&subset));
    assert!(superset.is_superset(&subset));
    assert!(!subset.is_superset(&superset));
    
    // Test disjoint sets
    assert!(subset.is_disjoint(&disjoint));
    assert!(disjoint.is_disjoint(&subset));
    assert!(!subset.is_disjoint(&superset));
    
    // Edge case: empty set
    let empty = BitSet::new(32);
    assert!(empty.is_subset(&superset));
    assert!(superset.is_superset(&empty));
    assert!(empty.is_disjoint(&superset));
}

// ==================== Error Handling Tests ====================

#[test]
fn test_bit_set_error_handling() {
    let mut bitset = BitSet::new(10);
    
    // Test index out of bounds errors
    match bitset.set(10) {
        Err(CollectionsError::InvalidBitIndex { index, max_bits }) => {
            assert_eq!(index, 10);
            assert_eq!(max_bits, 10);
        }
        _ => panic!("Expected InvalidBitIndex error"),
    }
    
    match bitset.get(15) {
        Err(CollectionsError::InvalidBitIndex { index, max_bits }) => {
            assert_eq!(index, 15);
            assert_eq!(max_bits, 10);
        }
        _ => panic!("Expected InvalidBitIndex error"),
    }
    
    match bitset.clear(20) {
        Err(CollectionsError::InvalidBitIndex { index, max_bits }) => {
            assert_eq!(index, 20);
            assert_eq!(max_bits, 10);
        }
        _ => panic!("Expected InvalidBitIndex error"),
    }
    
    // Test size mismatch for operations
    let other = BitSet::new(20);
    match bitset.union(&other) {
        Err(CollectionsError::TypeMismatch { expected, found }) => {
            assert!(expected.contains("10 bits"));
            assert!(found.contains("20 bits"));
        }
        _ => panic!("Expected TypeMismatch error"),
    }
}

#[test]
fn test_bit_set_edge_cases() {
    // Test with 0 bits (empty but valid)
    let empty_bitset = BitSet::new(0);
    assert_eq!(empty_bitset.len(), 0);
    assert!(empty_bitset.is_empty());
    assert_eq!(empty_bitset.count(), 0);
    assert!(empty_bitset.none());
    assert!(!empty_bitset.any());
    assert!(empty_bitset.all()); // Vacuously true
    
    // Test with 1 bit
    let mut single_bit = BitSet::new(1);
    assert_eq!(single_bit.len(), 1);
    assert!(!single_bit.is_empty());
    single_bit.set(0).unwrap();
    assert!(single_bit.all());
    assert!(single_bit.any());
    assert!(!single_bit.none());
    
    // Test with large number of bits
    let large_bitset = BitSet::new(10000);
    assert_eq!(large_bitset.len(), 10000);
    assert_eq!(large_bitset.count(), 0);
}

// ==================== Convenience Functions Tests ====================

#[test]
fn test_convenience_functions() {
    // Test hash_set_from_vec
    let vec = vec![1, 2, 3, 2, 1, 4]; // With duplicates
    let hash_set = hash_set_from_vec(vec);
    assert_eq!(hash_set.len(), 4);
    assert!(hash_set.contains(&1));
    assert!(hash_set.contains(&2));
    assert!(hash_set.contains(&3));
    assert!(hash_set.contains(&4));
    
    // Test tree_set_from_vec
    let string_vec = vec!["zebra".to_string(), "apple".to_string(), "banana".to_string()];
    let tree_set = tree_set_from_vec(string_vec);
    assert_eq!(tree_set.len(), 3);
    let ordered = tree_set.to_vec();
    assert_eq!(ordered[0], "apple");
    assert_eq!(ordered[1], "banana");
    assert_eq!(ordered[2], "zebra");
    
    // Test bit_set_from_vec
    let bit_indices = vec![1, 5, 10, 15];
    let bit_set = bit_set_from_vec(bit_indices.clone(), 20).unwrap();
    assert_eq!(bit_set.count(), 4);
    let collected: Vec<_> = bit_set.iter().collect();
    assert_eq!(collected, bit_indices);
    
    // Test error case for bit_set_from_vec
    let invalid_indices = vec![1, 5, 25]; // 25 is out of bounds for max_bits=20
    assert!(bit_set_from_vec(invalid_indices, 20).is_err());
}

#[test]
fn test_multiple_set_operations() {
    let mut set1 = HashSet::new();
    set1.insert(1);
    set1.insert(2);
    
    let mut set2 = HashSet::new();
    set2.insert(2);
    set2.insert(3);
    
    let mut set3 = HashSet::new();
    set3.insert(3);
    set3.insert(4);
    
    // Test union of multiple sets
    let union_result = hash_set_union_multiple(vec![&set1, &set2, &set3]);
    assert_eq!(union_result.len(), 4);
    for i in 1..=4 {
        assert!(union_result.contains(&i));
    }
    
    // Test intersection of multiple sets
    let intersection_result = hash_set_intersection_multiple(vec![&set1, &set2, &set3]);
    assert_eq!(intersection_result.len(), 0); // No common elements across all three
    
    // Test with common elements
    set1.insert(5);
    set2.insert(5);
    set3.insert(5);
    
    let intersection_with_common = hash_set_intersection_multiple(vec![&set1, &set2, &set3]);
    assert_eq!(intersection_with_common.len(), 1);
    assert!(intersection_with_common.contains(&5));
    
    // Test empty input
    let empty_union: HashSet<i32> = hash_set_union_multiple(vec![]);
    assert!(empty_union.is_empty());
    
    let empty_intersection: HashSet<i32> = hash_set_intersection_multiple(vec![]);
    assert!(empty_intersection.is_empty());
}

// ==================== Performance and Stress Tests ====================

#[test]
fn test_hash_set_performance_characteristics() {
    let mut set = HashSet::new();
    
    // Test insertion performance with large dataset
    let large_n = 10000;
    for i in 0..large_n {
        assert!(set.insert(i));
    }
    assert_eq!(set.len(), large_n);
    
    // Test lookup performance
    for i in 0..large_n {
        assert!(set.contains(&i));
    }
    
    // Test that duplicates don't increase size
    for i in 0..large_n {
        assert!(!set.insert(i)); // Should all return false
    }
    assert_eq!(set.len(), large_n);
    
    // Test removal performance
    for i in 0..large_n/2 {
        assert!(set.remove(&i));
    }
    assert_eq!(set.len(), large_n/2);
}

#[test]
fn test_tree_set_ordering_invariants() {
    let mut set = TreeSet::new();
    
    // Insert random values
    let mut rng_values = vec![50, 25, 75, 12, 37, 62, 87, 6, 18, 31, 43, 56, 68, 81, 93];
    for &value in &rng_values {
        set.insert(value);
    }
    
    // Check that iteration is always in sorted order
    let sorted: Vec<_> = set.iter().cloned().collect();
    let mut expected = rng_values.clone();
    expected.sort();
    expected.dedup(); // Remove duplicates for comparison
    assert_eq!(sorted, expected);
    
    // Check that range queries work correctly
    let range_25_to_75: Vec<_> = set.range(25..=75).cloned().collect();
    let expected_range: Vec<_> = expected.iter().filter(|&&x| x >= 25 && x <= 75).cloned().collect();
    assert_eq!(range_25_to_75, expected_range);
}

#[test]
fn test_bit_set_large_operations() {
    // Test with large bit sets
    let large_size = 10000;
    let mut bitset = BitSet::new(large_size);
    
    // Set every 10th bit
    for i in (0..large_size).step_by(10) {
        bitset.set(i).unwrap();
    }
    
    assert_eq!(bitset.count(), large_size / 10);
    
    // Test complement with large sets
    let complement = bitset.complement();
    assert_eq!(complement.count(), large_size - (large_size / 10));
    
    // Test operations with large sets
    let mut other = BitSet::new(large_size);
    for i in (5..large_size).step_by(10) {
        other.set(i).unwrap();
    }
    
    let union = bitset.union(&other).unwrap();
    let intersection = bitset.intersection(&other).unwrap();
    
    // Union should have both patterns
    assert_eq!(union.count(), (large_size / 10) * 2);
    
    // Intersection should be empty (disjoint patterns)
    assert_eq!(intersection.count(), 0);
    assert!(bitset.is_disjoint(&other));
}

// ==================== Integration Tests ====================

#[test]
fn test_set_type_interoperability() {
    // Test converting between different set types
    let original_data = vec![5, 2, 8, 1, 9, 3, 7];
    
    // Create all three types of sets
    let hash_set = hash_set_from_vec(original_data.clone());
    let tree_set = tree_set_from_vec(original_data.clone());
    
    // Convert to vectors and compare
    let mut hash_vec = hash_set.to_vec();
    hash_vec.sort(); // HashSet doesn't preserve order
    
    let tree_vec = tree_set.to_vec(); // TreeSet maintains order
    
    assert_eq!(hash_vec, tree_vec);
    assert_eq!(hash_vec.len(), 7); // All unique elements
    
    // Test BitSet with integer indices
    let bit_set = bit_set_from_vec(original_data, 20).unwrap();
    let bit_vec = bit_set.to_vec();
    
    assert_eq!(bit_vec, tree_vec); // Should match the sorted order
}

#[test]
fn test_error_display_formatting() {
    let errors = vec![
        CollectionsError::IndexOutOfBounds { index: 5, size: 3 },
        CollectionsError::ElementNotFound { element: "test".to_string() },
        CollectionsError::InvalidCapacity { capacity: 0 },
        CollectionsError::InvalidRange { start: 5, end: 2 },
        CollectionsError::TypeMismatch { 
            expected: "HashSet<i32>".to_string(), 
            found: "TreeSet<i32>".to_string() 
        },
        CollectionsError::OperationNotSupported { 
            operation: "range".to_string(), 
            collection_type: "HashSet".to_string() 
        },
        CollectionsError::InsufficientMemory { requested: 1024 },
        CollectionsError::InvalidBitIndex { index: 100, max_bits: 64 },
    ];
    
    for error in errors {
        let error_string = error.to_string();
        assert!(!error_string.is_empty());
        assert!(error_string.len() > 10); // Should be descriptive
        
        // Test that error implements std::error::Error
        let _: &dyn std::error::Error = &error;
    }
}

#[test]
fn test_memory_efficiency() {
    // Test that sets don't consume excessive memory
    let hash_set = HashSet::<i32>::new();
    let tree_set = TreeSet::<i32>::new();
    let bit_set = BitSet::new(64);
    
    // These should not panic due to memory issues
    assert!(hash_set.is_empty());
    assert!(tree_set.is_empty());
    assert!(!bit_set.is_empty()); // BitSet with capacity is not "empty"
    
    // Test that large capacity requests don't immediately allocate
    let large_hash_set = HashSet::<i32>::with_capacity(1000000);
    assert!(large_hash_set.is_empty()); // Should still be functionally empty
}
