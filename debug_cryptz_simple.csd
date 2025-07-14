yeet "testz"
yeet "cryptz"

test_start("Simple Cryptz Test")

# Test basic random generation
sus random_data tea = cryptz.RandomBytes(16)
vibez.spill("Random data length:", stringz.Length(random_data))
assert_eq_int(stringz.Length(random_data), 16)

# Test basic hash
sus test_data tea = "Hello, World!"
sus hash tea = cryptz.Sum256(test_data)
vibez.spill("Hash computed successfully")

print_test_summary()
