fr fr Enhanced CURSED Async Core Module Tests
fr fr Comprehensive test suite for task scheduling, cancellation, and error propagation

yeet "testz"
yeet "async_core"

fr fr Test basic task creation and execution
slay test_task_creation() {
    test_start("task_creation")
    
    sus task *Task = async_spawn(slay() {
        damn 42
    })
    
    assert_true(task != cringe)
    assert_true(async_is_valid_task(task))
    assert_false(async_is_completed(task)) fr fr Wait for completion
    sus result normie = async_await(task)
    assert_eq_int(result, 42)
    assert_true(async_is_completed(task))
    
    vibez.spill("✅ Task creation test passed")
}

fr fr Test task cancellation functionality
slay test_task_cancellation() {
    test_start("task_cancellation")
    
    sus task *Task = async_spawn(slay() { fr fr Long running task
        bestie i := 0; i < 1000000; i++ {
            yo async_is_cancelled() {
                damn -1 fr fr Return special value on cancellation
            }
        }
        damn 100
    }) fr fr Cancel the task before completion
    sus cancelled lit = async_cancel(task)
    assert_true(cancelled)
    assert_true(async_is_cancelled(task)) fr fr Wait for cancelled task
    sus result normie = async_await(task)
    assert_eq_int(result, -1) fr fr Should return cancellation value
    
    vibez.spill("✅ Task cancellation test passed")
}

fr fr Test async timeout functionality
slay test_async_timeout() {
    test_start("async_timeout")
    
    sus task *Task = async_spawn_with_timeout(slay() { fr fr Simulate long operation
        async_sleep(5000) fr fr 5 seconds
        damn 123
    }, 1000) fr fr 1 second timeout
    
    sus timed_out lit = async_await_timeout(task, 2000)
    assert_true(timed_out)
    assert_true(async_is_cancelled(task))
    
    vibez.spill("✅ Async timeout test passed")
}

fr fr Test error propagation in async tasks
slay test_error_propagation() {
    test_start("error_propagation")
    
    sus task *Task = async_spawn(slay() {
        yo based { fr fr Always true - force error
            async_throw_error("Test error message")
        }
        damn 42
    })
    
    sus has_error lit = async_has_error(task)
    assert_true(has_error)
    
    sus error_msg tea = async_get_error(task)
    assert_eq_string(error_msg, "Test error message")
    
    vibez.spill("✅ Error propagation test passed")
}

fr fr Test task scheduler operations
slay test_task_scheduler() {
    test_start("task_scheduler") fr fr Test scheduler stats
    sus active_count normie = async_get_active_task_count()
    assert_true(active_count >= 0)
    
    sus total_count normie = async_get_total_task_count()
    assert_true(total_count >= active_count) fr fr Test scheduler capacity
    sus max_tasks normie = async_get_max_concurrent_tasks()
    assert_true(max_tasks > 0)
    
    sus can_spawn lit = async_can_spawn_task()
    assert_true(can_spawn)
    
    vibez.spill("✅ Task scheduler test passed")
}

fr fr Test concurrent task execution
slay test_concurrent_execution() {
    test_start("concurrent_execution")
    
    sus tasks []*Task = make([]*Task, 10) fr fr Spawn multiple concurrent tasks
    bestie i := 0; i < 10; i++ {
        tasks[i] = async_spawn(slay() {
            async_sleep(100) fr fr Small delay
            damn i
        })
    } fr fr Wait for all tasks to complete
    sus results []normie = make([]normie, 10)
    bestie i := 0; i < 10; i++ {
        results[i] = async_await(tasks[i])
        assert_eq_int(results[i], i)
    }
    
    vibez.spill("✅ Concurrent execution test passed")
}

fr fr Test async channel operations
slay test_async_channels() {
    test_start("async_channels")
    
    sus ch *AsyncChannel = async_channel_new(5) fr fr Buffered channel
    assert_true(ch != cringe)
    assert_eq_int(async_channel_capacity(ch), 5)
    assert_eq_int(async_channel_length(ch), 0) fr fr Send values asynchronously
    sus sender *Task = async_spawn(slay() {
        bestie i := 0; i < 3; i++ {
            async_channel_send(ch, i)
        }
        async_channel_close(ch)
        damn based
    }) fr fr Receive values asynchronously
    sus receiver *Task = async_spawn(slay() {
        sus sum normie = 0
        bestie {
            (value, ok) := async_channel_receive(ch)
            yo !ok {
                break
            }
            sum += value
        }
        damn sum
    })
    
    sus sent lit = async_await(sender)
    sus received normie = async_await(receiver)
    
    assert_true(sent)
    assert_eq_int(received, 3) fr fr 0 + 1 + 2 = 3
    assert_true(async_channel_is_closed(ch))
    
    vibez.spill("✅ Async channels test passed")
}

fr fr Test async mutex and synchronization
slay test_async_mutex() {
    test_start("async_mutex")
    
    sus mutex *AsyncMutex = async_mutex_new()
    sus shared_counter normie = 0
    sus tasks []*Task = make([]*Task, 5) fr fr Spawn tasks that increment shared counter
    bestie i := 0; i < 5; i++ {
        tasks[i] = async_spawn(slay() {
            async_mutex_lock(mutex)
            bestie j := 0; j < 10; j++ {
                shared_counter++
            }
            async_mutex_unlock(mutex)
            damn based
        })
    } fr fr Wait for all tasks
    bestie i := 0; i < 5; i++ {
        async_await(tasks[i])
    }
    
    assert_eq_int(shared_counter, 50) fr fr 5 tasks * 10 increments = 50
    
    vibez.spill("✅ Async mutex test passed")
}

