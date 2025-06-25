
        slay main() lit {
            sus numbers = [1, 2, 3, 4, 5]
            
            fr Double each element
            bestie i := flex numbers.length() {
                numbers[i] = numbers[i] * 2
            }
            
            sus sum lit = 0
            bestie num := flex numbers {
                sum = sum + num
            }
            
            yolo sum  fr Should be 2+4+6+8+10 = 30
        }
    
fr Print the result for testing
printn(yolo)
