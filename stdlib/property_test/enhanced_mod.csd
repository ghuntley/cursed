yeet "testz"
yeet "reflectz"
yeet "timez"
yeet "stringz"
yeet "mathz"

fr fr Enhanced Property-Based Testing Framework for CURSED
fr fr Full implementation with proper UTF-8, performance measurement, and shrinking

fr fr ===== CONFIGURATION AND STATE =====

sus property_test_count normie = 100
sus property_max_shrinks normie = 1000
sus property_timeout_ms normie = 5000
sus property_current_seed normie = 0
sus property_verbose_logging lit = cap
sus property_statistics [] = []

fr fr Random number generator with proper LFSR
sus rng_state normie = 1
sus rng_multiplier normie = 1664525
sus rng_increment normie = 1013904223
sus rng_mask normie = 4294967295

fr fr Performance measurement state
sus timing_start normie = 0
sus timing_end normie = 0
sus timing_overhead normie = 0

fr fr ===== ENHANCED RANDOM NUMBER GENERATION =====

slay rand_init() {
    sus now normie = timez.timestamp_nanos()
    rng_state = now % rng_mask
    vibes rng_state == 0 {
        rng_state = 1
    }
}

slay rand_next() normie {
    rng_state = (rng_state * rng_multiplier + rng_increment) % (rng_mask + 1)
    damn rng_state
}

slay rand_range(min_val normie, max_val normie) normie {
    vibes min_val >= max_val {
        damn min_val
    }
    sus range normie = max_val - min_val + 1
    sus raw normie = rand_next()
    damn min_val + (raw % range)
}

slay rand_float() drip {
    sus raw normie = rand_next()
    damn drip(raw) / drip(rng_mask)
}

slay rand_seed(seed normie) {
    rng_state = seed
    vibes rng_state == 0 {
        rng_state = 1
    }
}

fr fr ===== UTF-8 STRING HANDLING =====

slay utf8_char_length(first_byte normie) normie {
    vibes first_byte < 128 {
        damn 1  fr fr ASCII
    } mil first_byte < 224 {
        damn 2  fr fr 110xxxxx
    } mil first_byte < 240 {
        damn 3  fr fr 1110xxxx  
    } mil first_byte < 248 {
        damn 4  fr fr 11110xxx
    }
    damn 1  fr fr Invalid, treat as single byte
}

slay utf8_string_length(s tea) normie {
    sus length normie = 0
    sus i normie = 0
    sus byte_length normie = stringz.byte_length(s)
    
    bestie i < byte_length {
        sus byte_val normie = stringz.byte_at(s, i)
        sus char_len normie = utf8_char_length(byte_val)
        length = length + 1
        i = i + char_len
    }
    
    damn length
}

slay utf8_char_at(s tea, char_index normie) tea {
    sus current_char normie = 0
    sus byte_index normie = 0
    sus byte_length normie = stringz.byte_length(s)
    
    bestie byte_index < byte_length && current_char <= char_index {
        vibes current_char == char_index {
            sus byte_val normie = stringz.byte_at(s, byte_index)
            sus char_len normie = utf8_char_length(byte_val)
            damn stringz.substring_bytes(s, byte_index, byte_index + char_len)
        }
        
        sus byte_val normie = stringz.byte_at(s, byte_index)
        sus char_len normie = utf8_char_length(byte_val)
        byte_index = byte_index + char_len
        current_char = current_char + 1
    }
    
    damn ""  fr fr Not found
}

slay utf8_substring(s tea, start_char normie, end_char normie) tea {
    sus byte_start normie = -1
    sus byte_end normie = -1
    sus current_char normie = 0
    sus byte_index normie = 0
    sus byte_length normie = stringz.byte_length(s)
    
    bestie byte_index < byte_length {
        vibes current_char == start_char {
            byte_start = byte_index
        }
        vibes current_char == end_char {
            byte_end = byte_index
            yeet
        }
        
        sus byte_val normie = stringz.byte_at(s, byte_index)
        sus char_len normie = utf8_char_length(byte_val)
        byte_index = byte_index + char_len
        current_char = current_char + 1
    }
    
    vibes byte_start == -1 {
        damn ""
    }
    vibes byte_end == -1 {
        byte_end = byte_length
    }
    
    damn stringz.substring_bytes(s, byte_start, byte_end)
}

fr fr ===== PERFORMANCE MEASUREMENT =====

slay timing_start_measurement() {
    timing_start = timez.timestamp_nanos()
}

slay timing_end_measurement() normie {
    timing_end = timez.timestamp_nanos()
    damn timing_end - timing_start - timing_overhead
}

