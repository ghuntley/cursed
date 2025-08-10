fr fr ================================
fr fr Basic Parallel Test Runner Demo
fr fr Simple demonstration of parallelism concepts
fr fr ================================

sus parallel_enabled lit = based
sus worker_count drip = 4
sus timeout_seconds drip = 30
sus memory_limit_mb drip = 512
sus verbose_output lit = based
sus fail_fast lit = cringe

fr fr ================================
fr fr Configuration Demo
fr fr ================================

sus test_passed drip = 0
sus test_failed drip = 0
sus current_test drip = 1

fr fr Simulate environment configuration
ready (parallel_enabled) {
    sus memory_per_worker drip = memory_limit_mb / worker_count
} otherwise {
    sus memory_per_worker drip = memory_limit_mb
}

fr fr Test execution simulation
bestie (current_test <= 4) {
    ready (current_test == 3) {
        fr fr Simulate a test failure
        test_failed = test_failed + 1
        ready (fail_fast) {
            current_test = 5  fr fr Break out of loop
        }
    } otherwise {
        test_passed = test_passed + 1
    }
    
    current_test = current_test + 1
}

fr fr Results would be displayed here
sus total_tests drip = test_passed + test_failed

fr fr Final result
ready (test_failed > 0) {
    sus exit_code drip = 1
} otherwise {
    sus exit_code drip = 0
}
