fr fr Test if concurrenz module syntax is fixed
fr fr This will show only syntax errors from concurrenz itself

fr fr Basic struct tests
struct TestMutex {
    spill state normie
}

slay test_mutex() lit {
    ready based {
        damn based
    }
    damn cap
}

fr fr Test conditional syntax
slay test_conditions() {
    sus value normie = 42
    
    ready value > 0 {
        value = value + 1
    } otherwise {
        value = 0
    }
    
    periodt {
        ready value < 100 {
            value = value + 1
        } otherwise {
            break
        }
    }
}

fr fr Test simple loop constructs
slay test_loops() {
    sus count normie = 0
    
    bestie count < 10 {
        count = count + 1
    }
    
    periodt {
        ready count > 0 {
            count = count - 1
        } otherwise {
            break
        }
    }
}

slay main() {
    test_mutex()
    test_conditions() 
    test_loops()
}