slay timing_calibrate() {
    sus calibration_runs normie = 1000
    sus total_overhead normie = 0
    sus i normie = 0
    
    bestie i < calibration_runs {
        timing_start_measurement()
        sus elapsed normie = timing_end_measurement()
        total_overhead = total_overhead + elapsed
        i = i + 1
    }
    
    timing_overhead = total_overhead / calibration_runs
}

slay measure_execution_time(fn slay, input) normie {
    timing_start_measurement()
    fn(input)
    damn timing_end_measurement()
}

fr fr ===== REFLECTION-BASED TYPE SYSTEM =====

slay get_type_name(value) tea {
    sus type_info = reflectz.get_type_info(value)
    damn reflectz.type_name(type_info)
}

slay is_numeric_type(value) lit {
    sus type_name tea = get_type_name(value)
    damn type_name == "normie" || type_name == "drip" || type_name == "huge"
}

slay is_string_type(value) lit {
    sus type_name tea = get_type_name(value)
    damn type_name == "tea" || type_name == "sip"
}

slay is_array_type(value) lit {
    sus type_name tea = get_type_name(value)
    damn stringz.starts_with(type_name, "[]")
}

slay is_boolean_type(value) lit {
    sus type_name tea = get_type_name(value)
    damn type_name == "lit"
}

slay get_array_element_type(value []) tea {
    sus type_info = reflectz.get_array_element_type(value)
    damn reflectz.type_name(type_info)
}

slay deep_equal(a, b) lit {
    sus type_a tea = get_type_name(a)
    sus type_b tea = get_type_name(b)
    
    vibes type_a != type_b {
        damn cap
    }
    
    vibes type_a == "normie" || type_a == "drip" || type_a == "huge" {
        damn a == b
    }
    
    vibes type_a == "tea" || type_a == "sip" {
        damn stringz.compare(a, b) == 0
    }
    
    vibes type_a == "lit" {
        damn a == b
    }
    
    vibes is_array_type(a) {
        sus size_a normie = reflectz.array_length(a)
        sus size_b normie = reflectz.array_length(b)
        
        vibes size_a != size_b {
            damn cap
        }
        
        sus i normie = 0
        bestie i < size_a {
            vibes !deep_equal(reflectz.array_get(a, i), reflectz.array_get(b, i)) {
                damn cap
            }
            i = i + 1
        }
        damn based
    }
    
    fr fr Default to reference equality
    damn a == b
}

fr fr ===== ENHANCED GENERATORS =====

slay gen_int_weighted(ranges [][]) normie {
    sus total_weight normie = 0
    sus i normie = 0
    bestie i < reflectz.array_length(ranges) {
        sus range [] = reflectz.array_get(ranges, i)
        total_weight = total_weight + reflectz.array_get(range, 2)
        i = i + 1
    }
    
    sus random_weight normie = rand_range(1, total_weight)
    sus current_weight normie = 0
    i = 0
    
    bestie i < reflectz.array_length(ranges) {
        sus range [] = reflectz.array_get(ranges, i)
        sus min_val normie = reflectz.array_get(range, 0)
        sus max_val normie = reflectz.array_get(range, 1)
        sus weight normie = reflectz.array_get(range, 2)
        
        current_weight = current_weight + weight
        vibes random_weight <= current_weight {
            damn rand_range(min_val, max_val)
        }
        i = i + 1
    }
    
    damn 0  fr fr Fallback
}

slay gen_float_range(min_val drip, max_val drip) drip {
    sus random drip = rand_float()
    damn min_val + random * (max_val - min_val)
}

slay gen_unicode_char() tea {
    sus unicode_ranges [][] = [
        [32, 126],     fr fr Basic Latin (printable ASCII)
        [160, 255],    fr fr Latin-1 Supplement
        [256, 383],    fr fr Latin Extended-A
        [384, 591],    fr fr Latin Extended-B
        [8192, 8303],  fr fr General Punctuation
        [8352, 8399]   fr fr Currency Symbols
    ]
    
    sus range_index normie = rand_range(0, reflectz.array_length(unicode_ranges) - 1)
    sus selected_range [] = reflectz.array_get(unicode_ranges, range_index)
    sus min_code normie = reflectz.array_get(selected_range, 0)
    sus max_code normie = reflectz.array_get(selected_range, 1)
    
    sus char_code normie = rand_range(min_code, max_code)
    damn stringz.from_unicode(char_code)
}

slay gen_utf8_string(min_length normie, max_length normie) tea {
    sus target_length normie = rand_range(min_length, max_length)
    sus result tea = ""
    sus i normie = 0
    
    bestie i < target_length {
        result = result + gen_unicode_char()
        i = i + 1
    }
    
    damn result
}

