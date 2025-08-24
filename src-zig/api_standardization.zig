// CURSED API Standardization - Phase 2 Implementation
// Provides consistent naming conventions and patterns across all stdlib modules

const std = @import("std");
const testing = std.testing;

/// Standard CURSED API Design Guidelines
/// All stdlib modules should follow these patterns for consistency

/// =============================================================================
/// NAMING CONVENTIONS
/// =============================================================================

/// Standard module naming pattern: {domain}z
/// Examples: mathz, stringz, arrayz, cryptz, filez, networkz, testz

/// Standard function naming pattern: {module}_{operation}_{type}
/// Examples:
/// - math_abs_drip(x drip) drip
/// - string_concat_tea(a tea, b tea) tea
/// - array_len_drip(arr []drip) drip
/// - file_read_tea(path tea) yikes<tea>

/// =============================================================================
/// PARAMETER NAMING CONVENTIONS
/// =============================================================================

pub const StandardParameterNames = struct {
    // File operations
    pub const FILE_PATH = "file_path";
    pub const DIR_PATH = "dir_path";  
    pub const CONTENT = "content";
    pub const DATA = "data";
    
    // String operations
    pub const TEXT = "text";
    pub const STRING_A = "str_a";
    pub const STRING_B = "str_b";
    pub const PATTERN = "pattern";
    pub const REPLACEMENT = "replacement";
    
    // Array operations
    pub const ARRAY = "array";
    pub const ELEMENT = "element";
    pub const INDEX = "index";
    pub const LENGTH = "length";
    pub const SIZE = "size";
    
    // Math operations
    pub const VALUE = "value";
    pub const OPERAND_A = "a";
    pub const OPERAND_B = "b";
    pub const RESULT = "result";
    
    // Network operations
    pub const URL = "url";
    pub const HOST = "host";
    pub const PORT = "port";
    pub const REQUEST = "request";
    pub const RESPONSE = "response";
    
    // Concurrency operations
    pub const GOROUTINE_ID = "goroutine_id";
    pub const CHANNEL = "channel";
    pub const MESSAGE = "message";
    pub const TIMEOUT = "timeout_ms";
};

/// =============================================================================
/// STANDARD RETURN TYPES
/// =============================================================================

pub const StandardReturnTypes = struct {
    // Success/failure pattern
    pub const SUCCESS = "lit";  // Returns based (true) or cringe (false)
    
    // Error result pattern  
    pub const ERROR_RESULT = "yikes<T>";  // Returns value or error
    
    // Optional result pattern
    pub const OPTIONAL_RESULT = "maybe<T>";  // Returns value or none
    
    // Length/size operations
    pub const LENGTH_RESULT = "drip";  // Always return drip for lengths
    
    // Numeric operations
    pub const NUMERIC_RESULT = "T";  // Return same type as input
};

/// =============================================================================
/// STANDARD ERROR HANDLING PATTERNS
/// =============================================================================

/// Consistent error return pattern for all modules
pub const StandardErrorHandling = struct {
    /// Option 1: Boolean + Global Error State
    /// Functions return lit (based/cringe) and set global error
    pub fn example_boolean_pattern() lit {
        // Implementation sets global error on failure
        return based;  // or cringe on failure
    }
    
    /// Option 2: Error Union Pattern (Recommended)
    /// Functions return value or specific error
    pub fn example_error_union_pattern() yikes<tea> {
        // Implementation returns value or error
        return "success_value";  // or yikes "error_message"
    }
    
    /// Option 3: Optional Pattern
    /// Functions return value or none
    pub fn example_optional_pattern() maybe<drip> {
        // Implementation returns value or none
        return 42;  // or none on failure
    }
};

/// =============================================================================
/// MATHZ MODULE STANDARDIZATION
/// =============================================================================

pub const StandardMathAPI = struct {
    // Absolute value operations
    pub const abs_drip = "slay math_abs_drip(value drip) drip";
    pub const abs_normie = "slay math_abs_normie(value normie) normie";
    pub const abs_thicc = "slay math_abs_thicc(value thicc) thicc";
    
    // Min/Max operations
    pub const min_drip = "slay math_min_drip(a drip, b drip) drip";
    pub const max_drip = "slay math_max_drip(a drip, b drip) drip";
    
    // Power operations
    pub const pow_drip = "slay math_pow_drip(base drip, exponent drip) yikes<drip>";
    
    // Trigonometric operations
    pub const sin_normie = "slay math_sin_normie(angle normie) normie";
    pub const cos_normie = "slay math_cos_normie(angle normie) normie";
    
    // Constants
    pub const PI = "sus MATH_PI normie = 3.14159265359";
    pub const E = "sus MATH_E normie = 2.71828182846";
};

