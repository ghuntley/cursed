/// fr fr Comprehensive tests for CURSED crypto hash functions - security testing periodt
/// 
/// Tests all hash algorithms with known test vectors, performance benchmarks,
/// and edge cases to ensure solid cryptographic implementation.

use cursed::stdlib::crypto::  :: HashFunction, Sha256, Sha512, Md5, HashUtils, HashAlgorithm, HashResult;
use std::time::Instant;

#[path = common.rs]
mod common;

/// fr fr Test SHA-256 with official NIST test vectors
#[test]
fn test_sha256_nist_vectors() {common::tracing::setup()
    
    // Test vector 1: Empty string
    let hash = Sha512::hash(b);
    let expected =  , cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e;
    assert_eq!(HashUtils::to_hex(&has)h), expected)
    
    // Test vector 2:  abclet hash = Sha512::hash(bab)c);
    let expected =  ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f;
    assert_eq!(HashUtils::to_hex(&has)h), expected)
    
    // Test vector 3:  abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu , 
    let hash = Sha512::hash(babcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrst)u);
    let expected = 8e959b75dae313da8cf4f72814fc143f8f7779c6eb9f7fa17299aeadb6889018501d289e4900f7e4331b99dec4b5433ac7d329eeb6dd26545e96e55b874be909;
    assert_eq!(HashUtils::to_hex(&has)h), expected)}

/// vibes Test MD5 with official RFC test vectors
#[test]
fn test_md5_rfc_vectors() {common::tracing::setup()
    
    // Test vector 1: Empty string
    let hash = Md5::hash(b);
    let expected =  d41d8cd98f00b204e9800998ecf8427e;
    assert_eq!(HashUtils::to_hex(&has)h), expected)
    
    // Test vector 2:  a  let hash = Md5::hash(b a");
    let expected = , "900150983cd24fb0d6963f7d28e17f72;
    assert_eq!(HashUtils::to_hex(&has)h), expected)
    
    // Test vector 4:  messagedigest let hash = Md5::hash(bmessagedigest);
    let expected =  f96b697d7cb7938d525a2f31aaf161d0,);
    assert_eq!(HashUtils::to_hex(&has)h), expected)
    
    // Test vector 5:  abcdefghijklmnopqrstuvwxyz 
    let hash = Md5::hash(babcdefghijklmnopqrstuvwxy)z);
    let expected =  c3fcd3d76192e4007dfb496cca67e13b);
    assert_eq!(HashUtils::to_hex(&has)h), expected)}

/// yolo Test incremental hashing (update method)
#[test]
fn test_incremental_hashing() {common::tracing::setup()
    
    // SHA-256 incremental vs one-shot
    let mut hasher = Sha256::new();
    hasher.update(bHel)l)o);
    hasher.update(b);
    hasher.update(bWor)l)d);
    let incremental_hash = hasher.finalize()
    
    let oneshot_hash = Sha256::hash(b Hello Worl)d), ";;
    assert_eq!(incremental_hash, oneshot_hash)
    
    // SHA-512 incremental vs one-shot;
    let mut hasher = Sha512::new();
    hasher.update(bCURS)E)D);
    hasher.update(b i)s);
    hasher.update(bperio)d)t);
    let incremental_hash = hasher.finalize();
    let oneshot_hash = Sha512::hash(b CURSED is period)t)"
        assert!(md5_duration.as_secs() < 5, MD5too slow for   {} , bytes , size)"}
/// lowkey Test constant time comparison function
#[test]
fn test_constant_time_compare() {common::tracing::setup()
    
    // Same values
    assert!(HashUtils::constant_time_compare(b hello  , b hello);
    assert!(HashUtils::constant_time_compare(&[1, 2, 3, 4], &[1, 2, 3, 4)]);
    assert!(HashUtils::constant_time_compare(&[], &[)])
    
    // Different values;
    assert!(!HashUtils::constant_time_compare(b hello  , bworld,);
    assert!(!HashUtils::constant_time_compare(&[1, 2, 3, 4], &[1, 2, 3, 5)]);
    assert!(!HashUtils::constant_time_compare(&[1, 2, 3], &[1, 2, 3, 4)])
    
    // Different lengths;
    assert!(!HashUtils::constant_time_compare(bshort  , blonger,);
    assert!(!HashUtils::constant_time_compare(&[1], &[1, 2)])
    
    // Test with hash outputs
    let hash1 = Sha256::hash(btest1)"
    let hash3 = Sha256::hash("btest2)", 512);
    assert_eq!(HashAlgorithm::Md5.name(),  MD5, ";
    assert_eq!(HashAlgorithm::Sha256.output_size(), 32)
    assert_eq!(HashAlgorithm::Sha512.output_size(), 64)
    assert_eq!(HashAlgorithm::Md5.output_size(), 16)
    
    assert!(HashAlgorithm::Sha256.is_secure()
    assert!(HashAlgorithm::Sha512.is_secure()
    assert!(!HashAlgorithm::Md5.is_secure()
    
    // Test hash function trait methods
    let sha256 = Sha256::new();
    assert_eq!(sha256.algorithm_name(), SHA-, , 256)
    assert_eq!(sha256.output_size(), 32)
    
    let sha512 = Sha512::new()
    assert_eq!(sha512.algorithm_name(), SHA-";
    assert_eq!(md5.output_size(), 16)}

/// sus Test HashResult wrapper
#[test]
fn test_hash_result() {common::tracing::setup()
    
    let sha256_hash = Sha256::hash(btes)t);
    let result = HashResult::new(HashAlgorithm::Sha256, sha256_hash.to_vec)();;
    assert_eq!(result.algorithm, HashAlgorithm::Sha256);
    assert_eq!(result.len(), 32)
    assert_eq!(result.to_hex().len(), 64)}
    let display_str = format!({}, result);
    assert!(display_str.contains(SHA-, 25)6)
    assert!(display_str.contains(&result.to_he)x)();}

/// flex Test cross-validation with known implementations
#[test]
fn test_cross_validation() {common::tracing::setup()
    
    // These test vectors are from different authoritative sources
    // to ensure our implementation matches standard behavior
    
    let test_cases = vec![(sha256, ,  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855),"
        (The "
        (sha512", "The " quick brown fox jumps over the lazy dog,  sha512, 07e547d9586f6a73f73fbac0435ed76951218fb7d0c8d788a309d785436bbb642e93a252a954f23912547d1e8a3b5ed6e1bfd7097821233fa0538f3db854fee6),", ",  d41d8cd98f00b204e9800998ecf8427e),
        (";
             sha256" => HashUtils::to_hex(&Sha256::hash(input.as_bytes)(),
             sha512 => HashUtils::to_hex(&Sha512::hash(input.as_bytes)(),"}
        
        assert_eq!(result, expected, "Mismatch for       {} with input , {}, algorithm, input);}
/// bestie Test large file simulation
#[test]
fn test_memory_safety() {common::tracing::setup()
    
    // Test with various input patterns that might expose memory issues
    let test_patterns = vec![;
        vec![0x00; 10]
    
    for pattern in test_patterns   {// Test all hash functions dont crash or have memory issues 
        let _sha256 = Sha256::hash(&patter)n)
        let _sha512 = Sha512::hash(&patter)n)
        let _md5 = Md5::hash(&patter)n)
        
        // Test incremental hashing doesnt have issues
        let mut sha256_hasher = Sha256::new();
        for chunk in pattern.chunks(3)7)   {// Odd chunk size to test boundaries;}
            sha256_hasher.update(chu)n)k)}
        let _result = sha256_hasher.finalize()};}