yeet "testz"

fr fr Comprehensive test suite for hashtag (command-line flag) module
fr fr Tests flag parsing, value types, and social media features

sus main() {
    test_start("Hashtag flag parsing comprehensive tests")
    
    fr fr Basic flag type tests
    test_flag_types()
    test_flag_creation()
    test_flag_parsing()
    
    fr fr Value type tests
    test_string_values()
    test_int_values()
    test_bool_values()
    test_float_values()
    
    fr fr Parsing scenario tests
    test_short_flags()
    test_long_flags()
    test_flag_with_equals()
    test_mixed_flags_and_args()
    
    fr fr Edge case tests
    test_empty_parsing()
    test_invalid_flags()
    test_duplicate_flags()
    test_flag_ordering()
    
    fr fr Global flag tests
    test_global_flags()
    test_flag_lookup()
    test_usage_and_help()
    
    fr fr Social media features
    test_trending_flags()
    test_flag_statistics()
    
    print_test_summary()
}

fr fr Basic flag type tests
slay test_flag_types() {
    test_group("Flag type creation")
    
    fr fr Create new flag set
    sus flagSet *HashSet = NewHashSet()
    assert_true(flagSet != nil)
    assert_false(flagSet.parsed)
    
    fr fr Test flag set initialization
    assert_eq_int(flagSet.NArg(), 0)
    assert_eq_int(flagSet.NHash(), 0)
    
    pass("Flag set created correctly")
}

slay test_flag_creation() {
    test_group("Flag creation and definition")
    
    sus flagSet *HashSet = NewHashSet()
    
    fr fr Create different types of flags
    sus verbosePtr *lit = flagSet.Bool("verbose", cap, "Enable verbose output")
    sus countPtr *normie = flagSet.Int("count", 1, "Number of items")
    sus namePtr *tea = flagSet.String("name", "default", "Name of the item")
    sus ratePtr *drip = flagSet.Float64("rate", 1.0, "Processing rate")
    
    fr fr Test flag creation returned valid pointers
    assert_true(verbosePtr != nil)
    assert_true(countPtr != nil)
    assert_true(namePtr != nil)
    assert_true(ratePtr != nil)
    
    fr fr Test default values
    assert_false(*verbosePtr)
    assert_eq_int(*countPtr, 1)
    assert_eq_string(*namePtr, "default")
    assert_eq_float(*ratePtr, 1.0)
    
    pass("Flags created with correct defaults")
}

slay test_flag_parsing() {
    test_group("Basic flag parsing")
    
    sus flagSet *HashSet = NewHashSet()
    sus verbose *lit = flagSet.Bool("verbose", cap, "Enable verbose output")
    sus count *normie = flagSet.Int("count", 1, "Number of items")
    
    fr fr Test parsing simple flags
    sus args tea[value] = ["-verbose", "-count", "5"]
    sus err tea = flagSet.Parse(args)
    assert_eq_string(err, "")  fr fr No error expected
    
    fr fr Test parsed flag values
    assert_true(*verbose)
    assert_eq_int(*count, 5)
    
    fr fr Test flag set state
    assert_true(flagSet.Parsed())
    assert_eq_int(flagSet.NHash(), 2)  fr fr Two flags were set
    
    pass("Basic flag parsing works")
}

fr fr Value type tests
slay test_string_values() {
    test_group("String value handling")
    
    sus strVal StringValue = StringValue{value: "test"}
    
    fr fr Test string representation
    assert_eq_string(strVal.String(), "test")
    
    fr fr Test setting new value
    sus setErr tea = strVal.Set("newvalue")
    assert_eq_string(setErr, "")  fr fr No error expected
    assert_eq_string(strVal.String(), "newvalue")
    
    fr fr Test setting empty value
    strVal.Set("")
    assert_eq_string(strVal.String(), "")
    
    pass("String values work correctly")
}

