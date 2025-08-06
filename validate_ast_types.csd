slay main() {
    // Test ArrayExpression parsing
    sus arr = [1, 2, 3]
    
    // Test StructExpression parsing 
    squad TestStruct {
        spill value drip
    }
    
    sus obj = TestStruct{
        value: 42
    }
    
    // Test MethodCallExpression parsing
    obj.someMethod()
    
    // Test YikesExpression parsing
    yikes "Test error"
    
    // Test ShookExpression parsing  
    shook obj.someMethod()
    
    // Test FamExpression parsing
    fam {
        vibez.spill("test")
    } catch(e) {
        vibez.spill("error")
    }
    
    vibez.spill("All AST types validated!")
}
