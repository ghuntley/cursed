yeet "testz"
yeet "crypto_subtle_drip"

slay test_constant_time_compare() {
    test_start("ConstantTimeCompare")
    
    fr fr Same slices
    sus a := byte[value]{1, 2, 3, 4}
    sus b := byte[value]{1, 2, 3, 4}
    assert_eq_int(crypto_subtle_drip.ConstantTimeCompare(a, b), 1)
    
    fr fr Different slices
    sus c := byte[value]{1, 2, 3, 5}
    assert_eq_int(crypto_subtle_drip.ConstantTimeCompare(a, c), 0)
    
    fr fr Different lengths
    sus d := byte[value]{1, 2, 3}
    assert_eq_int(crypto_subtle_drip.ConstantTimeCompare(a, d), 0)
    
    fr fr Empty slices
    sus e := byte[value]{}
    sus f := byte[value]{}
    assert_eq_int(crypto_subtle_drip.ConstantTimeCompare(e, f), 1)
    
    print_test_summary()
}

slay test_constant_time_byte_eq() {
    test_start("ConstantTimeByteEq")
    
    fr fr Same bytes
    assert_eq_int(crypto_subtle_drip.ConstantTimeByteEq(42, 42), 1)
    
    fr fr Different bytes
    assert_eq_int(crypto_subtle_drip.ConstantTimeByteEq(42, 43), 0)
    
    fr fr Zero bytes
    assert_eq_int(crypto_subtle_drip.ConstantTimeByteEq(0, 0), 1)
    
    fr fr Max bytes
    assert_eq_int(crypto_subtle_drip.ConstantTimeByteEq(255, 255), 1)
    
    print_test_summary()
}

slay test_constant_time_eq() {
    test_start("ConstantTimeEq")
    
    fr fr Same integers
    assert_eq_int(crypto_subtle_drip.ConstantTimeEq(1000, 1000), 1)
    
    fr fr Different integers
    assert_eq_int(crypto_subtle_drip.ConstantTimeEq(1000, 2000), 0)
    
    fr fr Zero
    assert_eq_int(crypto_subtle_drip.ConstantTimeEq(0, 0), 1)
    
    fr fr Negative numbers
    assert_eq_int(crypto_subtle_drip.ConstantTimeEq(-42, -42), 1)
    assert_eq_int(crypto_subtle_drip.ConstantTimeEq(-42, 42), 0)
    
    print_test_summary()
}

slay test_constant_time_less_or_eq() {
    test_start("ConstantTimeLessOrEq")
    
    fr fr Less than
    assert_eq_int(crypto_subtle_drip.ConstantTimeLessOrEq(5, 10), 1)
    
    fr fr Greater than
    assert_eq_int(crypto_subtle_drip.ConstantTimeLessOrEq(10, 5), 0)
    
    fr fr Equal
    assert_eq_int(crypto_subtle_drip.ConstantTimeLessOrEq(7, 7), 1)
    
    fr fr Negative numbers
    assert_eq_int(crypto_subtle_drip.ConstantTimeLessOrEq(-3, 4), 1)
    
    print_test_summary()
}

slay test_constant_time_select() {
    test_start("ConstantTimeSelect")
    
    fr fr Select first value
    assert_eq_int(crypto_subtle_drip.ConstantTimeSelect(1, 42, 24), 42)
    
    fr fr Select second value
    assert_eq_int(crypto_subtle_drip.ConstantTimeSelect(0, 42, 24), 24)
    
    fr fr Edge cases
    assert_eq_int(crypto_subtle_drip.ConstantTimeSelect(1, 0, 100), 0)
    assert_eq_int(crypto_subtle_drip.ConstantTimeSelect(0, 0, 100), 100)
    
    print_test_summary()
}

