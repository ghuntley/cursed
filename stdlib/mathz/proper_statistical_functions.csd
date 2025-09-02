fr fr CURSED Proper Statistical Functions Module
fr fr Mathematically correct and robust statistical implementations
fr fr Includes proper sorting-based median, robust variance, and advanced statistics

yeet "testz"

fr fr ==========================================
fr fr SORTING ALGORITHMS FOR STATISTICAL FUNCTIONS
fr fr ==========================================

slay insertion_sort(arr meal[value], count drip) lit {
    fr fr Simple insertion sort for small arrays
    sus i drip = 1
    bestie (i < count) {
        sus key meal = arr[i]
        sus j drip = i - 1
        
        bestie (j >= 0 && arr[j] > key) {
            arr[j + 1] = arr[j]
            j = j - 1
        }
        arr[j + 1] = key
        i = i + 1
    }
    damn based
}

slay merge_arrays(arr meal[value], left drip, mid drip, right drip) lit {
    fr fr Merge function for merge sort
    sus left_size drip = mid - left + 1
    sus right_size drip = right - mid
    
    fr fr Create temporary arrays
    sus left_arr meal[value] = [0.0; left_size]
    sus right_arr meal[value] = [0.0; right_size]
    
    fr fr Copy data to temporary arrays
    sus i drip = 0
    bestie (i < left_size) {
        left_arr[i] = arr[left + i]
        i = i + 1
    }
    
    i = 0
    bestie (i < right_size) {
        right_arr[i] = arr[mid + 1 + i]
        i = i + 1
    }
    
    fr fr Merge the temporary arrays back
    sus left_idx drip = 0
    sus right_idx drip = 0
    sus merged_idx drip = left
    
    bestie (left_idx < left_size && right_idx < right_size) {
        ready (left_arr[left_idx] <= right_arr[right_idx]) {
            arr[merged_idx] = left_arr[left_idx]
            left_idx = left_idx + 1
        } sus {
            arr[merged_idx] = right_arr[right_idx]
            right_idx = right_idx + 1
        }
        merged_idx = merged_idx + 1
    }
    
    fr fr Copy remaining elements
    bestie (left_idx < left_size) {
        arr[merged_idx] = left_arr[left_idx]
        left_idx = left_idx + 1
        merged_idx = merged_idx + 1
    }
    
    bestie (right_idx < right_size) {
        arr[merged_idx] = right_arr[right_idx]
        right_idx = right_idx + 1
        merged_idx = merged_idx + 1
    }
    
    damn based
}

slay merge_sort(arr meal[value], left drip, right drip) lit {
    fr fr Merge sort for stable, O(n log n) sorting
    ready (left < right) {
        sus mid drip = left + (right - left) / 2
        
        merge_sort(arr, left, mid)
        merge_sort(arr, mid + 1, right)
        merge_arrays(arr, left, mid, right)
    }
    damn based
}

slay copy_array(source meal[value], dest meal[value], count drip) lit {
    fr fr Utility function to copy arrays
    sus i drip = 0
    bestie (i < count) {
        dest[i] = source[i]
        i = i + 1
    }
    damn based
}

fr fr ==========================================
fr fr PROPER MEDIAN CALCULATION
fr fr ==========================================

slay median_sorting_based(values meal[value], count drip) meal {
    fr fr Mathematically correct median using proper sorting
    fr fr Definition: middle value in sorted dataset
    ready (count <= 0) {
        damn 0.0
    }
    ready (count == 1) {
        damn values[0]
    }
    
    fr fr Create copy to avoid modifying original array
    sus sorted_values meal[value] = [0.0; count]
    copy_array(values, sorted_values, count)
    
    fr fr Sort using merge sort for guaranteed O(n log n)
    merge_sort(sorted_values, 0, count - 1)
    
    fr fr Calculate median according to mathematical definition
    ready (count % 2 == 1) {
        fr fr Odd number of elements: return middle element
        damn sorted_values[count / 2]
    }
    
    fr fr Even number of elements: return average of two middle elements
    sus mid1 drip = count / 2 - 1
    sus mid2 drip = count / 2
    damn (sorted_values[mid1] + sorted_values[mid2]) / 2.0
}