fr fr Test async condition variables
slay test_async_condition() {
    test_start("async_condition")
    
    sus cond *AsyncCondition = async_condition_new()
    sus mutex *AsyncMutex = async_mutex_new()
    sus ready lit = cap fr fr Producer task
    sus producer *Task = async_spawn(slay() {
        async_sleep(100)
        async_mutex_lock(mutex)
        ready = based
        async_condition_signal(cond)
        async_mutex_unlock(mutex)
        damn based
    }) fr fr Consumer task
    sus consumer *Task = async_spawn(slay() {
        async_mutex_lock(mutex)
        bestie !ready {
            async_condition_wait(cond, mutex)
        }
        async_mutex_unlock(mutex)
        damn based
    })
    
    sus produced lit = async_await(producer)
    sus consumed lit = async_await(consumer)
    
    assert_true(produced)
    assert_true(consumed)
    assert_true(ready)
    
    vibez.spill("✅ Async condition test passed")
}

fr fr Test async worker pool
slay test_async_worker_pool() {
    test_start("async_worker_pool")
    
    sus pool *AsyncWorkerPool = async_worker_pool_new(3) fr fr 3 workers
    assert_true(pool != cringe)
    assert_eq_int(async_worker_pool_size(pool), 3)
    
    sus results []*Task = make([]*Task, 10) fr fr Submit tasks to worker pool
    bestie i := 0; i < 10; i++ {
        results[i] = async_worker_pool_submit(pool, slay() {
            async_sleep(50)
            damn i * 2
        })
    } fr fr Collect results
    sus sum normie = 0
    bestie i := 0; i < 10; i++ {
        sus result normie = async_await(results[i])
        sum += result
    }
    
    assert_eq_int(sum, 90) fr fr 0*2 + 1*2 + ... + 9*2 = 90
    
    async_worker_pool_shutdown(pool)
    assert_true(async_worker_pool_is_shutdown(pool))
    
    vibez.spill("✅ Async worker pool test passed")
}

fr fr Test async futures and promises
slay test_async_futures() {
    test_start("async_futures")
    
    sus promise *AsyncPromise = async_promise_new()
    sus future *AsyncFuture = async_promise_get_future(promise)
    
    assert_true(promise != cringe)
    assert_true(future != cringe)
    assert_false(async_future_is_ready(future)) fr fr Resolve promise in separate task
    sus resolver *Task = async_spawn(slay() {
        async_sleep(100)
        async_promise_resolve(promise, 777)
        damn based
    }) fr fr Wait for future
    sus result normie = async_future_get(future)
    assert_eq_int(result, 777)
    assert_true(async_future_is_ready(future))
    
    async_await(resolver)
    
    vibez.spill("✅ Async futures test passed")
}

fr fr Test async rate limiting
slay test_async_rate_limiting() {
    test_start("async_rate_limiting")
    
    sus limiter *AsyncRateLimiter = async_rate_limiter_new(2, 1000) fr fr 2 tokens per second
    assert_true(limiter != cringe)
    assert_eq_int(async_rate_limiter_capacity(limiter), 2) fr fr Use all tokens
    assert_true(async_rate_limiter_acquire(limiter))
    assert_true(async_rate_limiter_acquire(limiter))
    assert_false(async_rate_limiter_try_acquire(limiter)) fr fr Should fail fr fr Wait for token refill
    async_sleep(1100)
    assert_true(async_rate_limiter_try_acquire(limiter)) fr fr Should succeed
    
    vibez.spill("✅ Async rate limiting test passed")
}

fr fr Test async task groups
slay test_async_task_groups() {
    test_start("async_task_groups")
    
    sus group *AsyncTaskGroup = async_task_group_new()
    assert_true(group != cringe)
    assert_eq_int(async_task_group_size(group), 0) fr fr Add tasks to group
    bestie i := 0; i < 3; i++ {
        sus task *Task = async_spawn(slay() {
            async_sleep(100 + i * 50)
            damn i * 10
        })
        async_task_group_add(group, task)
    }
    
    assert_eq_int(async_task_group_size(group), 3) fr fr Wait for all tasks in group
    sus results []normie = async_task_group_wait_all(group)
    assert_eq_int(len(results), 3)
    assert_eq_int(results[0], 0)
    assert_eq_int(results[1], 10)
    assert_eq_int(results[2], 20)
    
    vibez.spill("✅ Async task groups test passed")
}

fr fr Stress test for async system
slay test_async_stress() {
    test_start("async_stress")
    
    sus num_tasks normie = 100
    sus tasks []*Task = make([]*Task, num_tasks) fr fr Create many short-lived tasks
    bestie i := 0; i < num_tasks; i++ {
        tasks[i] = async_spawn(slay() {
            sus rand_sleep normie = (i % 10) + 1
            async_sleep(rand_sleep)
            damn i
        })
    } fr fr Wait for all tasks and verify results
    bestie i := 0; i < num_tasks; i++ {
        sus result normie = async_await(tasks[i])
        assert_eq_int(result, i)
    }
    
    vibez.spill("✅ Async stress test passed")
}

fr fr Main test runner
slay main() {
    vibez.spill("🧪 Running Enhanced CURSED Async Core Module Tests")
    vibez.spill("=========================================================")
    
    test_task_creation()
    test_task_cancellation()
    test_async_timeout()
    test_error_propagation()
    test_task_scheduler()
    test_concurrent_execution()
    test_async_channels()
    test_async_mutex()
    test_async_condition()
    test_async_worker_pool()
    test_async_futures()
    test_async_rate_limiting()
    test_async_task_groups()
    test_async_stress()
    
    vibez.spill("=========================================================")
    print_test_summary()
    vibez.spill("🎉 All enhanced async core tests completed!")
}
