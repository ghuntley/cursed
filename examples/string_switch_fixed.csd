vibe main

slay string_switch(day txt) txt {
    sus result txt = "unknown";
    
    vibe_check day {
        mood "Monday":
            result = "Start of week";
        mood "Tuesday":
            result = "Mid-week";
        mood "Friday":
            result = "End of week";
        basic:
            result = "Weekend";
    }
    
    damn result;
}

slay main_character() {
    vibez.spill(string_switch("Monday"));
    vibez.spill(string_switch("Tuesday"));
    vibez.spill(string_switch("Saturday"));
}