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
fn test_sha256_nist_vectors() {common::tracing::setup(})
    
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
fn test_md5_rfc_vectors() {common::tracing::setup(})
    
    // Test vector 1: Empty string
    let hash = Md5::hash(b);
    let expected =  d41d8cd98f00b204e9800998ecf8427e;
    assert_eq!(HashUtils::to_hex(&has)h), expected)
    
    // Test vector 2:  a  let hash = Md5::hash(b ";)
    let expected = , ", 900150983cd24fb0d6963f7d28e17f72;"
    let oneshot_hash = Sha256::hash(b Hello Worl)d), ;;""
    let oneshot_hash = Sha512::hash(b CURSED is period)t)"
        assert!(md5_duration.as_secs() < 5, MD5too slow for   {} , bytes , size)"}
    let hash1 = Sha256::hash(btest1)""
    let hash3 = Sha256::hash(, "")
    assert_eq!(HashAlgorithm::Md5.name(),  MD5, ;"")
    assert_eq!(sha512.algorithm_name(), SHA-;")
    let test_cases = vec![(sha256, ,  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855),"]
        (The "")
        (sha512, ", The quick brown fox jumps over the lazy dog,  sha512, 07e547d9586f6a73f73fbac0435ed76951218fb7d0c8d788a309d785436bbb642e93a252a954f23912547d1e8a3b5ed6e1bfd7097821233fa0538f3db854fee6),", ,  d41d8cd98f00b204e9800998ecf8427e),"
        (";)
             sha256 => HashUtils::to_hex(&Sha256::hash(input.as_bytes)()")
             sha512 => HashUtils::to_hex(&Sha512::hash(input.as_bytes)(),]"")
        assert_eq!(result, expected, ,  for       {} with input , {}, algorithm, input);]"fixed"