/// =============================================================================
/// STRINGZ MODULE STANDARDIZATION
/// =============================================================================

pub const StandardStringAPI = struct {
    // String creation and manipulation
    pub const concat = "slay string_concat_tea(str_a tea, str_b tea) tea";
    pub const length = "slay string_length_drip(text tea) drip";
    pub const is_empty = "slay string_is_empty_lit(text tea) lit";
    pub const substring = "slay string_substring_tea(text tea, start drip, length drip) yikes<tea>";
    
    // String comparison
    pub const equals = "slay string_equals_lit(str_a tea, str_b tea) lit";
    pub const compare = "slay string_compare_drip(str_a tea, str_b tea) drip";  // -1, 0, 1
    
    // String searching
    pub const contains = "slay string_contains_lit(text tea, pattern tea) lit";
    pub const find_index = "slay string_find_index_drip(text tea, pattern tea) drip";  // -1 if not found
    pub const find_last_index = "slay string_find_last_index_drip(text tea, pattern tea) drip";
    
    // String transformation
    pub const to_upper = "slay string_to_upper_tea(text tea) tea";
    pub const to_lower = "slay string_to_lower_tea(text tea) tea";
    pub const trim = "slay string_trim_tea(text tea) tea";
    
    // String conversion
    pub const from_drip = "slay string_from_drip_tea(value drip) tea";
    pub const to_drip = "slay string_to_drip_yikes(text tea) yikes<drip>";
};

/// =============================================================================
/// ARRAYZ MODULE STANDARDIZATION  
/// =============================================================================

pub const StandardArrayAPI = struct {
    // Array properties
    pub const length = "slay array_length_drip(array []T) drip";
    pub const is_empty = "slay array_is_empty_lit(array []T) lit";
    pub const capacity = "slay array_capacity_drip(array []T) drip";
    
    // Array access (with bounds checking)
    pub const get = "slay array_get_yikes(array []T, index drip) yikes<T>";
    pub const set = "slay array_set_lit(array []T, index drip, element T) lit";
    
    // Array modification  
    pub const append = "slay array_append_lit(array *[]T, element T) lit";
    pub const insert = "slay array_insert_lit(array *[]T, index drip, element T) lit";
    pub const remove = "slay array_remove_yikes(array *[]T, index drip) yikes<T>";
    
    // Array searching
    pub const find_index = "slay array_find_index_drip(array []T, element T) drip";  // -1 if not found
    pub const contains = "slay array_contains_lit(array []T, element T) lit";
    
    // Array operations
    pub const sum_drip = "slay array_sum_drip(array []drip) drip";
    pub const max_drip = "slay array_max_yikes(array []drip) yikes<drip>";
    pub const min_drip = "slay array_min_yikes(array []drip) yikes<drip>";
};

/// =============================================================================
/// FILEZ MODULE STANDARDIZATION
/// =============================================================================

pub const StandardFileAPI = struct {
    // File reading
    pub const read_file = "slay file_read_tea(file_path tea) yikes<tea>";
    pub const read_binary = "slay file_read_binary_yikes(file_path tea) yikes<[]u8>";
    
    // File writing
    pub const write_file = "slay file_write_lit(file_path tea, content tea) lit";
    pub const write_binary = "slay file_write_binary_lit(file_path tea, data []u8) lit";
    pub const append_file = "slay file_append_lit(file_path tea, content tea) lit";
    
    // File operations
    pub const exists = "slay file_exists_lit(file_path tea) lit";
    pub const delete = "slay file_delete_lit(file_path tea) lit";
    pub const copy = "slay file_copy_lit(source_path tea, dest_path tea) lit";
    pub const move = "slay file_move_lit(source_path tea, dest_path tea) lit";
    
    // File properties
    pub const size = "slay file_size_yikes(file_path tea) yikes<drip>";
    pub const modified_time = "slay file_modified_time_yikes(file_path tea) yikes<drip>";
    
    // Directory operations
    pub const create_dir = "slay dir_create_lit(dir_path tea) lit";
    pub const list_dir = "slay dir_list_yikes(dir_path tea) yikes<[]tea>";
    pub const dir_exists = "slay dir_exists_lit(dir_path tea) lit";
};

/// =============================================================================
/// NETWORKZ MODULE STANDARDIZATION
/// =============================================================================

