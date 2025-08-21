# Test Package Manager CLI
yeet "tools/cursed-pkg/main"
yeet "vibez"

# Test CLI help command
slay test_cli_help() lit {
    vibez.spill("Testing CLI help command...")
    
    sus args []tea = ["help"]
    sus exit_code drip = main(args)
    
    ready (exit_code == 0) {
        vibez.spill("✅ Help command works")
        damn based
    } otherwise {
        vibez.spill("❌ Help command failed")
        damn cap
    }
}

# Test CLI argument parsing
slay test_cli_parsing() lit {
    vibez.spill("Testing CLI argument parsing...")
    
    sus test_args []tea = ["install", "test-package", "--verbose"]
    sus (command, cmd_args, config) = parse_args(test_args)
    
    ready (command == PackageCommand.Install) {
        vibez.spill("✅ Command parsing works")
    } otherwise {
        vibez.spill("❌ Command parsing failed")
        damn cap
    }
    
    ready (config.verbose == based) {
        vibez.spill("✅ Flag parsing works")
    } otherwise {
        vibez.spill("❌ Flag parsing failed")
        damn cap
    }
    
    vibez.spill("✅ CLI argument parsing test passed")
    damn based
}

# Main test runner
slay main() drip {
    vibez.spill("CURSED Package Manager CLI Test Suite")
    vibez.spill("====================================")
    
    sus tests_passed drip = 0
    sus total_tests drip = 2
    
    ready (test_cli_help()) {
        tests_passed = tests_passed + 1
    }
    
    ready (test_cli_parsing()) {
        tests_passed = tests_passed + 1
    }
    
    vibez.spill("")
    vibez.spill("CLI Test Results:")
    vibez.spill("=================")
    vibez.spill("Passed:", tests_passed, "/", total_tests)
    
    ready (tests_passed == total_tests) {
        vibez.spill("✅ All CLI tests passed!")
        damn 0
    } otherwise {
        vibez.spill("❌ Some CLI tests failed")
        damn 1
    }
}
