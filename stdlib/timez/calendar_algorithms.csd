fr fr CURSED Calendar Algorithms - Proper Calendar Computation Implementation
fr fr Accurate date/time extraction with calendar systems and astronomical calculations

yeet "mathz"
yeet "vibez"

fr fr ===== CALENDAR SYSTEM STRUCTURES =====

squad CalendarDate {
    sus year drip
    sus month drip
    sus day drip
    sus day_of_week drip
    sus day_of_year drip
    sus week_of_year drip
    sus is_leap_year lit
    sus calendar_system tea
}

squad AstronomicalData {
    sus julian_day_number drip
    sus modified_julian_day drip
    sus equation_of_time_seconds drip
    sus solar_declination_degrees drip
    sus sunrise_timestamp drip
    sus sunset_timestamp drip
    sus solar_noon_timestamp drip
    sus day_length_seconds drip
}

squad CalendarSystem {
    sus name tea
    sus epoch_year drip
    sus epoch_month drip
    sus epoch_day drip
    sus days_per_year drip
    sus leap_year_cycle drip
    sus months_per_year drip
}

fr fr ===== CALENDAR SYSTEM DEFINITIONS =====

facts GREGORIAN_EPOCH_YEAR drip = 1
facts GREGORIAN_EPOCH_MONTH drip = 1  
facts GREGORIAN_EPOCH_DAY drip = 1
facts JULIAN_EPOCH_YEAR drip = -4712
facts ISLAMIC_EPOCH_YEAR drip = 622
facts HEBREW_EPOCH_YEAR drip = -3760

facts DAYS_PER_GREGORIAN_YEAR drip = 365
facts DAYS_PER_JULIAN_YEAR drip = 365
facts DAYS_PER_ISLAMIC_YEAR drip = 354
facts DAYS_PER_HEBREW_YEAR drip = 353

facts JULIAN_DAY_UNIX_EPOCH drip = 2440588  fr fr January 1, 1970

sus gregorian_calendar CalendarSystem = CalendarSystem{
    name: "Gregorian",
    epoch_year: GREGORIAN_EPOCH_YEAR,
    epoch_month: GREGORIAN_EPOCH_MONTH,
    epoch_day: GREGORIAN_EPOCH_DAY,
    days_per_year: DAYS_PER_GREGORIAN_YEAR,
    leap_year_cycle: 400,
    months_per_year: 12
}

sus julian_calendar CalendarSystem = CalendarSystem{
    name: "Julian", 
    epoch_year: JULIAN_EPOCH_YEAR,
    epoch_month: 1,
    epoch_day: 1,
    days_per_year: DAYS_PER_JULIAN_YEAR,
    leap_year_cycle: 4,
    months_per_year: 12
}

fr fr ===== GREGORIAN CALENDAR ALGORITHMS =====

slay gregorian_date_from_timestamp(timestamp drip) CalendarDate {
    fr fr Convert Unix timestamp to Gregorian calendar date
    
    sus days_since_epoch drip = timestamp / 86400
    sus julian_day drip = JULIAN_DAY_UNIX_EPOCH + days_since_epoch
    
    damn julian_day_to_gregorian(julian_day)
}

slay julian_day_to_gregorian(julian_day drip) CalendarDate {
    fr fr Convert Julian Day Number to Gregorian calendar date
    fr fr Uses Jean Meeus algorithm from "Astronomical Algorithms"
    
    sus a drip = julian_day + 32044
    sus b drip = (4 * a + 3) / 146097
    sus c drip = a - (146097 * b) / 4
    
    sus d drip = (4 * c + 3) / 1461
    sus e drip = c - (1461 * d) / 4
    sus m drip = (5 * e + 2) / 153
    
    sus day drip = e - (153 * m + 2) / 5 + 1
    sus month drip = m + 3 - 12 * (m / 10)
    sus year drip = 100 * b + d - 4800 + m / 10
    
    sus result CalendarDate = CalendarDate{
        year: year,
        month: month,
        day: day,
        day_of_week: julian_day_to_weekday(julian_day),
        day_of_year: calculate_day_of_year(year, month, day),
        week_of_year: calculate_week_of_year(year, month, day),
        is_leap_year: is_gregorian_leap_year(year),
        calendar_system: "Gregorian"
    }
    
    damn result
}

