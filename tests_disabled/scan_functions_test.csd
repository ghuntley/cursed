fr fr Test for scan and scanln functions in vibez package

vibe main

yeet "vibez"
yeet "dropz"

slay test_scan() lit {
    sus input tea = "42 3.14 hello"
    sus a normie
    sus b meal
    sus c tea
    
    vibez.scan_string(input, &a, &b, &c)
    
    yolo a == 42 && b == 3.14 && c == "hello"
}

slay test_scanln() lit {
    sus input tea = "42 3.14 hello\n24 2.71 world"
    sus a normie
    sus b meal
    sus c tea
    
    vibez.scanln_string(input, &a, &b, &c)
    
    yolo a == 42 && b == 3.14 && c == "hello"
}

slay main() {
    lowkey !test_scan() {
        vibez.spill("scan test failed")
    } highkey {
        vibez.spill("scan test passed")
    }
    
    lowkey !test_scanln() {
        vibez.spill("scanln test failed")
    } highkey {
        vibez.spill("scanln test passed")
    }
}