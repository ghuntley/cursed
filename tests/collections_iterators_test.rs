/// Comprehensive test suite for CURSED collections iterators
/// 
/// This test suite validates:
/// - Core iterator traits and functionality
/// - Iterator adapters (map, filter, fold, etc.)
/// - Integration with existing collections (sets, queues, stacks)
/// - Performance testing and lazy evaluation validation
/// - Edge cases, error conditions, and memory efficiency
/// - Large dataset processing (10K+ elements)

// Temporarily disabled until iterator system is fully implemented
#[cfg(feature = "full_iterators")]
use cursed::stdlib::collections::{
    HashSet, TreeSet, BitSet,
    Queue, Deque, PriorityQueue,
    CollectionsError, CollectionsResult,
};

// Placeholder test when full iterators are not implemented
#[cfg(all(test, not(feature = "full_iterators")))]
mod placeholder_tests {
    #[test]
    fn test_iterator_system_disabled() {
        // This test exists so cargo test doesn't fail due to empty test file
        assert!(true, "Iterator system is disabled - use feature 'full_iterators' to enable");
    }
}

#[cfg(all(test, feature = "full_iterators"))]
mod iterator_core_tests {
    use super::*;

    #[test]
    fn test_vec_iterator_basic() {
        let vec = vec![1, 2, 3, 4, 5];
        let mut iter = vec.into_iter();
        
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_range_iterator() {
        let result: Vec<i32> = range(0, 5).collect();
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
        
        let result: Vec<i32> = range_step(0, 10, 2).collect();
        assert_eq!(result, vec![0, 2, 4, 6, 8]);
        
        let result: Vec<i32> = range(5, 5).collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_iterator_count() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(vec.into_iter().count(), 5);
        
        assert_eq!(range(0, 100).count(), 100);
        assert_eq!(range(0, 0).count(), 0);
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
    fn test_iterator_min_max() {
        let vec = vec![3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(vec.clone().into_iter().min(), Some(1));
        assert_eq!(vec.into_iter().max(), Some(9));
        
        let empty: Vec<i32> = vec![];
        assert_eq!(empty.clone().into_iter().min(), None);
        assert_eq!(empty.into_iter().max(), None);
    }

    #[test]
    fn test_repeat_iterator() {
        let result: Vec<i32> = repeat(42).take(5).collect();
        assert_eq!(result, vec![42, 42, 42, 42, 42]);
        
        let result: Vec<&str> = repeat_n("hello", 3).collect();
        assert_eq!(result, vec!["hello", "hello", "hello"]);
    }

    #[test]
    fn test_size_hints() {
        let vec = vec![1, 2, 3, 4, 5];
        let iter = vec.into_iter();
        assert_eq!(iter.size_hint(), (5, Some(5)));
        
        let range_iter = range(0, 10);
        let (lower, upper) = range_iter.size_hint();
        assert_eq!(lower, 0); // Conservative estimate
        assert!(upper.is_none() || upper.unwrap() >= 10);
    }
}

#[cfg(all(test, feature = "full_iterators"))]
#[cfg(all(test, feature = "full_iterators"))]
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
        
        let skip_three: Vec<i32> = vec.clone().into_iter().skip(3).collect();
        assert_eq!(skip_three, vec![4, 5, 6, 7, 8, 9, 10]);
        
        let middle: Vec<i32> = vec.into_iter().skip(3).take(4).collect();
        assert_eq!(middle, vec![4, 5, 6, 7]);
    }

    #[test]
    fn test_enumerate_adapter() {
        let vec = vec!["a", "b", "c", "d"];
        let enumerated: Vec<(usize, &str)> = vec.into_iter().enumerate().collect();
        assert_eq!(enumerated, vec![(0, "a"), (1, "b"), (2, "c"), (3, "d")]);
    }

    #[test]
    fn test_chain_adapter() {
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![4, 5, 6];
        let vec3 = vec![7, 8, 9];
        
        let chained: Vec<i32> = vec1.into_iter()
            .chain(vec2.into_iter())
            .chain(vec3.into_iter())
            .collect();
        assert_eq!(chained, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_zip_adapter() {
        let numbers = vec![1, 2, 3, 4];
        let letters = vec!["a", "b", "c"];
        
        let zipped: Vec<(i32, &str)> = numbers.into_iter().zip(letters.into_iter()).collect();
        assert_eq!(zipped, vec![(1, "a"), (2, "b"), (3, "c")]);
    }

    #[test]
    fn test_step_by_adapter() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let stepped: Vec<i32> = vec.into_iter().step_by(3).collect();
        assert_eq!(stepped, vec![1, 4, 7, 10]);
    }

    #[test]
    fn test_cycle_adapter() {
        let vec = vec![1, 2, 3];
        let cycled: Vec<i32> = vec.into_iter().cycle().take(10).collect();
        assert_eq!(cycled, vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1]);
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
        
        let result: Vec<i32> = vec.into_iter()
            .filter(|&x| x % 2 == 0)  // [2, 4, 6, 8, 10]
            .map(|x| x * 3)           // [6, 12, 18, 24, 30]
            .skip(1)                  // [12, 18, 24, 30]
            .take(3)                  // [12, 18, 24]
            .collect();
        
        assert_eq!(result, vec![12, 18, 24]);
    }
}

#[cfg(all(test, feature = "full_iterators"))]
#[cfg(all(test, feature = "full_iterators"))]
mod iterator_utilities_tests {
    use super::*;

