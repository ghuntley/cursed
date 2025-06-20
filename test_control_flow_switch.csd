slay process_day(day tea) {
    vibe_check day {
        mood "Monday", "Tuesday":
            print("Start of week")
        mood "Wednesday":
            print("Midweek")
        mood "Friday":
            print("End of week")
        basic:
            print("Weekend or other")
    }
}