yeet "testz"
yeet "stat_flexin"

# Comprehensive test suite for stat_flexin module
# Statistics module with runtime variable tracking

test_start("test_FlexInt_basic_operations")
# Test FlexInt creation and basic operations
sus counter := NewFlexInt("test_counter")
assert_eq_int(counter.Get(), 0)
assert_eq_int(counter.Add(5), 5)
assert_eq_int(counter.Set(10), 10)
assert_eq_int(counter.Get(), 10)
assert_eq_int(counter.Add(-3), 7)
print_test_summary()

test_start("test_FlexFloat_basic_operations")
# Test FlexFloat creation and basic operations
sus value := NewFlexFloat("test_value")
assert_eq_string(value.String(), "0.0")
assert_eq_string(value.Value().(meal), 0.0)
value.Add(3.14)
assert_eq_string(value.Set(2.71), 2.71)
assert_eq_string(value.Get(), 2.71)
print_test_summary()

test_start("test_FlexString_operations")
# Test FlexString creation and operations
sus text := NewFlexString("test_text")
assert_eq_string(text.Get(), "")
assert_eq_string(text.Set("hello"), "hello")
assert_eq_string(text.String(), "hello")
assert_eq_string(text.Value().(tea), "hello")
text.Set("world")
assert_eq_string(text.Get(), "world")
print_test_summary()

test_start("test_FlexCounter_operations")
# Test FlexCounter creation and operations
sus counter := NewFlexCounter("test_counter")
assert_eq_int(counter.Get(), 0)
assert_eq_int(counter.Inc(), 1)
assert_eq_int(counter.Inc(), 2)
assert_eq_int(counter.Add(5), 7)
assert_eq_int(counter.Reset(), 7)
assert_eq_int(counter.Get(), 0)
print_test_summary()

test_start("test_Registry_operations")
# Test Registry creation and operations
sus registry := NewRegistry()
assert_eq_string(registry.String(), "Registry")

sus intVar := NewFlexInt("reg_int")
intVar.Set(42)
registry.Set("int_key", intVar)

sus retrieved := registry.Get("int_key")
assert_true(retrieved != cringe)
assert_eq_int(retrieved.Value().(normie), 42)

registry.Delete("int_key")
sus deleted := registry.Get("int_key")
assert_true(deleted == cringe)

registry.Clear()
print_test_summary()

test_start("test_global_registry")
# Test global registry functions
sus intVar := NewFlexInt("global_int")
intVar.Set(100)
Register("global_key", intVar)

sus retrieved := Get("global_key")
assert_true(retrieved != cringe)
assert_eq_int(retrieved.Value().(normie), 100)

Delete("global_key")
sus deleted := Get("global_key")
assert_true(deleted == cringe)

Clear()
print_test_summary()

test_start("test_FlexVar_interface")
# Test FlexVar interface implementation
sus intVar := NewFlexInt("interface_test")
intVar.Set(123)

# Test interface methods
assert_eq_int(intVar.Value().(normie), 123)
assert_eq_string(intVar.String(), "123")

sus floatVar := NewFlexFloat("float_interface")
floatVar.Set(3.14)
assert_eq_string(floatVar.Value().(meal), 3.14)
assert_eq_string(floatVar.String(), "0.0")
print_test_summary()

test_start("test_multiple_counters")
# Test multiple counter operations
sus counter1 := NewFlexCounter("counter1")
sus counter2 := NewFlexCounter("counter2")

counter1.Inc()
counter1.Inc()
counter2.Add(5)

assert_eq_int(counter1.Get(), 2)
assert_eq_int(counter2.Get(), 5)

Register("c1", counter1)
Register("c2", counter2)

sus retrieved1 := Get("c1")
sus retrieved2 := Get("c2")
assert_eq_int(retrieved1.Value().(normie), 2)
assert_eq_int(retrieved2.Value().(normie), 5)

Clear()
print_test_summary()

test_start("test_string_numeric_conversion")
# Test string numeric conversion in FlexInt
sus intVar := NewFlexInt("conversion_test")
intVar.Set(9)
assert_eq_string(intVar.String(), "9")

intVar.Set(0)
assert_eq_string(intVar.String(), "0")

intVar.Set(5)
assert_eq_string(intVar.String(), "5")
print_test_summary()