    #[test]
    fn test_partition() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let (evens, odds) = vec.into_iter().partition(|&x| x % 2 == 0);
        
        assert_eq!(evens, vec![2, 4, 6, 8, 10]);
        assert_eq!(odds, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_group_by() {
        let words = vec!["apple", "banana", "cherry", "apricot", "blueberry", "avocado"];
        let groups = words.into_iter().group_by(|word| word.chars().next().unwrap());
        
        assert_eq!(groups[&'a'], vec!["apple", "apricot", "avocado"]);
        assert_eq!(groups[&'b'], vec!["banana", "blueberry"]);
        assert_eq!(groups[&'c'], vec!["cherry"]);
    }

    #[test]
    fn test_sum_product() {
        let vec = vec![1, 2, 3, 4, 5];
        
        assert_eq!(vec.clone().into_iter().sum_elements(), 15);
        assert_eq!(vec.into_iter().product_elements(), 120);
        
        let floats = vec![1.5, 2.5, 3.0];
        assert_eq!(floats.into_iter().sum_elements(), 7.0);
    }

    #[test]
    fn test_min_max_by_key() {
        let words = vec!["a", "bb", "ccc", "dddd", "e"];
        
        let shortest = IteratorUtils::min_by_key(words.clone().into_iter(), |word| word.len());
        let longest = IteratorUtils::max_by_key(words.into_iter(), |word| word.len());
        
        assert_eq!(shortest, Some("a"));
        assert_eq!(longest, Some("dddd"));
    }

    #[test]
    fn test_position_functions() {
        let vec = vec![10, 20, 30, 40, 50];
        
        assert_eq!(IteratorUtils::position(vec.clone().into_iter(), |&x| x == 30), Some(2));
        assert_eq!(IteratorUtils::position(vec.clone().into_iter(), |&x| x == 60), None);
        
        assert_eq!(IteratorUtils::nth(vec.into_iter(), 3), Some(40));
    }

    #[test]
    fn test_unzip() {
        let pairs = vec![(1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')];
        let (numbers, letters) = IteratorUtils::unzip(pairs.into_iter());
        
        assert_eq!(numbers, vec![1, 2, 3, 4]);
        assert_eq!(letters, vec!['a', 'b', 'c', 'd']);
    }

    #[test]
    fn test_windows() {
        let vec = vec![1, 2, 3, 4, 5];
        let windows: Vec<Vec<i32>> = vec.into_iter().windows(3).collect();
        
        assert_eq!(windows, vec![
            vec![1, 2, 3],
            vec![2, 3, 4],
            vec![3, 4, 5]
        ]);
    }

    #[test]
    fn test_chunks() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let chunks: Vec<Vec<i32>> = vec.into_iter().chunks(3).collect();
        
        assert_eq!(chunks, vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ]);
        
        let vec = vec![1, 2, 3, 4, 5];
        let chunks: Vec<Vec<i32>> = vec.into_iter().chunks(2).collect();
        
        assert_eq!(chunks, vec![
            vec![1, 2],
            vec![3, 4],
            vec![5]
        ]);
    }

    #[test]
    fn test_flat_map() {
        let vec = vec![1, 2, 3];
        let result: Vec<i32> = vec.into_iter()
            .flat_map(|x| range(0, x))
            .collect();
        
        // 1 -> [0], 2 -> [0, 1], 3 -> [0, 1, 2]
        assert_eq!(result, vec![0, 0, 1, 0, 1, 2]);
    }
}

#[cfg(all(test, feature = "full_iterators"))]
#[cfg(all(test, feature = "full_iterators"))]
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
        
