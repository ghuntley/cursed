yeet "testz"

fr fr ========================================
fr fr Simple Time Library Test
fr fr ========================================

fr fr Simple time constants
sus SECONDS_PER_MINUTE normie = 60
sus MINUTES_PER_HOUR normie = 60
sus HOURS_PER_DAY normie = 24

fr fr Simple DateTime structure
be_like DateTime squad {
    year normie
    month normie
    day normie
    hour normie
    minute normie
    second normie
}

fr fr Simple time functions
slay time_create(year normie, month normie, day normie, hour normie, minute normie, second normie) DateTime {
    sus dt DateTime
    dt.year = year
    dt.month = month
    dt.day = day
    dt.hour = hour
    dt.minute = minute
    dt.second = second
    damn dt
}

slay time_year(dt DateTime) normie {
    damn dt.year
}

slay time_month(dt DateTime) normie {
    damn dt.month
}

slay time_day(dt DateTime) normie {
    damn dt.day
}

slay time_hour(dt DateTime) normie {
    damn dt.hour
}

slay time_minute(dt DateTime) normie {
    damn dt.minute
}

slay time_second(dt DateTime) normie {
    damn dt.second
}

slay time_add_years(dt DateTime, years normie) DateTime {
    sus new_dt DateTime = dt
    new_dt.year = dt.year + years
    damn new_dt
}

slay time_add_months(dt DateTime, months normie) DateTime {
    sus new_dt DateTime = dt
    new_dt.month = dt.month + months
    
    yoink new_dt.month > 12 {
        new_dt.year = new_dt.year + 1
        new_dt.month = new_dt.month - 12
    }
    
    damn new_dt
}

slay time_add_days(dt DateTime, days normie) DateTime {
    sus new_dt DateTime = dt
    new_dt.day = dt.day + days
    
    yoink new_dt.day > 31 {
        new_dt.month = new_dt.month + 1
        new_dt.day = new_dt.day - 31
    }
    
    damn new_dt
}

slay time_is_leap_year(year normie) lit {
    yoink year % 4 == 0 {
        yoink year % 100 == 0 {
            yoink year % 400 == 0 {
                damn based
            }
            damn cap
        }
        damn based
    }
    damn cap
}

slay time_days_in_month(year normie, month normie) normie {
    yoink month == 2 {
        yoink time_is_leap_year(year) {
            damn 29
        }
        damn 28
    }
    
    yoink month == 4 || month == 6 || month == 9 || month == 11 {
        damn 30
    }
    
    damn 31
}

slay time_is_valid_date(year normie, month normie, day normie) lit {
    yoink year < 1970 || year > 2100 {
        damn cap
    }
    
    yoink month < 1 || month > 12 {
        damn cap
    }
    
    sus max_days normie = time_days_in_month(year, month)
    damn day >= 1 && day <= max_days
}

slay time_seconds_per_minute() normie {
    damn SECONDS_PER_MINUTE
}

slay time_minutes_per_hour() normie {
    damn MINUTES_PER_HOUR
}

slay time_hours_per_day() normie {
    damn HOURS_PER_DAY
}

fr fr ========================================
fr fr Test Functions
fr fr ========================================

slay test_time_creation() {
    test_start("Time Creation")
    
    sus dt DateTime = time_create(2021, 6, 15, 10, 30, 45)
    assert_eq_int(time_year(dt), 2021)
    assert_eq_int(time_month(dt), 6)
    assert_eq_int(time_day(dt), 15)
    assert_eq_int(time_hour(dt), 10)
    assert_eq_int(time_minute(dt), 30)
    assert_eq_int(time_second(dt), 45)
    
    vibez.spill("✅ Time creation works")
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
    
    vibez.spill("✅ Time arithmetic works")
}

slay test_time_validation() {
    test_start("Time Validation")
    
    assert_true(time_is_leap_year(2020))
    assert_false(time_is_leap_year(2021))
    assert_eq_int(time_days_in_month(2021, 2), 28)
    assert_eq_int(time_days_in_month(2020, 2), 29)
    assert_true(time_is_valid_date(2021, 6, 15))
    assert_false(time_is_valid_date(2021, 13, 1))
    
    vibez.spill("✅ Time validation works")
}

slay test_time_constants() {
    test_start("Time Constants")
    
    assert_eq_int(time_seconds_per_minute(), 60)
    assert_eq_int(time_minutes_per_hour(), 60)
    assert_eq_int(time_hours_per_day(), 24)
    
    vibez.spill("✅ Time constants work")
}

slay run_simple_time_tests() {
    vibez.spill("⏰ Running Simple Time Tests")
    vibez.spill("============================")
    
    test_time_creation()
    test_time_arithmetic()
    test_time_validation()
    test_time_constants()
    
    print_test_summary()
    vibez.spill("🎉 Simple time tests completed!")
}

run_simple_time_tests()
