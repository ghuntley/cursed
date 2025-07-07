yeet "testz"

fr fr ========================================
fr fr CURSED Standard Library Test Suite
fr fr Master Test Runner
fr fr ========================================

slay run_all_stdlib_tests() {
    vibez.spill("🚀 CURSED Standard Library Test Suite")
    vibez.spill("=====================================")
    vibez.spill("")
    
    fr fr Reset test state for clean run
    testz.reset_test_state()
    
    fr fr Run all stdlib module tests
    vibez.spill("Running Math Library Tests...")
    yeet "math/test_math"
    
    vibez.spill("\nRunning String Library Tests...")
    yeet "string/test_string"
    
    vibez.spill("\nRunning Crypto Library Tests...")
    yeet "crypto/test_crypto"
    
    vibez.spill("\nRunning I/O Library Tests...")
    yeet "io/test_io"
    
    vibez.spill("\nRunning Collections Library Tests...")
    yeet "collections/test_collections"
    
    vibez.spill("\nRunning Time Library Tests...")
    yeet "time/test_time"
    
    fr fr Final summary
    vibez.spill("\n" + "=".repeat(50))
    vibez.spill("📊 FINAL STDLIB TEST SUMMARY")
    vibez.spill("=".repeat(50))
    
    testz.print_test_summary()
    
    lowkey testz.test_failed == 0 {
        vibez.spill("")
        vibez.spill("🎉 ALL STDLIB TESTS PASSED! 🎉")
        vibez.spill("The CURSED standard library is fully functional!")
        vibez.spill("")
        vibez.spill("Tested modules:")
        vibez.spill("  ✓ Math      - Mathematical functions and constants")
        vibez.spill("  ✓ String    - String manipulation and processing")
        vibez.spill("  ✓ Crypto    - Cryptographic operations and security")
        vibez.spill("  ✓ I/O       - File and console input/output")
        vibez.spill("  ✓ Collections - Data structures and algorithms")
        vibez.spill("  ✓ Time      - Date/time operations and formatting")
        vibez.spill("")
    } highkey {
        vibez.spill("")
        vibez.spill("❌ SOME STDLIB TESTS FAILED")
        vibez.spill("Please check the test output above for details.")
        vibez.spill("")
    }
    
    damn testz.run_all_tests()
}

fr fr Auto-run when this file is executed
run_all_stdlib_tests()
