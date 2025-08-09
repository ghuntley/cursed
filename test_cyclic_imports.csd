// Test case for cyclic module dependencies to reproduce double-free
// This will create a cycle: test_cyclic_imports -> module_a -> module_b -> module_a

yeet "module_a"

slay main() drip {
    vibez.spill("Testing cyclic imports")
    damn 0
}