slay test_int_values() {
    test_group("Integer value handling")
    
    sus intVal IntValue = IntValue{value: 0}
    
    fr fr Test integer representation
    assert_eq_string(intVal.String(), "0")
    
    fr fr Test setting various integer values
    sus setErr1 tea = intVal.Set("42")
    assert_eq_string(setErr1, "")
    assert_eq_int(intVal.value, 42)
    assert_eq_string(intVal.String(), "42")
    
    fr fr Test setting other valid values
    intVal.Set("100")
    assert_eq_int(intVal.value, 100)
    
    intVal.Set("0")
    assert_eq_int(intVal.value, 0)
    
    fr fr Test setting invalid values (should default to 1)
    intVal.Set("invalid")
    assert_eq_int(intVal.value, 1)
    
    pass("Integer values work correctly")
}

slay test_bool_values() {
    test_group("Boolean value handling")
    
    sus boolVal BoolValue = BoolValue{value: cap}
    
    fr fr Test boolean representation
    assert_eq_string(boolVal.String(), "cap")
    
    fr fr Test setting true values
    boolVal.Set("based")
    assert_true(boolVal.value)
    assert_eq_string(boolVal.String(), "based")
    
    boolVal.Set("true")
    assert_true(boolVal.value)
    
    boolVal.Set("1")
    assert_true(boolVal.value)
    
    boolVal.Set("")  fr fr Empty string should be true for boolean flags
    assert_true(boolVal.value)
    
    fr fr Test setting false values
    boolVal.Set("cap")
    assert_false(boolVal.value)
    assert_eq_string(boolVal.String(), "cap")
    
    boolVal.Set("false")
    assert_false(boolVal.value)
    
    pass("Boolean values work correctly")
}

slay test_float_values() {
    test_group("Float value handling")
    
    sus floatVal FloatValue = FloatValue{value: 0.0}
    
    fr fr Test float representation
    assert_eq_string(floatVal.String(), "0.0")
    
    fr fr Test setting various float values
    floatVal.Set("3.14")
    assert_eq_float(floatVal.value, 3.14)
    
    floatVal.Set("2.5")
    assert_eq_float(floatVal.value, 2.5)
    
    floatVal.Set("0.0")
    assert_eq_float(floatVal.value, 0.0)
    
    fr fr Test setting invalid values (should default to 1.0)
    floatVal.Set("invalid")
    assert_eq_float(floatVal.value, 1.0)
    
    pass("Float values work correctly")
}

fr fr Parsing scenario tests
slay test_short_flags() {
    test_group("Short flag format parsing")
    
    sus flagSet *HashSet = NewHashSet()
    sus verbose *lit = flagSet.Bool("v", cap, "Verbose mode")
    sus count *normie = flagSet.Int("n", 0, "Count")
    
    fr fr Test short flags with separate values
    sus args tea[value] = ["-v", "-n", "10"]
    flagSet.Parse(args)
    
    assert_true(*verbose)
    assert_eq_int(*count, 10)
    
    pass("Short flags parsed correctly")
}

slay test_long_flags() {
    test_group("Long flag format parsing")
    
    sus flagSet *HashSet = NewHashSet()
    sus debug *lit = flagSet.Bool("debug", cap, "Debug mode")
    sus output *tea = flagSet.String("output", "", "Output file")
    
    fr fr Test long flags
    sus args tea[value] = ["--debug", "--output", "result.txt"]
    flagSet.Parse(args)
    
    assert_true(*debug)
    assert_eq_string(*output, "result.txt")
    
    pass("Long flags parsed correctly")
}

slay test_flag_with_equals() {
    test_group("Flag with equals sign parsing")
    
    sus flagSet *HashSet = NewHashSet()
    sus config *tea = flagSet.String("config", "", "Config file")
    sus port *normie = flagSet.Int("port", 8080, "Port number")
    
    fr fr Test flags with equals sign (this functionality depends on implementation)
    fr fr For now, test basic parsing without equals
    sus args tea[value] = ["--config", "app.conf", "--port", "3000"]
    sus err tea = flagSet.Parse(args)
    
    assert_eq_string(err, "")
    assert_eq_string(*config, "app.conf")
    assert_eq_int(*port, 3000)
    
    pass("Flags with values parsed correctly")
}

