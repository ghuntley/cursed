yeet "enhanced_mod"
yeet "reflectz"
yeet "stringz"
yeet "mathz"
yeet "timez"

fr fr Advanced Generator Library for Property Testing
fr fr Sophisticated test data generation with distribution control

fr fr ===== STATISTICAL GENERATORS =====

slay gen_normal_int(mean normie, std_dev normie) normie {
    fr fr Box-Muller transformation for normal distribution
    sus static_has_spare lit = cap
    sus static_spare drip = 0.0
    
    vibes static_has_spare {
        static_has_spare = cap
        damn normie(drip(mean) + static_spare * drip(std_dev))
    }
    
    sus u drip = 0.0
    sus v drip = 0.0
    sus s drip = 0.0
    
    bestie s >= 1.0 || s == 0.0 {
        u = rand_float() * 2.0 - 1.0
        v = rand_float() * 2.0 - 1.0
        s = u * u + v * v
    }
    
    sus multiplier drip = mathz.sqrt(-2.0 * mathz.log(s) / s)
    static_spare = v * multiplier
    static_has_spare = based
    
    damn normie(drip(mean) + u * multiplier * drip(std_dev))
}

slay gen_exponential_int(lambda drip) normie {
    sus u drip = rand_float()
    sus result drip = -mathz.log(1.0 - u) / lambda
    damn mathz.max(1, normie(result))
}

slay gen_power_law_int(alpha drip, min_val normie, max_val normie) normie {
    sus u drip = rand_float()
    sus range drip = drip(max_val - min_val)
    sus result drip = mathz.pow(u, 1.0 / alpha) * range + drip(min_val)
    damn mathz.max(min_val, mathz.min(max_val, normie(result)))
}

slay gen_zipf_int(n normie, s drip) normie {
    fr fr Zipf distribution using rejection sampling
    sus harmonic drip = 0.0
    sus i normie = 1
    bestie i <= n {
        harmonic = harmonic + 1.0 / mathz.pow(drip(i), s)
        i = i + 1
    }
    
    sus target drip = rand_float() * harmonic
    sus cumulative drip = 0.0
    i = 1
    
    bestie i <= n {
        cumulative = cumulative + 1.0 / mathz.pow(drip(i), s)
        vibes cumulative >= target {
            damn i
        }
        i = i + 1
    }
    
    damn n
}

fr fr ===== FREQUENCY-BASED GENERATORS =====

slay gen_weighted_choice(choices [], weights []) {
    sus total_weight drip = 0.0
    sus i normie = 0
    bestie i < reflectz.array_length(weights) {
        total_weight = total_weight + reflectz.array_get(weights, i)
        i = i + 1
    }
    
    sus random_weight drip = rand_float() * total_weight
    sus cumulative drip = 0.0
    i = 0
    
    bestie i < reflectz.array_length(choices) {
        cumulative = cumulative + reflectz.array_get(weights, i)
        vibes cumulative >= random_weight {
            damn reflectz.array_get(choices, i)
        }
        i = i + 1
    }
    
    damn reflectz.array_get(choices, 0)  fr fr Fallback
}

slay gen_frequency_string(char_frequencies []) tea {
    sus length normie = rand_range(1, 50)
    sus result tea = ""
    sus i normie = 0
    
    bestie i < length {
        sus chars [] = []
        sus weights [] = []
        sus j normie = 0
        
        bestie j < reflectz.array_length(char_frequencies) {
            sus entry [] = reflectz.array_get(char_frequencies, j)
            chars = reflectz.array_append(chars, reflectz.array_get(entry, 0))
            weights = reflectz.array_append(weights, reflectz.array_get(entry, 1))
            j = j + 1
        }
        
        sus chosen_char tea = gen_weighted_choice(chars, weights)
        result = result + chosen_char
        i = i + 1
    }
    
    damn result
}

fr fr ===== DOMAIN-SPECIFIC GENERATORS =====

