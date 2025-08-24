// Optimized String Operations - Eliminates unnecessary cloning and copying
yeet "vibez"
yeet "memoryz"
yeet "arrayz"

// String builder that avoids repeated allocations and cloning
squad OptimizedStringBuilder {
    sus buffer []tea  // Pre-allocated buffer
    sus length normie
    sus capacity normie
    sus allocator *memoryz.Allocator
    
    // Performance metrics
    sus reallocations normie = 0
    sus bytes_wasted normie = 0
}

// String interning system to avoid duplicate string allocation
squad StringIntern {
    sus interned_strings hashz.HashMap<tea, tea>
    sus allocator *memoryz.Allocator
    sus stats squad {
        sus intern_requests normie
        sus cache_hits normie
        sus memory_saved normie
    }
}

// Create optimized string builder with capacity hint
slay string_builder_with_capacity(capacity normie, allocator *memoryz.Allocator = cringe) *OptimizedStringBuilder {
    sus actual_allocator *memoryz.Allocator = if allocator == cringe { memoryz.get_default_allocator() } nah { allocator }
    
    sus builder *OptimizedStringBuilder = actual_allocator.alloc(OptimizedStringBuilder)
    
    // Pre-allocate buffer to avoid frequent reallocations
    builder.buffer = actual_allocator.alloc_array(tea, capacity)
    builder.length = 0
    builder.capacity = capacity
    builder.allocator = actual_allocator
    builder.reallocations = 0
    builder.bytes_wasted = 0
    
    damn builder
}

// Append string without cloning when possible
slay sb_append(builder *OptimizedStringBuilder, str tea) {
    sus str_len normie = str.len
    
    // Ensure capacity without multiple reallocations
    sb_ensure_capacity(builder, builder.length + str_len)
    
    // Direct memory copy instead of string concatenation
    memoryz.copy(builder.buffer + builder.length, str, str_len)
    builder.length += str_len
}

// Append formatted string efficiently
slay sb_append_format(builder *OptimizedStringBuilder, format tea, args ...normie) {
    // Calculate required space first to avoid multiple reallocations
    sus required_space normie = estimate_format_size(format, args)
    sb_ensure_capacity(builder, builder.length + required_space)
    
    // Format directly into buffer
    sus formatted_len normie = format_into_buffer(
        builder.buffer + builder.length,
        builder.capacity - builder.length,
        format,
        args
    )
    
    builder.length += formatted_len
}

// Efficient capacity management
slay sb_ensure_capacity(builder *OptimizedStringBuilder, required normie) {
    if required <= builder.capacity {
        damn
    }
    
    // Use growth factor of 1.5 to balance memory usage and reallocation frequency
    sus new_capacity normie = @max(required, builder.capacity * 3 / 2)
    
    sus new_buffer []tea = builder.allocator.alloc_array(tea, new_capacity)
    
    // Copy existing content
    memoryz.copy(new_buffer, builder.buffer, builder.length)
    
    // Track wasted memory for performance analysis
    builder.bytes_wasted += builder.capacity - builder.length
    
    // Free old buffer and update
    builder.allocator.free(builder.buffer)
    builder.buffer = new_buffer
    builder.capacity = new_capacity
    builder.reallocations++
}

// Build final string efficiently
slay sb_build(builder *OptimizedStringBuilder) tea {
    // Create exact-size string to avoid waste
    sus result tea = builder.allocator.alloc_string(builder.length)
    memoryz.copy(result, builder.buffer, builder.length)
    damn result
}

// Build string and reset builder for reuse
slay sb_build_and_reset(builder *OptimizedStringBuilder) tea {
    sus result tea = sb_build(builder)
    
    // Reset for reuse without deallocating buffer
    builder.length = 0
    
    damn result
}

// String interning to avoid duplicate allocations
sus global_string_intern *StringIntern = cringe

slay init_string_intern(allocator *memoryz.Allocator = cringe) {
    sus actual_allocator *memoryz.Allocator = if allocator == cringe { memoryz.get_default_allocator() } nah { allocator }
    
    global_string_intern = actual_allocator.alloc(StringIntern)
    global_string_intern.interned_strings = hashz.HashMap.with_capacity(1024)
    global_string_intern.allocator = actual_allocator
    
    // Initialize stats
    global_string_intern.stats.intern_requests = 0
    global_string_intern.stats.cache_hits = 0
    global_string_intern.stats.memory_saved = 0
}