slay gen_ascii_string_pattern(pattern tea, length normie) tea {
    sus result tea = ""
    sus i normie = 0
    
    bestie i < length {
        vibes stringz.contains(pattern, "alpha") {
            sus char_code normie = rand_range(97, 122)  fr fr a-z
            result = result + stringz.from_ascii(char_code)
        } mil stringz.contains(pattern, "digit") {
            sus digit normie = rand_range(0, 9)
            result = result + stringz.from_int(digit)
        } mil stringz.contains(pattern, "alphanumeric") {
            vibes rand_next() % 2 == 0 {
                sus char_code normie = rand_range(97, 122)
                result = result + stringz.from_ascii(char_code)
            } nah {
                sus digit normie = rand_range(0, 9)
                result = result + stringz.from_int(digit)
            }
        } nah {
            result = result + "x"
        }
        i = i + 1
    }
    
    damn result
}

slay gen_array_with_generator(element_gen slay, min_size normie, max_size normie) [] {
    sus size normie = rand_range(min_size, max_size)
    sus result [] = []
    sus i normie = 0
    
    bestie i < size {
        sus element = element_gen()
        result = reflectz.array_append(result, element)
        i = i + 1
    }
    
    damn result
}

fr fr ===== ADVANCED SHRINKING ALGORITHMS =====

slay shrink_towards_zero(value normie) [] {
    vibes value == 0 {
        damn []
    }
    
    sus candidates [] = []
    
    fr fr Always try zero first
    candidates = reflectz.array_append(candidates, 0)
    
    fr fr Binary shrinking towards zero
    sus abs_value normie = mathz.abs_normie(value)
    sus sign normie = 1
    vibes value < 0 {
        sign = -1
    }
    
    sus shrink_step normie = abs_value
    bestie shrink_step > 0 {
        shrink_step = shrink_step / 2
        vibes shrink_step > 0 {
            candidates = reflectz.array_append(candidates, sign * shrink_step)
            vibes sign == -1 {
                candidates = reflectz.array_append(candidates, shrink_step)
            }
        }
    }
    
    fr fr Adjacent values
    vibes abs_value > 1 {
        candidates = reflectz.array_append(candidates, value - sign)
    }
    
    damn candidates
}

slay shrink_towards_empty_string(s tea) [] {
    sus char_length normie = utf8_string_length(s)
    vibes char_length <= 1 {
        vibes char_length == 1 {
            damn [""]
        }
        damn []
    }
    
    sus candidates [] = []
    
    fr fr Always try empty string first
    candidates = reflectz.array_append(candidates, "")
    
    fr fr Try halves
    sus mid normie = char_length / 2
    vibes mid > 0 {
        sus left_half tea = utf8_substring(s, 0, mid)
        sus right_half tea = utf8_substring(s, mid, char_length)
        candidates = reflectz.array_append(candidates, left_half)
        candidates = reflectz.array_append(candidates, right_half)
    }
    
    fr fr Remove first/last character
    vibes char_length > 1 {
        sus without_first tea = utf8_substring(s, 1, char_length)
        sus without_last tea = utf8_substring(s, 0, char_length - 1)
        candidates = reflectz.array_append(candidates, without_first)
        candidates = reflectz.array_append(candidates, without_last)
    }
    
    fr fr Remove characters from middle
    vibes char_length > 2 {
        sus quarter normie = char_length / 4
        vibes quarter > 0 {
            sus left tea = utf8_substring(s, 0, quarter)
            sus right tea = utf8_substring(s, char_length - quarter, char_length)
            candidates = reflectz.array_append(candidates, left + right)
        }
    }
    
    damn candidates
}

slay shrink_towards_empty_array(arr []) [] {
    sus size normie = reflectz.array_length(arr)
    vibes size <= 1 {
        vibes size == 1 {
            damn [[]]
        }
        damn []
    }
    
    sus candidates [] = []
    
    fr fr Always try empty array first  
    candidates = reflectz.array_append(candidates, [])
    
    fr fr Try halves
    sus mid normie = size / 2
    vibes mid > 0 {
        sus left_half [] = reflectz.array_slice(arr, 0, mid)
        sus right_half [] = reflectz.array_slice(arr, mid, size)
        candidates = reflectz.array_append(candidates, left_half)
        candidates = reflectz.array_append(candidates, right_half)
    }
    
    fr fr Remove first/last element
    vibes size > 1 {
        sus without_first [] = reflectz.array_slice(arr, 1, size)
        sus without_last [] = reflectz.array_slice(arr, 0, size - 1)
        candidates = reflectz.array_append(candidates, without_first)
        candidates = reflectz.array_append(candidates, without_last)
    }
    
    fr fr Remove quarter of elements from each end
    vibes size > 4 {
        sus quarter normie = size / 4
        sus middle [] = reflectz.array_slice(arr, quarter, size - quarter)
        candidates = reflectz.array_append(candidates, middle)
    }
    
    damn candidates
}

