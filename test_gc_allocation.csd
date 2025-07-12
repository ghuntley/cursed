sus arr [10]normie = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
sus str tea = "This is a test string for GC"
sus big_array [100]normie

bestie i := 0; i < 100; i++ {
    big_array[i] = i * 2
}

sus sum normie = 0
bestie j := 0; j < 10; j++ {
    sum += arr[j]
}

vibez.spill("Sum:", sum)
vibez.spill("String:", str)
vibez.spill("GC allocation test complete")
