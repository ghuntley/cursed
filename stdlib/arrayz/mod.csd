fr fr CURSED Array Operations Module - Essential Array Functions
fr fr Pure CURSED implementation for maximum compatibility
fr fr 
fr fr NOTE: CURSED has excellent built-in array support:
fr fr - Array creation: sus arr []drip = [1, 2, 3]
fr fr - Array indexing: arr[0], arr[1], etc.
fr fr - Array length: len(arr) (built-in function)
fr fr - Array types: []drip for integers, []tea for strings

fr fr ===== ARRAY ARITHMETIC =====

slay sum_array(nums []drip) drip {
    sus total drip = 0
    sus i drip = 0
    bestie (i < len(nums)) {
        total = total + nums[i]
        i = i + 1
    }
    damn total
}

slay average_array(nums []drip) drip {
    ready (len(nums) == 0) {
        damn 0
    }
    sus total drip = sum_array(nums)
    damn total / len(nums)
}

slay product_array(nums []drip) drip {
    ready (len(nums) == 0) {
        damn 0
    }
    sus product drip = 1
    sus i drip = 0
    bestie (i < len(nums)) {
        product = product * nums[i]
        i = i + 1
    }
    damn product
}

fr fr ===== ARRAY SEARCH =====

slay find_max(nums []drip) drip {
    ready (len(nums) == 0) {
        damn 0
    }
    sus max_val drip = nums[0]
    sus i drip = 1
    bestie (i < len(nums)) {
        ready (nums[i] > max_val) {
            max_val = nums[i]
        }
        i = i + 1
    }
    damn max_val
}

slay find_min(nums []drip) drip {
    ready (len(nums) == 0) {
        damn 0
    }
    sus min_val drip = nums[0]
    sus i drip = 1
    bestie (i < len(nums)) {
        ready (nums[i] < min_val) {
            min_val = nums[i]
        }
        i = i + 1
    }
    damn min_val
}

slay contains_value(nums []drip, value drip) lit {
    sus i drip = 0
    bestie (i < len(nums)) {
        ready (nums[i] == value) {
            damn based
        }
        i = i + 1
    }
    damn cringe
}

slay find_index(nums []drip, value drip) drip {
    sus i drip = 0
    bestie (i < len(nums)) {
        ready (nums[i] == value) {
            damn i
        }
        i = i + 1
    }
    damn -1
}

fr fr ===== ARRAY VALIDATION =====

slay is_empty_array(nums []drip) lit {
    damn len(nums) == 0
}

slay array_size(nums []drip) drip {
    damn len(nums)
}

slay arrays_equal_size(a []drip, b []drip) lit {
    damn len(a) == len(b)
}

fr fr ===== ARRAY COUNTING =====

slay count_positive(nums []drip) drip {
    sus count drip = 0
    sus i drip = 0
    bestie (i < len(nums)) {
        ready (nums[i] > 0) {
            count = count + 1
        }
        i = i + 1
    }
    damn count
}

slay count_negative(nums []drip) drip {
    sus count drip = 0
    sus i drip = 0
    bestie (i < len(nums)) {
        ready (nums[i] < 0) {
            count = count + 1
        }
        i = i + 1
    }
    damn count
}

slay count_zeros(nums []drip) drip {
    sus count drip = 0
    sus i drip = 0
    bestie (i < len(nums)) {
        ready (nums[i] == 0) {
            count = count + 1
        }
        i = i + 1
    }
    damn count
}

slay count_occurrences(nums []drip, value drip) drip {
    sus count drip = 0
    sus i drip = 0
    bestie (i < len(nums)) {
        ready (nums[i] == value) {
            count = count + 1
        }
        i = i + 1
    }
    damn count
}

fr fr ===== ARRAY BOUNDS CHECKING =====

slay is_valid_index(nums []drip, index drip) lit {
    damn index >= 0 && index < len(nums)
}

slay safe_get(nums []drip, index drip, default_value drip) drip {
    ready (is_valid_index(nums, index)) {
        damn nums[index]
    }
    damn default_value
}

fr fr ===== ARRAY PROPERTIES =====

slay all_positive(nums []drip) lit {
    sus i drip = 0
    bestie (i < len(nums)) {
        ready (nums[i] <= 0) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay all_negative(nums []drip) lit {
    sus i drip = 0
    bestie (i < len(nums)) {
        ready (nums[i] >= 0) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay has_duplicates(nums []drip) lit {
    sus i drip = 0
    bestie (i < len(nums)) {
        sus j drip = i + 1
        bestie (j < len(nums)) {
            ready (nums[i] == nums[j]) {
                damn based
            }
            j = j + 1
        }
        i = i + 1
    }
    damn cringe
}

fr fr ===== STRING ARRAY FUNCTIONS =====

slay join_string_array(strings []tea, separator tea) tea {
    ready (len(strings) == 0) {
        damn ""
    }
    sus result tea = strings[0]
    sus i drip = 1
    bestie (i < len(strings)) {
        result = result + separator + strings[i]
        i = i + 1
    }
    damn result
}

slay concat_string_array(strings []tea) tea {
    sus result tea = ""
    sus i drip = 0
    bestie (i < len(strings)) {
        result = result + strings[i]
        i = i + 1
    }
    damn result
}

slay string_array_contains(strings []tea, value tea) lit {
    sus i drip = 0
    bestie (i < len(strings)) {
        ready (strings[i] == value) {
            damn based
        }
        i = i + 1
    }
    damn cringe
}
