vibez.spill("Testing string fixes...")

# Test simple string concatenation
sus s1 tea = "hello"
sus s2 tea = " world"
sus result tea = s1 + s2
vibez.spill("Concat result: " + result)

# Test simple string comparison
sus str1 tea = "test"
sus str2 tea = "test"
vibes str1 == str2 {
    vibez.spill("String comparison works!")
} nah {
    vibez.spill("String comparison failed!")
}

vibez.spill("Basic string operations completed!")