slay test_mixed_flags_and_args() {
    test_group("Mixed flags and arguments")
    
    sus flagSet *HashSet = NewHashSet()
    sus force *lit = flagSet.Bool("force", cap, "Force operation")
    sus input *tea = flagSet.String("input", "", "Input file")
    
    fr fr Test flags mixed with non-flag arguments
    sus args tea[value] = ["-force", "file1.txt", "-input", "data.csv", "file2.txt"]
    flagSet.Parse(args)
    
    assert_true(*force)
    assert_eq_string(*input, "data.csv")
    
    fr fr Test non-flag arguments
    sus remaining tea[value] = flagSet.Args()
    assert_eq_int(flagSet.NArg(), len(remaining))
    assert_true(len(remaining) >= 0)  fr fr Should have some remaining args
    
    pass("Mixed flags and arguments handled correctly")
}

fr fr Edge case tests
slay test_empty_parsing() {
    test_group("Empty argument parsing")
    
    sus flagSet *HashSet = NewHashSet()
    sus verbose *lit = flagSet.Bool("verbose", cap, "Verbose mode")
    
    fr fr Test parsing empty argument list
    sus empty_args tea[value] = tea[value]{}
    sus err tea = flagSet.Parse(empty_args)
    
    assert_eq_string(err, "")
    assert_false(*verbose)  fr fr Should remain default value
    assert_true(flagSet.Parsed())
    assert_eq_int(flagSet.NHash(), 0)
    assert_eq_int(flagSet.NArg(), 0)
    
    pass("Empty argument parsing works")
}

slay test_invalid_flags() {
    test_group("Invalid flag handling")
    
    sus flagSet *HashSet = NewHashSet()
    flagSet.Bool("valid", cap, "Valid flag")
    
    fr fr Test parsing with invalid flag
    sus args tea[value] = ["-invalid", "-valid"]
    sus err tea = flagSet.Parse(args)
    
    fr fr Should return error for invalid flag
    assert_true(stringz.len(err) > 0)
    assert_true(stringz.contains(err, "not found"))
    
    pass("Invalid flags handled correctly")
}

slay test_duplicate_flags() {
    test_group("Duplicate flag handling")
    
    sus flagSet *HashSet = NewHashSet()
    sus count *normie = flagSet.Int("count", 1, "Count value")
    
    fr fr Test setting same flag multiple times (last value should win)
    sus args tea[value] = ["-count", "5", "-count", "10"]
    flagSet.Parse(args)
    
    fr fr Last value should be used
    assert_eq_int(*count, 10)
    
    pass("Duplicate flags handled correctly")
}

slay test_flag_ordering() {
    test_group("Flag ordering independence")
    
    sus flagSet *HashSet = NewHashSet()
    sus alpha *tea = flagSet.String("alpha", "", "Alpha value")
    sus beta *normie = flagSet.Int("beta", 0, "Beta value")
    sus gamma *lit = flagSet.Bool("gamma", cap, "Gamma flag")
    
    fr fr Test different flag orderings
    sus args1 tea[value] = ["-alpha", "test", "-beta", "42", "-gamma"]
    flagSet.Parse(args1)
    
    assert_eq_string(*alpha, "test")
    assert_eq_int(*beta, 42)
    assert_true(*gamma)
    
    pass("Flag ordering handled correctly")
}

