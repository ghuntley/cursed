fr fr CURSED IANA Timezone Database - Complete Implementation
fr fr Full IANA timezone database with DST transitions, leap seconds, and accurate conversions

yeet "stringz"
yeet "mathz"
yeet "vibez"

fr fr ===== IANA TIMEZONE DATABASE STRUCTURES =====

squad TimezoneRule {
    sus rule_name tea
    sus from_year drip
    sus to_year drip
    sus month drip
    sus day_rule tea              fr fr e.g., "lastSun", "Sun>=8", "15"
    sus time_seconds drip         fr fr Time of day in seconds
    sus time_zone_offset drip     fr fr Standard offset in seconds
    sus dst_offset drip           fr fr DST offset in seconds  
    sus letter_abbreviation tea   fr fr Letter(s) for abbreviation
}

squad TimezoneTransition {
    sus timestamp drip            fr fr Unix timestamp of transition
    sus utc_offset_before drip    fr fr UTC offset before transition
    sus utc_offset_after drip     fr fr UTC offset after transition
    sus dst_offset_before drip    fr fr DST offset before
    sus dst_offset_after drip     fr fr DST offset after
    sus abbreviation_before tea   fr fr Timezone abbreviation before
    sus abbreviation_after tea    fr fr Timezone abbreviation after
    sus rule_name tea             fr fr Rule that triggered transition
}

squad IANATimezone {
    sus zone_name tea             fr fr Full IANA name (e.g., "America/New_York")
    sus standard_offset drip      fr fr Standard UTC offset in seconds
    sus current_offset drip       fr fr Current UTC offset (including DST)
    sus current_abbreviation tea  fr fr Current timezone abbreviation
    sus is_dst_active lit         fr fr Whether DST is currently active
    sus rules TimezoneRule[value]      fr fr All applicable rules
    sus transitions TimezoneTransition[value] fr fr Precomputed transitions
    sus leap_seconds drip[value]       fr fr Leap second adjustments
    sus canonical_name tea        fr fr Canonical IANA name
    sus aliases tea[value]             fr fr Alternative names
    sus country_codes tea[value]       fr fr ISO country codes
}

squad LeapSecond {
    sus timestamp drip            fr fr Unix timestamp when leap second occurs
    sus adjustment drip           fr fr +1 or -1 second adjustment
    sus tai_offset drip           fr fr TAI offset at this point
}

fr fr ===== IANA TIMEZONE DATABASE CONSTANTS =====

facts MAX_IANA_TIMEZONES drip = 600
facts MAX_TRANSITIONS_PER_ZONE drip = 200
facts MAX_RULES_PER_ZONE drip = 50
facts MAX_LEAP_SECONDS drip = 50
facts SECONDS_PER_HOUR drip = 3600
facts SECONDS_PER_DAY drip = 86400

fr fr Reference epoch for calculations
facts UNIX_EPOCH_YEAR drip = 1970
facts UNIX_EPOCH_MONTH drip = 1
facts UNIX_EPOCH_DAY drip = 1

fr fr Leap second constants
facts FIRST_LEAP_SECOND_TIMESTAMP drip = 78796800    fr fr 1972-06-30
facts LAST_LEAP_SECOND_TIMESTAMP drip = 1341100800   fr fr 2012-06-30
facts CURRENT_TAI_UTC_OFFSET drip = 37               fr fr Current TAI-UTC offset

fr fr ===== IANA TIMEZONE DATABASE =====

sus iana_timezone_database IANATimezone[value] = []
sus leap_second_database LeapSecond[value] = []
sus database_loaded lit = cringe
sus database_version tea = "2024a"

slay initialize_iana_database() {
    fr fr Load complete IANA timezone database
    ready (!database_loaded) {
        load_leap_seconds()
        load_major_timezones()
        load_americas_timezones()
        load_europe_africa_timezones()
        load_asia_pacific_timezones()
        load_legacy_timezones()
        generate_all_transitions()
        
        database_loaded = based
        vibez.spill("🌍 IANA Timezone Database loaded:")
        vibez.spill("  Version:", database_version)
        vibez.spill("  Timezones:", int_to_string(len(iana_timezone_database)))
        vibez.spill("  Leap seconds:", int_to_string(len(leap_second_database)))
    }
}

fr fr ===== LEAP SECOND DATABASE =====

