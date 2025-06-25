vibe main

slay main() {
    txt day = "Monday";
    
    vibe_check day {
        mood "Monday", "Tuesday":
            vibez.spill("Start of week");
        mood "Friday":
            vibez.spill("End of week");
        basic:
            vibez.spill("Mid-week");
    }
    
    yolo 0;
}