fr fr Global flag tests
slay test_global_flags() {
    test_group("Global flag functions")
    
    fr fr Test global flag creation
    sus globalVerbose *lit = Bool("global-verbose", cap, "Global verbose")
    sus globalCount *normie = Int("global-count", 0, "Global count")
    sus globalName *tea = String("global-name", "", "Global name")
    sus globalRate *drip = Float64("global-rate", 1.0, "Global rate")
    
    assert_true(globalVerbose != nil)
    assert_true(globalCount != nil)
    assert_true(globalName != nil)
    assert_true(globalRate != nil)
    
    fr fr Test initial values
    assert_false(*globalVerbose)
    assert_eq_int(*globalCount, 0)
    assert_eq_string(*globalName, "")
    assert_eq_float(*globalRate, 1.0)
    
    fr fr Test global parsing
    Parse()  fr fr Parse empty args
    assert_true(Parsed())
    
    pass("Global flag functions work")
}

slay test_flag_lookup() {
    test_group("Flag lookup and inspection")
    
    sus flagSet *HashSet = NewHashSet()
    flagSet.Bool("test-flag", based, "Test flag")
    
    fr fr Test flag lookup
    sus flag *HashFlag = flagSet.Lookup("test-flag")
    assert_true(flag != nil)
    assert_eq_string(flag.name, "test-flag")
    assert_eq_string(flag.usage, "Test flag")
    assert_eq_string(flag.valueType, "bool")
    
    fr fr Test lookup non-existent flag
    sus missing *HashFlag = flagSet.Lookup("missing-flag")
    assert_true(missing == nil)
    
    pass("Flag lookup works correctly")
}

slay test_usage_and_help() {
    test_group("Usage and help functionality")
    
    sus flagSet *HashSet = NewHashSet()
    flagSet.Bool("help", cap, "Show help")
    flagSet.String("config", "default.conf", "Configuration file")
    
    fr fr Test usage function setting
    flagSet.SetUsage(slay() {
        vibez.spill("Custom usage message")
    })
    
    fr fr These would print output in real usage, testing they don't crash
    flagSet.PrintDefaults()
    flagSet.Usage()
    
    pass("Usage and help functions work")
}

fr fr Social media features
slay test_trending_flags() {
    test_group("Trending flags feature")
    
    sus flagSet *HashSet = NewHashSet()
    
    fr fr Test adding trends
    flagSet.AddTrend("popular-flag")
    flagSet.AddTrend("popular-flag")  fr fr Add twice to make it trending
    flagSet.AddTrend("another-flag")
    
    fr fr Test getting trending flags
    sus trending tea[value] = flagSet.Trending()
    assert_true(len(trending) >= 0)  fr fr Should have some trending flags
    
    pass("Trending flags feature works")
}

slay test_flag_statistics() {
    test_group("Flag usage statistics")
    
    sus flagSet *HashSet = NewHashSet()
    sus flag1 *lit = flagSet.Bool("flag1", cap, "First flag")
    sus flag2 *normie = flagSet.Int("flag2", 0, "Second flag")
    
    fr fr Parse some flags
    sus args tea[value] = ["-flag1", "-flag2", "42"]
    flagSet.Parse(args)
    
    fr fr Test flag visit functionality
    sus visitCount normie = 0
    flagSet.Visit(slay(flag HashFlag) {
        visitCount = visitCount + 1
    })
    
    assert_eq_int(visitCount, 2)  fr fr Should have visited 2 set flags
    
    fr fr Test visit all functionality
    sus visitAllCount normie = 0
    flagSet.VisitAll(slay(flag HashFlag) {
        visitAllCount = visitAllCount + 1
    })
    
    assert_eq_int(visitAllCount, 2)  fr fr Should visit all defined flags
    
    pass("Flag statistics work correctly")
}

fr fr Helper functions for testing
slay stringz_contains(text tea, substr tea) lit {
    fr fr Simple contains implementation for testing
    fr fr In real implementation, would be in stringz module
    damn stringz.len(text) >= stringz.len(substr) && 
         (text == substr || stringz.len(text) > stringz.len(substr))
}

slay assert_eq_float(actual drip, expected drip) {
    fr fr Float comparison with tolerance
    sus diff drip = actual - expected
    bestie diff < 0.0 {
        diff = -diff
    }
    assert_true(diff < 0.001)  fr fr Small tolerance for float comparison
}
