// Test error handling functionality in CURSED language

vibe error_handling_test

slay test_panic() {
    yeet_error "This is a test panic message"
}

slay test_catch_basic() {
    catch {
        yeet_error "This should be caught"
    } recover {
        // Error was caught and handled
        sus recovered = facts
    }
}

slay test_catch_with_error_var() {
    catch {
        yeet_error "Error with details"
    } recover (error) {
        // Error variable contains the error details
        sus error_msg = error
    }
}

slay main() {
    test_catch_basic()
    test_catch_with_error_var()
}
