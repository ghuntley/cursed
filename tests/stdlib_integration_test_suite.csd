fr fr Standard Library Integration Test Suite
fr fr Tests integration between stdlib modules and language features

yeet "testz"
yeet "collections"
yeet "stringz"
yeet "math"
yeet "fs"
yeet "error_drip"
yeet "atomic_drip"
yeet "concurrenz"

fr fr ===== COLLECTIONS INTEGRATION =====

test_start("Vector Operations Integration")
sus vec = collections.new_vector<drip>()
vec.push(10)
vec.push(20)
vec.push(30)

assert_eq_int(vec.size(), 3)
assert_eq_int(vec.get(1), 20)

sus doubled = vec.map(slay(x drip) drip { damn x * 2 })
assert_eq_int(doubled.get(0), 20)
assert_eq_int(doubled.get(2), 60)

test_start("HashMap Operations Integration")
sus map = collections.new_hashmap<tea, drip>()
map.put("one", 1)
map.put("two", 2)
map.put("three", 3)

assert_eq_int(map.size(), 3)
assert_true(map.contains_key("two"))

sus value, exists = map.get("two")
assert_true(exists)
assert_eq_int(value, 2)

test_start("Set Operations Integration")
sus set1 = collections.new_set<drip>()
sus set2 = collections.new_set<drip>()

set1.add(1)
set1.add(2)
set1.add(3)

set2.add(2)
set2.add(3)
set2.add(4)

sus intersection = set1.intersection(set2)
assert_eq_int(intersection.size(), 2)
assert_true(intersection.contains(2))
assert_true(intersection.contains(3))

fr fr ===== STRING PROCESSING INTEGRATION =====

test_start("String Operations with Collections")
sus text tea = "hello,world,cursed,language"
sus parts = stringz.split(text, ",")

assert_eq_int(len(parts), 4)
assert_eq_string(parts[0], "hello")
assert_eq_string(parts[3], "language")

sus upperParts = collections.new_vector<tea>()
bestie part in parts {
    upperParts.push(stringz.to_upper(part))
}

assert_eq_string(upperParts.get(0), "HELLO")
assert_eq_string(upperParts.get(2), "CURSED")

test_start("String Template Integration")
sus name = "CURSED"
sus version = "1.0"
sus template = "Welcome to {} version {}"

sus formatted = stringz.format(template, [name, version])
assert_eq_string(formatted, "Welcome to CURSED version 1.0")

test_start("String Validation Chain")
sus emails []tea = [
    "valid@example.com",
    "invalid-email",
    "another@test.org",
    "bad@",
    "good@domain.co.uk"
]

sus validEmails = collections.new_vector<tea>()
bestie email in emails {
    lowkey stringz.contains(email, "@") && stringz.contains(email, ".") {
        validEmails.push(email)
    }
}

assert_eq_int(validEmails.size(), 3)

fr fr ===== MATHEMATICAL OPERATIONS INTEGRATION =====

test_start("Math with Collections")
sus numbers []meal = [1.1, 2.2, 3.3, 4.4, 5.5]
sus sum meal = 0.0
sus product meal = 1.0

bestie num in numbers {
    sum = sum + num
    product = product * num
}

sus mean = sum / meal(len(numbers))
assert_true(math.abs(mean - 3.3) < 0.01)

test_start("Statistical Operations")
sus dataset []meal = [10.0, 20.0, 30.0, 40.0, 50.0]
sus total meal = 0.0
sus count = len(dataset)

bestie value in dataset {
    total = total + value
}

sus average = total / meal(count)
assert_eq_int(drip(average), 30)

sus variance meal = 0.0
bestie value in dataset {
    diff := value - average
    variance = variance + (diff * diff)
}
variance = variance / meal(count)

sus stdDev = math.sqrt(variance)
assert_true(stdDev > 15.0 && stdDev < 16.0)