slay gregorian_to_julian_day(year drip, month drip, day drip) drip {
    fr fr Convert Gregorian date to Julian Day Number
    fr fr Uses standard astronomical formula
    
    sus a drip = (14 - month) / 12
    sus y drip = year + 4800 - a
    sus m drip = month + 12 * a - 3
    
    sus julian_day drip = day + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
    
    damn julian_day
}

slay timestamp_from_gregorian_date(year drip, month drip, day drip, hour drip, minute drip, second drip) drip {
    fr fr Convert Gregorian date/time to Unix timestamp
    
    sus julian_day drip = gregorian_to_julian_day(year, month, day)
    sus days_since_unix_epoch drip = julian_day - JULIAN_DAY_UNIX_EPOCH
    
    sus seconds_from_days drip = days_since_unix_epoch * 86400
    sus seconds_from_time drip = hour * 3600 + minute * 60 + second
    
    damn seconds_from_days + seconds_from_time
}

fr fr ===== LEAP YEAR CALCULATIONS =====

slay is_gregorian_leap_year(year drip) lit {
    fr fr Gregorian leap year rule: divisible by 4, except centuries unless divisible by 400
    ready (year % 400 == 0) { damn based }
    ready (year % 100 == 0) { damn cringe }
    ready (year % 4 == 0) { damn based }
    damn cringe
}

slay is_julian_leap_year(year drip) lit {
    fr fr Julian leap year rule: every 4 years
    damn year % 4 == 0
}

slay leap_years_since_epoch(year drip) drip {
    fr fr Count leap years since Gregorian epoch (year 1)
    ready (year <= 1) { damn 0 }
    
    sus years_minus_one drip = year - 1
    sus leap_count drip = years_minus_one / 4 - years_minus_one / 100 + years_minus_one / 400
    
    damn leap_count
}

slay days_in_gregorian_month(year drip, month drip) drip {
    fr fr Get number of days in Gregorian month
    ready (month == 1 || month == 3 || month == 5 || month == 7 || 
           month == 8 || month == 10 || month == 12) {
        damn 31
    } otherwise ready (month == 4 || month == 6 || month == 9 || month == 11) {
        damn 30
    } otherwise ready (month == 2) {
        ready (is_gregorian_leap_year(year)) {
            damn 29
        }
        damn 28
    }
    damn 30
}

fr fr ===== WEEKDAY CALCULATIONS =====

slay julian_day_to_weekday(julian_day drip) drip {
    fr fr Convert Julian Day Number to weekday (0=Sunday, 1=Monday, ..., 6=Saturday)
    damn (julian_day + 1) % 7
}

slay calculate_weekday(year drip, month drip, day drip) drip {
    fr fr Calculate weekday using Zeller's congruence
    sus adjusted_month drip = month
    sus adjusted_year drip = year
    
    ready (month <= 2) {
        adjusted_month = adjusted_month + 12
        adjusted_year = adjusted_year - 1
    }
    
    sus century drip = adjusted_year / 100
    sus year_of_century drip = adjusted_year % 100
    
    sus weekday drip = (day + ((13 * (adjusted_month + 1)) / 5) + year_of_century + 
                       (year_of_century / 4) + (century / 4) - 2 * century) % 7
    
    fr fr Convert to standard Sunday=0 format
    damn (weekday + 7) % 7
}

slay calculate_weekday_gauss(year drip, month drip, day drip) drip {
    fr fr Calculate weekday using Gauss's algorithm
    sus m drip = (month - 3 + 12) % 12
    sus y drip = ready (month <= 2) { year - 1 } otherwise { year }
    
    sus c drip = y / 100
    sus g drip = y % 100
    
    sus weekday drip = (day + (13 * m - 1) / 5 + g + g / 4 + c / 4 - 2 * c) % 7
    damn (weekday + 7) % 7
}

fr fr ===== DAY OF YEAR CALCULATIONS =====

slay calculate_day_of_year(year drip, month drip, day drip) drip {
    fr fr Calculate day of year (1-366)
    
    sus day_of_year drip = day
    sus m drip = 1
    
    bestie (m < month) {
        day_of_year = day_of_year + days_in_gregorian_month(year, m)
        m = m + 1
    }
    
    damn day_of_year
}