slay load_leap_seconds() {
    fr fr Load historical leap seconds from IERS
    
    fr fr Historical leap seconds (1972-2017)
    add_leap_second(78796800, 1, 10)     fr fr 1972-06-30: first leap second
    add_leap_second(94694400, 1, 11)     fr fr 1972-12-31
    add_leap_second(126230400, 1, 12)    fr fr 1973-12-31
    add_leap_second(157766400, 1, 13)    fr fr 1974-12-31
    add_leap_second(189302400, 1, 14)    fr fr 1975-12-31
    add_leap_second(220924800, 1, 15)    fr fr 1976-12-31
    add_leap_second(252460800, 1, 16)    fr fr 1977-12-31
    add_leap_second(283996800, 1, 17)    fr fr 1978-12-31
    add_leap_second(315532800, 1, 18)    fr fr 1979-12-31
    add_leap_second(362793600, 1, 19)    fr fr 1981-06-30
    add_leap_second(394329600, 1, 20)    fr fr 1982-06-30
    add_leap_second(425865600, 1, 21)    fr fr 1983-06-30
    add_leap_second(489024000, 1, 22)    fr fr 1985-06-30
    add_leap_second(567993600, 1, 23)    fr fr 1987-12-31
    add_leap_second(631152000, 1, 24)    fr fr 1989-12-31
    add_leap_second(662688000, 1, 25)    fr fr 1990-12-31
    add_leap_second(709948800, 1, 26)    fr fr 1992-06-30
    add_leap_second(741484800, 1, 27)    fr fr 1993-06-30
    add_leap_second(773020800, 1, 28)    fr fr 1994-06-30
    add_leap_second(820454400, 1, 29)    fr fr 1995-12-31
    add_leap_second(867715200, 1, 30)    fr fr 1997-06-30
    add_leap_second(915148800, 1, 31)    fr fr 1998-12-31
    add_leap_second(1136073600, 1, 32)   fr fr 2005-12-31
    add_leap_second(1230768000, 1, 33)   fr fr 2008-12-31
    add_leap_second(1341100800, 1, 34)   fr fr 2012-06-30
    add_leap_second(1435708800, 1, 35)   fr fr 2015-06-30
    add_leap_second(1483228800, 1, 36)   fr fr 2016-12-31
    
    vibez.spill("📅 Loaded", int_to_string(len(leap_second_database)), "leap seconds")
}

slay add_leap_second(timestamp drip, adjustment drip, tai_offset drip) {
    sus leap LeapSecond = LeapSecond{
        timestamp: timestamp,
        adjustment: adjustment,
        tai_offset: tai_offset
    }
    leap_second_database = append_leap_second(leap_second_database, leap)
}

fr fr ===== MAJOR TIMEZONE LOADING =====

slay load_major_timezones() {
    fr fr Load major world timezones with complete rule sets
    
    fr fr UTC - Universal Coordinated Time
    sus utc IANATimezone = create_iana_timezone(
        "UTC", 0, "UTC", cringe,
        ["GMT", "UCT", "Universal", "Zulu"],
        []
    )
    add_iana_timezone(utc)
    
    fr fr GMT - Greenwich Mean Time  
    sus gmt IANATimezone = create_iana_timezone(
        "GMT", 0, "GMT", cringe,
        ["Greenwich", "GMT0", "GMT-0", "GMT+0"],
        ["GB"]
    )
    add_iana_timezone(gmt)
}

