
        slay main() lit {
            sus sum lit = 0
            
            fr Outer loop
            bestie i := flex 5 {
                fr Inner loop
                bestie j := flex 3 {
                    sum = sum + i * j
                }
            }
            
            damn sum  fr Should be 30 (0*0 + 0*1 + 0*2 + 1*0 + 1*1 + 1*2 + ... + 4*2)
        }
    
fr Print the result for testing
printn(damn)