test_start("Complex Number Operations")
sus complexResults = collections.new_vector<meal>()

bestie i := 0; i < 10; i = i + 1 {
    angle := meal(i) * math.pi / 5.0
    value := math.sin(angle) * math.cos(angle)
    complexResults.push(value)
}

assert_eq_int(complexResults.size(), 10)
assert_true(complexResults.get(0) == 0.0) fr fr sin(0) * cos(0) = 0

fr fr ===== FILE SYSTEM INTEGRATION =====

test_start("File Operations with Error Handling")
sus testFile = "/tmp/cursed_test.txt"
sus testContent = "CURSED test content\nLine 2\nLine 3"

sus writeErr = fs.write_file(testFile, testContent)
assert_true(writeErr == cringe)

sus readContent, readErr = fs.read_file(testFile)
assert_true(readErr == cringe)
assert_eq_string(readContent, testContent)

sus lines = stringz.split(readContent, "\n")
assert_eq_int(len(lines), 3)
assert_eq_string(lines[1], "Line 2")

sus cleanupErr = fs.remove_file(testFile)
assert_true(cleanupErr == cringe)

test_start("Directory Operations")
sus testDir = "/tmp/cursed_test_dir"

sus mkdirErr = fs.create_dir(testDir)
assert_true(mkdirErr == cringe)

sus exists = fs.exists(testDir)
assert_true(exists)

sus rmdirErr = fs.remove_dir(testDir)
assert_true(rmdirErr == cringe)

fr fr ===== ERROR HANDLING INTEGRATION =====

test_start("Error Chain with Multiple Operations")
slay complex_operation() yikes {
    sus numbers = collections.new_vector<drip>()
    
    fam {
        numbers.push(10)
        numbers.push(0)
        numbers.push(5)
        
        sus result = 100 / numbers.get(1) fr fr Division by zero
        damn cringe
    } sus panic_value {
        damn error_drip.new_error("Math operation failed: " + panic_value.message())
    }
}

sus err = complex_operation()
assert_false(err == cringe)
assert_true(stringz.contains(err.message(), "Math operation failed"))

test_start("Error Recovery with Resource Cleanup")
sus resourcesCleaned lit = cringe

slay risky_file_operation() yikes {
    sus tempFile = "/tmp/risky_test.txt"
    
    later {
        fs.remove_file(tempFile)
        resourcesCleaned = based
    }
    
    sus writeErr = fs.write_file(tempFile, "test data")
    lowkey writeErr != cringe {
        damn writeErr
    }
    
    fr fr Simulate failure
    shook("Simulated failure")
    
    damn cringe
}

fam {
    risky_file_operation()
} sus _ {
    fr fr Panic recovered
}

assert_true(resourcesCleaned)

fr fr ===== CONCURRENCY INTEGRATION =====

test_start("Concurrent Collections Access")
sus sharedMap = collections.new_concurrent_hashmap<drip, drip>()
sus workerCount drip = 5
sus itemsPerWorker drip = 10
sus resultCh = make_channel<lit>(workerCount)

bestie workerId := 0; workerId < workerCount; workerId = workerId + 1 {
    stan {
        start := workerId * itemsPerWorker
        bestie i := start; i < start + itemsPerWorker; i = i + 1 {
            sharedMap.put(i, i * i)
        }
        dm_send(resultCh, based)
    }
}

fr fr Wait for all workers to complete
bestie i := 0; i < workerCount; i = i + 1 {
    dm_recv(resultCh)
}

assert_eq_int(sharedMap.size(), workerCount * itemsPerWorker)

sus value, exists = sharedMap.get(25)
assert_true(exists)
assert_eq_int(value, 625) fr fr 25²

test_start("Atomic Operations Integration")
sus counter = atomic_drip.new_counter()
sus goroutines drip = 10
sus incrementsPerGoroutine drip = 100
sus done = make_channel<lit>(goroutines)

