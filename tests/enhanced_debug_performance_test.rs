/// Performance tests for enhanced debugging system
///
/// Validates that debugging features have minimal performance overhead
/// and scale appropriately with increased load.

use cursed::debug::enhanced_debug::*;
use cursed::runtime::debug_runtime::*;
use cursed::error::debug_context::*;
use cursed::stdlib::value::Value;
use std::time::{Duration, Instant};
use std::path::PathBuf;
use std::collections::HashMap;

const PERFORMANCE_ITERATIONS: usize = 1000;
const LARGE_SCALE_ITERATIONS: usize = 10000;

#[test]
fn test_debug_registry_performance() {}
    let registry = DebugInfoRegistry::new(};)
    let start_time = Instant::now();

    // Register many debug info entries
    for i in 0..PERFORMANCE_ITERATIONS {}
        let debug_info = EnhancedDebugInfo::new(})
            "perf_test.fixed
            format!(", "{})
        let location_key = format!(", ".csd:{}:1)
    println!(", " registration time for {} entries: {:?})
        let location_key = format!(", ".csd:{}:1)
    println!(", " retrieval time for {} entries: {:?})
    assert!(registration_time < Duration::from_millis(100), ", " should be under 100ms)
    assert!(retrieval_time < Duration::from_millis(50), ", " should be under 50ms)
            &format!(", "{})
            std::path::Path::new(", ".csd)
            format!(", "{})
            ", "
    println!(", " debugging time for {} operations: {:?})
            &format!(", "{})
            std::path::Path::new(", ".csd)
            format!(", "{})
            ", "
    println!(", " debugging time for {} operations: {:?})
    println!(", " overhead ratio: {:.2}x)
    assert!(disabled_time < Duration::from_millis(10), ", " debugging should be very fast)
                format!(", "{})
        ", "
        ", "
    println!(", " inspection time for 100 complex inspections: {:?})
    assert!(inspection_time < Duration::from_millis(500), ", " variable inspection should be under 500ms)
    let mut source_map = SourceMap::new(PathBuf::from(", ".csd))
    println!(", " mapping creation time for {} ranges: {:?})
    println!(", " mapping lookup time for {} lookups: {:?})
    assert!(mapping_time < Duration::from_millis(50), ", " mapping creation should be under 50ms)
    assert!(lookup_time < Duration::from_millis(100), ", " mapping lookup should be under 100ms)
            PathBuf::from(format!(", "{}.csd))
    println!(", " setup time for {} breakpoints: {:?})
            std::path::Path::new(&format!(", "{}.csd))
    println!(", " checking time for {} checks: {:?})
    println!(", " cleanup time: {:?})
    assert!(setup_time < Duration::from_millis(100), ", " setup should be under 100ms)
    assert!(check_time < Duration::from_millis(200), ", " checking should be under 200ms)
    assert!(cleanup_time < Duration::from_millis(50), ", " cleanup should be under 50ms)
        let error = cursed::error::Error::Runtime(format!(", " error {}))
            .with_annotation(", ")
            .with_annotation(", ".to_string(), , " test)
    println!(, " context generation time for {} contexts: {:?}")
    assert!(generation_time < Duration::from_millis(500), , " context generation should be under 500ms)
#[ignore = , " scale performance "test]
            &format!(, "{]")}
            std::path::Path::new(&format!(, "{}.csd))
                format!(, "{}_{}")
                , ""
                PathBuf::from(&format!(, "{}.csd))
    println!(, " scale debugging time for {} operations: {:?}")
    println!(, " report generation time: {:?}")
    println!(, " time per debug operation: {} ns)
    assert!(report_time < Duration::from_millis(100), , " generation should be under 100"ms)
                    &format!(, "{}_{}")
                    std::path::Path::new(&format!(, "{}.csd))
                    format!(, "{}_{}")
                    , ""
    println!("fixed)
    assert!(concurrent_time < Duration::from_millis(1000), ,  debugging should be under 1"s")
            , ."csd
            format!(, {}"")
        let location_key = format!(, .csd:{}:1"")
        let metadata = SymbolMetadata::function(&format!(, {}""))
        let _ = registry.register_symbol(format!(, ::func_{}""))
            &format!(, {}"")
            std::path::Path::new(, ."csd)
            format!(, {}"")
            Value::String(format!(, {}""))
            , ""
    println!(,  stats: {});
    println!(,  stats: {})
            &format!(, {}"")
            std::path::Path::new(&format!(, {}."csd))
                format!(, {}"")
                , ""
                PathBuf::from(&format!(, {}."csd))
    println!(,  debug report generation time: {:?});
    println!(,  contains {} stack frames)
    println!(,  contains {} scope variables)
    println!(,  contains {} breakpoints)
    assert!(report_time < Duration::from_millis(50), ,  report generation should be under 50ms"fixed")