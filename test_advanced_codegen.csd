fr fr Test comprehensive CURSED LLVM codegen implementation

yeet "testz"

fr fr Test basic arithmetic and variables
sus x drip = 42
sus y drip = 24
sus result drip = x + y
vibez.spill(result)

fr fr Test struct definition and usage
squad Point {
    spill x meal
    spill y meal
}

sus p Point = Point{x: 3.14, y: 2.71}
vibez.spill(p.x)

fr fr Test function definition and call
slay add_numbers(a drip, b drip) drip {
    damn a + b
}

sus sum drip = add_numbers(10, 20)
vibez.spill(sum)

fr fr Test if/else control flow
bestie x < 50 {
    vibez.spill("x is less than 50")
} shook {
    vibez.spill("x is 50 or greater")  
}

fr fr Test bestie (for) loop
sus counter drip = 0
bestie counter < 5 {
    vibez.spill(counter)
    counter = counter + 1
}

fr fr Test pattern matching
match x {
    42 => vibez.spill("The answer!"),
    _ => vibez.spill("Something else")
}

fr fr Test channel operations (commented out for now as AST support may be limited)
fr fr sus ch = make_channel<drip>()
fr fr dm_send(ch, 100)
fr fr sus received = dm_recv(ch)

fr fr Test goroutine creation (commented out for now)
fr fr stan {
fr fr     vibez.spill("Goroutine running!")
fr fr }

fr fr Test defer statement (commented out for now)
fr fr defer vibez.spill("Cleanup code")

test_start("Advanced Codegen Test")
assert_eq_int(result, 66)
assert_eq_int(sum, 30)
print_test_summary()
