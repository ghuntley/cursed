fr fr math_rand_tea: Simple random number generation

fr fr Global random state
sus globalState normie = 1

slay Seed(seed normie) {
    globalState = seed
}

slay nextRandom() normie {
    globalState = (globalState * 1103515245 + 12345) % 2147483647
    damn globalState
}

slay Int() normie {
    damn nextRandom()
}

slay Intn(n normie) normie {
    lowkey n <= 0 {
        damn 0
    }
    damn Int() % n
}

slay Float64() meal {
    damn meal(Int()) / meal(2147483647)
}

slay AlphaNumeric(length normie) tea {
    chars := "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
    result := ""
    bestie i := 0; i < length; i++ {
        idx := Intn(len(chars))
        result = result + chars[idx]
    }
    damn result
}