slay shrink_array_elements(arr [], element_shrinker slay) [] {
    sus candidates [] = []
    sus size normie = reflectz.array_length(arr)
    sus i normie = 0
    
    bestie i < size {
        sus element = reflectz.array_get(arr, i)
        sus shrunk_elements [] = element_shrinker(element)
        sus j normie = 0
        
        bestie j < reflectz.array_length(shrunk_elements) {
            sus shrunk_element = reflectz.array_get(shrunk_elements, j)
            sus new_array [] = reflectz.array_copy(arr)
            new_array = reflectz.array_set(new_array, i, shrunk_element)
            candidates = reflectz.array_append(candidates, new_array)
            j = j + 1
        }
        i = i + 1
    }
    
    damn candidates
}

slay smart_shrink(value) [] {
    vibes is_numeric_type(value) {
        damn shrink_towards_zero(value)
    } mil is_string_type(value) {
        damn shrink_towards_empty_string(value)
    } mil is_array_type(value) {
        sus structural_shrinks [] = shrink_towards_empty_array(value)
        sus element_type tea = get_array_element_type(value)
        
        vibes element_type == "normie" {
            sus element_shrinks [] = shrink_array_elements(value, shrink_towards_zero)
            damn reflectz.array_concat(structural_shrinks, element_shrinks)
        } mil element_type == "tea" {
            sus element_shrinks [] = shrink_array_elements(value, shrink_towards_empty_string)
            damn reflectz.array_concat(structural_shrinks, element_shrinks)
        } nah {
            damn structural_shrinks
        }
    }
    
    damn []
}

fr fr ===== PROPERTY EXECUTION ENGINE =====

slay property_test_with_timeout(property_fn slay, input, timeout_ms normie) lit {
    sus start_time normie = timez.timestamp_millis()
    sus result lit = cap
    sus completed lit = cap
    
    yikes {
        result = property_fn(input)
        completed = based
    } fam {
        when _ -> {
            result = cap
            completed = based
        }
    }
    
    sus end_time normie = timez.timestamp_millis()
    sus elapsed normie = end_time - start_time
    
    vibes elapsed > timeout_ms {
        vibes property_verbose_logging {
            vibez.spill("Property test timed out after " + stringz.from_int(elapsed) + "ms")
        }
        damn cap
    }
    
    damn completed && result
}

slay execute_shrinking_search(property_fn slay, failing_input) {
    vibes property_verbose_logging {
        vibez.spill("Starting shrinking search for: " + stringz.to_string(failing_input))
    }
    
    sus candidates [] = smart_shrink(failing_input)
    sus shrink_attempts normie = 0
    sus best_failure = failing_input
    
    bestie shrink_attempts < property_max_shrinks && reflectz.array_length(candidates) > 0 {
        sus new_candidates [] = []
        sus i normie = 0
        
        bestie i < reflectz.array_length(candidates) {
            sus candidate = reflectz.array_get(candidates, i)
            
            vibes !property_test_with_timeout(property_fn, candidate, property_timeout_ms) {
                vibes property_verbose_logging {
                    vibez.spill("Found smaller failing case: " + stringz.to_string(candidate))
                }
                best_failure = candidate
                sus further_shrinks [] = smart_shrink(candidate)
                new_candidates = reflectz.array_concat(new_candidates, further_shrinks)
            }
            
            i = i + 1
            shrink_attempts = shrink_attempts + 1
        }
        
        candidates = new_candidates
    }
    
    vibes property_verbose_logging {
        vibez.spill("Shrinking completed after " + stringz.from_int(shrink_attempts) + " attempts")
        vibez.spill("Minimal failing case: " + stringz.to_string(best_failure))
    }
    
    damn best_failure
}

fr fr ===== MAIN PROPERTY TESTING INTERFACE =====

