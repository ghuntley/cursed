slay check_day(day normie) {
    vibe_check day {
        mood 1:
            vibez.spill("Monday");
        mood 2:
            vibez.spill("Tuesday");
        mood 5:
            vibez.spill("Friday");
        basic:
            vibez.spill("Other day");
    }
}

slay main() {
    check_day(1);
    check_day(5);
    check_day(7);
    yolo 0;
}
