// Test CURSED advanced control flow features
// This file tests periodt (while), bestie (for), and vibe_check (switch) keywords

slay test_while() {
    sus count = 0;
    
    // Test periodt (while loop)
    periodt count < 3 {
        count = count + 1;
    }
    
    yolo count;
}

slay test_for() {
    sus result = 0;
    
    // Test bestie (for loop)
    bestie sus i = 0; i < 5; i = i + 1 {
        result = result + i;
    }
    
    yolo result;
}

slay test_switch() {
    sus value = 2;
    sus result = 0;
    
    // Test vibe_check (switch statement)
    vibe_check value {
        mood 1 {
            result = 10;
        }
        mood 2 {
            result = 20;
        }
        mood 3 {
            result = 30;
        }
        basic {
            result = 99;
        }
    }
    
    yolo result;
}

slay main() {
    sus while_result = test_while();
    sus for_result = test_for();
    sus switch_result = test_switch();
    
    // Expected: while_result = 3, for_result = 10, switch_result = 20
    yolo while_result + for_result + switch_result;
}
