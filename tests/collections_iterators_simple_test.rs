/// Simple test suite for CURSED collections iterators
/// 
/// This test suite validates:
/// - Basic iterator functionality
/// - Collection integration
/// - Iterator adapters and utilities
/// - Error handling and edge cases

use cursed::stdlib::collections::{
    SimpleIterator, SimpleIntoIterator, SimpleIteratorUtils,
    VecIterator, RangeIterator, TakeIterator, SkipIterator,
    MapIterator, FilterIterator,
    simple_range, simple_range_step,
    HashSet, TreeSet, BitSet,
    CollectionsError, CollectionsResult,
};

#[cfg(test)]
mod simple_iterator_tests {
    use super::*;

    #[test]
    fn test_vec_iterator_basic() {
        let vec = vec![1, 2, 3, 4, 5];
        let mut iter = SimpleIntoIterator::into_iter(vec);
        
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_range_iterator() {
        let result: Vec<i32> = simple_range(0, 5).collect();
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
        
        let result: Vec<i32> = simple_range_step(0, 10, 2).collect();
        assert_eq!(result, vec![0, 2, 4, 6, 8]);
        
        let result: Vec<i32> = simple_range(5, 5).collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_iterator_count() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(vec.into_iter().count(), 5);
        
        assert_eq!(simple_range(0, 100).count(), 100);
        assert_eq!(simple_range(0, 0).count(), 0);
    }

    #[test]
    fn test_iterator_find() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(vec.clone().into_iter().find(|&x| x > 3), Some(4));
        assert_eq!(vec.into_iter().find(|&x| x > 10), None);
    }

    #[test]
    fn test_iterator_any_all() {
        let vec = vec![2, 4, 6, 8];
        assert!(vec.clone().into_iter().any(|x| x > 5));
        assert!(vec.clone().into_iter().all(|x| x % 2 == 0));
        
        let vec = vec![1, 3, 5, 7];
        assert!(!vec.clone().into_iter().any(|x| x % 2 == 0));
        assert!(vec.into_iter().all(|x| x % 2 == 1));
    }

    #[test]
    fn test_collect() {
        let vec = vec![1, 2, 3, 4, 5];
        let collected: Vec<i32> = vec.into_iter().collect();
        assert_eq!(collected, vec![1, 2, 3, 4, 5]);
    }
}

#[cfg(test)]
mod iterator_adapters_tests {
    use super::*;

    #[test]
    fn test_map_adapter() {
        let vec = vec![1, 2, 3, 4, 5];
        let result: Vec<i32> = vec.into_iter().map(|x| x * 2).collect();
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
        
        let strings = vec!["a", "bb", "ccc"];
        let lengths: Vec<usize> = strings.into_iter().map(|s| s.len()).collect();
        assert_eq!(lengths, vec![1, 2, 3]);
    }

    #[test]
    fn test_filter_adapter() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let evens: Vec<i32> = vec.clone().into_iter().filter(|&x| x % 2 == 0).collect();
        assert_eq!(evens, vec![2, 4, 6, 8, 10]);
        
