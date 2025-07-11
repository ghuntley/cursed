yeet "testz"

fr fr ========================================
fr fr CURSED Pure Time Library Test Suite
fr fr ========================================

slay test_time_creation() {
    test_start("Pure Time Creation")
    
    sus dt DateTime = time_create(2021, 6, 15, 10, 30, 45)
    assert_eq_int(time_year(dt), 2021)
    assert_eq_int(time_month(dt), 6)
    assert_eq_int(time_day(dt), 15)
    assert_eq_int(time_hour(dt), 10)
    assert_eq_int(time_minute(dt), 30)
    assert_eq_int(time_second(dt), 45)
    
    vibez.spill("✅ Time creation working")
}

slay test_time_formatting() {
    test_start("Time Formatting")
    
    sus dt DateTime = time_create(2021, 6, 15, 10, 30, 45)
    sus formatted tea = time_format(dt, "%Y-%m-%d %H:%M:%S")
    
    assert_true(string_contains(formatted, "2021"))
    assert_true(string_contains(formatted, "06"))
    assert_true(string_contains(formatted, "15"))
    
    vibez.spill("✅ Time formatting working")
}

slay test_time_arithmetic() {
    test_start("Time Arithmetic")
    
    sus base DateTime = time_create(2021, 6, 15, 10, 30, 45)
    sus plus_year DateTime = time_add_years(base, 1)
    sus plus_month DateTime = time_add_months(base, 1)
    sus plus_day DateTime = time_add_days(base, 1)
    
    assert_eq_int(time_year(plus_year), 2022)
    assert_eq_int(time_month(plus_month), 7)
    assert_eq_int(time_day(plus_day), 16)
    
    vibez.spill("✅ Time arithmetic working")
}

slay test_duration_operations() {
    test_start("Duration Operations")
    
    sus dur1 Duration = duration_from_seconds(3600)
    sus dur2 Duration = duration_from_seconds(1800)
    sus dur_sum Duration = duration_add(dur1, dur2)
    
    assert_eq_int(duration_to_seconds(dur_sum), 5400)
    
    vibez.spill("✅ Duration operations working")
}

slay test_time_validation() {
    test_start("Time Validation")
    
    assert_true(time_is_leap_year(2020))
    assert_false(time_is_leap_year(2021))
    assert_eq_int(time_days_in_month(2021, 2), 28)
    assert_eq_int(time_days_in_month(2020, 2), 29)
    assert_true(time_is_valid_date(2021, 6, 15))
    assert_false(time_is_valid_date(2021, 13, 1))
    
    vibez.spill("✅ Time validation working")
}

slay test_time_constants() {
    test_start("Time Constants")
    
    assert_eq_int(time_seconds_per_minute(), 60)
    assert_eq_int(time_minutes_per_hour(), 60)
    assert_eq_int(time_hours_per_day(), 24)
    assert_eq_int(time_days_per_week(), 7)
    assert_eq_int(time_months_per_year(), 12)
    
    vibez.spill("✅ Time constants working")
}

slay test_benchmarking() {
    test_start("Benchmarking")
    
    slay test_function() {
        sus sum normie = 0
        bestie i := 0; i < 100; i++ {
            sum = sum + i
        }
        damn sum
    }
    
    sus benchmark_result Duration = time_benchmark(test_function)
    assert_true(duration_to_seconds(benchmark_result) >= 0)
    
    vibez.spill("✅ Benchmarking working")
}

slay run_all_pure_time_tests() {
    vibez.spill("⏰ Running Pure CURSED Time Tests")
    vibez.spill("================================")
    
    test_time_creation()
    test_time_formatting()
    test_time_arithmetic()
    test_duration_operations()
    test_time_validation()
    test_time_constants()
    test_benchmarking()
    
    print_test_summary()
    vibez.spill("🎉 All pure time tests completed!")
}

run_all_pure_time_tests()