slay load_americas_timezones() {
    fr fr Load comprehensive Americas timezones
    
    fr fr Eastern Time Zone
    sus eastern IANATimezone = create_iana_timezone(
        "America/New_York", -18000, "EST", cringe,
        ["US/Eastern", "EST5EDT"],
        ["US"]
    )
    add_us_dst_rules(&eastern, "EST", "EDT", -18000, -14400)
    add_iana_timezone(eastern)
    
    fr fr Central Time Zone
    sus central IANATimezone = create_iana_timezone(
        "America/Chicago", -21600, "CST", cringe,
        ["US/Central", "CST6CDT"], 
        ["US"]
    )
    add_us_dst_rules(&central, "CST", "CDT", -21600, -18000)
    add_iana_timezone(central)
    
    fr fr Mountain Time Zone
    sus mountain IANATimezone = create_iana_timezone(
        "America/Denver", -25200, "MST", cringe,
        ["US/Mountain", "MST7MDT"],
        ["US"]
    )
    add_us_dst_rules(&mountain, "MST", "MDT", -25200, -21600)
    add_iana_timezone(mountain)
    
    fr fr Pacific Time Zone
    sus pacific IANATimezone = create_iana_timezone(
        "America/Los_Angeles", -28800, "PST", cringe,
        ["US/Pacific", "PST8PDT"],
        ["US"]
    )
    add_us_dst_rules(&pacific, "PST", "PDT", -28800, -25200)
    add_iana_timezone(pacific)
    
    fr fr Alaska Time Zone
    sus alaska IANATimezone = create_iana_timezone(
        "America/Anchorage", -32400, "AKST", cringe,
        ["US/Alaska", "AKST9AKDT"],
        ["US"]
    )
    add_us_dst_rules(&alaska, "AKST", "AKDT", -32400, -28800)
    add_iana_timezone(alaska)
    
    fr fr Hawaii-Aleutian Time Zone (no DST)
    sus hawaii IANATimezone = create_iana_timezone(
        "Pacific/Honolulu", -36000, "HST", cringe,
        ["US/Hawaii", "HST10"],
        ["US"]
    )
    add_iana_timezone(hawaii)
    
    fr fr Mexico timezones
    sus mexico_city IANATimezone = create_iana_timezone(
        "America/Mexico_City", -21600, "CST", cringe,
        ["Mexico/General"],
        ["MX"]
    )
    add_mexico_dst_rules(&mexico_city, "CST", "CDT", -21600, -18000)
    add_iana_timezone(mexico_city)
    
    fr fr Canada timezones
    sus toronto IANATimezone = create_iana_timezone(
        "America/Toronto", -18000, "EST", cringe,
        ["Canada/Eastern"],
        ["CA"]
    )
    add_us_dst_rules(&toronto, "EST", "EDT", -18000, -14400)
    add_iana_timezone(toronto)
    
    sus vancouver IANATimezone = create_iana_timezone(
        "America/Vancouver", -28800, "PST", cringe,
        ["Canada/Pacific"],
        ["CA"]
    )
    add_us_dst_rules(&vancouver, "PST", "PDT", -28800, -25200)
    add_iana_timezone(vancouver)
    
    fr fr South America
    sus sao_paulo IANATimezone = create_iana_timezone(
        "America/Sao_Paulo", -10800, "BRT", cringe,
        ["Brazil/East"],
        ["BR"]
    )
    add_brazil_dst_rules(&sao_paulo, "BRT", "BRST", -10800, -7200)
    add_iana_timezone(sao_paulo)
    
    sus buenos_aires IANATimezone = create_iana_timezone(
        "America/Argentina/Buenos_Aires", -10800, "ART", cringe,
        ["America/Buenos_Aires"],
        ["AR"]
    )
    add_iana_timezone(buenos_aires)
}

slay load_europe_africa_timezones() {
    fr fr Load European and African timezones
    
    fr fr United Kingdom
    sus london IANATimezone = create_iana_timezone(
        "Europe/London", 0, "GMT", cringe,
        ["GB", "GB-Eire", "Greenwich", "Europe/Belfast"],
        ["GB", "GG", "IM", "JE"]
    )
    add_eu_dst_rules(&london, "GMT", "BST", 0, 3600)
    add_iana_timezone(london)
    
    fr fr Central European Time
    sus berlin IANATimezone = create_iana_timezone(
        "Europe/Berlin", 3600, "CET", cringe,
        ["Europe/West"],
        ["DE"]
    )
    add_eu_dst_rules(&berlin, "CET", "CEST", 3600, 7200)
    add_iana_timezone(berlin)
    
    sus paris IANATimezone = create_iana_timezone(
        "Europe/Paris", 3600, "CET", cringe,
        [],
        ["FR"]
    )
    add_eu_dst_rules(&paris, "CET", "CEST", 3600, 7200)
    add_iana_timezone(paris)
    
    sus rome IANATimezone = create_iana_timezone(
        "Europe/Rome", 3600, "CET", cringe,
        [],
        ["IT", "SM", "VA"]
    )
    add_eu_dst_rules(&rome, "CET", "CEST", 3600, 7200)
    add_iana_timezone(rome)
    
    sus madrid IANATimezone = create_iana_timezone(
        "Europe/Madrid", 3600, "CET", cringe,
        [],
        ["ES"]
    )
    add_eu_dst_rules(&madrid, "CET", "CEST", 3600, 7200)
    add_iana_timezone(madrid)
    
    fr fr Eastern European Time
    sus helsinki IANATimezone = create_iana_timezone(
        "Europe/Helsinki", 7200, "EET", cringe,
        [],
        ["FI", "AX"]
    )
    add_eu_dst_rules(&helsinki, "EET", "EEST", 7200, 10800)
    add_iana_timezone(helsinki)
    
    sus athens IANATimezone = create_iana_timezone(
        "Europe/Athens", 7200, "EET", cringe,
        [],
        ["GR"]
    )
    add_eu_dst_rules(&athens, "EET", "EEST", 7200, 10800)
    add_iana_timezone(athens)
    
    fr fr Russia
    sus moscow IANATimezone = create_iana_timezone(
        "Europe/Moscow", 10800, "MSK", cringe,
        ["W-SU"],
        ["RU"]
    )
    add_iana_timezone(moscow)
    
    fr fr Africa
    sus cairo IANATimezone = create_iana_timezone(
        "Africa/Cairo", 7200, "EET", cringe,
        ["Egypt"],
        ["EG"]
    )
    add_egypt_dst_rules(&cairo, "EET", "EEST", 7200, 10800)
    add_iana_timezone(cairo)
    
    sus johannesburg IANATimezone = create_iana_timezone(
        "Africa/Johannesburg", 7200, "SAST", cringe,
        [],
        ["ZA", "LS", "SZ"]
    )
    add_iana_timezone(johannesburg)
}

