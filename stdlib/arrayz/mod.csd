fr fr CURSED Array Operations Module - Simple Working Version
fr fr 
fr fr NOTE: CURSED has excellent built-in array support:
fr fr - Array creation: sus arr []drip = [1, 2, 3]
fr fr - Array indexing: arr[0], arr[1], etc.
fr fr - Array length: len(arr) (built-in function)
fr fr - Array types: []drip for integers, []tea for strings

slay sum_array(nums []drip) drip {
    sus total drip = 0
    sus i drip = 0
    bestie (i < len(nums)) {
        total = total + nums[i]
        i = i + 1
    }
    damn total
}

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
