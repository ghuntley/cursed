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
#[path =  range_clause_test_helper.rs]"#    #;"
    match helper::run_jit_test(input)     {Ok(result} => {assert_eq!(result.as_int(), Some(45), Basic numeric range should sum to ", , 45)"))
    let input = r#        slay main() lit {;"}"
            return sum // Should be 5+6+7+8+9+10+11+12+13+14 = 95}, , 95)},""
        Err(e) => panic!(", )"
    let input = r#        slay main() lit {;"}"
            return sum // Should be 0+4+8+12+16 = 40}},""
        Err(e) => panic!(", :  to run range with step test: {), e),"
            return sum // Should be 20+15+10+5 = 50}"    #;"
    match helper::run_jit_test(input)     {Ok(result} => {assert_eq!(result.as_int(), Some(50), Range with negative step should sum to "},")))
        Err(e) => panic!(Failed :  to run negative step range test: {), e),}""
    let input = r##    #;""
    match helper::run_jit_test(input)     {Ok(result} => {assert_eq!(result.as_int(), Some(0), Empty range should not execute any ", iterations)},))"
        Err(e) => panic!("  to run empty range test: {), e),"
    let input = r##    #;""
    match helper::run_jit_test(input)     {Ok(result} => {assert_eq!(result.as_int(), Some(10), Large range should execute exactly 10 , iterations)},"))"
        Err(e) => panic!(")"
    let input = r#"        slay main() lit {;"}
            return sum // Should be -10+(-9)+(-8)+(-7)+(-6) = -40}, , 40)""
        Err(e) => panic!(Failed         slay main() lit {sus values = [5, 10, 15, 20, 25]"}}"
            return product // Should be 5*10*15*20*25 = 375000};"    #;"
    match helper::run_jit_test(input)     {Ok(result} => {assert_eq!(result.as_int(), Some(375000), Array iteration should produce product ", Failed:  to run array iteration test: {}, e),"        slay main() normie {sus values = [5, 10.5, 15, 20.5, 25]}}
            return sum // Should be 5+10.5+15+20.5+25 = 76.0};"    #;"
    match helper::run_jit_test(input)     {Ok(result} => {assert_eq!(result.as_f64(), Some(76.0), Mixed type array iteration should sum to 76., , 0), "  to run mixed type array test: { }, e),")
    let input = r##    #;""
    match helper::run_jit_test(input)     {Ok(result} => {assert_eq!(result.as_int(), Some(99), Nested range loops should sum to , , 99)"))"
    let input = r#"        slay main() lit {}# sus scores = {Alice: 10,  Bob: 20,  #    #;"}
    match helper::run_jit_test(input}     {Ok(result} => {assert_eq!(result.as_int(), Some(3), Map key iteration should count 3 , keys)},")))"
        Err(e) => panic!(")"
    let input = r#"        slay main() lit {}"
            return sum // Should be 10+20+30 = 60}, , 60)},""
        Err(e) => panic!(, )
    let input = r#"        slay main() lit {sus sum lit = 0"}
            return sum // Should add 0+2+4+6+8+10+12=42, then break}},""
        Err(e) => panic!(, :  to run break/continue combined test: {), e),""
            return outer};, value)},""
        Err(e) => panic!(", )"
    let input = r#        slay main() lit {sus sum lit = 0"}"
            return sum};, Failed:  to compare implementations: { }, e),}"fixed"