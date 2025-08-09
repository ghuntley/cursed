yikes MyError {
    message tea
}

slay risky_function() fam MyError {
    vibez.spill("Before error")
    yikes MyError "Something went wrong"
    vibez.spill("This should not print - after error")
    damn "success"
}

slay test_error_propagation() {
    vibez.spill("Testing error propagation")
    
    sus result fam MyError = risky_function() shook
    vibez.spill("This should not print - after shook")
    
    damn result
}

# Test the broken behavior
sus main_result fam MyError = test_error_propagation()
vibez.spill("Main result:", main_result)