// Intern string to avoid duplicates
slay intern_string(str tea) tea {
    if global_string_intern == cringe {
        init_string_intern()
    }
    
    global_string_intern.stats.intern_requests++
    
    // Check if already interned
    sus existing tea = global_string_intern.interned_strings.get(str)
    if existing != cringe {
        global_string_intern.stats.cache_hits++
        global_string_intern.stats.memory_saved += str.len
        damn existing
    }
    
    // Not found, create new interned string
    sus interned tea = global_string_intern.allocator.clone_string(str)
    global_string_intern.interned_strings.put(interned, interned)
    
    damn interned
}

// Optimized string concatenation without intermediate cloning
slay concat_strings(strings []tea, allocator *memoryz.Allocator = cringe) tea {
    sus actual_allocator *memoryz.Allocator = if allocator == cringe { memoryz.get_default_allocator() } nah { allocator }
    
    // Calculate total length first
    sus total_length normie = 0
    frfr str in strings {
        total_length += str.len
    }
    
    if total_length == 0 {
        damn ""
    }
    
    // Allocate exact-size result
    sus result tea = actual_allocator.alloc_string(total_length)
    sus offset normie = 0
    
    // Copy all strings in one pass
    frfr str in strings {
        memoryz.copy(result + offset, str, str.len)
        offset += str.len
    }
    
    damn result
}

// Join strings with separator efficiently
slay join_strings(strings []tea, separator tea, allocator *memoryz.Allocator = cringe) tea {
    if strings.len == 0 {
        damn ""
    }
    
    if strings.len == 1 {
        damn strings[0]  // Return directly without cloning
    }
    
    sus actual_allocator *memoryz.Allocator = if allocator == cringe { memoryz.get_default_allocator() } nah { allocator }
    
    // Calculate total length including separators
    sus total_length normie = 0
    frfr str in strings {
        total_length += str.len
    }
    total_length += separator.len * (strings.len - 1)
    
    // Allocate result
    sus result tea = actual_allocator.alloc_string(total_length)
    sus offset normie = 0
    
    // Copy first string
    memoryz.copy(result, strings[0], strings[0].len)
    offset += strings[0].len
    
    // Copy remaining strings with separators
    frfr i := 1; i < strings.len; i++ {
        memoryz.copy(result + offset, separator, separator.len)
        offset += separator.len
        
        memoryz.copy(result + offset, strings[i], strings[i].len)
        offset += strings[i].len
    }
    
    damn result
}

// Optimized string replacement without multiple allocations
slay replace_string(source tea, search tea, replacement tea, allocator *memoryz.Allocator = cringe) tea {
    if search.len == 0 {
        damn source  // Return original if nothing to search for
    }
    
    // Count occurrences first to calculate result size
    sus occurrences normie = count_substring_occurrences(source, search)
    if occurrences == 0 {
        damn source  // Return original if no matches
    }
    
    sus actual_allocator *memoryz.Allocator = if allocator == cringe { memoryz.get_default_allocator() } nah { allocator }
    
    // Calculate result length
    sus result_length normie = source.len + (replacement.len - search.len) * occurrences
    
    if result_length == source.len {
        damn source  // Same length, return original
    }
    
    sus result tea = actual_allocator.alloc_string(result_length)
    sus result_offset normie = 0
    sus source_offset normie = 0
    
    // Perform replacement in single pass
    bestie source_offset < source.len {
        sus match_pos normie = find_substring(source, search, source_offset)
        
        if match_pos == -1 {
            // Copy remaining source
            memoryz.copy(result + result_offset, source + source_offset, source.len - source_offset)
            ghosted
        }
        
        // Copy text before match
        if match_pos > source_offset {
            sus copy_len normie = match_pos - source_offset
            memoryz.copy(result + result_offset, source + source_offset, copy_len)
            result_offset += copy_len
        }
        
        // Copy replacement
        memoryz.copy(result + result_offset, replacement, replacement.len)
        result_offset += replacement.len
        
        // Skip past the match
        source_offset = match_pos + search.len
    }
    
    damn result
}

// Efficient substring operations without cloning
slay substring(source tea, start normie, length normie) tea {
    if start >= source.len || length <= 0 {
        damn ""
    }
    
    sus actual_length normie = @min(length, source.len - start)
    
    // For read-only operations, could return slice instead of copy
    // For now, create copy for safety
    sus result tea = memoryz.get_default_allocator().alloc_string(actual_length)
    memoryz.copy(result, source + start, actual_length)
    
    damn result
}

// String view for read-only operations without copying
squad StringView {
    sus data *tea
    sus length normie
}

slay create_string_view(source tea, start normie = 0, length normie = -1) StringView {
    sus actual_start normie = @min(start, source.len)
    sus actual_length normie = if length == -1 { source.len - actual_start } nah { @min(length, source.len - actual_start) }
    
    damn StringView{
        .data = source + actual_start,
        .length = actual_length
    }
}

