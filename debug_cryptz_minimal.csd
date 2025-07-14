yeet "cryptz"

# Test basic functionality
sus random_data tea = cryptz.RandomBytes(16)
vibez.spill("Random data generated successfully")

sus test_data tea = "Hello, World!"
sus hash tea = cryptz.Sum256(test_data)
vibez.spill("Hash computed successfully")

vibez.spill("Cryptz module basic test passed!")