slay quantile(values meal[value], count drip, q meal) meal {
    fr fr Calculate q-quantile (q between 0 and 1)
    fr fr Uses proper interpolation method
    ready (count <= 0 || q < 0.0 || q > 1.0) {
        damn 0.0
    }
    
    sus sorted_values meal[value] = [0.0; count]
    copy_array(values, sorted_values, count)
    merge_sort(sorted_values, 0, count - 1)
    
    sus index meal = q * (count - 1).(meal)
    sus lower_index drip = floor_proper(index)
    sus upper_index drip = ceil_proper(index)
    
    ready (lower_index == upper_index) {
        damn sorted_values[lower_index]
    }
    
    fr fr Linear interpolation between adjacent values
    sus fraction meal = index - lower_index.(meal)
    damn sorted_values[lower_index] * (1.0 - fraction) + sorted_values[upper_index] * fraction
}

slay percentile(values meal[value], count drip, p meal) meal {
    fr fr Calculate percentile (p between 0 and 100)
    damn quantile(values, count, p / 100.0)
}

slay quartiles_proper(values meal[value], count drip) meal[value]{
    fr fr Calculate Q1, Q2 (median), Q3 using proper definitions
    sus result meal[value] = [0.0, 0.0, 0.0]
    ready (count <= 0) {
        damn result
    }
    
    result[0] = quantile(values, count, 0.25)  fr fr Q1
    result[1] = quantile(values, count, 0.50)  fr fr Q2 (median)
    result[2] = quantile(values, count, 0.75)  fr fr Q3
    
    damn result
}

fr fr ==========================================
fr fr ROBUST STATISTICAL MEASURES
fr fr ==========================================

slay mean_arithmetic_stable(values meal[value], count drip) meal {
    fr fr Numerically stable mean using Kahan summation
    ready (count <= 0) {
        damn 0.0
    }
    
    sus sum meal = 0.0
    sus compensation meal = 0.0  fr fr Compensation for lost low-order bits
    
    sus i drip = 0
    bestie (i < count) {
        sus corrected_input meal = values[i] - compensation
        sus new_sum meal = sum + corrected_input
        compensation = (new_sum - sum) - corrected_input
        sum = new_sum
        i = i + 1
    }
    
    damn sum / count.(meal)
}

slay variance_welford(values meal[value], count drip) meal {
    fr fr Welford's online algorithm for numerically stable variance
    fr fr This is the mathematically preferred method for variance calculation
    ready (count <= 1) {
        damn 0.0
    }
    
    sus mean_val meal = 0.0
    sus m2 meal = 0.0
    
    sus i drip = 0
    bestie (i < count) {
        sus delta meal = values[i] - mean_val
        mean_val = mean_val + delta / (i + 1).(meal)
        sus delta2 meal = values[i] - mean_val
        m2 = m2 + delta * delta2
        i = i + 1
    }
    
    damn m2 / (count - 1).(meal)  fr fr Sample variance (N-1 denominator)
}

slay variance_population_welford(values meal[value], count drip) meal {
    fr fr Population variance using Welford's algorithm
    ready (count <= 0) {
        damn 0.0
    }
    
    sus mean_val meal = 0.0
    sus m2 meal = 0.0
    
    sus i drip = 0
    bestie (i < count) {
        sus delta meal = values[i] - mean_val
        mean_val = mean_val + delta / (i + 1).(meal)
        sus delta2 meal = values[i] - mean_val
        m2 = m2 + delta * delta2
        i = i + 1
    }
    
    damn m2 / count.(meal)  fr fr Population variance (N denominator)
}

slay standard_deviation_sample(values meal[value], count drip) meal {
    sus variance_val meal = variance_welford(values, count)
    ready (variance_val <= 0.0) {
        damn 0.0
    }
    damn sqrt_stable(variance_val)
}

slay standard_deviation_population(values meal[value], count drip) meal {
    sus variance_val meal = variance_population_welford(values, count)
    ready (variance_val <= 0.0) {
        damn 0.0
    }
    damn sqrt_stable(variance_val)
}

fr fr ==========================================
fr fr ADVANCED STATISTICAL MEASURES
fr fr ==========================================

