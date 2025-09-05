slay test_return_in_if(x drip) drip {
    lowkey (x > 0) {
        damn x + 10
    }
    damn x - 10
}

slay test_nested_if(x drip, y drip) drip {
    lowkey (x > 0) {
        lowkey (y > 0) {
            damn x + y
        }
        damn x
    }
    damn y
}

slay test_multiple_returns(x drip) drip {
    lowkey (x == 0) {
        damn 0
    }
    lowkey (x == 1) {
        damn 1
    }
    lowkey (x == 2) {
        damn 4
    }
    damn x * x
}

slay main_character() cap {
    sus result1 drip = test_return_in_if(5)
    sus result2 drip = test_return_in_if(-3)
    sus result3 drip = test_nested_if(2, 3)
    sus result4 drip = test_multiple_returns(2)
    
    vibez.spill(result1)
    vibez.spill(result2)
    vibez.spill(result3)
    vibez.spill(result4)
}