slay gen_email_realistic() tea {
    sus first_names [] = ["john", "jane", "alice", "bob", "charlie", "diana", "eve", "frank"]
    sus last_names [] = ["smith", "johnson", "brown", "davis", "miller", "wilson", "moore", "taylor"]
    sus domains [] = ["gmail.com", "yahoo.com", "hotmail.com", "outlook.com", "company.com"]
    
    sus first_name tea = gen_weighted_choice(first_names, [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0])
    sus last_name tea = gen_weighted_choice(last_names, [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0])
    sus domain tea = gen_weighted_choice(domains, [3.0, 2.0, 1.5, 1.0, 0.5])
    
    sus separator tea = ""
    vibes rand_next() % 3 == 0 {
        separator = "."
    } mil rand_next() % 3 == 1 {
        separator = "_"
    } nah {
        separator = ""
        sus number normie = rand_range(1, 999)
        separator = stringz.from_int(number)
    }
    
    damn first_name + separator + last_name + "@" + domain
}

slay gen_phone_number(format tea) tea {
    vibes stringz.compare(format, "US") == 0 {
        sus area normie = rand_range(200, 999)
        sus exchange normie = rand_range(200, 999)  
        sus number normie = rand_range(1000, 9999)
        damn "(" + stringz.from_int(area) + ") " + stringz.from_int(exchange) + "-" + stringz.from_int(number)
    } mil stringz.compare(format, "international") == 0 {
        sus country normie = rand_range(1, 999)
        sus area normie = rand_range(1, 999)
        sus number normie = rand_range(100000, 9999999)
        damn "+" + stringz.from_int(country) + "-" + stringz.from_int(area) + "-" + stringz.from_int(number)
    }
    
    damn "555-0123"  fr fr Fallback
}

slay gen_credit_card(type tea) tea {
    vibes stringz.compare(type, "visa") == 0 {
        sus prefix tea = "4"
        sus remaining normie = 15
        sus i normie = 0
        bestie i < remaining {
            prefix = prefix + stringz.from_int(rand_range(0, 9))
            i = i + 1
        }
        damn prefix
    } mil stringz.compare(type, "mastercard") == 0 {
        sus prefix tea = "5" + stringz.from_int(rand_range(1, 5))
        sus remaining normie = 14
        sus i normie = 0
        bestie i < remaining {
            prefix = prefix + stringz.from_int(rand_range(0, 9))
            i = i + 1
        }
        damn prefix
    } mil stringz.compare(type, "amex") == 0 {
        sus prefixes [] = ["34", "37"]
        sus chosen tea = gen_weighted_choice(prefixes, [1.0, 1.0])
        sus remaining normie = 13
        sus i normie = 0
        bestie i < remaining {
            chosen = chosen + stringz.from_int(rand_range(0, 9))
            i = i + 1
        }
        damn chosen
    }
    
    damn "4111111111111111"  fr fr Fallback test card
}

slay gen_url(scheme tea, with_params lit) tea {
    sus schemes [] = ["http", "https", "ftp", "file"]
    sus selected_scheme tea = scheme
    vibes stringz.compare(scheme, "") == 0 {
        selected_scheme = gen_weighted_choice(schemes, [1.0, 3.0, 0.5, 0.2])
    }
    
    sus domains [] = ["example.com", "test.org", "demo.net", "sample.io", "localhost"]
    sus domain tea = gen_weighted_choice(domains, [2.0, 1.5, 1.0, 1.0, 0.5])
    
    sus paths [] = ["/", "/api", "/users", "/products", "/admin", "/docs"]
    sus path tea = gen_weighted_choice(paths, [3.0, 2.0, 1.5, 1.0, 0.5, 1.0])
    
    sus result tea = selected_scheme + "://" + domain + path
    
    vibes with_params && rand_next() % 3 == 0 {
        sus param_count normie = rand_range(1, 4)
        result = result + "?"
        sus i normie = 0
        bestie i < param_count {
            vibes i > 0 {
                result = result + "&"
            }
            sus param_name tea = "param" + stringz.from_int(i + 1)
            sus param_value normie = rand_range(1, 100)
            result = result + param_name + "=" + stringz.from_int(param_value)
            i = i + 1
        }
    }
    
    damn result
}

fr fr ===== TIME AND DATE GENERATORS =====