bestie i := 0; i < goroutines; i = i + 1 {
    stan {
        bestie j := 0; j < incrementsPerGoroutine; j = j + 1 {
            counter.increment()
        }
        dm_send(done, based)
    }
}

fr fr Wait for all goroutines
bestie i := 0; i < goroutines; i = i + 1 {
    dm_recv(done)
}

sus finalValue = counter.get()
assert_eq_int(finalValue, goroutines * incrementsPerGoroutine)

test_start("Producer-Consumer with Stdlib")
sus items = make_channel<drip>(5)
sus processed = collections.new_vector<drip>()
sus producerDone = make_channel<lit>()
sus consumerDone = make_channel<lit>()

fr fr Producer goroutine
stan {
    bestie i := 1; i <= 10; i = i + 1 {
        dm_send(items, i)
    }
    dm_close(items)
    dm_send(producerDone, based)
}

fr fr Consumer goroutine
stan {
    bestie item, ok := dm_recv(items); ok {
        squared := math.pow(meal(item), 2.0)
        processed.push(drip(squared))
    }
    dm_send(consumerDone, based)
}

dm_recv(producerDone)
dm_recv(consumerDone)

assert_eq_int(processed.size(), 10)
assert_eq_int(processed.get(0), 1)   fr fr 1²
assert_eq_int(processed.get(9), 100) fr fr 10²

fr fr ===== COMPLEX INTEGRATION SCENARIOS =====

test_start("Text Processing Pipeline")
sus inputText = "The quick brown fox jumps over the lazy dog"
sus words = stringz.split(inputText, " ")
sus wordCounts = collections.new_hashmap<tea, drip>()

fr fr Count word lengths
bestie word in words {
    length := len(word)
    currentCount, exists := wordCounts.get(tea(length))
    lowkey !exists {
        currentCount = 0
    }
    wordCounts.put(tea(length), currentCount + 1)
}

fr fr Verify counts
sus count3, exists3 = wordCounts.get("3")
assert_true(exists3)
assert_eq_int(count3, 3) fr fr "The", "fox", "the", "dog"

test_start("Data Analysis Pipeline")
sus salesData []meal = [100.0, 150.0, 200.0, 175.0, 225.0, 180.0, 195.0]
sus analysisResults = collections.new_hashmap<tea, meal>()

fr fr Calculate statistics
sus total meal = 0.0
sus maxValue meal = salesData[0]
sus minValue meal = salesData[0]

bestie value in salesData {
    total = total + value
    lowkey value > maxValue {
        maxValue = value
    }
    lowkey value < minValue {
        minValue = value
    }
}

analysisResults.put("total", total)
analysisResults.put("average", total / meal(len(salesData)))
analysisResults.put("max", maxValue)
analysisResults.put("min", minValue)

sus avgValue, _ = analysisResults.get("average")
assert_true(avgValue > 175.0 && avgValue < 185.0)

sus maxVal, _ = analysisResults.get("max")
assert_eq_int(drip(maxVal), 225)

fr fr ===== PERFORMANCE INTEGRATION =====

test_start("Large Dataset Processing")
sus largeDataset = collections.new_vector<drip>()

fr fr Create large dataset
bestie i := 0; i < 1000; i = i + 1 {
    largeDataset.push(i)
}

fr fr Process with mathematical operations
sus evenSum drip = 0
sus oddSum drip = 0

bestie i := 0; i < largeDataset.size(); i = i + 1 {
    value := largeDataset.get(i)
    lowkey value % 2 == 0 {
        evenSum = evenSum + value
    } highkey {
        oddSum = oddSum + value
    }
}

fr fr Verify mathematical correctness
fr fr Sum of even numbers 0 to 998: 249500
fr fr Sum of odd numbers 1 to 999: 250000
assert_eq_int(evenSum, 249500)
assert_eq_int(oddSum, 250000)

fr fr ===== FINAL INTEGRATION SUMMARY =====

print_test_summary()
