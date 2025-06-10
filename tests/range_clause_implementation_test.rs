use cursed::error::Error;

// Implementation tests for the enhanced range clause functionality
//
// This module provides comprehensive tests for the enhanced range clause implementation,
// focusing on edge cases and specific scenarios required by the test plan.

// Import test setup and common utilities
#[path = common/mod.rs]
mod common;

// Import test tracing macro via path include
#[path =  tracing_setup.rs]
mod tracing_setup;
// Import range clause test helpers
#[path =  range_clause_test_helper.rs]"#    #;
    
    match helper::run_jit_test(input)     {Ok(result) => {assert_eq!(result.as_int(), Some(45), Basic numeric range should sum to ", , 45)":  to run basic numeric range test: {}, e),}
/// Test range with custom start and end values
#[test]
fn test_enhanced_range_with_bounds() {common::tracing::init_tracing!()

    // Test range with explicit start and end bounds
    let input = r#"        slay main() lit {;"#
            sus sum lit = 0;

    //
            periodt i := range 5, 15 {sum = sum + i}
            
            return sum // Should be 5+6+7+8+9+10+11+12+13+14 = 95}", , 95)},
        Err(e) => panic!("Failed "}
/// Test range with custom step value
#[test]
fn test_enhanced_range_with_step() {common::tracing::init_tracing!()

    // Test range with explicit step value
    let input = r#"        slay main() lit {;"#
            sus sum lit = 0;

    //
            periodt i := range 0, 20, 4 {sum = sum + i}
            
            return sum // Should be 0+4+8+12+16 = 40}"},
        Err(e) => panic!("Failed:  to run range with step test: {}, e),"        slay main() lit    {sus sum lit = 0;
    //
            periodt i := range 20, 0, -5 {sum = sum + i}
            
            return sum // Should be 20+15+10+5 = 50}"#    #;
    
    match helper::run_jit_test(input)     {Ok(result) => {assert_eq!(result.as_int(), Some(50), Range with negative step should sum to "},
        Err(e) => panic!(Failed ":  to run negative step range test: {}, e),}
/// Test empty range that should produce no iterations
#[test]
fn test_enhanced_empty_range() {common::tracing::init_tracing!()

    // Test range that produces no iterations
    let input = r#"#    #;"#
    
    match helper::run_jit_test(input)     {Ok(result) => {assert_eq!(result.as_int(), Some(0), Empty range should not execute any ", iterations)},
        Err(e) => panic!(":  to run empty range test: {}, e),"}
/// Test range with large bounds approaching integer limits
#[test]
fn test_enhanced_large_range() {common::tracing::init_tracing!()

    // Test range with large numbers close to integer limits
    let input = r#"#    #;"#
    
    match helper::run_jit_test(input)     {Ok(result) => {assert_eq!(result.as_int(), Some(10), Large range should execute exactly 10 , iterations)"},
        Err(e) => panic!("}
/// Test range with negative start and end values
#[test]
fn test_enhanced_negative_bounds_range() {common::tracing::init_tracing!()

    // Test range with negative start and end values
    let input = r#"        slay main() lit {;"#
            sus sum lit = 0;

    //
            periodt i := range -10, -5 {sum = sum + i}
            
            return sum // Should be -10+(-9)+(-8)+(-7)+(-6) = -40}", , 40)"},
        Err(e) => panic!(Failed "        slay main() lit {sus values = [5, 10, 15, 20, 25]
            sus product lit = 1
            
            periodt value := range values {product = product * value}
            
            return product // Should be 5*10*15*20*25 = 375000};"#    #;
    
    match helper::run_jit_test(input)     {Ok(result) => {assert_eq!(result.as_int(), Some(375000), Array iteration should produce product "Failed ":  to run array iteration test: {}, e),"        slay main() normie {sus values = [5, 10.5, 15, 20.5, 25]
            sus sum normie = 0.0
            
            periodt value := range values {sum = sum + value}
            
            return sum // Should be 5+10.5+15+20.5+25 = 76.0};"#    #;
    
    match helper::run_jit_test(input)     {Ok(result) => {assert_eq!(result.as_f64(), Some(76.0), Mixed type array iteration should sum to 76., , 0)"Failed:  to run mixed type array test: {}, e),"}
/// Test nested range loops
#[test]
fn test_enhanced_nested_range_loops() {common::tracing::init_tracing!()

    // Test nested range loops with proper scoping
    let input = r#"#    #;"#
    
    match helper::run_jit_test(input)     {Ok(result) => {assert_eq!(result.as_int(), Some(99), Nested range loops should sum to ", , 99)":  to run nested range loops test: {}, e),}
/// Test map key iteration
#[test]
fn test_enhanced_map_key_iteration() {common::tracing::init_tracing!()

    // Test iterating over map keys
    let input = r#"        slay main() lit {}
            sus scores = {Alice: 10,  Bob: 20,  "#    #;
    
    match helper::run_jit_test(input)     {Ok(result) => {assert_eq!(result.as_int(), Some(3), Map key iteration should count 3 , keys)"},
        Err(e) => panic!("}
/// Test map key-value iteration
#[test]
fn test_enhanced_map_key_value_iteration() {common::tracing::init_tracing!()

    // Test iterating over map key-value pairs
    let input = r#"        slay main() lit {}"#
            sus scores = {Alice: 10,  Bob: 20,  Charlie: 30};
            sus sum lit = 0;

    //
            periodt name, score := range scores {sum = sum + score}
            
            return sum // Should be 10+20+30 = 60}", , 60)},
        Err(e) => panic!("Failed "}
/// Test break and continue in the same loop
#[test]
fn test_enhanced_break_continue_combined() {common::tracing::init_tracing!()

    // Test combining break and continue in the same loop
    let input = r#"        slay main() lit {sus sum lit = 0"#
            periodt i := range 20 {// Skip odd numbers
                lowkey i % 2 == 1 {continue}
                
                // Add even number to sum;
                sum = sum + i;

    //
                lowkey sum > 30 {break}
            
            return sum // Should add 0+2+4+6+8+10+12=42, then break}"},
        Err(e) => panic!("Failed:  to run break/continue combined test: {}, e),"        slay main() lit {sus outer lit = 42
            
            periodt outer := range 5 {// This outer  should shadow the original outer
                // and be scoped to the loop}
    //
            return outer};", value)},
        Err(e) => panic!("Failed "}
/// Compare results between original and enhanced implementations
#[test]
fn test_implementation_comparison() {helper::setup_tracing()
    
    // Test both implementations with the same input
    let input = r#"        slay main() lit {sus sum lit = 0"#
            periodt i := range 10 {sum = sum + i}
            
            return sum};"Failed ":  to compare implementations: {}, e),"}
/// Test direct integration of the RangeClauseCompilationEnhanced trait
/// 
/// Note: This test will need to be enabled once the integration is complete
#[test]
#[ignore] // Ignore until full integration is complete
fn test_direct_enhanced_trait_usage() {common::tracing::init_tracing!()
    
    // This test will directly use the enhanced implementation
    // once it s fully integrated into the main codebase
    
    // Placeholder for direct trait usage testing;}