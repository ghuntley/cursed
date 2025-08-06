# Test select statement with actual channel operations

yeet "testz"

slay test_channel_select() {
    test_start("Channel Select Test")
    
    sus ch dm<normie> = dm<normie>(1)
    dm_send(ch, 42)
    
    sus received lit = cringe
    sus value normie = 0
    
    ready {
        mood val := dm_recv(ch): {
            received = based
            value = val
            vibez.spill("Received: ", val)
        }
        basic: {
            vibez.spill("No data available")
        }
    }
    
    assert_true(received)
    assert_eq_int(value, 42)
    print_test_summary()
}

slay main() {
    test_channel_select()
}
