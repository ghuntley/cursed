fr fr Simple cryptographic test
yeet "cryptz"
yeet "vibez"

vibez.spill("Testing basic SHA-256...")
sus test_data tea = "Hello, CURSED!"
sus hash tea = cryptz.sha256_hash(test_data)
vibez.spill("Input: " + test_data)
vibez.spill("SHA-256: " + hash)
vibez.spill("Length: " + json_number_to_string(string_length(hash)))

vibez.spill("\nTesting random generation...")
sus random1 tea = cryptz.generate_random_bytes(8)
sus random2 tea = cryptz.generate_random_bytes(8)
vibez.spill("Random1 length: " + json_number_to_string(string_length(random1)))
vibez.spill("Random2 length: " + json_number_to_string(string_length(random2)))

vibez.spill("\nTesting constant time comparison...")
sus data1 tea = "test123"
sus data2 tea = "test123" 
sus data3 tea = "different"
sus equal_result lit = cryptz.constant_time_compare(data1, data2)
sus different_result lit = cryptz.constant_time_compare(data1, data3)
vibez.spill("Equal result: " + ready equal_result { "TRUE" } otherwise { "FALSE" })
vibez.spill("Different result: " + ready different_result { "TRUE" } otherwise { "FALSE" })

vibez.spill("\nCrypto test complete!")
