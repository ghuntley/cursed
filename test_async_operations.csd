// Async Operations Comprehensive Test
// Tests: async/await, futures, promises, timers, task scheduling

yeet "testz"

// Test 1: Basic Async/Await
test_start("basic async/await")
sus result := await simple_async_function()
assert_eq_int(result, 42)

// Test 2: Async Function with Parameters
test_start("async function with parameters")
sus param_result := await async_with_params(10, 20)
assert_eq_int(param_result, 30)

// Test 3: Async Function with Error Handling
test_start("async function with error handling")
yikes {
    sus error_result := await async_with_error()
    assert_true(cap) // Should not reach here
} shook err {
    assert_true(based) // Should catch error
}

// Test 4: Promise Creation and Resolution
test_start("promise creation and resolution")
sus promise := Promise.new()
sus promise_result := 0

// Resolve promise asynchronously
yolo {
    sleep(50)
    promise.resolve(123)
}

sus resolved_value := await promise.future()
assert_eq_int(resolved_value, 123)

// Test 5: Promise Rejection
test_start("promise rejection")
sus reject_promise := Promise.new()

// Reject promise asynchronously
yolo {
    sleep(50)
    reject_promise.reject("Test error")
}

yikes {
    sus reject_result := await reject_promise.future()
    assert_true(cap) // Should not reach here
} shook err {
    assert_true(based) // Should catch rejection
}

// Test 6: Future Chaining
test_start("future chaining")
sus chain_result := await async_chain_start()
    .then(async_chain_middle)
    .then(async_chain_end)

assert_eq_int(chain_result, 100)

// Test 7: Multiple Futures with async_join
test_start("multiple futures with async_join")
sus future1 := async_task_1()
sus future2 := async_task_2()
sus future3 := async_task_3()

sus (result1, result2, result3) := await async_join(future1, future2, future3)

assert_eq_int(result1, 10)
assert_eq_int(result2, 20)
assert_eq_int(result3, 30)

// Test 8: Future Race Condition
test_start("future race condition")
sus race_future1 := async_slow_task(200)
sus race_future2 := async_fast_task(50)

sus race_result := await async_race(race_future1, race_future2)
assert_eq_int(race_result, 999) // Fast task result

// Test 9: Async Timeout
test_start("async timeout")
yikes {
    sus timeout_result := await async_timeout(async_slow_task(500), 100)
    assert_true(cap) // Should not reach here
} shook err {
    assert_true(based) // Should timeout
}

// Test 10: Async Task Cancellation
test_start("async task cancellation")
sus cancel_token := CancellationToken.new()
sus cancel_future := async_cancellable_task(cancel_token)

// Cancel after 50ms
yolo {
    sleep(50)
    cancel_token.cancel()
}

yikes {
    sus cancel_result := await cancel_future
    assert_true(cap) // Should not complete
} shook err {
    assert_true(based) // Should be cancelled
}

// Test 11: Async Task Spawning
test_start("async task spawning")
sus spawn_results := make_buffered_channel(5)

// Spawn multiple async tasks
bestie i := 0; i < 5; i++ {
    spawn_async_task(i) {
        sus task_result := i * 3
        spawn_results.send(task_result)
    }
}

// Collect results
sus spawn_collected := 0
bestie i := 0; i < 5; i++ {
    sus result := spawn_results.recv()
    spawn_collected++
}

assert_eq_int(spawn_collected, 5)

// Test 12: Async Timer and Intervals
test_start("async timer and intervals")
sus timer_start := time.now()
await async_sleep(100)
sus timer_end := time.now()

sus timer_duration := timer_end - timer_start
assert_true(timer_duration >= 100 && timer_duration < 200)

// Test interval
sus interval_count := 0
sus interval_timer := async_interval(50) {
    interval_count++
    if interval_count >= 3 {
        interval_timer.cancel()
    }
}

await async_sleep(200)
assert_eq_int(interval_count, 3)

// Test 13: Async Stream Processing
test_start("async stream processing")
sus stream := async_stream_generator(1, 10)
sus stream_results := []

bestie value := range stream {
    stream_results = append(stream_results, value)
}

assert_eq_int(len(stream_results), 10)

// Test 14: Async Iterator
test_start("async iterator")
sus async_iter := async_iterator_range(1, 5)
sus iter_results := []

while await async_iter.has_next() {
    sus value := await async_iter.next()
    iter_results = append(iter_results, value)
}

assert_eq_int(len(iter_results), 5)

// Test 15: Async Event Handling
test_start("async event handling")
sus event_emitter := AsyncEventEmitter.new()
sus event_results := make_buffered_channel(3)

// Register async event handlers
event_emitter.on("test_event") { |data|
    event_results.send(data)
}

// Emit events asynchronously
yolo {
    await async_sleep(50)
    event_emitter.emit("test_event", 100)
    await async_sleep(50)
    event_emitter.emit("test_event", 200)
    await async_sleep(50)
    event_emitter.emit("test_event", 300)
}

