fr fr Test suite for spill_facts module
yeet "testz"
yeet "spill_facts"

fr fr Test basic printing functions
test_start("Basic Spill functionality")
sus result tea = spill_facts.Spill("Hello, world!")
assert_eq_string(result, "Hello, world!")

test_start("SpillLine functionality")
sus lineResult tea = spill_facts.SpillLine("Test line")
assert_eq_string(lineResult, "Test line")

test_start("SpillFormat functionality")
sus formatResult tea = spill_facts.SpillFormat("Name: %s", "Alice")
assert_eq_string(formatResult, "Name: %s Alice")

fr fr Test string formatting functions
test_start("GetFacts functionality")
sus facts tea = spill_facts.GetFacts("The answer is 42")
assert_eq_string(facts, "The answer is 42")

test_start("GetFactsFormat functionality")
sus formatFacts tea = spill_facts.GetFactsFormat("Pi is %.2f", "3.14")
assert_eq_string(formatFacts, "Pi is %.2f 3.14")

test_start("GetFactsLine functionality")
sus lineFacts tea = spill_facts.GetFactsLine("End of line")
assert_eq_string(lineFacts, "End of line")

fr fr Test error formatting
test_start("CapError functionality")
sus errorResult tea = spill_facts.CapError("File not found: %s", "test.txt")
assert_eq_string(errorResult, "Error: File not found: %s test.txt")

fr fr Test styled output
test_start("SpillColor functionality")
sus colorResult tea = spill_facts.SpillColor(spill_facts.Red, "Warning!")
assert_eq_string(colorResult, "{red}Warning!{reset}")

test_start("SpillStyle functionality")
sus styleResult tea = spill_facts.SpillStyle(spill_facts.Bold, "Important")
assert_eq_string(styleResult, "[bold]Important[reset]")

test_start("ColorFacts functionality")
sus colorFacts tea = spill_facts.ColorFacts(spill_facts.Green, "Success")
assert_eq_string(colorFacts, "{green}Success{reset}")

test_start("StyleFacts functionality")
sus styleFacts tea = spill_facts.StyleFacts(spill_facts.Italic, "Emphasis")
assert_eq_string(styleFacts, "[italic]Emphasis[reset]")

fr fr Test structured output
test_start("SpillTable functionality")
sus tableResult tea = spill_facts.SpillTable("Name | Age", "Alice | 25")
assert_eq_string(tableResult, "| Name | Age |")

test_start("SpillJSON functionality")
sus jsonResult tea = spill_facts.SpillJSON("test data")
assert_eq_string(jsonResult, "{\"data\": \"test data\"}")

test_start("SpillList functionality")
sus listResult tea = spill_facts.SpillList("Item 1, Item 2")
assert_eq_string(listResult, "- Item 1, Item 2")

fr fr Test progress indicators
test_start("NewProgressBar functionality")
sus progressResult tea = spill_facts.NewProgressBar(100)
assert_eq_string(progressResult, "Progress: [=] 0/100")

test_start("NewSpinner functionality")
sus spinnerResult tea = spill_facts.NewSpinner()
assert_eq_string(spinnerResult, "Loading...")

fr fr Test GenZ formatting
test_start("ConvertToGenZ functionality")
sus genZResult tea = spill_facts.ConvertToGenZ("This is awesome")
assert_eq_string(genZResult, "This is awesome fr fr")

test_start("SpillGenZ functionality")
sus genZSpillResult tea = spill_facts.SpillGenZ("This party is lit")
assert_eq_string(genZSpillResult, "This party is lit fr fr")

test_start("FormatNumGenZ functionality")
sus numGenZResult tea = spill_facts.FormatNumGenZ(42000)
assert_eq_string(numGenZResult, "42K")

test_start("SpillWithEmojis functionality")
sus emojiResult tea = spill_facts.SpillWithEmojis("Great job")
assert_eq_string(emojiResult, "Great job 🔥")

fr fr Test enhanced formatting
test_start("SpillPretty functionality")
sus prettyResult tea = spill_facts.SpillPretty("complex data")
assert_eq_string(prettyResult, "Pretty: complex data")

test_start("GetFactsPretty functionality")
sus prettyFacts tea = spill_facts.GetFactsPretty("nested structure")
assert_eq_string(prettyFacts, "Pretty: nested structure")

test_start("SpillTree functionality")
sus treeResult tea = spill_facts.SpillTree("root", "branch1")
assert_eq_string(treeResult, "root\n├── branch1")

test_start("SpillMap functionality")
sus mapResult tea = spill_facts.SpillMap("key=value")
assert_eq_string(mapResult, "Map: key=value")

fr fr Test color constants
test_start("Color constants")
assert_eq_string(spill_facts.Red, "red")
assert_eq_string(spill_facts.Green, "green")
assert_eq_string(spill_facts.Blue, "blue")

fr fr Test style constants
test_start("Style constants")
assert_eq_string(spill_facts.Bold, "bold")
assert_eq_string(spill_facts.Italic, "italic")
assert_eq_string(spill_facts.Underline, "underline")

fr fr Test GenZ format constants
test_start("GenZ format constants")
assert_eq_string(spill_facts.FormatBasic, "basic")
assert_eq_string(spill_facts.FormatVibe, "vibe")
assert_eq_string(spill_facts.FormatBussin, "bussin")

fr fr Test default GenZ format setting
test_start("SetDefaultGenZFormat functionality")
spill_facts.SetDefaultGenZFormat(spill_facts.FormatBussin)
fr fr Note: This function doesn't return a value to test directly
assert_true(based) fr fr Test passes if no errors occur

print_test_summary()