slay skewness_proper(values meal[value], count drip) meal {
    fr fr Sample skewness using the standard unbiased formula
    ready (count <= 2) {
        damn 0.0
    }
    
    sus mean_val meal = mean_arithmetic_stable(values, count)
    sus std_dev meal = standard_deviation_sample(values, count)
    ready (std_dev <= 0.0) {
        damn 0.0
    }
    
    sus third_moment meal = 0.0
    sus i drip = 0
    bestie (i < count) {
        sus standardized meal = (values[i] - mean_val) / std_dev
        third_moment = third_moment + standardized * standardized * standardized
        i = i + 1
    }
    
    fr fr Apply bias correction factor for sample skewness
    sus n meal = count.(meal)
    sus bias_correction meal = n / ((n - 1.0) * (n - 2.0))
    damn bias_correction * third_moment
}

slay kurtosis_proper(values meal[value], count drip) meal {
    fr fr Sample kurtosis using the standard unbiased formula
    ready (count <= 3) {
        damn 0.0
    }
    
    sus mean_val meal = mean_arithmetic_stable(values, count)
    sus std_dev meal = standard_deviation_sample(values, count)
    ready (std_dev <= 0.0) {
        damn 0.0
    }
    
    sus fourth_moment meal = 0.0
    sus i drip = 0
    bestie (i < count) {
        sus standardized meal = (values[i] - mean_val) / std_dev
        sus standardized_sq meal = standardized * standardized
        fourth_moment = fourth_moment + standardized_sq * standardized_sq
        i = i + 1
    }
    
    fr fr Apply bias correction for sample kurtosis
    sus n meal = count.(meal)
    sus bias_correction meal = (n * (n + 1.0)) / ((n - 1.0) * (n - 2.0) * (n - 3.0))
    sus excess_correction meal = (3.0 * (n - 1.0) * (n - 1.0)) / ((n - 2.0) * (n - 3.0))
    
    damn bias_correction * fourth_moment - excess_correction
}

slay moment_central(values meal[value], count drip, order drip) meal {
    fr fr Calculate central moment of specified order
    ready (count <= 0 || order < 0) {
        damn 0.0
    }
    ready (order == 0) {
        damn 1.0
    }
    ready (order == 1) {
        damn 0.0  fr fr First central moment is always zero
    }
    
    sus mean_val meal = mean_arithmetic_stable(values, count)
    sus moment meal = 0.0
    
    sus i drip = 0
    bestie (i < count) {
        sus deviation meal = values[i] - mean_val
        sus power meal = power_stable(deviation, order)
        moment = moment + power
        i = i + 1
    }
    
    damn moment / count.(meal)
}

fr fr ==========================================
fr fr ROBUST STATISTICS (OUTLIER RESISTANT)
fr fr ==========================================

slay median_absolute_deviation(values meal[value], count drip) meal {
    fr fr MAD - robust measure of variability
    ready (count <= 0) {
        damn 0.0
    }
    
    sus median_val meal = median_sorting_based(values, count)
    sus deviations meal[value] = [0.0; count]
    
    sus i drip = 0
    bestie (i < count) {
        deviations[i] = abs_float_stable(values[i] - median_val)
        i = i + 1
    }
    
    damn median_sorting_based(deviations, count)
}

slay interquartile_range(values meal[value], count drip) meal {
    fr fr IQR = Q3 - Q1, robust measure of spread
    sus q_values meal[value] = quartiles_proper(values, count)
    damn q_values[2] - q_values[0]  fr fr Q3 - Q1
}

slay trimmed_mean(values meal[value], count drip, trim_fraction meal) meal {
    fr fr Trimmed mean - remove extreme values before calculating mean
    ready (count <= 0 || trim_fraction < 0.0 || trim_fraction >= 0.5) {
        damn 0.0
    }
    
    sus sorted_values meal[value] = [0.0; count]
    copy_array(values, sorted_values, count)
    merge_sort(sorted_values, 0, count - 1)
    
    sus trim_count drip = floor_proper(count.(meal) * trim_fraction)
    sus start_idx drip = trim_count
    sus end_idx drip = count - trim_count
    sus valid_count drip = end_idx - start_idx
    
    ready (valid_count <= 0) {
        damn 0.0
    }
    
    sus sum meal = 0.0
    sus i drip = start_idx
    bestie (i < end_idx) {
        sum = sum + sorted_values[i]
        i = i + 1
    }
    
    damn sum / valid_count.(meal)
}