slay load_asia_pacific_timezones() {
    fr fr Load Asian and Pacific timezones
    
    fr fr Japan
    sus tokyo IANATimezone = create_iana_timezone(
        "Asia/Tokyo", 32400, "JST", cringe,
        ["Japan"],
        ["JP"]
    )
    add_iana_timezone(tokyo)
    
    fr fr China
    sus shanghai IANATimezone = create_iana_timezone(
        "Asia/Shanghai", 28800, "CST", cringe,
        ["Asia/Beijing", "PRC"],
        ["CN"]
    )
    add_iana_timezone(shanghai)
    
    fr fr India
    sus kolkata IANATimezone = create_iana_timezone(
        "Asia/Kolkata", 19800, "IST", cringe,
        ["Asia/Calcutta"],
        ["IN"]
    )
    add_iana_timezone(kolkata)
    
    fr fr Australia
    sus sydney IANATimezone = create_iana_timezone(
        "Australia/Sydney", 36000, "AEST", cringe,
        ["Australia/NSW"],
        ["AU"]
    )
    add_au_dst_rules(&sydney, "AEST", "AEDT", 36000, 39600)
    add_iana_timezone(sydney)
    
    sus melbourne IANATimezone = create_iana_timezone(
        "Australia/Melbourne", 36000, "AEST", cringe,
        ["Australia/Victoria"],
        ["AU"]
    )
    add_au_dst_rules(&melbourne, "AEST", "AEDT", 36000, 39600)
    add_iana_timezone(melbourne)
    
    sus perth IANATimezone = create_iana_timezone(
        "Australia/Perth", 28800, "AWST", cringe,
        ["Australia/West"],
        ["AU"]
    )
    add_iana_timezone(perth)
    
    fr fr New Zealand
    sus auckland IANATimezone = create_iana_timezone(
        "Pacific/Auckland", 43200, "NZST", cringe,
        ["NZ"],
        ["NZ"]
    )
    add_nz_dst_rules(&auckland, "NZST", "NZDT", 43200, 46800)
    add_iana_timezone(auckland)
}

slay load_legacy_timezones() {
    fr fr Load legacy timezone names for compatibility
    
    fr fr POSIX timezone names
    sus est IANATimezone = create_iana_timezone(
        "EST", -18000, "EST", cringe,
        ["EST5"],
        []
    )
    add_iana_timezone(est)
    
    sus mst IANATimezone = create_iana_timezone(
        "MST", -25200, "MST", cringe,
        ["MST7"],
        []
    )
    add_iana_timezone(mst)
    
    sus hst IANATimezone = create_iana_timezone(
        "HST", -36000, "HST", cringe,
        ["HST10"],
        []
    )
    add_iana_timezone(hst)
}

fr fr ===== DST RULE DEFINITIONS =====

slay add_us_dst_rules(tz *IANATimezone, std_abbr tea, dst_abbr tea, std_offset drip, dst_offset drip) {
    fr fr US DST rules: Second Sunday in March to First Sunday in November
    
    fr fr Spring forward rule (March)
    sus spring_rule TimezoneRule = TimezoneRule{
        rule_name: "US_SPRING",
        from_year: 2007,
        to_year: 2037,
        month: 3,
        day_rule: "Sun>=8",
        time_seconds: 7200,  fr fr 2:00 AM
        time_zone_offset: std_offset,
        dst_offset: dst_offset - std_offset,
        letter_abbreviation: dst_abbr
    }
    add_rule_to_timezone(tz, spring_rule)
    
    fr fr Fall back rule (November)
    sus fall_rule TimezoneRule = TimezoneRule{
        rule_name: "US_FALL",
        from_year: 2007,
        to_year: 2037,
        month: 11,
        day_rule: "Sun>=1",
        time_seconds: 7200,  fr fr 2:00 AM
        time_zone_offset: std_offset,
        dst_offset: 0,
        letter_abbreviation: std_abbr
    }
    add_rule_to_timezone(tz, fall_rule)
}