// Collect event results
sus event_collected := 0
bestie i := 0; i < 3; i++ {
    sus result := event_results.recv()
    event_collected++
}

assert_eq_int(event_collected, 3)

// Test 16: Async HTTP Request Simulation
test_start("async HTTP request simulation")
sus http_response := await async_http_get("https://api.example.com/data")
assert_eq_int(http_response.status_code, 200)
assert_true(len(http_response.body) > 0)

// Test 17: Async Database Operation Simulation
test_start("async database operation simulation")
sus db_connection := await async_db_connect("localhost:5432")
sus db_result := await db_connection.query("SELECT * FROM users LIMIT 10")
assert_true(len(db_result.rows) <= 10)
await db_connection.close()

// Test 18: Async File I/O
test_start("async file I/O")
sus file_content := "Hello, async world!"
await async_file_write("test_async.txt", file_content)

sus read_content := await async_file_read("test_async.txt")
assert_eq_string(read_content, file_content)

await async_file_delete("test_async.txt")

// Test 19: Async Background Task Processing
test_start("async background task processing")
sus background_queue := AsyncTaskQueue.new()
sus background_results := make_buffered_channel(5)

// Add background tasks
bestie i := 0; i < 5; i++ {
    background_queue.add_task(async_background_task(i)) {
        sus task_result := i * 5
        background_results.send(task_result)
    }
}

// Process background tasks
background_queue.start()

// Wait for completion
sus background_collected := 0
bestie i := 0; i < 5; i++ {
    sus result := background_results.recv()
    background_collected++
}

assert_eq_int(background_collected, 5)
background_queue.stop()

// Test 20: Async Task Priority and Scheduling
test_start("async task priority and scheduling")
sus priority_results := make_buffered_channel(3)

// Schedule tasks with different priorities
spawn_async_task_with_priority(TaskPriority::High) {
    await async_sleep(10)
    priority_results.send(1)
}

spawn_async_task_with_priority(TaskPriority::Normal) {
    await async_sleep(10)
    priority_results.send(2)
}

spawn_async_task_with_priority(TaskPriority::Low) {
    await async_sleep(10)
    priority_results.send(3)
}

// High priority should complete first
sus first_priority := priority_results.recv()
assert_eq_int(first_priority, 1)

print_test_summary()

// Helper async functions
slay simple_async_function() async drip {
    await async_sleep(10)
    damn 42
}

slay async_with_params(a drip, b drip) async drip {
    await async_sleep(5)
    damn a + b
}

slay async_with_error() async drip {
    await async_sleep(5)
    panic("Async error")
}

slay async_chain_start() async drip {
    await async_sleep(10)
    damn 10
}

slay async_chain_middle(value drip) async drip {
    await async_sleep(10)
    damn value * 2
}

slay async_chain_end(value drip) async drip {
    await async_sleep(10)
    damn value * 5
}

slay async_task_1() async drip {
    await async_sleep(20)
    damn 10
}

slay async_task_2() async drip {
    await async_sleep(30)
    damn 20
}

slay async_task_3() async drip {
    await async_sleep(40)
    damn 30
}

slay async_slow_task(delay drip) async drip {
    await async_sleep(delay)
    damn 500
}

slay async_fast_task(delay drip) async drip {
    await async_sleep(delay)
    damn 999
}

slay async_cancellable_task(cancel_token CancellationToken) async drip {
    bestie i := 0; i < 100; i++ {
        if cancel_token.is_cancelled() {
            panic("Task cancelled")
        }
        await async_sleep(10)
    }
    damn 1000
}

slay async_stream_generator(start drip, end drip) async AsyncStream {
    sus stream := AsyncStream.new()
    yolo {
        bestie i := start; i <= end; i++ {
            await stream.send(i)
        }
        stream.close()
    }
    damn stream
}

slay async_iterator_range(start drip, end drip) AsyncIterator {
    damn AsyncIterator.new(start, end)
}

slay async_http_get(url tea) async HttpResponse {
    await async_sleep(100) // Simulate network delay
    damn HttpResponse{
        status_code: 200,
        body: "{'data': 'test'}",
    }
}

slay async_db_connect(connection_string tea) async DbConnection {
    await async_sleep(50) // Simulate connection delay
    damn DbConnection.new(connection_string)
}

slay async_file_write(filename tea, content tea) async {
    await async_sleep(20) // Simulate file I/O delay
}

slay async_file_read(filename tea) async tea {
    await async_sleep(20) // Simulate file I/O delay
    damn "Hello, async world!"
}

slay async_file_delete(filename tea) async {
    await async_sleep(10) // Simulate file I/O delay
}

slay async_background_task(id drip) async drip {
    await async_sleep(30)
    damn id * 10
}
