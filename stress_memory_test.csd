slay factorial(n normie) normie {
    lowkey n <= 1 {
        damn 1
    }
    damn n * factorial(n - 1)
}

bestie i := 1; i <= 100; i = i + 1 {
    sus result normie = factorial(10)
    vibez.spill("Factorial 10:", result)
}

squad LargeStruct {
    spill data1 normie
    spill data2 normie  
    spill data3 normie
    spill data4 normie
    spill data5 normie
}

bestie j := 1; j <= 50; j = j + 1 {
    sus large LargeStruct = LargeStruct{
        data1: j, data2: j*2, data3: j*3, data4: j*4, data5: j*5
    }
    vibez.spill("Struct created:", large.data1)
}