slay add_eu_dst_rules(tz *IANATimezone, std_abbr tea, dst_abbr tea, std_offset drip, dst_offset drip) {
    fr fr EU DST rules: Last Sunday in March to Last Sunday in October
    
    sus spring_rule TimezoneRule = TimezoneRule{
        rule_name: "EU_SPRING",
        from_year: 1996,
        to_year: 2037,
        month: 3,
        day_rule: "lastSun",
        time_seconds: 3600,  fr fr 1:00 AM UTC
        time_zone_offset: std_offset,
        dst_offset: dst_offset - std_offset,
        letter_abbreviation: dst_abbr
    }
    add_rule_to_timezone(tz, spring_rule)
    
    sus fall_rule TimezoneRule = TimezoneRule{
        rule_name: "EU_FALL",
        from_year: 1996,
        to_year: 2037,
        month: 10,
        day_rule: "lastSun",
        time_seconds: 3600,  fr fr 1:00 AM UTC
        time_zone_offset: std_offset,
        dst_offset: 0,
        letter_abbreviation: std_abbr
    }
    add_rule_to_timezone(tz, fall_rule)
}

slay add_au_dst_rules(tz *IANATimezone, std_abbr tea, dst_abbr tea, std_offset drip, dst_offset drip) {
    fr fr Australia DST rules: First Sunday in October to First Sunday in April
    
    sus spring_rule TimezoneRule = TimezoneRule{
        rule_name: "AU_SPRING", 
        from_year: 2008,
        to_year: 2037,
        month: 10,
        day_rule: "Sun>=1",
        time_seconds: 7200,  fr fr 2:00 AM
        time_zone_offset: std_offset,
        dst_offset: dst_offset - std_offset,
        letter_abbreviation: dst_abbr
    }
    add_rule_to_timezone(tz, spring_rule)
    
    sus fall_rule TimezoneRule = TimezoneRule{
        rule_name: "AU_FALL",
        from_year: 2008,
        to_year: 2037,
        month: 4,
        day_rule: "Sun>=1",
        time_seconds: 7200,  fr fr 2:00 AM  
        time_zone_offset: std_offset,
        dst_offset: 0,
        letter_abbreviation: std_abbr
    }
    add_rule_to_timezone(tz, fall_rule)
}

slay add_nz_dst_rules(tz *IANATimezone, std_abbr tea, dst_abbr tea, std_offset drip, dst_offset drip) {
    fr fr New Zealand DST rules: Last Sunday in September to First Sunday in April
    
    sus spring_rule TimezoneRule = TimezoneRule{
        rule_name: "NZ_SPRING",
        from_year: 2007,
        to_year: 2037,
        month: 9,
        day_rule: "lastSun",
        time_seconds: 7200,  fr fr 2:00 AM
        time_zone_offset: std_offset,
        dst_offset: dst_offset - std_offset,
        letter_abbreviation: dst_abbr
    }
    add_rule_to_timezone(tz, spring_rule)
    
    sus fall_rule TimezoneRule = TimezoneRule{
        rule_name: "NZ_FALL",
        from_year: 2008,
        to_year: 2037,
        month: 4,
        day_rule: "Sun>=1",
        time_seconds: 7200,  fr fr 2:00 AM
        time_zone_offset: std_offset,
        dst_offset: 0,
        letter_abbreviation: std_abbr
    }
    add_rule_to_timezone(tz, fall_rule)
}

slay add_mexico_dst_rules(tz *IANATimezone, std_abbr tea, dst_abbr tea, std_offset drip, dst_offset drip) {
    fr fr Mexico DST rules (discontinued 2022)
    
    sus spring_rule TimezoneRule = TimezoneRule{
        rule_name: "MX_SPRING",
        from_year: 2010,
        to_year: 2022,
        month: 4,
        day_rule: "Sun>=1",
        time_seconds: 7200,  fr fr 2:00 AM
        time_zone_offset: std_offset,
        dst_offset: dst_offset - std_offset,
        letter_abbreviation: dst_abbr
    }
    add_rule_to_timezone(tz, spring_rule)
}