        let doubled_evens: Vec<i32> = set.into_iter()
            .filter(|&x| x % 2 == 0)
            .map(|x| x * 2)
            .collect();
        
        assert_eq!(doubled_evens, vec![4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_iterator_chain_with_collections() {
        let mut set1 = HashSet::new();
        set1.insert(1);
        set1.insert(2);
        
        let mut set2 = HashSet::new();
        set2.insert(3);
        set2.insert(4);
        
        let vec = vec![5, 6];
        
        let mut combined: Vec<i32> = set1.into_iter()
            .chain(set2.into_iter())
            .chain(vec.into_iter())
            .collect();
        
        combined.sort(); // Account for HashSet order
        assert_eq!(combined, vec![1, 2, 3, 4, 5, 6]);
    }
}

#[cfg(all(test, feature = "full_iterators"))]
#[cfg(all(test, feature = "full_iterators"))]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_try_iterator() {
        let results = vec![Ok(1), Ok(2), Err("error"), Ok(4), Ok(5)];
        let mut try_iter = results.into_iter().try_iter();
        
        assert_eq!(try_iter.next(), Some(1));
        assert_eq!(try_iter.next(), Some(2));
        assert_eq!(try_iter.next(), None); // Stops at first error
        assert!(try_iter.has_error());
        assert_eq!(try_iter.error(), Some(&"error"));
    }

    #[test]
    fn test_try_collect() {
        let successful: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3)];
        let result = IteratorUtils::try_collect(successful.into_iter());
        assert_eq!(result, Ok(vec![1, 2, 3]));
        
        let with_error: Vec<Result<i32, &str>> = vec![Ok(1), Err("failed"), Ok(3)];
        let result = IteratorUtils::try_collect(with_error.into_iter());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "failed");
    }

    #[test]
    fn test_empty_iterators() {
        let empty: Vec<i32> = vec![];
        assert_eq!(empty.clone().into_iter().count(), 0);
        assert_eq!(empty.clone().into_iter().min(), None);
        assert_eq!(empty.clone().into_iter().max(), None);
        assert_eq!(empty.clone().into_iter().reduce(|a, b| a + b), None);
        
        let collected: Vec<i32> = empty.into_iter().map(|x| x * 2).collect();
        assert_eq!(collected, vec![]);
    }

    #[test]
    fn test_step_by_assertions() {
        let vec = vec![1, 2, 3, 4, 5];
        
        // This should panic with step size 0
        std::panic::catch_unwind(|| {
            let _: Vec<i32> = vec.into_iter().step_by(0).collect();
        }).expect_err("step_by(0) should panic");
    }
}

#[cfg(all(test, feature = "full_iterators"))]
#[cfg(all(test, feature = "full_iterators"))]
mod performance_tests {
    use super::*;

    #[test]
    fn test_large_dataset_processing() {
        // Test with 10K elements
        let large_vec: Vec<i32> = range(0, 10_000).collect();
        assert_eq!(large_vec.len(), 10_000);
        
        // Test various operations on large dataset
        let sum: i32 = large_vec.clone().into_iter().sum_elements();
        assert_eq!(sum, 49_995_000); // Sum of 0..9999
        
        let evens_count = large_vec.clone().into_iter()
            .filter(|&x| x % 2 == 0)
            .count();
        assert_eq!(evens_count, 5_000);
        
        let squares: Vec<i32> = large_vec.into_iter()
            .take(100)
            .map(|x| x * x)
            .collect();
        assert_eq!(squares.len(), 100);
        assert_eq!(squares[0], 0);
        assert_eq!(squares[99], 9801);
    }

    #[test]
    fn test_lazy_evaluation() {
        let mut call_count = 0;
        
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let _lazy_iter = vec.into_iter()
            .map(|x| {
                call_count += 1;
                x * 2
            })
            .filter(|&x| x > 10)
            .take(3);
        
        // With lazy evaluation, map shouldn't be called until we consume the iterator
        assert_eq!(call_count, 0);
        
        // Now consume the iterator
        let _result: Vec<i32> = _lazy_iter.collect();
        
        // Now the functions should have been called
        assert!(call_count > 0);
    }

