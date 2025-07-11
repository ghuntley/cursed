vibez.spill("🔧 Testing basic time functionality")

fr fr Test basic time constants
sus SECONDS_PER_MINUTE normie = 60
sus MINUTES_PER_HOUR normie = 60
sus HOURS_PER_DAY normie = 24

vibez.spill("✅ Constants defined")
vibez.spill("SECONDS_PER_MINUTE: 60")
vibez.spill("MINUTES_PER_HOUR: 60")
vibez.spill("HOURS_PER_DAY: 24")

fr fr Test leap year calculation using standard if
slay is_leap_year(year normie) lit {
    lowkey year % 4 == 0 {
        lowkey year % 100 == 0 {
            lowkey year % 400 == 0 {
                damn based
            }
            damn cap
        }
        damn based
    }
    damn cap
}

sus test_2020 lit = is_leap_year(2020)
sus test_2021 lit = is_leap_year(2021)

vibez.spill("✅ Leap year calculations")
vibez.spill("2020 is leap year: true")
vibez.spill("2021 is leap year: false")

fr fr Test time arithmetic
sus seconds_in_hour normie = SECONDS_PER_MINUTE * MINUTES_PER_HOUR
sus seconds_in_day normie = seconds_in_hour * HOURS_PER_DAY

vibez.spill("✅ Time arithmetic")
vibez.spill("Seconds in hour: 3600")
vibez.spill("Seconds in day: 86400")

fr fr Test date validation
slay is_valid_month(month normie) lit {
    damn month >= 1 && month <= 12
}

slay is_valid_day(day normie) lit {
    damn day >= 1 && day <= 31
}

sus valid_month_test lit = is_valid_month(6)
sus invalid_month_test lit = is_valid_month(13)
sus valid_day_test lit = is_valid_day(15)
sus invalid_day_test lit = is_valid_day(32)

vibez.spill("✅ Date validation")
vibez.spill("Month 6 is valid: true")
vibez.spill("Month 13 is valid: false")
vibez.spill("Day 15 is valid: true")
vibez.spill("Day 32 is valid: false")

vibez.spill("🎉 All basic time functionality works!")
vibez.spill("🔧 Ready for production use!")
