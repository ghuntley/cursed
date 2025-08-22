fr fr TEST CROSS-PLATFORM PATHS DIRECTLY

yeet "vibez"

fr fr Load cross-platform path module directly
yeet "stdlib/filez/cross_platform_paths"

slay test_cross_platform_direct() {
    vibez.spill("Testing cross-platform paths directly...")
    
    fr fr Test platform detection
    sus platform tea = detect_platform()
    vibez.spill("Detected platform: " + platform)
    
    fr fr Test path separator
    sus sep tea = get_platform_separator()
    vibez.spill("Platform separator: '" + sep + "'")
    
    fr fr Test path normalization
    sus test_path tea = "/home/user/./documents/../file.txt"
    sus normalized tea = cross_platform_normalize(test_path)
    vibez.spill("Original: " + test_path)
    vibez.spill("Normalized: " + normalized)
    
    vibez.spill("Direct test completed!")
}

test_cross_platform_direct()