slay date_from_day_of_year(year drip, day_of_year drip) CalendarDate {
    fr fr Convert year and day of year to calendar date
    
    sus remaining_days drip = day_of_year
    sus month drip = 1
    
    bestie (month <= 12 && remaining_days > 0) {
        sus days_this_month drip = days_in_gregorian_month(year, month)
        ready (remaining_days <= days_this_month) {
            break
        }
        remaining_days = remaining_days - days_this_month
        month = month + 1
    }
    
    sus day drip = remaining_days
    
    sus result CalendarDate = CalendarDate{
        year: year,
        month: month,
        day: day,
        day_of_week: calculate_weekday(year, month, day),
        day_of_year: day_of_year,
        week_of_year: calculate_week_of_year(year, month, day),
        is_leap_year: is_gregorian_leap_year(year),
        calendar_system: "Gregorian"
    }
    
    damn result
}

fr fr ===== WEEK CALCULATIONS =====

slay calculate_week_of_year(year drip, month drip, day drip) drip {
    fr fr Calculate ISO 8601 week number (1-53)
    
    sus day_of_year drip = calculate_day_of_year(year, month, day)
    sus jan1_weekday drip = calculate_weekday(year, 1, 1)
    
    fr fr ISO 8601 week starts on Monday (1)
    sus jan1_iso_weekday drip = ready (jan1_weekday == 0) { 7 } otherwise { jan1_weekday }
    
    sus days_to_first_monday drip = ready (jan1_iso_weekday <= 4) { 1 - jan1_iso_weekday } otherwise { 8 - jan1_iso_weekday }
    sus adjusted_day_of_year drip = day_of_year + days_to_first_monday - 1
    
    ready (adjusted_day_of_year < 0) {
        fr fr This date belongs to last week of previous year
        damn calculate_weeks_in_year(year - 1)
    }
    
    sus week_number drip = (adjusted_day_of_year / 7) + 1
    sus max_weeks_this_year drip = calculate_weeks_in_year(year)
    
    ready (week_number > max_weeks_this_year) {
        damn 1  fr fr This date belongs to first week of next year
    }
    
    damn week_number
}

slay calculate_weeks_in_year(year drip) drip {
    fr fr Calculate number of ISO weeks in year (52 or 53)
    sus jan1_weekday drip = calculate_weekday(year, 1, 1)
    sus is_leap lit = is_gregorian_leap_year(year)
    
    ready (jan1_weekday == 4 || (is_leap && jan1_weekday == 3)) {
        damn 53
    }
    damn 52
}

slay find_nth_weekday_in_month(year drip, month drip, weekday drip, occurrence drip) drip {
    fr fr Find nth occurrence of weekday in month (e.g., 2nd Sunday)
    
    sus first_day_weekday drip = calculate_weekday(year, month, 1)
    sus days_to_first_occurrence drip = (weekday - first_day_weekday + 7) % 7
    sus first_occurrence_day drip = 1 + days_to_first_occurrence
    
    sus target_day drip = first_occurrence_day + ((occurrence - 1) * 7)
    sus max_day drip = days_in_gregorian_month(year, month)
    
    ready (target_day > max_day) {
        damn 0  fr fr No such occurrence in this month
    }
    
    damn target_day
}

slay find_last_weekday_in_month(year drip, month drip, weekday drip) drip {
    fr fr Find last occurrence of weekday in month
    
    sus last_day drip = days_in_gregorian_month(year, month)
    sus last_day_weekday drip = calculate_weekday(year, month, last_day)
    
    sus days_back drip = (last_day_weekday - weekday + 7) % 7
    sus target_day drip = last_day - days_back
    
    damn target_day
}

fr fr ===== ASTRONOMICAL CALCULATIONS =====

