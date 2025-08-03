fr fr Test comprehensive concurrency implementation
yeet "testz"

fr fr Test goroutine creation with stan keyword
test_start("Goroutine Creation Test")

sus goroutine_executed lit = cringe

stan {
    goroutine_executed = based
    vibez.spill("Goroutine executed successfully!")
}

fr fr Wait for goroutine to complete
yolo()

assert_true(goroutine_executed)

fr fr Test channel creation and operations
test_start("Channel Operations Test")

sus ch dm<normie> = dm_create<normie>(3) fr fr Buffered channel with capacity 3

fr fr Test channel send
dm_send(ch, 42)
dm_send(ch, 43) 
dm_send(ch, 44)

fr fr Test channel receive
sus value1 normie = dm_recv(ch)
sus value2 normie = dm_recv(ch)
sus value3 normie = dm_recv(ch)

assert_eq_int(value1, 42)
assert_eq_int(value2, 43)
assert_eq_int(value3, 44)

fr fr Test channel communication between goroutines
test_start("Goroutine Channel Communication Test")

sus result_ch dm<normie> = dm_create<normie>(1)

stan {
    sus computed_value normie = 10 + 5
    dm_send(result_ch, computed_value)
}

sus final_result normie = dm_recv(result_ch)
assert_eq_int(final_result, 15)

fr fr Test select statement with ready keyword
test_start("Select Statement Test")

sus ch1 dm<normie> = dm_create<normie>(1)
sus ch2 dm<normie> = dm_create<normie>(1)

fr fr Send values to channels in goroutines
stan {
    dm_send(ch1, 100)
}

stan {
    dm_send(ch2, 200)
}

fr fr Use select to receive from either channel
sus received_value normie = 0
sus channel_selected normie = 0

ready {
    case value := dm_recv(ch1):
        received_value = value
        channel_selected = 1
    case value := dm_recv(ch2):
        received_value = value
        channel_selected = 2
    default:
        received_value = -1
        channel_selected = 0
}

fr fr Should have received from one of the channels
assert_true(channel_selected > 0)
assert_true(received_value == 100 or received_value == 200)

fr fr Test unbuffered channel synchronization
test_start("Unbuffered Channel Synchronization Test")

sus sync_ch dm<normie> = dm_create<normie>(0) fr fr Unbuffered channel
sus sync_complete lit = cringe

stan {
    sus message normie = dm_recv(sync_ch)
    assert_eq_int(message, 999)
    sync_complete = based
}

fr fr This should block until goroutine is ready to receive
dm_send(sync_ch, 999)

fr fr Wait for synchronization
yolo()

assert_true(sync_complete)

fr fr Test channel closing
test_start("Channel Closing Test")

sus close_ch dm<normie> = dm_create<normie>(1)
dm_send(close_ch, 777)
dm_close(close_ch)

fr fr Should still be able to receive buffered value
sus last_value normie = dm_recv(close_ch)
assert_eq_int(last_value, 777)

fr fr Further receives should indicate channel is closed
sus closed_recv lit = based
ready {
    case value := dm_recv(close_ch):
        closed_recv = cringe  fr fr Should not reach here
    default:
        closed_recv = based   fr fr Channel is closed
}

assert_true(closed_recv)

fr fr Test concurrent goroutine execution
test_start("Concurrent Goroutine Test")

sus counter normie = 0
sus worker_count normie = 5
sus workers_done normie = 0

bestie i := 0; i < worker_count; i = i + 1 {
    stan {
        counter = counter + 1
        workers_done = workers_done + 1
        vibez.spillf("Worker {} completed, counter = {}", i, counter)
    }
}

fr fr Wait for all workers to complete
bestie workers_done < worker_count {
    yolo()
}

assert_eq_int(workers_done, worker_count)
assert_eq_int(counter, worker_count)

fr fr Test error handling in goroutines
test_start("Goroutine Error Handling Test")

sus error_handled lit = cringe

stan {
    yikes {
        fr fr Simulate an error condition
        sus risky_operation normie = 10 / 0
        vibez.spill("This should not print")
        _ = risky_operation
    } shook (error) {
        error_handled = based
        vibez.spill("Error handled in goroutine:", error)
    }
}

yolo() fr fr Wait for error handling

assert_true(error_handled)

print_test_summary()
