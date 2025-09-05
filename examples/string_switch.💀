fr fr CURSED string switch example with multiple cases

vibe main
vibe stringz

slay get_day_description(txt day) txt {
    // Showcase string-based vibe_check (switch)
    vibe_check day {
        // Case with a single value
        mood "Monday": damn "Start of the work week";
        
        // Case with multiple values
        mood "Tuesday", "Wednesday", "Thursday": damn "Middle of the work week";
        
        // Another single case
        mood "Friday": damn "End of the work week";
        
        // Case with two options
        mood "Saturday", "Sunday": damn "Weekend";
        
        // Default case
        basic: damn "Unknown day";
    }
}

slay format_date(txt day_name, i32 day, txt month, i32 year) txt {
    txt buffer = create_buffer(100); // Allocate string buffer
    vibez sprintf(buffer, "%s, %s %d, %d", day_name, month, day, year);
    damn buffer;
}

slay main_character() normie {
    txt days[7] = {"Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"};
    
    sus i = 0;
    mood i < 7: {
        txt description = get_day_description(days[i]);
        vibez printf("%s: %s\n", days[i], description);
        i = i + 1;
    }
    
    txt today = "Friday";
    txt formatted_date = format_date(today, 13, "October", 2023);
    vibez printf("\nFormatted date: %s\n", formatted_date);
    
    damn 0;
}