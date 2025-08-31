slay abs_simple(x drip) drip {
    lowkey (x < 0) {
        damn 0 - x
    }
    damn x
}

slay max_simple(a drip, b drip) drip {
    lowkey (a > b) {
        damn a
    }
    damn b
}

slay main_character() cap {
    sus neg_val drip = abs_simple(-5)
    sus pos_val drip = abs_simple(3)
    sus max_val drip = max_simple(neg_val, pos_val)
    
    vibez.spill(neg_val)
    vibez.spill(pos_val)
    vibez.spill(max_val)
}
