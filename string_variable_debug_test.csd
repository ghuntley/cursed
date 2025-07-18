# Test to reproduce string variable vs constant codegen issue
vibez.spill("String constant works")
sus msg tea = "String variable broken"
vibez.spill(msg)