slay gen_timestamp_range(start_year normie, end_year normie) normie {
    sus start_timestamp normie = (start_year - 1970) * 365 * 24 * 60 * 60
    sus end_timestamp normie = (end_year - 1970) * 365 * 24 * 60 * 60
    damn rand_range(start_timestamp, end_timestamp)
}

slay gen_iso_date_string() tea {
    sus year normie = rand_range(2000, 2030)
    sus month normie = rand_range(1, 12)
    sus day normie = rand_range(1, 28)  fr fr Safe day range
    
    sus month_str tea = stringz.pad_left(stringz.from_int(month), 2, "0")
    sus day_str tea = stringz.pad_left(stringz.from_int(day), 2, "0")
    
    damn stringz.from_int(year) + "-" + month_str + "-" + day_str
}

slay gen_time_string(format tea) tea {
    sus hour normie = 0
    sus minute normie = rand_range(0, 59)
    sus second normie = rand_range(0, 59)
    
    vibes stringz.compare(format, "24h") == 0 {
        hour = rand_range(0, 23)
        sus hour_str tea = stringz.pad_left(stringz.from_int(hour), 2, "0")
        sus min_str tea = stringz.pad_left(stringz.from_int(minute), 2, "0")
        sus sec_str tea = stringz.pad_left(stringz.from_int(second), 2, "0")
        damn hour_str + ":" + min_str + ":" + sec_str
    } nah {
        hour = rand_range(1, 12)
        sus ampm tea = "AM"
        vibes rand_next() % 2 == 0 {
            ampm = "PM"
        }
        damn stringz.from_int(hour) + ":" + stringz.pad_left(stringz.from_int(minute), 2, "0") + " " + ampm
    }
}

fr fr ===== STRUCTURED DATA GENERATORS =====

slay gen_json_like_object(max_depth normie) tea {
    vibes max_depth <= 0 {
        sus value_types [] = ["string", "number", "boolean", "null"]
        sus chosen tea = gen_weighted_choice(value_types, [2.0, 2.0, 1.0, 0.5])
        
        vibes stringz.compare(chosen, "string") == 0 {
            damn "\"" + gen_ascii_string_pattern("alphanumeric", rand_range(3, 15)) + "\""
        } mil stringz.compare(chosen, "number") == 0 {
            damn stringz.from_int(rand_range(-1000, 1000))
        } mil stringz.compare(chosen, "boolean") == 0 {
            vibes rand_next() % 2 == 0 {
                damn "true"
            } nah {
                damn "false"
            }
        } nah {
            damn "null"
        }
    }
    
    sus container_types [] = ["object", "array"]
    sus chosen tea = gen_weighted_choice(container_types, [2.0, 1.0])
    
    vibes stringz.compare(chosen, "object") == 0 {
        sus field_count normie = rand_range(1, 5)
        sus result tea = "{"
        sus i normie = 0
        bestie i < field_count {
            vibes i > 0 {
                result = result + ","
            }
            sus field_name tea = "field" + stringz.from_int(i + 1)
            sus field_value tea = gen_json_like_object(max_depth - 1)
            result = result + "\"" + field_name + "\":" + field_value
            i = i + 1
        }
        result = result + "}"
        damn result
    } nah {
        sus element_count normie = rand_range(0, 4)
        sus result tea = "["
        sus i normie = 0
        bestie i < element_count {
            vibes i > 0 {
                result = result + ","
            }
            sus element tea = gen_json_like_object(max_depth - 1)
            result = result + element
            i = i + 1
        }
        result = result + "]"
        damn result
    }
}

slay gen_csv_row(column_types []) tea {
    sus result tea = ""
    sus i normie = 0
    bestie i < reflectz.array_length(column_types) {
        vibes i > 0 {
            result = result + ","
        }
        
        sus col_type tea = reflectz.array_get(column_types, i)
        vibes stringz.compare(col_type, "string") == 0 {
            sus value tea = gen_ascii_string_pattern("alpha", rand_range(3, 12))
            result = result + "\"" + value + "\""
        } mil stringz.compare(col_type, "int") == 0 {
            result = result + stringz.from_int(rand_range(-1000, 1000))
        } mil stringz.compare(col_type, "float") == 0 {
            sus float_val drip = gen_float_range(-1000.0, 1000.0)
            result = result + stringz.from_float(float_val, 2)
        } mil stringz.compare(col_type, "boolean") == 0 {
            vibes rand_next() % 2 == 0 {
                result = result + "true"
            } nah {
                result = result + "false"
            }
        } nah {
            result = result + "unknown"
        }
        
        i = i + 1
    }
    damn result
}