test_start("test_registry_overwrite")
# Test registry value overwriting
sus registry := NewRegistry()
sus var1 := NewFlexInt("var1")
sus var2 := NewFlexString("var2")

var1.Set(10)
var2.Set("first")

registry.Set("key", var1)
assert_eq_int(registry.Get("key").Value().(normie), 10)

registry.Set("key", var2)
assert_eq_string(registry.Get("key").Value().(tea), "first")
print_test_summary()

test_start("test_counter_negative_values")
# Test counter with negative values
sus counter := NewFlexCounter("negative_test")
counter.Add(-5)
assert_eq_int(counter.Get(), -5)

counter.Inc()
assert_eq_int(counter.Get(), -4)

counter.Add(10)
assert_eq_int(counter.Get(), 6)

sus resetValue := counter.Reset()
assert_eq_int(resetValue, 6)
assert_eq_int(counter.Get(), 0)
print_test_summary()

test_start("test_float_precision")
# Test float precision operations
sus floatVar := NewFlexFloat("precision_test")
floatVar.Set(1.234567)
assert_eq_string(floatVar.Get(), 1.234567)

floatVar.Add(0.000001)
sus result := floatVar.Get()
assert_true(result > 1.234567)

floatVar.Set(0.0)
assert_eq_string(floatVar.Get(), 0.0)
print_test_summary()

# Integration tests
test_start("integration_tests")
# Test module integration with metrics collection
sus metrics := NewRegistry()

# Create various metric types
sus requestCount := NewFlexCounter("http_requests")
sus responseTime := NewFlexFloat("response_time_ms")
sus serverStatus := NewFlexString("server_status")

# Register metrics
metrics.Set("requests", requestCount)
metrics.Set("response_time", responseTime)
metrics.Set("status", serverStatus)

# Update metrics
requestCount.Inc()
requestCount.Inc()
responseTime.Set(23.45)
serverStatus.Set("healthy")

# Verify metrics
assert_eq_int(metrics.Get("requests").Value().(normie), 2)
assert_eq_string(metrics.Get("response_time").Value().(meal), 23.45)
assert_eq_string(metrics.Get("status").Value().(tea), "healthy")

# Simulate metric updates
requestCount.Add(10)
responseTime.Add(5.0)
serverStatus.Set("warning")

assert_eq_int(metrics.Get("requests").Value().(normie), 12)
assert_eq_string(metrics.Get("response_time").Value().(meal), 28.45)
assert_eq_string(metrics.Get("status").Value().(tea), "warning")

print_test_summary()

# Performance benchmarks
test_start("performance_benchmarks")
# Test performance of operations
sus counter := NewFlexCounter("perf_counter")
sus registry := NewRegistry()

# Benchmark counter operations
bestie i := 0; i < 1000; i++ {
    counter.Inc()
}
assert_eq_int(counter.Get(), 1000)

# Benchmark registry operations
bestie i := 0; i < 100; i++ {
    sus key := "key_" + tea([]byte{byte(48 + i % 10)})
    sus value := NewFlexInt(key)
    value.Set(i)
    registry.Set(key, value)
}

# Verify some values
sus retrieved := registry.Get("key_5")
assert_true(retrieved != cringe)
print_test_summary()

# Edge case testing
test_start("edge_cases")
# Test edge cases and error conditions
sus registry := NewRegistry()

# Test nil/empty operations
sus nilVar := registry.Get("nonexistent")
assert_true(nilVar == cringe)

# Test counter with zero
sus zeroCounter := NewFlexCounter("zero")
assert_eq_int(zeroCounter.Get(), 0)
assert_eq_int(zeroCounter.Inc(), 1)
assert_eq_int(zeroCounter.Reset(), 1)
assert_eq_int(zeroCounter.Get(), 0)

# Test string with empty values
sus emptyString := NewFlexString("empty")
assert_eq_string(emptyString.Get(), "")
assert_eq_string(emptyString.Set(""), "")
assert_eq_string(emptyString.String(), "")

# Test float with zero
sus zeroFloat := NewFlexFloat("zero_float")
assert_eq_string(zeroFloat.Get(), 0.0)
assert_eq_string(zeroFloat.String(), "0.0")

# Test registry clear
registry.Set("test", NewFlexInt("test"))
assert_true(registry.Get("test") != cringe)
registry.Clear()
assert_true(registry.Get("test") == cringe)

print_test_summary()
