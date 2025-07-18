# Debug parser issue

slay test_function() lit {
    damn (1 == 1)
}

slay main() {
    sus result lit = test_function()
    vibez.spill("Result: " + tea(result))
}