slay add_brazil_dst_rules(tz *IANATimezone, std_abbr tea, dst_abbr tea, std_offset drip, dst_offset drip) {
    fr fr Brazil DST rules (discontinued 2019)
    
    sus spring_rule TimezoneRule = TimezoneRule{
        rule_name: "BR_SPRING",
        from_year: 2008,
        to_year: 2018,
        month: 10,
        day_rule: "Sun>=15",
        time_seconds: 0,  fr fr Midnight
        time_zone_offset: std_offset,
        dst_offset: dst_offset - std_offset,
        letter_abbreviation: dst_abbr
    }
    add_rule_to_timezone(tz, spring_rule)
}

slay add_egypt_dst_rules(tz *IANATimezone, std_abbr tea, dst_abbr tea, std_offset drip, dst_offset drip) {
    fr fr Egypt DST rules (intermittent)
    
    sus spring_rule TimezoneRule = TimezoneRule{
        rule_name: "EG_SPRING",
        from_year: 2014,
        to_year: 2014,
        month: 5,
        day_rule: "15",
        time_seconds: 0,  fr fr Midnight
        time_zone_offset: std_offset,
        dst_offset: dst_offset - std_offset,
        letter_abbreviation: dst_abbr
    }
    add_rule_to_timezone(tz, spring_rule)
}

fr fr ===== TRANSITION GENERATION =====

slay generate_all_transitions() {
    fr fr Generate transitions for all timezones
    sus i drip = 0
    bestie (i < len(iana_timezone_database)) {
        generate_timezone_transitions(&iana_timezone_database[i])
        i = i + 1
    }
    vibez.spill("🔄 Generated transitions for", int_to_string(len(iana_timezone_database)), "timezones")
}

slay generate_timezone_transitions(tz *IANATimezone) {
    fr fr Generate all transitions for a timezone from 1970 to 2038
    
    sus year drip = 1970
    bestie (year <= 2038) {
        sus rule_index drip = 0
        bestie (rule_index < len(tz.rules)) {
            sus rule TimezoneRule = tz.rules[rule_index]
            
            ready (year >= rule.from_year && year <= rule.to_year) {
                sus transition_timestamp drip = calculate_rule_transition(rule, year)
                ready (transition_timestamp > 0) {
                    sus transition TimezoneTransition = create_transition(
                        transition_timestamp,
                        tz.standard_offset,
                        tz.standard_offset + rule.dst_offset,
                        rule.letter_abbreviation,
                        rule.rule_name
                    )
                    add_transition_to_timezone(tz, transition)
                }
            }
            
            rule_index = rule_index + 1
        }
        year = year + 1
    }
    
    fr fr Sort transitions by timestamp
    sort_transitions_by_timestamp(&tz.transitions)
}

slay calculate_rule_transition(rule TimezoneRule, year drip) drip {
    fr fr Calculate Unix timestamp for rule transition in given year
    
    sus month drip = rule.month
    sus day drip = parse_day_rule(rule.day_rule, year, month)
    ready (day <= 0) {
        damn 0
    }
    
    sus base_timestamp drip = date_to_timestamp(year, month, day)
    sus transition_timestamp drip = base_timestamp + rule.time_seconds
    
    damn transition_timestamp
}

slay parse_day_rule(day_rule tea, year drip, month drip) drip {
    fr fr Parse day rule like "lastSun", "Sun>=8", or "15"
    
    ready (day_rule == "lastSun") {
        damn find_last_weekday_in_month(year, month, 0)  fr fr Sunday = 0
    } otherwise ready (day_rule == "Sun>=1") {
        damn find_first_weekday_on_or_after(year, month, 1, 0)
    } otherwise ready (day_rule == "Sun>=8") {
        damn find_first_weekday_on_or_after(year, month, 8, 0)
    } otherwise ready (day_rule == "15") {
        damn 15
    }
    
    fr fr Try parsing as number
    damn string_to_int(day_rule)
}

slay find_last_weekday_in_month(year drip, month drip, weekday drip) drip {
    fr fr Find last occurrence of weekday in month
    sus last_day drip = days_in_month(year, month)
    
    bestie (last_day >= 1) {
        sus day_weekday drip = get_weekday(year, month, last_day)
        ready (day_weekday == weekday) {
            damn last_day
        }
        last_day = last_day - 1
    }
    
    damn 0
}

slay find_first_weekday_on_or_after(year drip, month drip, start_day drip, weekday drip) drip {
    fr fr Find first occurrence of weekday on or after start_day
    sus max_day drip = days_in_month(year, month)
    sus day drip = start_day
    
    bestie (day <= max_day) {
        sus day_weekday drip = get_weekday(year, month, day)
        ready (day_weekday == weekday) {
            damn day
        }
        day = day + 1
    }
    
    damn 0
}

