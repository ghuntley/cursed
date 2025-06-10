// Pipeline Processing Example
// Demonstrates data flowing through multiple processing stages

func stage1_generate(out dm<int>) {
    print("Stage 1: Generating numbers")
    
    for i := 1; i <= 20; i++ {
        print("Stage 1: Generated", i)
        out <- i
        sleep(50)
    }
    
    close(out)
    print("Stage 1: Finished generating")
}

func stage2_square(in dm<int>, out dm<int>) {
    print("Stage 2: Squaring numbers")
    
    for num := range in {
        squared := num * num
        print("Stage 2:", num, "->", squared)
        out <- squared
        sleep(30)
    }
    
    close(out)
    print("Stage 2: Finished squaring")
}

func stage3_filter_even(in dm<int>, out dm<int>) {
    print("Stage 3: Filtering even numbers")
    
    for num := range in {
        if num % 2 == 0 {
            print("Stage 3: Passed", num, "(even)")
            out <- num
        } else {
            print("Stage 3: Filtered", num, "(odd)")
        }
        sleep(20)
    }
    
    close(out)
    print("Stage 3: Finished filtering")
}

func stage4_accumulate(in dm<int>) {
    print("Stage 4: Accumulating results")
    
    facts total = 0
    facts count = 0
    
    for num := range in {
        total += num
        count++
        print("Stage 4: Added", num, "- Running total:", total)
        sleep(10)
    }
    
    if count > 0 {
        facts average = total / count
        print("Stage 4: Final total:", total)
        print("Stage 4: Count:", count)
        print("Stage 4: Average:", average)
    } else {
        print("Stage 4: No numbers processed")
    }
    
    print("Stage 4: Finished accumulating")
}

func main() {
    print("Starting pipeline processing...")
    
    // Create channels for each stage
    facts stage1_to_2 = make(dm<int>, 3)
    facts stage2_to_3 = make(dm<int>, 3)
    facts stage3_to_4 = make(dm<int>, 3)
    
    // Start all pipeline stages
    stan stage1_generate(stage1_to_2)
    stan stage2_square(stage1_to_2, stage2_to_3)
    stan stage3_filter_even(stage2_to_3, stage3_to_4)
    stan stage4_accumulate(stage3_to_4)
    
    // Wait for pipeline to complete
    sleep(3000) // 3 seconds
    
    print("Pipeline processing completed!")
}