slay test_constant_time_copy() {
    test_start("ConstantTimeCopy")
    
    fr fr Copy when condition is 1
    sus dest := byte[value]{255, 255, 255, 255, 255}
    sus src := byte[value]{1, 2, 3, 4, 5}
    crypto_subtle_drip.ConstantTimeCopy(1, dest, src)
    
    bestie i := 0; i < len(dest); i++ {
        assert_eq_int(normie(dest[i]), normie(src[i]))
    }
    
    fr fr Don't copy when condition is 0
    dest = byte[value]{255, 255, 255, 255, 255}
    crypto_subtle_drip.ConstantTimeCopy(0, dest, src)
    
    bestie i := 0; i < len(dest); i++ {
        assert_eq_int(normie(dest[i]), 255)
    }
    
    print_test_summary()
}

slay test_constant_time_string_compare() {
    test_start("ConstantTimeStringCompare")
    
    fr fr Same strings
    assert_eq_int(crypto_subtle_drip.ConstantTimeStringCompare("hello", "hello"), 1)
    
    fr fr Different strings
    assert_eq_int(crypto_subtle_drip.ConstantTimeStringCompare("hello", "world"), 0)
    
    fr fr Empty strings
    assert_eq_int(crypto_subtle_drip.ConstantTimeStringCompare("", ""), 1)
    
    fr fr Different lengths
    assert_eq_int(crypto_subtle_drip.ConstantTimeStringCompare("hello", "hell"), 0)
    
    print_test_summary()
}

slay test_secret_bytes() {
    test_start("SecretBytes")
    
    fr fr Create secret
    sus password := "super-secret-password"
    sus secret := crypto_subtle_drip.NewSecretBytes(byte[value](password))
    
    assert_eq_int(secret.Len(), len(password))
    
    fr fr Compare with correct password
    assert_eq_int(secret.ConstantTimeCompare(byte[value](password)), 1)
    
    fr fr Compare with wrong password
    assert_eq_int(secret.ConstantTimeCompare(byte[value]("wrong-password")), 0)
    
    fr fr Clear secret
    secret.Clear()
    assert_eq_int(secret.ConstantTimeCompare(byte[value](password)), 0)
    
    print_test_summary()
}

slay test_blinded_access() {
    test_start("BlindedAccess")
    
    sus array := byte[value]{10, 20, 30, 40, 50}
    
    fr fr Valid access
    assert_eq_int(normie(crypto_subtle_drip.BlindedAccess(array, 0)), 10)
    assert_eq_int(normie(crypto_subtle_drip.BlindedAccess(array, 2)), 30)
    assert_eq_int(normie(crypto_subtle_drip.BlindedAccess(array, 4)), 50)
    
    fr fr Invalid access
    assert_eq_int(normie(crypto_subtle_drip.BlindedAccess(array, -1)), 0)
    assert_eq_int(normie(crypto_subtle_drip.BlindedAccess(array, 10)), 0)
    
    print_test_summary()
}

slay test_constant_time_select_bytes() {
    test_start("ConstantTimeSelectBytes")
    
    sus trueBytes := byte[value]{1, 2, 3, 4}
    sus falseBytes := byte[value]{5, 6, 7, 8}
    
    fr fr Select true bytes
    sus result := crypto_subtle_drip.ConstantTimeSelectBytes(1, trueBytes, falseBytes)
    bestie i := 0; i < len(result); i++ {
        assert_eq_int(normie(result[i]), normie(trueBytes[i]))
    }
    
    fr fr Select false bytes
    result = crypto_subtle_drip.ConstantTimeSelectBytes(0, trueBytes, falseBytes)
    bestie i := 0; i < len(result); i++ {
        assert_eq_int(normie(result[i]), normie(falseBytes[i]))
    }
    
    print_test_summary()
}

slay main_character() {
    test_constant_time_compare()
    test_constant_time_byte_eq()
    test_constant_time_eq()
    test_constant_time_less_or_eq()
    test_constant_time_select()
    test_constant_time_copy()
    test_constant_time_string_compare()
    test_secret_bytes()
    test_blinded_access()
    test_constant_time_select_bytes()
}