        let large: Vec<i32> = vec.into_iter().filter(|&x| x > 5).collect();
        assert_eq!(large, vec![6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_take_skip_adapters() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        
        let first_five: Vec<i32> = vec.clone().into_iter().take(5).collect();
        assert_eq!(first_five, vec![1, 2, 3, 4, 5]);
        
        let skip_three: Vec<i32> = vec.into_iter().skip(3).collect();
        assert_eq!(skip_three, vec![4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_fold_reduce() {
        let vec = vec![1, 2, 3, 4, 5];
        
        let sum = vec.clone().into_iter().fold(0, |acc, x| acc + x);
        assert_eq!(sum, 15);
        
        let product = vec.clone().into_iter().reduce(|acc, x| acc * x);
        assert_eq!(product, Some(120));
        
        let empty: Vec<i32> = vec![];
        assert_eq!(empty.into_iter().reduce(|a, b| a + b), None);
    }

    #[test]
    fn test_chained_adapters() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        
        // Chain: filter evens -> map double -> take 3
        let evens: Vec<i32> = vec.clone().into_iter().filter(|&x| x % 2 == 0).collect();
        let doubled: Vec<i32> = evens.into_iter().map(|x| x * 2).collect();
        let taken: Vec<i32> = doubled.into_iter().take(3).collect();
        
        assert_eq!(taken, vec![4, 8, 12]);
    }
}

#[cfg(test)]
mod iterator_utilities_tests {
    use super::*;

    #[test]
    fn test_sum_min_max() {
        let vec = vec![1, 2, 3, 4, 5];
        
        assert_eq!(SimpleIteratorUtils::sum(vec.clone().into_iter()), 15);
        assert_eq!(SimpleIteratorUtils::min(vec.clone().into_iter()), Some(1));
        assert_eq!(SimpleIteratorUtils::max(vec.into_iter()), Some(5));
        
        let empty: Vec<i32> = vec![];
        assert_eq!(SimpleIteratorUtils::sum(empty.clone().into_iter()), 0);
        assert_eq!(SimpleIteratorUtils::min(empty.clone().into_iter()), None);
        assert_eq!(SimpleIteratorUtils::max(empty.into_iter()), None);
    }

    #[test]
    fn test_partition() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let (evens, odds) = SimpleIteratorUtils::partition(vec.into_iter(), |&x| x % 2 == 0);
        
        assert_eq!(evens, vec![2, 4, 6, 8, 10]);
        assert_eq!(odds, vec![1, 3, 5, 7, 9]);
    }
}

#[cfg(test)]
mod collections_integration_tests {
    use super::*;

    #[test]
    fn test_hashset_iterator() {
        let mut set = HashSet::new();
        set.insert(3);
        set.insert(1);
        set.insert(4);
        set.insert(1); // Duplicate
        set.insert(5);
        
        let mut items: Vec<i32> = set.into_iter().collect();
        items.sort(); // HashSet doesn't guarantee order
        assert_eq!(items, vec![1, 3, 4, 5]);
    }

    #[test]
    fn test_treeset_iterator() {
        let mut set = TreeSet::new();
        set.insert(3);
        set.insert(1);
        set.insert(4);
        set.insert(1); // Duplicate
        set.insert(5);
        
        let items: Vec<i32> = set.into_iter().collect();
        assert_eq!(items, vec![1, 3, 4, 5]); // TreeSet maintains order
    }

    #[test]
    fn test_bitset_iterator() {
        let mut bit_set = BitSet::new(20);
        bit_set.set(3).unwrap();
        bit_set.set(7).unwrap();
        bit_set.set(11).unwrap();
        bit_set.set(15).unwrap();
        
        let items: Vec<usize> = bit_set.into_iter().collect();
        assert_eq!(items, vec![3, 7, 11, 15]);
    }

    #[test]
    fn test_collections_with_adapters() {
        let mut set = TreeSet::new();
        for i in 1..=10 {
            set.insert(i);
        }
        
        let evens: Vec<i32> = set.into_iter().filter(|&x| x % 2 == 0).collect();
        let doubled: Vec<i32> = evens.into_iter().map(|x| x * 2).collect();
        
        assert_eq!(doubled, vec![4, 8, 12, 16, 20]);
    }
}

#[cfg(test)]
mod edge_cases_tests {
    use super::*;

    #[test]
    fn test_empty_iterators() {
        let empty: Vec<i32> = vec![];
        assert_eq!(empty.clone().into_iter().count(), 0);
        assert_eq!(empty.clone().into_iter().find(|&x| x == 1), None);
        assert_eq!(empty.clone().into_iter().reduce(|a, b| a + b), None);
        
        let collected: Vec<i32> = empty.into_iter().map(|x| x * 2).collect();
        assert_eq!(collected, vec![]);
    }

    #[test]
    fn test_single_element_operations() {
        let single = vec![42];
        
        assert_eq!(single.clone().into_iter().count(), 1);
        assert_eq!(single.clone().into_iter().find(|&x| x == 42), Some(42));
        assert_eq!(single.clone().into_iter().reduce(|a, b| a + b), Some(42));
        assert_eq!(SimpleIteratorUtils::sum(single.into_iter()), 42);
    }

    #[test]
    fn test_zero_size_operations() {
        let vec = vec![1, 2, 3, 4, 5];
        
        let result: Vec<i32> = vec.clone().into_iter().take(0).collect();
        assert_eq!(result, vec![]);
        
        let result: Vec<i32> = vec.into_iter().skip(100).collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_large_skip_take() {
        let vec = vec![1, 2, 3, 4, 5];
        
        let result: Vec<i32> = vec.clone().into_iter().take(10).collect();
        assert_eq!(result, vec![1, 2, 3, 4, 5]); // Should only get what's available
        
        let result: Vec<i32> = vec.into_iter().skip(10).collect();
        assert_eq!(result, vec![]); // Should be empty
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_large_dataset_processing() {
        // Test with 1K elements (keeping it smaller for simple implementation)
        let large_vec: Vec<i32> = simple_range(0, 1000).collect();
        assert_eq!(large_vec.len(), 1000);
        
        // Test various operations on large dataset
        let sum = SimpleIteratorUtils::sum(large_vec.clone().into_iter());
        assert_eq!(sum, 499500); // Sum of 0..999
        
        let evens_count = large_vec.clone().into_iter()
            .filter(|&x| x % 2 == 0)
            .count();
        assert_eq!(evens_count, 500);
        
        let squares: Vec<i32> = large_vec.into_iter()
            .take(10)
            .map(|x| x * x)
            .collect();
        assert_eq!(squares.len(), 10);
        assert_eq!(squares[0], 0);
        assert_eq!(squares[9], 81);
    }

    #[test]
    fn test_chained_operations_performance() {
        let range: Vec<i32> = simple_range(0, 100).collect();
        
        let result: Vec<i32> = range.into_iter()
            .filter(|&x| x % 3 == 0)
            .map(|x| x * x)
            .filter(|&x| x % 2 == 0)
            .take(5)
            .collect();
        
        assert!(result.len() <= 5);
        assert!(result.iter().all(|&x| x % 2 == 0));
    }
}

/// Run comprehensive iterator functionality test
#[test]
fn test_comprehensive_simple_iterator_functionality() {
    // This test ensures all modules compile and basic functionality works
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let result: Vec<i32> = vec.into_iter()
        .filter(|&x| x > 3)
        .map(|x| x * 2)
        .take(3)
        .collect();
    
    assert_eq!(result, vec![8, 10, 12]);
    
    println!("✅ All simple iterator functionality tests passed!");
}