fr fr ===== EDGE CASE GENERATORS =====

slay gen_boundary_int(center normie, radius normie) normie {
    sus boundary_points [] = [
        center - radius - 1,
        center - radius,
        center - 1,
        center,
        center + 1,
        center + radius,
        center + radius + 1
    ]
    
    damn gen_weighted_choice(boundary_points, [1.0, 2.0, 1.0, 0.5, 1.0, 2.0, 1.0])
}

slay gen_edge_case_string() tea {
    sus edge_cases [] = [
        "",                          fr fr Empty string
        " ",                         fr fr Single space
        "\n",                        fr fr Newline
        "\t",                        fr fr Tab
        "\"",                        fr fr Quote
        "'",                         fr fr Apostrophe
        "\\",                        fr fr Backslash
        "\0",                        fr fr Null character
        "🙂",                       fr fr Unicode emoji
        "中文",                      fr fr Non-Latin characters
        "a".repeat(1000),           fr fr Very long string
        "\u200B",                   fr fr Zero-width space
        "\r\n",                     fr fr Windows line ending
        "SELECT * FROM users;",     fr fr SQL injection attempt
        "<script>alert('xss')</script>", fr fr XSS attempt
        "../../../etc/passwd"       fr fr Path traversal attempt
    ]
    
    sus weights [] = [
        3.0, 2.0, 2.0, 2.0, 1.5, 1.5, 1.5, 1.0,
        1.0, 1.0, 0.5, 1.0, 2.0, 0.3, 0.3, 0.3
    ]
    
    damn gen_weighted_choice(edge_cases, weights)
}

slay gen_problematic_float() drip {
    sus special_values [] = [
        0.0,
        -0.0,
        1.0,
        -1.0,
        mathz.inf(),
        -mathz.inf(),
        mathz.nan(),
        mathz.epsilon(),
        mathz.max_float(),
        mathz.min_float(),
        3.14159265359,      fr fr Pi
        2.71828182846,      fr fr e
        1.41421356237,      fr fr sqrt(2)
        0.57721566490       fr fr Euler-Mascheroni constant
    ]
    
    sus weights [] = [
        2.0, 1.0, 2.0, 2.0, 0.5, 0.5, 0.5, 1.0,
        0.3, 0.3, 1.0, 1.0, 1.0, 1.0
    ]
    
    damn gen_weighted_choice(special_values, weights)
}

fr fr ===== COMPOSITE GENERATORS =====

slay gen_realistic_person() [] {
    sus first_names [] = ["Emma", "Liam", "Olivia", "Noah", "Ava", "Oliver", "Sophia", "Elijah"]
    sus last_names [] = ["Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis"]
    
    sus first_name tea = gen_weighted_choice(first_names, [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0])
    sus last_name tea = gen_weighted_choice(last_names, [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0])
    sus age normie = gen_normal_int(35, 15)
    sus email tea = stringz.to_lower(first_name) + "." + stringz.to_lower(last_name) + "@example.com"
    sus phone tea = gen_phone_number("US")
    
    damn [first_name, last_name, age, email, phone]
}

slay gen_financial_transaction() [] {
    sus transaction_types [] = ["debit", "credit", "transfer", "payment"]
    sus transaction_type tea = gen_weighted_choice(transaction_types, [3.0, 2.0, 1.0, 2.0])
    
    sus amount drip = 0.0
    vibes stringz.compare(transaction_type, "debit") == 0 {
        amount = gen_float_range(1.0, 5000.0)
    } mil stringz.compare(transaction_type, "credit") == 0 {
        amount = gen_float_range(10.0, 50000.0)
    } nah {
        amount = gen_float_range(5.0, 10000.0)
    }
    
    sus timestamp normie = gen_timestamp_range(2020, 2024)
    sus account normie = rand_range(1000000, 9999999)
    sus reference tea = "TXN" + stringz.from_int(rand_range(100000, 999999))
    
    damn [transaction_type, amount, timestamp, account, reference]
}

