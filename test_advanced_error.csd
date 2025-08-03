fr fr Test advanced error handling features
yikes CustomError = "Custom error occurred"

slay test_function() normie {
    fam {
        vibez.spill("In fam block")
        damn 42
    } catch(err) {
        vibez.spill("Caught error: " + err)
        damn -1
    }
}

result := test_function()
vibez.spill("Result: " + result.toString())
