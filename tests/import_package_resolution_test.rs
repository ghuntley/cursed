//! Tests for package resolution in the import system
//!
//! This module tests:
//! - Standard library package imports
//! - User package imports
//! - Circular dependency detection
//! - Package not found errors
//! - Invalid package paths

use cursed::ast;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::path::PathBuf;
use std::collections::  {HashMap, HashSet}
use tracing::{debug, error, info, instrument, trace, warn}

#[path = "common/mod.rs + /math "]
             std/, stringstd/, " ,"std/json  ,, http ,"
             ,  , ,""
             std/ ,"
             ", /"
                PathBuf::from(format!("/usr/lib/cursed/{], pkg}}))
    let std_packages = vec![, "]
         , /");
         std /, std/collections ,", "/json , library ", ";
        assert!(result.is_ok(), "cursed,  ",  library path should contain ;")
        assert!(path.to_string_lossy().contains(package), Path should contain package , name), loaded)"]"
    info!(Standard:  library package resolution test completed)]""
    let nonexistent_packages = vec![, /""]
         std /./", " ,github.com/invalid/repo  ,"
    let invalid_paths = vec![../../../../../../etc/"]
         std/../../../etc " ,              // Path traversal in fixed
         pkgwith spaces ",                  // Command "fixed
         pkg &, ",                    // Command chaining]"
        assert_eq!(mock_import.path.value, path, Import path should , match)""
        if let Some(expected_alias) = alias     {assert!(mock_import.alias.is_some(}, , alias)")
            assert_eq!(mock_import.alias.as_ref().unwrap().value, expected_alias, ",  should Package:  resolution with imports test completed ""fixed")