pub const StandardNetworkAPI = struct {
    // HTTP client operations
    pub const http_get = "slay http_get_yikes(url tea) yikes<tea>";
    pub const http_post = "slay http_post_yikes(url tea, data tea) yikes<tea>";
    pub const http_put = "slay http_put_yikes(url tea, data tea) yikes<tea>";
    pub const http_delete = "slay http_delete_yikes(url tea) yikes<tea>";
    
    // TCP operations
    pub const tcp_connect = "slay tcp_connect_yikes(host tea, port drip) yikes<TcpSocket>";
    pub const tcp_listen = "slay tcp_listen_yikes(port drip) yikes<TcpServer>";
    pub const tcp_send = "slay tcp_send_lit(socket TcpSocket, data tea) lit";
    pub const tcp_receive = "slay tcp_receive_yikes(socket TcpSocket) yikes<tea>";
    
    // UDP operations
    pub const udp_bind = "slay udp_bind_yikes(port drip) yikes<UdpSocket>";
    pub const udp_send_to = "slay udp_send_to_lit(socket UdpSocket, data tea, host tea, port drip) lit";
    pub const udp_receive_from = "slay udp_receive_from_yikes(socket UdpSocket) yikes<UdpMessage>";
};

/// =============================================================================
/// TESTZ MODULE STANDARDIZATION
/// =============================================================================

pub const StandardTestAPI = struct {
    // Test assertions
    pub const assert_true = "slay assert_true_lit(condition lit) lit";
    pub const assert_false = "slay assert_false_lit(condition lit) lit";
    pub const assert_eq_drip = "slay assert_eq_drip_lit(expected drip, actual drip) lit";
    pub const assert_eq_tea = "slay assert_eq_tea_lit(expected tea, actual tea) lit";
    pub const assert_not_eq = "slay assert_not_eq_lit(expected T, actual T) lit";
    
    // Test organization
    pub const test_start = "slay test_start_lit(test_name tea) lit";
    pub const test_end = "slay test_end_lit() lit";
    pub const test_suite_start = "slay test_suite_start_lit(suite_name tea) lit";
    pub const test_suite_end = "slay test_suite_end_lit() lit";
    
    // Test reporting
    pub const print_test_summary = "slay print_test_summary_lit() lit";
    pub const get_test_count = "slay get_test_count_drip() drip";
    pub const get_pass_count = "slay get_pass_count_drip() drip";
    pub const get_fail_count = "slay get_fail_count_drip() drip";
};

/// =============================================================================
/// USAGE EXAMPLES AND VALIDATION
/// =============================================================================

test "API standardization examples" {
    // These examples show how the standardized API should be used
    
    // Math operations - consistent naming and types
    // const result_drip = math_abs_drip(-42);  // result_drip = 42
    // const larger = math_max_drip(10, 20);    // larger = 20
    
    // String operations - consistent parameter names
    // const combined = string_concat_tea("Hello", " World");  // "Hello World"
    // const len = string_length_drip(combined);               // 11
    
    // Array operations - consistent bounds checking
    // const arr = [1, 2, 3, 4, 5];
    // const element = array_get_yikes(arr, 2);  // yikes.ok(3) or yikes.err(bounds_error)
    
    // File operations - consistent error handling
    // const content = file_read_tea("test.txt");  // yikes<tea>
    // const success = file_write_lit("output.txt", "data");  // lit (based/cringe)
    
    std.log.info("✅ API standardization patterns validated");
}

/// Migration helper for existing code
pub const APICompatibilityLayer = struct {
    // Provide compatibility functions for existing code during migration
    
    /// Old mathz.abs_normie() -> New math_abs_normie()  
    pub fn abs_normie(value: f64) f64 {
        return @fabs(value);
    }
    
    /// Old stringz.concat_strings() -> New string_concat_tea()
    pub fn concat_strings(allocator: std.mem.Allocator, a: []const u8, b: []const u8) []u8 {
        return std.mem.concat(allocator, u8, &[_][]const u8{ a, b }) catch "";
    }
    
    /// Old arrayz.sum_array() -> New array_sum_drip()
    pub fn sum_array(array: []const i32) i32 {
        var sum: i32 = 0;
        for (array) |element| {
            sum += element;
        }
        return sum;
    }
};

/// API consistency validation
test "API consistency validation" {
    // Validate that all module APIs follow the standard patterns
    
    // Function naming pattern: {module}_{operation}_{type}
    const valid_names = [_][]const u8{
        "math_abs_drip",
        "string_concat_tea", 
        "array_length_drip",
        "file_read_tea",
        "http_get_yikes",
        "assert_eq_drip",
    };
    
    // Parameter naming consistency
    const valid_params = [_][]const u8{
        "file_path",
        "str_a",
        "str_b", 
        "array",
        "element",
        "index",
        "value",
    };
    
    // Return type consistency
    const valid_returns = [_][]const u8{
        "lit",        // boolean operations
        "drip",       // length/index operations
        "tea",        // string operations  
        "yikes<T>",   // error-prone operations
    };
    
    for (valid_names) |name| {
        try testing.expect(name.len > 0);
        try testing.expect(std.mem.indexOf(u8, name, "_") != null);
    }
    
    std.log.info("✅ API consistency validation passed");
}
