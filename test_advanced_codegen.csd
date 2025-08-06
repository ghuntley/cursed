# Advanced codegen test
sus numbers = [1, 2, 3, 4, 5]
sus point_data = (10, 20)

facts MAX_SIZE = 100

slay process_number(num normie) normie {
    lowkey num > 10 {
        damn num * 2
    } highkey {
        damn num + 5
    }
}

slay main() normie {
    vibez.spill("Advanced codegen test")
    
    # Test array access
    sus first = numbers[0]
    
    # Test conditional processing  
    sus result = process_number(15)
    
    # Test tuple access
    sus x_coord = point_data.0
    
    vibez.spill("Test complete")
    damn 0
}
