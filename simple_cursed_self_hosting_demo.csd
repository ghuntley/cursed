vibe self_hosting_demo

fr fr Simple demonstration of CURSED self-hosting capability
fr fr This program shows that CURSED can compile and run its own code

slay parseSimpleExpression(input tea) tea {
    fr fr Simple parser demonstration
    lowkey input == "2 + 3" {
        damn "addition"
    }
    lowkey input == "5 * 4" {
        damn "multiplication"
    }
    damn "unknown"
}

slay evaluateExpression(operation tea) normie {
    fr fr Simple evaluator demonstration
    lowkey operation == "addition" {
        damn 2 + 3
    }
    lowkey operation == "multiplication" {
        damn 5 * 4
    }
    damn 0
}

slay compileAndRun(source tea) normie {
    fr fr Meta-compilation simulation
    vibez.spill("🔍 Parsing source:")
    vibez.spill(source)
    
    sus operation tea = parseSimpleExpression(source)
    vibez.spill("📋 Parsed operation:")
    vibez.spill(operation)
    
    sus result normie = evaluateExpression(operation)
    vibez.spill("⚡ Executed result:")
    vibez.spill(result)
    
    damn result
}

slay main() {
    vibez.spill("🚀 CURSED Self-Hosting Demo")
    vibez.spill("============================")
    
    fr fr Test 1: Addition
    sus result1 normie = compileAndRun("2 + 3")
    vibez.spill("✅ Test 1 result:")
    vibez.spill(result1)
    
    fr fr Test 2: Multiplication  
    sus result2 normie = compileAndRun("5 * 4")
    vibez.spill("✅ Test 2 result:")
    vibez.spill(result2)
    
    fr fr Demonstrate if prefix functionality
    lowkey x := result1 + result2; x > 20 {
        vibez.spill("🎉 Self-hosting demo successful!")
        vibez.spill("Total computed:")
        vibez.spill(x)
    }
}
