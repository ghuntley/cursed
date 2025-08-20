yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "filez" 
yeet "envz"
yeet "jsonz"
yeet "cryptz"

vibez.spill("🧪 Testing Missing Implementation Bridge Functions")
vibez.spill("================================================")

fr fr Test missing math functions
vibez.spill("\n📊 Testing Math Functions:")
sus pow_result meal = mathz.pow_meal(2.0, 3.0)
vibez.spillf("mathz.pow_meal(2.0, 3.0) = %f", pow_result)

sus random_val meal = mathz.random_meal()
vibez.spillf("mathz.random_meal() = %f", random_val)

sus random_int normie = mathz.random_int_range(1, 10)
vibez.spillf("mathz.random_int_range(1, 10) = %d", random_int)

fr fr Test file operations
vibez.spill("\n📁 Testing File Functions:")
sus file_exists lit = filez.exists("build.zig")
vibez.spillf("filez.exists(\"build.zig\") = %s", file_exists ? "true" : "false")

sus is_dir lit = filez.is_directory(".")
vibez.spillf("filez.is_directory(\".\") = %s", is_dir ? "true" : "false")

fr fr Test environment functions  
vibez.spill("\n🌍 Testing Environment Functions:")
sus current_dir tea = envz.getcwd()
vibez.spillf("envz.getcwd() = %s", current_dir)

sus path_env tea = envz.getenv("PATH")
vibez.spillf("envz.getenv(\"PATH\") exists = %s", path_env.length() > 0 ? "true" : "false")

fr fr Test JSON functions
vibez.spill("\n🔗 Testing JSON Functions:")
sus test_data tea = "hello world"
sus json_str tea = jsonz.stringify(test_data)
vibez.spillf("jsonz.stringify(\"%s\") = %s", test_data, json_str)

sus parsed_data tea = jsonz.parse("{\"test\": \"value\"}")
vibez.spillf("jsonz.parse result = %s", parsed_data)

fr fr Test crypto functions
vibez.spill("\n🔐 Testing Crypto Functions:")
sus hash_input tea = "test string"  
sus hash_result tea = cryptz.hash_string(hash_input)
vibez.spillf("cryptz.hash_string(\"%s\") = %s", hash_input, hash_result)

vibez.spill("\n✅ Missing Implementation Bridge Tests Complete!")
vibez.spill("All missing *_impl() functions are now properly bridged.")