slay winsorized_mean(values meal[value], count drip, trim_fraction meal) meal {
    fr fr Winsorized mean - replace extreme values with boundary values
    ready (count <= 0 || trim_fraction < 0.0 || trim_fraction >= 0.5) {
        damn 0.0
    }
    
    sus sorted_values meal[value] = [0.0; count]
    copy_array(values, sorted_values, count)
    merge_sort(sorted_values, 0, count - 1)
    
    sus trim_count drip = floor_proper(count.(meal) * trim_fraction)
    sus lower_bound meal = sorted_values[trim_count]
    sus upper_bound meal = sorted_values[count - trim_count - 1]
    
    sus sum meal = 0.0
    sus i drip = 0
    bestie (i < count) {
        sus winsorized_value meal = values[i]
        ready (winsorized_value < lower_bound) {
            winsorized_value = lower_bound
        }
        ready (winsorized_value > upper_bound) {
            winsorized_value = upper_bound
        }
        sum = sum + winsorized_value
        i = i + 1
    }
    
    damn sum / count.(meal)
}

fr fr ==========================================
fr fr HELPER FUNCTIONS
fr fr ==========================================

slay abs_float_stable(x meal) meal {
    ready (x < 0.0) {
        damn -x
    }
    damn x
}

slay sqrt_stable(x meal) meal {
    fr fr Stable square root using Newton-Raphson
    ready (x <= 0.0) {
        damn 0.0
    }
    ready (x == 1.0) {
        damn 1.0
    }
    
    sus guess meal = x / 2.0
    sus epsilon meal = 1e-15
    sus max_iterations drip = 50
    
    sus i drip = 0
    bestie (i < max_iterations) {
        sus new_guess meal = (guess + x / guess) / 2.0
        ready (abs_float_stable(new_guess - guess) < epsilon) {
            damn new_guess
        }
        guess = new_guess
        i = i + 1
    }
    
    damn guess
}

slay power_stable(base meal, exponent drip) meal {
    fr fr Stable power function for positive integer exponents
    ready (exponent == 0) {
        damn 1.0
    }
    ready (exponent == 1) {
        damn base
    }
    ready (exponent < 0) {
        ready (abs_float_stable(base) < 1e-15) {
            damn 0.0  fr fr Avoid division by zero
        }
        damn 1.0 / power_stable(base, -exponent)
    }
    
    sus result meal = 1.0
    sus current_base meal = base
    sus current_exp drip = exponent
    
    bestie (current_exp > 0) {
        ready (current_exp % 2 == 1) {
            result = result * current_base
        }
        current_base = current_base * current_base
        current_exp = current_exp / 2
    }
    
    damn result
}

slay floor_proper(x meal) drip {
    sus int_part drip = x.(drip)
    ready (x >= 0.0 || x == int_part.(meal)) {
        damn int_part
    }
    damn int_part - 1
}

slay ceil_proper(x meal) drip {
    sus int_part drip = x.(drip)
    ready (x <= 0.0 || x == int_part.(meal)) {
        damn int_part
    }
    damn int_part + 1
}

fr fr ==========================================
fr fr TEST FUNCTIONS
fr fr ==========================================

slay test_statistical_functions() lit {
    fr fr Test proper statistical implementations
    
    fr fr Test median calculation
    sus test_odd meal[value] = [3.0, 1.0, 4.0, 1.0, 5.0]  fr fr Should be 3.0
    sus median_odd meal = median_sorting_based(test_odd, 5)
    ready (abs_float_stable(median_odd - 3.0) > 1e-10) {
        spill_facts("ERROR: Odd median failed, expected 3.0, got", median_odd)
        damn cringe
    }
    
    sus test_even meal[value] = [2.0, 4.0, 1.0, 3.0]  fr fr Should be 2.5
    sus median_even meal = median_sorting_based(test_even, 4)
    ready (abs_float_stable(median_even - 2.5) > 1e-10) {
        spill_facts("ERROR: Even median failed, expected 2.5, got", median_even)
        damn cringe
    }
    
    fr fr Test Welford's variance
    sus test_variance meal[value] = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]
    sus variance_result meal = variance_welford(test_variance, 8)
    ready (abs_float_stable(variance_result - 4.571428571) > 1e-6) {
        spill_facts("ERROR: Welford variance failed, expected ~4.57, got", variance_result)
        damn cringe
    }
    
    spill_facts("All statistical function tests passed!")
    damn based
}
