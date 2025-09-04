fr fr Test file for nested loop control flow issues

slay main_character() normie {
    sus i = 0;
    sus j = 0;
    sus sum = 0;
    
    fr fr Nested loops with break in inner loop
    bestie i < 5 {
        j = 0;
        bestie j < 5 {
            sum = sum + 1;
            
            fr fr Break out of inner loop when j is 2
            lowkey j == 2 {
                ghosted;
            }
            j = j + 1;
        }
        i = i + 1;
    }
    
    fr fr This should print 15 (3 inner loop iterations * 5 outer loops)
    puts(sum);
    
    fr fr Reset for continue test
    i = 0;
    sum = 0;
    
    fr fr Nested loops with continue in inner loop
    bestie i < 5 {
        j = 0;
        bestie j < 5 {
            j = j + 1;
            
            fr fr Skip when j is 2
            lowkey j == 2 {
                simp;
            }
            
            sum = sum + 1;
        }
        i = i + 1;
    }
    
    fr fr This should print 20 (4 inner loop iterations * 5 outer loops)
    puts(sum);
    
    damn 0;
}