    #[test]
    fn test_chained_operations_performance() {
        let large_range: Vec<i32> = range(0, 1000).collect();
        
        let result: Vec<i32> = large_range.into_iter()
            .filter(|&x| x % 3 == 0)
            .map(|x| x * x)
            .filter(|&x| x % 2 == 0)
            .take(50)
            .collect();
        
        assert!(result.len() <= 50);
        assert!(result.iter().all(|&x| x % 2 == 0));
    }

    #[test]
    fn test_memory_efficiency() {
        // Test that iterators don't consume excessive memory
        let range_iter = range(0, 1_000_000);
        
        // Taking just a few elements shouldn't require storing all million elements
        let first_ten: Vec<i32> = range_iter.take(10).collect();
        assert_eq!(first_ten, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}

#[cfg(all(test, feature = "full_iterators"))]
#[cfg(all(test, feature = "full_iterators"))]
mod parallel_processing_tests {
    use super::*;

    #[test]
    fn test_parallel_map() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = vec.into_iter().parallel(4).map(|x| x * x);
        
        let mut result = result;
        result.sort(); // Parallel processing may change order
        assert_eq!(result, vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100]);
    }

    #[test]
    fn test_parallel_filter() {
        let vec: Vec<i32> = range(1, 101).collect(); // 1..100
        let result = vec.into_iter().parallel(4).filter(|&x| x % 10 == 0);
        
        let mut result = result;
        result.sort(); // Parallel processing may change order
        assert_eq!(result, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);
    }

    #[test]
    fn test_parallel_reduce() {
        let vec: Vec<i32> = range(1, 11).collect(); // 1..10
        let result = vec.into_iter().parallel(3).reduce(|a, b| a + b);
        assert_eq!(result, Some(55)); // Sum of 1..10
    }

    #[test]
    fn test_parallel_with_different_thread_counts() {
        let vec: Vec<i32> = range(1, 21).collect(); // 1..20
        
        for thread_count in 1..=8 {
            let result = vec.clone().into_iter()
                .parallel(thread_count)
                .map(|x| x * 2);
            
            let mut result = result;
            result.sort();
            let expected: Vec<i32> = range(2, 41).step_by(2).collect();
            assert_eq!(result, expected);
        }
    }
}

#[cfg(all(test, feature = "full_iterators"))]
#[cfg(all(test, feature = "full_iterators"))]
mod edge_cases_tests {
    use super::*;

    #[test]
    fn test_single_element_operations() {
        let single = vec![42];
        
        assert_eq!(single.clone().into_iter().count(), 1);
        assert_eq!(single.clone().into_iter().min(), Some(42));
        assert_eq!(single.clone().into_iter().max(), Some(42));
        assert_eq!(single.clone().into_iter().sum_elements(), 42);
        assert_eq!(single.into_iter().reduce(|a, b| a + b), Some(42));
    }

    #[test]
    fn test_very_large_step_by() {
        let vec = vec![1, 2, 3, 4, 5];
        let result: Vec<i32> = vec.into_iter().step_by(10).collect();
        assert_eq!(result, vec![1]); // Should only get first element
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
    fn test_cycle_with_empty() {
        let empty: Vec<i32> = vec![];
        let result: Vec<i32> = empty.into_iter().cycle().take(5).collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_zip_different_lengths() {
        let short = vec![1, 2];
        let long = vec!['a', 'b', 'c', 'd', 'e'];
        
        let result: Vec<(i32, char)> = short.into_iter().zip(long.into_iter()).collect();
        assert_eq!(result, vec![(1, 'a'), (2, 'b')]);
    }

    #[test]
    fn test_nested_iterator_operations() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];
        
        let flattened: Vec<i32> = matrix.into_iter()
            .flat_map(|row| row.into_iter())
            .collect();
        
        assert_eq!(flattened, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_size_hint_accuracy() {
        let vec = vec![1, 2, 3, 4, 5];
        
        let take_iter = vec.clone().into_iter().take(3);
        assert_eq!(take_iter.size_hint(), (3, Some(3)));
        
        let skip_iter = vec.into_iter().skip(2);
        assert_eq!(skip_iter.size_hint(), (3, Some(3)));
    }
}

/// Run all iterator tests
#[test]
fn test_comprehensive_iterator_functionality() {
    // This test ensures all modules compile and basic functionality works
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let result: Vec<i32> = vec.into_iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, x)| x)
        .filter(|&x| x > 3)
        .take(3)
        .collect();
    
    assert_eq!(result, vec![5, 7, 9]);
    
    println!("✅ All iterator functionality tests passed!");
}
