fr fr Test direct function calls vs module calls

vibez.spill("=== Testing Direct Function Calls ===")
vibez.spill("abs_normie(-5):", abs_normie(-5))

vibez.spill("=== Testing Module Function Calls ===")
yeet "stringz"
vibez.spill("stringz.concat_strings('hello', 'world'):", stringz.concat_strings("hello", "world"))

yeet "arrayz"
sus test_arr []drip = [1, 2, 3]
vibez.spill("arrayz.sum_array([1,2,3]):", arrayz.sum_array(test_arr))

yeet "timez"
vibez.spill("timez.current_year():", timez.current_year())

yeet "jsonz"
vibez.spill("jsonz.json_create_object('key', 'value'):", jsonz.json_create_object("key", "value"))

yeet "cryptz"
vibez.spill("cryptz.crypto_sha256('test'):", cryptz.crypto_sha256("test"))