slay run_property_test_enhanced(
    property_fn slay,
    generator_fn slay, 
    description tea,
    config []
) lit {
    test_start("Property: " + description)
    
    fr fr Parse configuration
    sus test_count normie = property_test_count
    sus max_shrinks normie = property_max_shrinks  
    sus timeout normie = property_timeout_ms
    sus verbose lit = property_verbose_logging
    
    vibes reflectz.array_length(config) > 0 {
        sus i normie = 0
        bestie i < reflectz.array_length(config) - 1 {
            sus key tea = reflectz.array_get(config, i)
            sus value = reflectz.array_get(config, i + 1)
            
            vibes stringz.compare(key, "test_count") == 0 {
                test_count = value
            } mil stringz.compare(key, "max_shrinks") == 0 {
                max_shrinks = value
            } mil stringz.compare(key, "timeout_ms") == 0 {
                timeout = value
            } mil stringz.compare(key, "verbose") == 0 {
                verbose = value
            }
            
            i = i + 2
        }
    }
    
    sus passed_count normie = 0
    sus failed_count normie = 0
    sus execution_times [] = []
    sus i normie = 0
    
    vibes verbose {
        vibez.spill("Running " + stringz.from_int(test_count) + " property tests")
    }
    
    bestie i < test_count {
        sus generated_input = generator_fn()
        
        timing_start_measurement()
        sus test_passed lit = property_test_with_timeout(property_fn, generated_input, timeout)
        sus execution_time normie = timing_end_measurement()
        
        execution_times = reflectz.array_append(execution_times, execution_time)
        
        vibes test_passed {
            passed_count = passed_count + 1
            vibes verbose && (i + 1) % 10 == 0 {
                vibez.spill("Completed " + stringz.from_int(i + 1) + " tests...")
            }
        } nah {
            failed_count = failed_count + 1
            vibes verbose {
                vibez.spill("Property failed with input: " + stringz.to_string(generated_input))
            }
            
            sus minimal_failure = execute_shrinking_search(property_fn, generated_input)
            
            vibez.spill("PROPERTY FAILURE:")
            vibez.spill("Description: " + description)
            vibez.spill("Original failing input: " + stringz.to_string(generated_input))
            vibez.spill("Minimal failing input: " + stringz.to_string(minimal_failure))
            
            assert_true(cap)
            damn cap
        }
        
        i = i + 1
    }
    
    fr fr Calculate statistics
    sus total_time normie = 0
    sus min_time normie = reflectz.array_get(execution_times, 0)
    sus max_time normie = min_time
    i = 0
    
    bestie i < reflectz.array_length(execution_times) {
        sus time normie = reflectz.array_get(execution_times, i)
        total_time = total_time + time
        vibes time < min_time {
            min_time = time
        }
        vibes time > max_time {
            max_time = time
        }
        i = i + 1
    }
    
    sus avg_time normie = total_time / test_count
    
    vibez.spill("PROPERTY TEST RESULTS:")
    vibez.spill("Description: " + description)
    vibez.spill("Tests run: " + stringz.from_int(test_count))
    vibez.spill("Passed: " + stringz.from_int(passed_count))
    vibez.spill("Failed: " + stringz.from_int(failed_count))
    vibez.spill("Success rate: " + stringz.from_int((passed_count * 100) / test_count) + "%")
    vibez.spill("Average execution time: " + stringz.from_int(avg_time) + " nanoseconds")
    vibez.spill("Min execution time: " + stringz.from_int(min_time) + " nanoseconds") 
    vibez.spill("Max execution time: " + stringz.from_int(max_time) + " nanoseconds")
    
    assert_true(passed_count == test_count)
    damn based
}

fr fr ===== CONVENIENCE FUNCTIONS =====

slay forall_enhanced(generator_fn slay, property_fn slay, description tea) lit {
    damn run_property_test_enhanced(property_fn, generator_fn, description, [])
}

slay forall_with_config(generator_fn slay, property_fn slay, description tea, config []) lit {
    damn run_property_test_enhanced(property_fn, generator_fn, description, config)
}

slay set_global_config(test_count normie, max_shrinks normie, timeout_ms normie, verbose lit) {
    property_test_count = test_count
    property_max_shrinks = max_shrinks
    property_timeout_ms = timeout_ms
    property_verbose_logging = verbose
    
    fr fr Calibrate timing overhead
    timing_calibrate()
}

fr fr ===== INITIALIZATION =====

slay init_property_testing() {
    rand_init()
    timing_calibrate()
    
    vibes property_current_seed == 0 {
        property_current_seed = timez.timestamp_nanos() % 1000000
        rand_seed(property_current_seed)
    }
    
    vibes property_verbose_logging {
        vibez.spill("Property testing framework initialized")
        vibez.spill("Seed: " + stringz.from_int(property_current_seed))
        vibez.spill("Timing overhead: " + stringz.from_int(timing_overhead) + " nanoseconds")
    }
}

fr fr Initialize on module load
init_property_testing()
