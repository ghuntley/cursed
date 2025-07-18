# Minimal test to isolate the parsing issue

# Simple test struct
be_like SimpleTest squad {
    name tea
    value normie
}

# Simple function
slay test_function() SimpleTest {
    sus result SimpleTest
    result.name = "test"
    result.value = 42
    damn result
}

slay main() {
    sus test SimpleTest = test_function()
    vibez.spill("Test name: " + test.name)
    vibez.spill("Test value: " + tea(test.value))
}
