// Test file to verify unknown attribute error handling

@inline(hint="always")
slay valid_function() {
    vibez.spill("This should work")
}

@invalid_unknown_attr(param="value")
slay invalid_function() {
    vibez.spill("This should fail to compile")
}

@another_typo
slay another_invalid() {
    vibez.spill("This should also fail")
}
