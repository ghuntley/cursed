vibe main
yeet "vibez"

fr fr Division Semantics Test
fr fr Tests: Correct integer vs float division behavior
fr fr Expected: Integer division returns integer, float division returns float

slay main_character() {
    vibez.spill("=== Division Semantics Test ===")
    
    fr fr Integer division (should stay integer)
    vibez.spill("Integer division results:")
    vibez.spill("10 / 3 =")
    vibez.spill(10 / 3)
    vibez.spill("15 / 4 =") 
    vibez.spill(15 / 4)
    vibez.spill("7 / 2 =")
    vibez.spill(7 / 2)
    
    fr fr Float division (should return float)
    vibez.spill("Float division results:")
    vibez.spill("10.0 / 3.0 =")
    vibez.spill(10.0 / 3.0)
    vibez.spill("15.0 / 4.0 =")
    vibez.spill(15.0 / 4.0)
    
    fr fr Mixed division (should promote to float)
    vibez.spill("Mixed division results:")
    vibez.spill("10 / 3.0 =")
    vibez.spill(10 / 3.0)
    vibez.spill("10.0 / 3 =")
    vibez.spill(10.0 / 3)
    
    vibez.spill("=== Test Complete ===")
}