fr fr ===== GENERATOR COMBINATORS =====

slay gen_oneof(generators []) {
    sus index normie = rand_range(0, reflectz.array_length(generators) - 1)
    sus chosen_gen slay = reflectz.array_get(generators, index)
    damn chosen_gen()
}

slay gen_frequency(weighted_generators []) {
    sus total_weight drip = 0.0
    sus i normie = 0
    bestie i < reflectz.array_length(weighted_generators) {
        sus entry [] = reflectz.array_get(weighted_generators, i)
        total_weight = total_weight + reflectz.array_get(entry, 0)
        i = i + 1
    }
    
    sus random_weight drip = rand_float() * total_weight
    sus cumulative drip = 0.0
    i = 0
    
    bestie i < reflectz.array_length(weighted_generators) {
        sus entry [] = reflectz.array_get(weighted_generators, i)
        cumulative = cumulative + reflectz.array_get(entry, 0)
        vibes cumulative >= random_weight {
            sus chosen_gen slay = reflectz.array_get(entry, 1)
            damn chosen_gen()
        }
        i = i + 1
    }
    
    fr fr Fallback to first generator
    sus first_entry [] = reflectz.array_get(weighted_generators, 0)
    sus fallback_gen slay = reflectz.array_get(first_entry, 1)
    damn fallback_gen()
}

slay gen_sized(size_fn slay, gen_fn slay) slay {
    damn slay() {
        sus size normie = size_fn()
        damn gen_fn(size)
    }
}

slay gen_list_of(element_gen slay, size_gen slay) [] {
    sus size normie = size_gen()
    sus result [] = []
    sus i normie = 0
    
    bestie i < size {
        sus element = element_gen()
        result = reflectz.array_append(result, element)
        i = i + 1
    }
    
    damn result
}

slay gen_tuple(generators []) [] {
    sus result [] = []
    sus i normie = 0
    
    bestie i < reflectz.array_length(generators) {
        sus gen slay = reflectz.array_get(generators, i)
        sus value = gen()
        result = reflectz.array_append(result, value)
        i = i + 1
    }
    
    damn result
}

fr fr ===== PROPERTY GENERATOR HELPERS =====

slay gen_sorted_array(element_gen slay, size normie) [] {
    sus unsorted [] = []
    sus i normie = 0
    
    bestie i < size {
        sus element = element_gen()
        unsorted = reflectz.array_append(unsorted, element)
        i = i + 1
    }
    
    fr fr Simple insertion sort for demonstration
    i = 1
    bestie i < reflectz.array_length(unsorted) {
        sus key = reflectz.array_get(unsorted, i)
        sus j normie = i - 1
        
        bestie j >= 0 && reflectz.array_get(unsorted, j) > key {
            unsorted = reflectz.array_set(unsorted, j + 1, reflectz.array_get(unsorted, j))
            j = j - 1
        }
        
        unsorted = reflectz.array_set(unsorted, j + 1, key)
        i = i + 1
    }
    
    damn unsorted
}

slay gen_unique_array(element_gen slay, size normie) [] {
    sus result [] = []
    sus attempts normie = 0
    sus max_attempts normie = size * 10
    
    bestie reflectz.array_length(result) < size && attempts < max_attempts {
        sus candidate = element_gen()
        sus is_unique lit = based
        sus i normie = 0
        
        bestie i < reflectz.array_length(result) {
            vibes deep_equal(candidate, reflectz.array_get(result, i)) {
                is_unique = cap
                yeet
            }
            i = i + 1
        }
        
        vibes is_unique {
            result = reflectz.array_append(result, candidate)
        }
        
        attempts = attempts + 1
    }
    
    damn result
}

slay gen_biased_towards_edge_cases(normal_gen slay, edge_gen slay, edge_probability drip) {
    vibes rand_float() < edge_probability {
        damn edge_gen()
    } nah {
        damn normal_gen()
    }
}
