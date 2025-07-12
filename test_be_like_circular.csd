// Test circular type alias detection

// This should fail with circular reference error
be_like TypeA = TypeB
be_like TypeB = TypeA

sus x TypeA = 42
