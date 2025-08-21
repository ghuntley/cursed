slay factorial(n drip) drip {
    ready (n <= 1) { damn 1 }
    damn n * factorial(n - 1)
}

slay string_work() drip {
    sus total drip = 0
    bestie (sus i drip = 0; i < 500; i++) {
        sus str tea = "test_string_number_"
        total = total + len(str) * i
    }
    damn total
}

slay array_work() drip {
    sus arr []drip = []
    bestie (sus i drip = 0; i < 100; i++) {
        arr = push(arr, i * 2)
    }
    
    sus sum drip = 0
    bestie (sus i drip = 0; i < 100; i++) {
        sum = sum + arr[i]
    }
    damn sum
}

slay main() drip {
    sus total drip = 0
    
    bestie (sus i drip = 0; i < 1000; i++) {
        total = total + factorial(i % 8)
        
        ready (i % 10 == 0) {
            total = total + string_work()
        }
        
        ready (i % 20 == 0) {
            total = total + array_work()
        }
    }
    
    damn total
}