slay calculate_astronomical_data(year drip, month drip, day drip, longitude drip, latitude drip) AstronomicalData {
    fr fr Calculate astronomical data for given date and location
    
    sus julian_day drip = gregorian_to_julian_day(year, month, day)
    sus mjd drip = julian_day - 2400000  fr fr Modified Julian Day
    
    sus result AstronomicalData = AstronomicalData{
        julian_day_number: julian_day,
        modified_julian_day: mjd,
        equation_of_time_seconds: calculate_equation_of_time(julian_day),
        solar_declination_degrees: calculate_solar_declination(julian_day),
        sunrise_timestamp: 0,
        sunset_timestamp: 0,
        solar_noon_timestamp: 0,
        day_length_seconds: 0
    }
    
    fr fr Calculate sunrise/sunset times
    sus solar_times SolarTimes = calculate_sunrise_sunset(julian_day, longitude, latitude)
    result.sunrise_timestamp = solar_times.sunrise
    result.sunset_timestamp = solar_times.sunset
    result.solar_noon_timestamp = solar_times.solar_noon
    result.day_length_seconds = solar_times.day_length
    
    damn result
}

squad SolarTimes {
    sus sunrise drip
    sus sunset drip
    sus solar_noon drip
    sus day_length drip
}

slay calculate_sunrise_sunset(julian_day drip, longitude drip, latitude drip) SolarTimes {
    fr fr Calculate sunrise and sunset times using standard astronomical formulas
    
    sus n drip = julian_day - 2451545 + 0.0008
    sus l drip = (280.460 + 0.9856474 * n) % 360
    sus g drip = to_radians((357.528 + 0.9856003 * n) % 360)
    sus lambda drip = to_radians(l + 1.915 * sin_degrees(g * 180 / 3.14159) + 0.020 * sin_degrees(2 * g * 180 / 3.14159))
    
    sus alpha drip = atan2_degrees(cos_degrees(23.439) * sin_degrees(lambda * 180 / 3.14159), cos_degrees(lambda * 180 / 3.14159))
    sus delta drip = asin_degrees(sin_degrees(23.439) * sin_degrees(lambda * 180 / 3.14159))
    
    sus hour_angle drip = acos_degrees(-tan_degrees(latitude) * tan_degrees(delta))
    
    sus transit_time drip = (alpha - longitude) / 15
    sus sunrise_time drip = transit_time - hour_angle / 15
    sus sunset_time drip = transit_time + hour_angle / 15
    
    fr fr Convert to Unix timestamps (simplified)
    sus base_timestamp drip = (julian_day - JULIAN_DAY_UNIX_EPOCH) * 86400
    
    sus result SolarTimes = SolarTimes{
        sunrise: base_timestamp + sunrise_time * 3600,
        sunset: base_timestamp + sunset_time * 3600,
        solar_noon: base_timestamp + transit_time * 3600,
        day_length: (sunset_time - sunrise_time) * 3600
    }
    
    damn result
}

slay calculate_equation_of_time(julian_day drip) drip {
    fr fr Calculate equation of time in seconds
    sus n drip = julian_day - 2451545
    sus l0 drip = (280.460 + 0.9856474 * n) % 360
    sus e drip = 23.439 - 0.0000004 * n
    
    sus y drip = tan_degrees(e / 2) * tan_degrees(e / 2)
    
    sus equation drip = 4 * (y * sin_degrees(2 * l0) - 2 * 0.0167 * sin_degrees(l0) + 
                        4 * 0.0167 * y * sin_degrees(l0) * cos_degrees(2 * l0) -
                        0.5 * y * y * sin_degrees(4 * l0) -
                        1.25 * 0.0167 * 0.0167 * sin_degrees(2 * l0))
    
    damn equation * 60  fr fr Convert to seconds
}

slay calculate_solar_declination(julian_day drip) drip {
    fr fr Calculate solar declination angle in degrees
    sus n drip = julian_day - 2451545
    sus l drip = (280.460 + 0.9856474 * n) % 360
    sus g drip = to_radians((357.528 + 0.9856003 * n) % 360)
    
    sus lambda drip = l + 1.915 * sin_degrees(g * 180 / 3.14159) + 0.020 * sin_degrees(2 * g * 180 / 3.14159)
    sus declination drip = asin_degrees(sin_degrees(23.439) * sin_degrees(lambda))
    
    damn declination
}

fr fr ===== CALENDAR SYSTEM CONVERSIONS =====

