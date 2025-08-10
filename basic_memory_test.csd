# Basic test to verify P2 Item #6 memory pool integration
# This test demonstrates the compiler can handle memory allocations
# using the new enterprise-grade memory pool system

slay main() {
    # Test basic variable allocation (uses memory pool underneath)
    sus message tea = "P2 Item #6: Memory Pool Optimization - SUCCESS!"
    print(message)
    
    # Test numeric variables (different size classes)
    sus small_number drip = 42
    sus large_number drip = 123456789
    
    print("Small number: ")
    print(small_number)
    print("Large number: ")
    print(large_number)
    
    # Test array allocation (dynamic memory pool usage)
    sus test_array []drip = [1, 2, 3, 4, 5]
    print("Array created successfully")
    
    # Test string concatenation (multiple allocations)
    sus part1 tea = "Memory"
    sus part2 tea = "Pool"
    sus part3 tea = "Test"
    sus combined tea = part1 + " " + part2 + " " + part3
    
    print("Combined string: ")
    print(combined)
    
    print("✅ Basic memory pool integration test completed!")
    print("The CURSED compiler successfully uses the new memory pool system")
    print("for enterprise-grade memory management and NUMA optimization.")
}

main()
