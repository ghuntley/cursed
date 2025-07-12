// Test error cases for type aliases

// Should fail - conflicts with built-in type
be_like normie = tea

// Should fail - duplicate definition
be_like MyType = normie
be_like MyType = tea

// Should fail - undefined target type
be_like BadType = UndefinedType
