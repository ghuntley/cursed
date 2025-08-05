fr fr Advanced CURSED Features Test Suite
fr fr Tests complex language features and edge cases

yeet "testz"

fr fr ===== ADVANCED PATTERN MATCHING =====

squad Shape {
    spill kind tea
    spill size meal
}

test_start("Complex Pattern Matching with Guards")
sus shapes []Shape = [
    Shape{kind: "circle", size: 5.0},
    Shape{kind: "square", size: 10.0},
    Shape{kind: "triangle", size: 3.0}
]

sus largeShapes drip = 0

bestie shape in shapes {
    match shape {
        Shape{kind: "circle", size: s} if s > 4.0 => largeShapes = largeShapes + 1,
        Shape{kind: "square", size: s} if s > 8.0 => largeShapes = largeShapes + 1,
        _ => fr fr ignore small shapes
    }
}

assert_eq_int(largeShapes, 2)

test_start("Nested Pattern Matching")
sus data any = [1, 2, [3, 4]]

match data {
    []any{a, b, []any{c, d}} => {
        assert_eq_int(a.(drip), 1)
        assert_eq_int(c.(drip), 3)
    },
    _ => assert_true(cringe) fr fr should not reach here
}

fr fr ===== ADVANCED GENERICS =====

collab Container<T> {
    slay add(item T)
    slay get(index drip) T
    slay size() drip
}

squad GenericList<T> {
    spill items []T
}

flex GenericList<T> => Container<T> {
    slay add(item T) {
        items = append(items, item)
    }
    
    slay get(index drip) T {
        damn items[index]
    }
    
    slay size() drip {
        damn len(items)
    }
}

test_start("Generic Interface Implementation")
sus intList GenericList<drip> = GenericList<drip>{items: []}
sus container Container<drip> = intList

container.add(10)
container.add(20)
container.add(30)

assert_eq_int(container.size(), 3)
assert_eq_int(container.get(1), 20)

test_start("Generic Type Constraints")
collab Numeric<T> {
    slay add(other T) T
    slay multiply(factor T) T
}

squad Number<T> {
    spill value T
}

flex Number<T> => Numeric<T> {
    slay add(other T) T {
        damn value + other
    }
    
    slay multiply(factor T) T {
        damn value * factor
    }
}

sus num Number<meal> = Number<meal>{value: 3.14}
sus result = num.add(2.86)
assert_true(result > 6.0)

fr fr ===== ADVANCED CONCURRENCY =====

test_start("Complex Select with Timeout")
sus workCh = make_channel<drip>()
sus timeoutCh = make_channel<lit>()
sus resultCh = make_channel<tea>()

stan {
    fr fr Simulate work
    time.sleep(100 * time.Millisecond)
    dm_send(workCh, 42)
}

stan {
    fr fr Simulate timeout
    time.sleep(200 * time.Millisecond)
    dm_send(timeoutCh, based)
}

stan {
    ready {
        mood result := dm_recv(workCh):
            dm_send(resultCh, "work completed")
        mood dm_recv(timeoutCh):
            dm_send(resultCh, "timeout occurred")
    }
}

sus outcome = dm_recv(resultCh)
assert_eq_string(outcome, "work completed")

test_start("Goroutine Pool Pattern")
sus jobCh = make_channel<drip>(10)
sus resultsCh = make_channel<drip>(10)
sus workerCount drip = 3

fr fr Start worker pool
bestie i := 0; i < workerCount; i = i + 1 {
    stan {
        bestie job := dm_recv(jobCh) {
            result := job * job
            dm_send(resultsCh, result)
        }
    }
}

fr fr Send jobs
bestie i := 1; i <= 5; i = i + 1 {
    dm_send(jobCh, i)
}

fr fr Collect results
sus totalResults drip = 0
bestie i := 0; i < 5; i = i + 1 {
    result := dm_recv(resultsCh)
    totalResults = totalResults + result
}

assert_eq_int(totalResults, 55) fr fr 1² + 2² + 3² + 4² + 5² = 55

test_start("Channel Pipeline Pattern")
sus numbers = make_channel<drip>()
sus squares = make_channel<drip>()
sus doubled = make_channel<drip>()

fr fr Stage 1: Square numbers
stan {
    bestie num := dm_recv(numbers) {
        dm_send(squares, num * num)
    }
}

fr fr Stage 2: Double the squares
stan {
    bestie sq := dm_recv(squares) {
        dm_send(doubled, sq * 2)
    }
}

fr fr Send input
stan {
    dm_send(numbers, 3)
    dm_send(numbers, 4)
    dm_close(numbers)
}

sus result1 = dm_recv(doubled) fr fr 3² * 2 = 18
sus result2 = dm_recv(doubled) fr fr 4² * 2 = 32

assert_eq_int(result1, 18)
assert_eq_int(result2, 32)

fr fr ===== ADVANCED ERROR HANDLING =====

test_start("Error Propagation with shook")
slay operation1() (drip, yikes) {
    damn 0, yikes("operation1 failed")
}

slay operation2() (drip, yikes) {
    sus result = operation1() shook
    damn result, cringe
}

slay operation3() (drip, yikes) {
    sus result = operation2() shook
    damn result, cringe
}

sus _, err = operation3()
assert_false(err == cringe)
assert_true(contains(err.message(), "operation1 failed"))

