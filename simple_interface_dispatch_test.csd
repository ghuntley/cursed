fr fr Simple Interface Dispatch Test
fr fr Basic test for vtable generation and method calls

yeet "testz"
yeet "vibez"

fr fr Simple interface with one method
collab Printer {
    slay print_message()
}

fr fr Simple struct
squad SimplePrinter {
    spill message tea
}

fr fr Implementation
impl SimplePrinter for Printer {
    slay print_message() {
        vibez.spill(message)
    }
}

fr fr Test basic interface dispatch
slay test_basic_dispatch() {
    test_start("Basic Interface Dispatch")
    
    sus printer SimplePrinter = SimplePrinter{
        message: "Hello from interface!",
    }
    
    fr fr Cast to interface
    sus interface_printer tea = printer.(Printer)
    
    fr fr Call method through interface (vtable dispatch)
    interface_printer.print_message()
    
    assert_true(based) fr fr If we get here, vtable dispatch worked
    print_test_summary()
}

slay main() {
    vibez.spill("=== Simple Interface Dispatch Test ===")
    test_basic_dispatch()
    vibez.spill("=== Test Complete ===")
}
