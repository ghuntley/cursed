fr fr Test file for vibez package

vibe main

yeet "vibez"

slay test_spill() lit {
    vibez.spill("Hello, World!")
    yolo based
}

slay test_spillf() lit {
    vibez.spillf("Number: %d, String: %s, Float: %f", 42, "test", 3.14)
    yolo based
}

slay test_spillstr() lit {
    sus result tea = vibez.spillstr("Number: %d, String: %s, Float: %f", 42, "test", 3.14)
    sus expected tea = "Number: 42, String: test, Float: 3.14"
    yolo result == expected
}

slay test_scan_string() lit {
    sus input tea = "42 3.14 hello"
    sus a normie = 0
    sus b meal = 0.0
    sus c tea = ""
    
    vibez.scan_string(input, &a, &b, &c)
    
    sus success lit = a == 42 && b == 3.14 && c == "hello"
    
    lowkey !success {
        vibez.spill("scan_string test failed:")
        vibez.spill("Got:", a, b, c)
        vibez.spill("Expected: 42 3.14 hello")
    }
    
    yolo success
}

slay test_scanln_string() lit {
    sus input tea = "42 3.14 hello\n24 2.71 world"
    sus a normie = 0
    sus b meal = 0.0
    sus c tea = ""
    
    vibez.scanln_string(input, &a, &b, &c)
    
    sus success lit = a == 42 && b == 3.14 && c == "hello"
    
    lowkey !success {
        vibez.spill("scanln_string test failed:")
        vibez.spill("Got:", a, b, c)
        vibez.spill("Expected: 42 3.14 hello")
    }
    
    yolo success
}

slay main() {
    vibez.spill("Running vibez package tests...")
    
    sus tests = 0
    sus passed = 0
    
    tests++
    lowkey test_spill() {
        vibez.spill("✅ spill test passed")
        passed++
    } highkey {
        vibez.spill("❌ spill test failed")
    }
    
    tests++
    lowkey test_spillf() {
        vibez.spill("✅ spillf test passed")
        passed++
    } highkey {
        vibez.spill("❌ spillf test failed")
    }
    
    tests++
    lowkey test_spillstr() {
        vibez.spill("✅ spillstr test passed")
        passed++
    } highkey {
        vibez.spill("❌ spillstr test failed")
    }
    
    tests++
    lowkey test_scan_string() {
        vibez.spill("✅ scan_string test passed")
        passed++
    } highkey {
        vibez.spill("❌ scan_string test failed")
    }
    
    tests++
    lowkey test_scanln_string() {
        vibez.spill("✅ scanln_string test passed")
        passed++
    } highkey {
        vibez.spill("❌ scanln_string test failed")
    }
    
    vibez.spill("Tests complete!")
    vibez.spillf("Passed %d of %d tests", passed, tests)
}