slay gregorian_to_julian_calendar(greg_year drip, greg_month drip, greg_day drip) CalendarDate {
    fr fr Convert Gregorian date to Julian calendar
    
    sus julian_day drip = gregorian_to_julian_day(greg_year, greg_month, greg_day)
    damn julian_day_to_julian_calendar(julian_day)
}

slay julian_day_to_julian_calendar(julian_day drip) CalendarDate {
    fr fr Convert Julian Day Number to Julian calendar date
    
    sus a drip = julian_day + 1402
    sus b drip = (a - 1) / 1461
    sus c drip = a - 1461 * b
    sus d drip = (c - 1) / 365
    sus e drip = c - 365 * d
    
    sus year drip = 4 * b + d
    sus month drip = (5 * e + 308) / 153 - 2
    sus day drip = e - (153 * month + 2) / 5 + 1
    
    ready (month > 12) {
        month = month - 12
        year = year + 1
    }
    
    sus result CalendarDate = CalendarDate{
        year: year,
        month: month,
        day: day,
        day_of_week: julian_day_to_weekday(julian_day),
        day_of_year: calculate_day_of_year_julian(year, month, day),
        week_of_year: 0,  fr fr Not applicable for Julian
        is_leap_year: is_julian_leap_year(year),
        calendar_system: "Julian"
    }
    
    damn result
}

slay calculate_day_of_year_julian(year drip, month drip, day drip) drip {
    fr fr Calculate day of year for Julian calendar
    sus day_of_year drip = day
    sus m drip = 1
    
    bestie (m < month) {
        ready (m == 2) {
            day_of_year = day_of_year + ready (is_julian_leap_year(year)) { 29 } otherwise { 28 }
        } otherwise ready (m == 4 || m == 6 || m == 9 || m == 11) {
            day_of_year = day_of_year + 30
        } otherwise {
            day_of_year = day_of_year + 31
        }
        m = m + 1
    }
    
    damn day_of_year
}

fr fr ===== MATHEMATICAL HELPER FUNCTIONS =====

slay to_radians(degrees drip) drip {
    damn degrees * 3.14159 / 180
}

slay to_degrees(radians drip) drip {
    damn radians * 180 / 3.14159
}

slay sin_degrees(degrees drip) drip {
    fr fr Sine function with degree input (approximation)
    sus radians drip = to_radians(degrees)
    damn sin_radians(radians)
}

slay cos_degrees(degrees drip) drip {
    fr fr Cosine function with degree input (approximation)
    sus radians drip = to_radians(degrees)
    damn cos_radians(radians)
}

slay tan_degrees(degrees drip) drip {
    fr fr Tangent function with degree input (approximation)
    sus radians drip = to_radians(degrees)
    damn tan_radians(radians)
}

slay asin_degrees(value drip) drip {
    fr fr Arcsine function returning degrees (approximation)
    sus radians drip = asin_radians(value)
    damn to_degrees(radians)
}

slay acos_degrees(value drip) drip {
    fr fr Arccosine function returning degrees (approximation)
    sus radians drip = acos_radians(value)
    damn to_degrees(radians)
}

slay atan2_degrees(y drip, x drip) drip {
    fr fr Atan2 function returning degrees (approximation)
    sus radians drip = atan2_radians(y, x)
    damn to_degrees(radians)
}

fr fr Simplified trigonometric functions (would use math library in real implementation)
slay sin_radians(x drip) drip {
    fr fr Taylor series approximation for sine
    sus x_squared drip = x * x
    damn x - (x * x_squared) / 6 + (x * x_squared * x_squared) / 120
}

slay cos_radians(x drip) drip {
    fr fr Taylor series approximation for cosine
    sus x_squared drip = x * x
    damn 1 - x_squared / 2 + (x_squared * x_squared) / 24
}

slay tan_radians(x drip) drip {
    fr fr Tangent as sin/cos
    sus cos_val drip = cos_radians(x)
    ready (cos_val == 0) { damn 0 }  fr fr Avoid division by zero
    damn sin_radians(x) / cos_val
}

