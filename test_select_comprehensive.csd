# Select statement comprehensive test
yeet "vibez"

slay test_default_only() {
    ready {
        basic:
            vibez.spill("Default case executed!")
    }
}

slay test_basic_usage() {
    vibez.spill("Testing select statement execution...")
    test_default_only()
    vibez.spill("Select test completed successfully!")
}

test_basic_usage()
