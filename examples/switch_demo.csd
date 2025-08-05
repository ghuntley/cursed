fr fr CURSED vibe_check (switch/case) demonstration

fr fr Switch demo main

fr fr Simple switch example with single values per case
slay test_day_of_week(day normie) tea {
    sus result tea = "unknown";
    
    vibe_check day {
        mood 1:
            result = "Monday";
        mood 2:
            result = "Tuesday";
        mood 3:
            result = "Wednesday";
        mood 4:
            result = "Thursday";
        mood 5:
            result = "Friday";
        mood 6:
            result = "Saturday";
        mood 7:
            result = "Sunday";
        basic:
            result = "Invalid day";
    }
    
    damn result;
}

fr fr Example with multiple values per case
slay test_grade(score normie) tea {
    sus grade tea = "F";
    
    vibe_check score {
        mood 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100:
            grade = "A";
        mood 80, 81, 82, 83, 84, 85, 86, 87, 88, 89:
            grade = "B";
        mood 70, 71, 72, 73, 74, 75, 76, 77, 78, 79:
            grade = "C";
        mood 60, 61, 62, 63, 64, 65, 66, 67, 68, 69:
            grade = "D";
        basic:
            grade = "F";
    }
    
    damn grade;
}

fr fr Fallthrough example
slay test_fallthrough(category normie) tea {
    sus priority tea = "unknown";
    sus requires_attention lit = cap;
    
    vibe_check category {
        mood 1: // Critical
            priority = "Critical";
            requires_attention = based;
            // Fallthrough to high priority case
        mood 2:
            priority = "High";
            requires_attention = based;
            ghosted; // Break out of the switch
        mood 3:
            priority = "Medium";
            ghosted;
        mood 4:
            priority = "Low";
            ghosted;
        basic:
            priority = "None";
    }
    
    sus status tea = priority;
    lowkey requires_attention {
        status = priority + " (requires attention)";
    }
    
    damn status;
}

slay main() {
    vibez.spill("=== Day of Week Test ===");
    sus day1 normie = 1;
    sus day3 normie = 3;
    sus day9 normie = 9;
    vibez.spill(test_day_of_week(day1));
    vibez.spill(test_day_of_week(day3));
    vibez.spill(test_day_of_week(day9));
    
    vibez.spill("=== Grade Test ===");
    vibez.spill(test_grade(95));
    vibez.spill(test_grade(85));
    vibez.spill(test_grade(75));
    vibez.spill(test_grade(65));
    vibez.spill(test_grade(30));
    
    vibez.spill("=== Fallthrough Test ===");
    vibez.spill(test_fallthrough(1));
    vibez.spill(test_fallthrough(2));
    vibez.spill(test_fallthrough(3));
    vibez.spill(test_fallthrough(4));
    vibez.spill(test_fallthrough(5));
}