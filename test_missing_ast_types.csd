// Test array expressions
sus arr []drip = [1, 2, 3, 4]

// Test struct expressions with field initializers  
squad Point {
    spill x drip
    spill y drip
}

sus point Point = Point{
    x: 10,
    y: 20
}

// Test method calls
slay test_method_call() {
    sus p Point = Point{x: 5, y: 10}
    vibez.spill("Method call test")
}

// Test error expressions
yikes "Something went wrong", 500
shook test_method_call()
fam {
    vibez.spill("trying something dangerous")
} catch(e) {
    vibez.spill("caught error")
}