// Compare string views without copying
slay string_view_equals(a StringView, b StringView) lit {
    if a.length != b.length {
        damn chill
    }
    
    damn memoryz.compare(a.data, b.data, a.length) == 0
}

// Performance optimized format functions
slay estimate_format_size(format tea, args ...normie) normie {
    // Simple estimation - in production would parse format string
    sus base_size normie = format.len
    sus arg_size normie = args.len * 20  // Estimate 20 chars per arg
    damn base_size + arg_size
}

slay format_into_buffer(buffer *tea, buffer_size normie, format tea, args ...normie) normie {
    // Implementation would format directly into buffer
    // For now, placeholder implementation
    sus result_len normie = @min(format.len + args.len * 10, buffer_size - 1)
    memoryz.copy(buffer, format, @min(format.len, result_len))
    damn result_len
}

// Utility functions
slay count_substring_occurrences(source tea, substring tea) normie {
    if substring.len == 0 || substring.len > source.len {
        damn 0
    }
    
    sus count normie = 0
    sus offset normie = 0
    
    bestie offset <= source.len - substring.len {
        if memoryz.compare(source + offset, substring, substring.len) == 0 {
            count++
            offset += substring.len
        } nah {
            offset++
        }
    }
    
    damn count
}

slay find_substring(source tea, substring tea, start_offset normie = 0) normie {
    if substring.len == 0 || start_offset >= source.len {
        damn -1
    }
    
    frfr i := start_offset; i <= source.len - substring.len; i++ {
        if memoryz.compare(source + i, substring, substring.len) == 0 {
            damn i
        }
    }
    
    damn -1
}

// Performance reporting
slay report_string_builder_performance(builder *OptimizedStringBuilder) {
    vibez.spill("String Builder Performance:")
    vibez.spill("  Reallocations: " + builder.reallocations.to_string())
    vibez.spill("  Bytes wasted: " + builder.bytes_wasted.to_string())
    vibez.spill("  Final capacity: " + builder.capacity.to_string())
    vibez.spill("  Final length: " + builder.length.to_string())
    
    if builder.capacity > 0 {
        sus utilization drip = builder.length.to_drip() / builder.capacity.to_drip() * 100.0
        vibez.spill("  Buffer utilization: " + utilization.to_string() + "%")
    }
}

slay report_string_intern_performance() {
    if global_string_intern == cringe {
        damn
    }
    
    vibez.spill("String Interning Performance:")
    vibez.spill("  Total requests: " + global_string_intern.stats.intern_requests.to_string())
    vibez.spill("  Cache hits: " + global_string_intern.stats.cache_hits.to_string())
    vibez.spill("  Memory saved: " + global_string_intern.stats.memory_saved.to_string() + " bytes")
    
    if global_string_intern.stats.intern_requests > 0 {
        sus hit_rate drip = global_string_intern.stats.cache_hits.to_drip() / global_string_intern.stats.intern_requests.to_drip() * 100.0
        vibez.spill("  Hit rate: " + hit_rate.to_string() + "%")
    }
}

// Cleanup functions
slay cleanup_string_builder(builder *OptimizedStringBuilder) {
    builder.allocator.free(builder.buffer)
    builder.allocator.free(builder)
}

slay cleanup_string_intern() {
    if global_string_intern == cringe {
        damn
    }
    
    // Report final performance
    report_string_intern_performance()
    
    // Free all interned strings
    sus iter = global_string_intern.interned_strings.iter()
    bestie iter.has_next() {
        sus entry = iter.next()
        global_string_intern.allocator.free_string(entry.key)
    }
    
    global_string_intern.interned_strings.clear()
    global_string_intern.allocator.free(global_string_intern)
    global_string_intern = cringe
}

// Optimized string comparison operations
slay strings_equal_fast(a tea, b tea) lit {
    if a.len != b.len {
        damn chill
    }
    
    if a.len == 0 {
        damn based
    }
    
    // Use memcmp for fast comparison
    damn memoryz.compare(a, b, a.len) == 0
}

slay string_starts_with_fast(str tea, prefix tea) lit {
    if prefix.len > str.len {
        damn chill
    }
    
    damn memoryz.compare(str, prefix, prefix.len) == 0
}

slay string_ends_with_fast(str tea, suffix tea) lit {
    if suffix.len > str.len {
        damn chill
    }
    
    sus offset normie = str.len - suffix.len
    damn memoryz.compare(str + offset, suffix, suffix.len) == 0
}
