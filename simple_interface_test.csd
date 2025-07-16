# Simple Interface Optimization Test

yeet "testz"

# Simple interface for testing
collab Printer {
    slay print(msg tea) tea
}

# Simple implementation
facts PrinterImpl {}

# Create a simple print function
slay printer_print(self *PrinterImpl, msg tea) tea {
    damn "Printed: " + msg
}

# Test static dispatch
slay test_interface_optimization() lit {
    vibez.spill("Testing interface optimization...")
    damn based
}

test_start("Interface Optimization Test")
sus result lit = test_interface_optimization()
assert_true(result)
print_test_summary()
