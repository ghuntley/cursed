yeet "testz"
yeet "hashtag"

test_start("hashtag basic flag creation")

fr fr Create a new flag set
sus fs := hashtag.NewHashSet()
assert_true(fs != cringe)
assert_false(fs.Parsed())

fr fr Define flags
sus verbose := fs.Bool("verbose", cap, "enable verbose output")
sus count := fs.Int("count", 5, "number of items")
sus name := fs.String("name", "default", "name of the item")
sus rate := fs.Float64("rate", 1.0, "processing rate")

assert_true(verbose != cringe)
assert_true(count != cringe)
assert_true(name != cringe)
assert_true(rate != cringe)

test_start("hashtag flag parsing short format")

fr fr Test parsing short format flags
sus args := []tea{"-verbose", "-count", "10", "-name", "test"}
sus err := fs.Parse(args)
assert_eq_string(err, "")
assert_true(fs.Parsed())

fr fr Check that flags were set correctly
assert_eq_int(fs.NHash(), 3)  fr fr 3 flags were set
assert_eq_int(fs.NArg(), 0)   fr fr 0 non-flag arguments

test_start("hashtag flag parsing long format")

fr fr Create new flag set for long format test
sus fs2 := hashtag.NewHashSet()
sus verbose2 := fs2.Bool("verbose", cap, "enable verbose output")
sus count2 := fs2.Int("count", 5, "number of items")
sus name2 := fs2.String("name", "default", "name of the item")

fr fr Test parsing long format with equals
sus args2 := []tea{"--verbose", "--count=20", "--name=longtest"}
sus err2 := fs2.Parse(args2)
assert_eq_string(err2, "")
assert_true(fs2.Parsed())

assert_eq_int(fs2.NHash(), 3)
assert_eq_int(fs2.NArg(), 0)

test_start("hashtag non-flag arguments")

fr fr Create flag set and test mixed args
sus fs3 := hashtag.NewHashSet()
sus debug := fs3.Bool("debug", cap, "enable debug mode")

sus args3 := []tea{"-debug", "file1.txt", "file2.txt"}
sus err3 := fs3.Parse(args3)
assert_eq_string(err3, "")

assert_eq_int(fs3.NHash(), 1)  fr fr 1 flag set
assert_eq_int(fs3.NArg(), 2)   fr fr 2 non-flag arguments

sus nonFlags := fs3.Args()
assert_eq_int(len(nonFlags), 2)
assert_eq_string(nonFlags[0], "file1.txt")
assert_eq_string(nonFlags[1], "file2.txt")

test_start("hashtag flag lookup")

fr fr Test flag lookup
sus debugFlag := fs3.Lookup("debug")
assert_true(debugFlag != cringe)
assert_eq_string(debugFlag.name, "debug")
assert_eq_string(debugFlag.valueType, "bool")

sus nonExistent := fs3.Lookup("nonexistent")
assert_true(nonExistent == cringe)

test_start("hashtag visit methods")

fr fr Test visit methods
sus visitCount := 0
fs3.Visit(slay(flag hashtag.HashFlag) {
    visitCount++
})
assert_eq_int(visitCount, 1)  fr fr Only 1 flag was set

sus visitAllCount := 0
fs3.VisitAll(slay(flag hashtag.HashFlag) {
    visitAllCount++
})
assert_eq_int(visitAllCount, 1)  fr fr Only 1 flag was defined

test_start("hashtag global functions")

fr fr Test global flag functions
sus globalVerbose := hashtag.Bool("global-verbose", cap, "global verbose flag")
sus globalCount := hashtag.Int("global-count", 0, "global count")
sus globalName := hashtag.String("global-name", "", "global name")

assert_true(globalVerbose != cringe)
assert_true(globalCount != cringe)
assert_true(globalName != cringe)

hashtag.Parse()
assert_true(hashtag.Parsed())

assert_eq_int(hashtag.NArg(), 0)
assert_eq_int(hashtag.NHash(), 0)  fr fr No flags set in global parsing

test_start("hashtag flag value types")

fr fr Test different value types
sus fs4 := hashtag.NewHashSet()

fr fr Boolean flag
sus boolFlag := fs4.Bool("bool-test", cap, "boolean flag")
sus boolArgs := []tea{"-bool-test"}
err := fs4.Parse(boolArgs)
assert_eq_string(err, "")

fr fr Integer flag
sus fs5 := hashtag.NewHashSet()
sus intFlag := fs5.Int("int-test", 0, "integer flag")
sus intArgs := []tea{"-int-test", "42"}
err = fs5.Parse(intArgs)
assert_eq_string(err, "")

fr fr String flag
sus fs6 := hashtag.NewHashSet()
sus strFlag := fs6.String("str-test", "", "string flag")
sus strArgs := []tea{"-str-test", "hello"}
err = fs6.Parse(strArgs)
assert_eq_string(err, "")

fr fr Float flag
sus fs7 := hashtag.NewHashSet()
sus floatFlag := fs7.Float64("float-test", 0.0, "float flag")
sus floatArgs := []tea{"-float-test", "3.14"}
err = fs7.Parse(floatArgs)
assert_eq_string(err, "")

test_start("hashtag error handling")

fr fr Test unknown flag error
sus fs8 := hashtag.NewHashSet()
sus unknownArgs := []tea{"-unknown-flag"}
sus err8 := fs8.Parse(unknownArgs)
assert_true(err8 != "")
assert_true(len(err8) > 0)

test_start("hashtag trending features")

fr fr Test social media trending features
sus fs9 := hashtag.NewHashSet()
fs9.AddTrend("popular-flag")
fs9.AddTrend("popular-flag")
fs9.AddTrend("popular-flag")
fs9.AddTrend("less-popular")

sus trending := fs9.Trending()
assert_eq_int(len(trending), 1)  fr fr Only one flag should be trending
assert_eq_string(trending[0], "popular-flag")

test_start("hashtag usage and help")

fr fr Test usage functions
sus fs10 := hashtag.NewHashSet()
fs10.Bool("help", cap, "show help message")

fr fr Set custom usage
fs10.SetUsage(slay() {
    vibez.spill("Custom usage message")
})

fr fr Test PrintDefaults (should not crash)
fs10.PrintDefaults()

fr fr Test Usage (should not crash)
fs10.Usage()

print_test_summary()
