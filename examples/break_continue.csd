vibe main

slay main_character() {
    sus i = 0;
    sus sum = 0;
    
    fr fr Test break statement
    periodt i < 10 {
        i = i + 1;
        sum = sum + i;
        
        fr fr Break out of the loop when i reaches 5
        lowkey i == 5 {
            ghosted;
        }
    }
    
    fr fr This should print 15 (1+2+3+4+5) if break works
    puts(sum);
    
    fr fr Test continue statement
    i = 0;
    sum = 0;
    
    periodt i < 10 {
        i = i + 1;
        
        fr fr Skip numbers that are even
        lowkey i % 2 == 0 {
            simp;
        }
        
        fr fr Only add odd numbers to sum
        sum = sum + i;
    }
    
    fr fr This should print 25 (1+3+5+7+9) if continue works
    puts(sum);
}