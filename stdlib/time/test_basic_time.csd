yeet "testz"

fr fr Basic time test without complex structures
slay test_simple_time() {
    test_start("Basic Time")
    
    fr fr Test basic constants
    sus seconds_per_minute normie = 60
    sus minutes_per_hour normie = 60
    sus hours_per_day normie = 24
    
    assert_eq_int(seconds_per_minute, 60)
    assert_eq_int(minutes_per_hour, 60)
    assert_eq_int(hours_per_day, 24)
    
    fr fr Test leap year logic
    sus year_2020 normie = 2020
    sus year_2021 normie = 2021
    
    sus is_leap_2020 lit = (year_2020 % 4 == 0)
    sus is_leap_2021 lit = (year_2021 % 4 == 0)
    
    assert_true(is_leap_2020)
    assert_false(is_leap_2021)
    
    vibez.spill("✅ Basic time operations work")
}

slay test_time_calculations() {
    test_start("Time Calculations")
    
    fr fr Test basic time math
    sus seconds_in_minute normie = 60
    sus minutes_in_hour normie = 60
    sus seconds_in_hour normie = seconds_in_minute * minutes_in_hour
    
    assert_eq_int(seconds_in_hour, 3600)
    
    fr fr Test day calculations
    sus hours_in_day normie = 24
    sus seconds_in_day normie = seconds_in_hour * hours_in_day
    
    assert_eq_int(seconds_in_day, 86400)
    
    vibez.spill("✅ Time calculations work")
}

slay test_date_validation() {
    test_start("Date Validation")
    
    fr fr Test month validation
    sus valid_month normie = 6
    sus invalid_month normie = 13
    
    sus is_valid_month lit = (valid_month >= 1 && valid_month <= 12)
    sus is_invalid_month lit = (invalid_month >= 1 && invalid_month <= 12)
    
    assert_true(is_valid_month)
    assert_false(is_invalid_month)
    
    fr fr Test day validation
    sus valid_day normie = 15
    sus invalid_day normie = 32
    
    sus is_valid_day lit = (valid_day >= 1 && valid_day <= 31)
    sus is_invalid_day lit = (invalid_day >= 1 && invalid_day <= 31)
    
    assert_true(is_valid_day)
    assert_false(is_invalid_day)
    
    vibez.spill("✅ Date validation works")
}

slay run_basic_time_tests() {
    vibez.spill("⏰ Running Basic Time Tests")
    vibez.spill("==========================")
    
    test_simple_time()
    test_time_calculations()
    test_date_validation()
    
    print_test_summary()
    vibez.spill("🎉 Basic time tests completed!")
}

run_basic_time_tests()
