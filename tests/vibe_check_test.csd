// CURSED switch/case test file

vibe check;

// Test basic switch/case functionality
slay test_simple_switch(x normie) tea {
    sus result tea = "unknown";
    
    vibe_check x {
        mood 1:
            result = "one";
        mood 2:
            result = "two";
        mood 3:
            result = "three";
        basic:
            result = "other";
    }
    
    yolo result;
}

// Test multiple values in a single case
slay test_multiple_cases(x normie) tea {
    sus result tea = "unknown";
    
    vibe_check x {
        mood 1, 2, 3:
            result = "small";
        mood 4, 5, 6:
            result = "medium";
        mood 7, 8, 9:
            result = "large";
        basic:
            result = "unknown";
    }
    
    yolo result;
}

// Test fallthrough behavior
slay test_fallthrough(day tea) tea {
    sus result tea = "unknown";
    
    vibe_check day {
        mood "Monday":
            result = "Start of week";
            // Fallthrough is implicit in CURSED without break
        mood "Tuesday":
            result = "Weekday";
            ghosted; // break to exit the case
        mood "Wednesday":
            result = "Mid-week";
            ghosted;
        mood "Thursday":
            result = "Almost weekend";
            ghosted;
        mood "Friday":
            result = "End of week";
            ghosted;
        basic:
            result = "Weekend";
    }
    
    yolo result;
}