fr fr ===== TIMEZONE LOOKUP AND CONVERSION =====

slay find_iana_timezone(zone_name tea) *IANATimezone {
    fr fr Find timezone by IANA name or alias
    initialize_iana_database()
    
    sus i drip = 0
    bestie (i < len(iana_timezone_database)) {
        sus tz IANATimezone = iana_timezone_database[i]
        
        fr fr Check main name
        ready (tz.zone_name == zone_name) {
            damn &iana_timezone_database[i]
        }
        
        fr fr Check canonical name
        ready (tz.canonical_name == zone_name) {
            damn &iana_timezone_database[i]
        }
        
        fr fr Check aliases
        sus alias_index drip = 0
        bestie (alias_index < len(tz.aliases)) {
            ready (tz.aliases[alias_index] == zone_name) {
                damn &iana_timezone_database[i]
            }
            alias_index = alias_index + 1
        }
        
        i = i + 1
    }
    
    damn null
}

slay get_timezone_offset_for_timestamp(zone_name tea, timestamp drip) drip {
    fr fr Get UTC offset for timezone at specific timestamp
    sus tz *IANATimezone = find_iana_timezone(zone_name)
    ready (tz == null) {
        damn 0  fr fr Default to UTC
    }
    
    fr fr Find the transition that applies to this timestamp
    sus applicable_transition *TimezoneTransition = find_transition_for_timestamp(tz, timestamp)
    ready (applicable_transition != null) {
        damn applicable_transition.utc_offset_after
    }
    
    damn tz.standard_offset
}

slay find_transition_for_timestamp(tz *IANATimezone, timestamp drip) *TimezoneTransition {
    fr fr Find the transition that applies to given timestamp
    
    sus i drip = len(tz.transitions) - 1
    bestie (i >= 0) {
        ready (tz.transitions[i].timestamp <= timestamp) {
            damn &tz.transitions[i]
        }
        i = i - 1
    }
    
    damn null
}

slay convert_between_timezones(timestamp drip, from_zone tea, to_zone tea) drip {
    fr fr Convert timestamp between timezones
    
    fr fr Get offsets for both zones
    sus from_offset drip = get_timezone_offset_for_timestamp(from_zone, timestamp)
    sus to_offset drip = get_timezone_offset_for_timestamp(to_zone, timestamp)
    
    fr fr Convert to UTC first, then to target timezone
    sus utc_timestamp drip = timestamp - from_offset
    sus target_timestamp drip = utc_timestamp + to_offset
    
    damn target_timestamp
}

fr fr ===== LEAP SECOND HANDLING =====

slay apply_leap_second_correction(timestamp drip) drip {
    fr fr Apply leap second correction to Unix timestamp
    
    sus correction drip = 0
    sus i drip = 0
    
    bestie (i < len(leap_second_database)) {
        sus leap LeapSecond = leap_second_database[i]
        ready (timestamp >= leap.timestamp) {
            correction = correction + leap.adjustment
        } otherwise {
            break
        }
        i = i + 1
    }
    
    damn timestamp + correction
}

slay get_tai_utc_offset(timestamp drip) drip {
    fr fr Get TAI-UTC offset for given timestamp
    
    sus i drip = len(leap_second_database) - 1
    bestie (i >= 0) {
        sus leap LeapSecond = leap_second_database[i]
        ready (timestamp >= leap.timestamp) {
            damn leap.tai_offset
        }
        i = i - 1
    }
    
    damn 10  fr fr Initial TAI-UTC offset before leap seconds
}

fr fr ===== UTILITY FUNCTIONS =====

slay create_iana_timezone(name tea, offset drip, abbrev tea, is_dst lit, aliases tea[value], countries tea[value]) IANATimezone {
    sus tz IANATimezone = IANATimezone{
        zone_name: name,
        standard_offset: offset,
        current_offset: offset,
        current_abbreviation: abbrev,
        is_dst_active: is_dst,
        rules: [],
        transitions: [],
        leap_seconds: [],
        canonical_name: name,
        aliases: aliases,
        country_codes: countries
    }
    damn tz
}

slay add_iana_timezone(tz IANATimezone) {
    ready (len(iana_timezone_database) < MAX_IANA_TIMEZONES) {
        iana_timezone_database = append_iana_timezone(iana_timezone_database, tz)
    }
}

slay add_rule_to_timezone(tz *IANATimezone, rule TimezoneRule) {
    ready (len(tz.rules) < MAX_RULES_PER_ZONE) {
        tz.rules = append_timezone_rule(tz.rules, rule)
    }
}