test_start("Panic Recovery with fam")
sus recovered lit = cringe
sus panicMessage tea = ""

fam {
    shook("critical failure")
    assert_true(cringe) fr fr should not reach here
} sus panic_value {
    recovered = based
    panicMessage = panic_value.message()
}

assert_true(recovered)
assert_eq_string(panicMessage, "critical failure")

test_start("Complex Error Wrapping")
slay database_connect() (drip, yikes) {
    damn 0, yikes("connection refused")
}

slay initialize_service() yikes {
    sus _, err = database_connect()
    lowkey err != cringe {
        damn wrap_error(err, "service initialization failed")
    }
    damn cringe
}

slay wrap_error(err yikes, context tea) yikes {
    damn yikes(context + ": " + err.message())
}

sus initErr = initialize_service()
assert_false(initErr == cringe)
assert_true(contains(initErr.message(), "service initialization failed"))
assert_true(contains(initErr.message(), "connection refused"))

fr fr ===== ADVANCED MEMORY MANAGEMENT =====

test_start("Complex Defer with Panic Recovery")
sus cleanupCalled lit = cringe
sus resourceReleased lit = cringe

slay risky_operation() {
    later {
        cleanupCalled = based
    }
    
    later {
        resourceReleased = based
    }
    
    shook("something went wrong")
}

fam {
    risky_operation()
} sus _ {
    fr fr Panic recovered
}

assert_true(cleanupCalled)
assert_true(resourceReleased)

test_start("Defer Order with Multiple Functions")
sus deferSequence []drip = []

slay function_a() {
    later deferSequence = append(deferSequence, 1)
    function_b()
}

slay function_b() {
    later deferSequence = append(deferSequence, 2)
    function_c()
}

slay function_c() {
    later deferSequence = append(deferSequence, 3)
}

function_a()

fr fr Each function's defers execute in LIFO order when the function returns
assert_eq_int(len(deferSequence), 3)
assert_eq_int(deferSequence[0], 3) fr fr function_c's defer
assert_eq_int(deferSequence[1], 2) fr fr function_b's defer  
assert_eq_int(deferSequence[2], 1) fr fr function_a's defer

fr fr ===== ADVANCED TYPE SYSTEM =====

test_start("Type Aliases and Custom Types")
be_like UserID drip
be_like Email tea

sus userID UserID = UserID(12345)
sus email Email = Email("user@example.com")

assert_eq_int(drip(userID), 12345)
assert_eq_string(tea(email), "user@example.com")

test_start("Interface Composition")
collab Reader {
    slay read() tea
}

collab Writer {
    slay write(data tea)
}

collab ReadWriter {
    Reader
    Writer
}

squad FileHandler {
    spill content tea
}

flex FileHandler => ReadWriter {
    slay read() tea {
        damn content
    }
    
    slay write(data tea) {
        content = data
    }
}

sus handler ReadWriter = FileHandler{content: "initial"}
handler.write("updated content")
sus result = handler.read()
assert_eq_string(result, "updated content")

test_start("Method Sets and Receiver Types")
squad Counter {
    spill value drip
}

slay (c @Counter) increment() {
    c.value = c.value + 1
}

slay (c @Counter) getValue() drip {
    damn c.value
}

sus counter Counter = Counter{value: 0}
counter.increment()
counter.increment()

assert_eq_int(counter.getValue(), 2)

fr fr ===== ADVANCED CONTROL FLOW =====

test_start("Labeled Break and Continue")
sus outerSum drip = 0

outer: bestie i := 0; i < 3; i = i + 1 {
    inner: bestie j := 0; j < 3; j = j + 1 {
        lowkey i == 1 && j == 1 {
            ghosted outer
        }
        outerSum = outerSum + i + j
    }
}

fr fr Should break out of both loops when i=1, j=1
fr fr Sum: (0,0)+(0,1)+(0,2)+(1,0) = 0+1+2+1 = 4
assert_eq_int(outerSum, 4)

test_start("Complex Switch with Type Assertions")
sus values []any = [42, "hello", 3.14, based]
sus typeResults []tea = []

bestie value in values {
    vibe_check value {
        mood v drip:
            typeResults = append(typeResults, "int")
        mood v tea:
            typeResults = append(typeResults, "string")
        mood v meal:
            typeResults = append(typeResults, "float")
        mood v lit:
            typeResults = append(typeResults, "bool")
        basic:
            typeResults = append(typeResults, "unknown")
    }
}

assert_eq_int(len(typeResults), 4)
assert_eq_string(typeResults[0], "int")
assert_eq_string(typeResults[1], "string")
assert_eq_string(typeResults[2], "float")
assert_eq_string(typeResults[3], "bool")

fr fr ===== UTILITY FUNCTIONS =====

slay contains(text tea, substring tea) lit {
    fr fr Simple contains implementation
    textLen := len(text)
    subLen := len(substring)
    
    lowkey subLen > textLen {
        damn cringe
    }
    
    bestie i := 0; i <= textLen - subLen; i = i + 1 {
        match := based
        bestie j := 0; j < subLen; j = j + 1 {
            lowkey text[i + j] != substring[j] {
                match = cringe
                ghosted
            }
        }
        lowkey match {
            damn based
        }
    }
    
    damn cringe
}

fr fr ===== FINAL TEST SUMMARY =====

print_test_summary()
