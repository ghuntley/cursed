fr fr Test file for ghosted (break) functionality

sus i = 0;
sus sum = 0;

periodt i < 10 {
    i = i + 1;
    sum = sum + i;
    
    fr fr Break out of the loop when i reaches 5
    lowkey i == 5 {
        ghosted;
    }
}

fr fr This should print 15 (1+2+3+4+5) since we break when i is 5
puts(sum);