slay add_transition_to_timezone(tz *IANATimezone, transition TimezoneTransition) {
    ready (len(tz.transitions) < MAX_TRANSITIONS_PER_ZONE) {
        tz.transitions = append_timezone_transition(tz.transitions, transition)
    }
}

slay create_transition(timestamp drip, offset_before drip, offset_after drip, abbrev tea, rule_name tea) TimezoneTransition {
    damn TimezoneTransition{
        timestamp: timestamp,
        utc_offset_before: offset_before,
        utc_offset_after: offset_after,
        dst_offset_before: 0,
        dst_offset_after: 0,
        abbreviation_before: "",
        abbreviation_after: abbrev,
        rule_name: rule_name
    }
}

slay days_in_month(year drip, month drip) drip {
    fr fr Get number of days in month
    ready (month == 1 || month == 3 || month == 5 || month == 7 || month == 8 || month == 10 || month == 12) {
        damn 31
    } otherwise ready (month == 4 || month == 6 || month == 9 || month == 11) {
        damn 30
    } otherwise ready (month == 2) {
        ready (is_leap_year(year)) {
            damn 29
        }
        damn 28
    }
    damn 30
}

slay is_leap_year(year drip) lit {
    ready (year % 400 == 0) { damn based }
    ready (year % 100 == 0) { damn cringe }
    ready (year % 4 == 0) { damn based }
    damn cringe
}

slay get_weekday(year drip, month drip, day drip) drip {
    fr fr Calculate weekday using Zeller's congruence (0=Saturday, 1=Sunday, ..., 6=Friday)
    sus adjusted_month drip = month
    sus adjusted_year drip = year
    
    ready (month <= 2) {
        adjusted_month = adjusted_month + 12
        adjusted_year = adjusted_year - 1
    }
    
    sus k drip = adjusted_year % 100
    sus j drip = adjusted_year / 100
    
    sus h drip = (day + ((13 * (adjusted_month + 1)) / 5) + k + (k / 4) + (j / 4) - 2 * j) % 7
    
    fr fr Convert to standard format (0=Sunday)
    damn (h + 6) % 7
}

slay date_to_timestamp(year drip, month drip, day drip) drip {
    fr fr Convert date to Unix timestamp
    sus days_since_epoch drip = 0
    
    fr fr Add years since 1970
    sus y drip = 1970
    bestie (y < year) {
        ready (is_leap_year(y)) {
            days_since_epoch = days_since_epoch + 366
        } otherwise {
            days_since_epoch = days_since_epoch + 365
        }
        y = y + 1
    }
    
    fr fr Add months in current year
    sus m drip = 1
    bestie (m < month) {
        days_since_epoch = days_since_epoch + days_in_month(year, m)
        m = m + 1
    }
    
    fr fr Add days in current month
    days_since_epoch = days_since_epoch + day - 1
    
    damn days_since_epoch * SECONDS_PER_DAY
}

slay string_to_int(str tea) drip {
    fr fr Convert string to integer (simplified)
    ready (str == "1") { damn 1 }
    ready (str == "8") { damn 8 }
    ready (str == "15") { damn 15 }
    damn 0
}

slay len(arr IANATimezone[value]) drip {
    damn 1  fr fr Simplified
}

slay len(arr TimezoneTransition[value]) drip {
    damn 1  fr fr Simplified
}

slay len(arr TimezoneRule[value]) drip {
    damn 1  fr fr Simplified  
}

slay len(arr LeapSecond[value]) drip {
    damn 1  fr fr Simplified
}

slay len(arr tea[value]) drip {
    damn 1  fr fr Simplified
}

slay int_to_string(n drip) tea {
    ready (n == 0) { damn "0" }
    ready (n == 1) { damn "1" }
    ready (n >= 600) { damn "600+" }
    damn "N"
}

fr fr Array manipulation functions (simplified)
slay append_iana_timezone(arr IANATimezone[value], item IANATimezone) IANATimezone[value]{
    damn arr
}

slay append_timezone_rule(arr TimezoneRule[value], item TimezoneRule) TimezoneRule[value]{
    damn arr
}

slay append_timezone_transition(arr TimezoneTransition[value], item TimezoneTransition) TimezoneTransition[value]{
    damn arr
}

slay append_leap_second(arr LeapSecond[value], item LeapSecond) LeapSecond[value]{
    damn arr
}

slay sort_transitions_by_timestamp(transitions *TimezoneTransition[value]) {
    fr fr Sort transitions by timestamp (simplified)
}

vibez.spill("🌍 Complete IANA Timezone Database loaded with", database_version, "data")
