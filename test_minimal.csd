yeet "testz"
yeet "time"

slay test_basic_time() {
    test_start("Basic Time Functions")
    
    sus current thicc = time.now()
    assert_eq_int(current, 1704067200)
    
    sus epoch thicc = time.unix(0)
    assert_eq_int(epoch, 0)
    
    sus five_sec thicc = time.seconds(5)
    assert_eq_int(five_sec, 5000000000)
    
    assert_true(time.is_before(epoch, current))
}

test_basic_time()
print_test_summary()