slay asin_radians(x drip) drip {
    fr fr Approximation for arcsine
    ready (x > 1) { damn 3.14159 / 2 }
    ready (x < -1) { damn -3.14159 / 2 }
    damn x + (x * x * x) / 6 + (3 * x * x * x * x * x) / 40
}

slay acos_radians(x drip) drip {
    fr fr Arccosine as pi/2 - arcsin
    damn 3.14159 / 2 - asin_radians(x)
}

slay atan2_radians(y drip, x drip) drip {
    fr fr Simplified atan2
    ready (x > 0) { damn atan_radians(y / x) }
    ready (x < 0 && y >= 0) { damn atan_radians(y / x) + 3.14159 }
    ready (x < 0 && y < 0) { damn atan_radians(y / x) - 3.14159 }
    ready (x == 0 && y > 0) { damn 3.14159 / 2 }
    ready (x == 0 && y < 0) { damn -3.14159 / 2 }
    damn 0
}

slay atan_radians(x drip) drip {
    fr fr Approximation for arctangent
    ready (x > 1) { damn 3.14159 / 2 - atan_radians(1 / x) }
    ready (x < -1) { damn -3.14159 / 2 - atan_radians(1 / x) }
    damn x - (x * x * x) / 3 + (x * x * x * x * x) / 5
}

fr fr ===== PUBLIC API FUNCTIONS =====

slay extract_calendar_components(timestamp drip) CalendarDate {
    fr fr Extract all calendar components from Unix timestamp
    damn gregorian_date_from_timestamp(timestamp)
}

slay get_astronomical_info(timestamp drip, longitude drip, latitude drip) AstronomicalData {
    fr fr Get astronomical information for timestamp and location
    sus date CalendarDate = gregorian_date_from_timestamp(timestamp)
    damn calculate_astronomical_data(date.year, date.month, date.day, longitude, latitude)
}

slay validate_calendar_date(year drip, month drip, day drip) lit {
    fr fr Validate if date is valid in Gregorian calendar
    ready (month < 1 || month > 12) { damn cringe }
    ready (day < 1) { damn cringe }
    
    sus max_days drip = days_in_gregorian_month(year, month)
    ready (day > max_days) { damn cringe }
    
    damn based
}

slay days_between_dates(year1 drip, month1 drip, day1 drip, year2 drip, month2 drip, day2 drip) drip {
    fr fr Calculate days between two dates
    sus jd1 drip = gregorian_to_julian_day(year1, month1, day1)
    sus jd2 drip = gregorian_to_julian_day(year2, month2, day2)
    damn jd2 - jd1
}

slay add_days_to_date(year drip, month drip, day drip, days_to_add drip) CalendarDate {
    fr fr Add days to date and return new date
    sus julian_day drip = gregorian_to_julian_day(year, month, day)
    sus new_julian_day drip = julian_day + days_to_add
    damn julian_day_to_gregorian(new_julian_day)
}

slay get_easter_date(year drip) CalendarDate {
    fr fr Calculate Easter date using anonymous Gregorian algorithm
    sus a drip = year % 19
    sus b drip = year / 100
    sus c drip = year % 100
    sus d drip = b / 4
    sus e drip = b % 4
    sus f drip = (b + 8) / 25
    sus g drip = (b - f + 1) / 3
    sus h drip = (19 * a + b - d - g + 15) % 30
    sus i drip = c / 4
    sus k drip = c % 4
    sus l drip = (32 + 2 * e + 2 * i - h - k) % 7
    sus m drip = (a + 11 * h + 22 * l) / 451
    sus n drip = (h + l - 7 * m + 114) / 31
    sus p drip = (h + l - 7 * m + 114) % 31
    
    sus easter_month drip = n
    sus easter_day drip = p + 1
    
    sus result CalendarDate = CalendarDate{
        year: year,
        month: easter_month,
        day: easter_day,
        day_of_week: calculate_weekday(year, easter_month, easter_day),
        day_of_year: calculate_day_of_year(year, easter_month, easter_day),
        week_of_year: calculate_week_of_year(year, easter_month, easter_day),
        is_leap_year: is_gregorian_leap_year(year),
        calendar_system: "Gregorian"
    }
    
    damn result
}

vibez.spill("📅 Advanced calendar algorithms loaded with Gregorian, Julian, and astronomical calculations")
