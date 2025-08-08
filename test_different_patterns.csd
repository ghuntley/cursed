slay constant_func() drip {
    damn 100
}

slay add_constants() drip {
    damn 50 + 25
}

slay call_other_func() drip {
    sus temp drip = constant_func()
    damn temp + 5
}

vibez.spill(constant_func())
vibez.spill(add_constants())
vibez.spill(call_other_func())
