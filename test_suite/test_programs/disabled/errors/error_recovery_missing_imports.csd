vibe main

fr fr Test: Parser error recovery with missing imports
fr fr Purpose: Test that parser can gracefully handle missing import errors
fr fr Expected: Should fail gracefully with clear error about missing module

fr fr Intentionally missing: yeet "vibez"
fr fr This should cause an error when trying to use vibez.spill

slay main_character() {
fr fr This should fail because vibez module is not imported
    vibez.spill("This should cause an import error");
    
fr fr Test other operations that don't require imports
    sus x normie = 42;
    sus y normie = x + 10;
    
fr fr This should also fail due to missing import
    vibez.spill("Result: {}", y);
    
    damn 0;
}
