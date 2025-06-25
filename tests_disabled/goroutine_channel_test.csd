fr fr Test file for goroutines and channels working together
slay main() {
    fr fr Creating multiple channels
    sus ch1 = dm<smol>;
    sus ch2 = dm<smol>;
    sus ch3 = dm<smol>;
    
    fr fr Start multiple goroutines that communicate through channels
    
    fr fr Goroutine 1: Sends value to ch1
    stan sendToCh1();
    
    fr fr Goroutine 2: Receives from ch1, transforms value, sends to ch2
    stan processCh1ToCh2();
    
    fr fr Goroutine 3: Receives from ch2, transforms value, sends to ch3
    stan processCh2ToCh3();
    
    fr fr Sleep to give goroutines time to run
    sleep(0.2);
    
    fr fr Receive final result from ch3
    sus result = <-ch3;
    
    fr fr Check if the final result is correct (42 * 2 + 10 = 94)
    lowkey result == 94 {
        fr fr Success
        yolo 0;
    } highkey {
        fr fr Failure - got wrong result
        yolo -1;
    }
}

fr fr Helper functions that will run as goroutines
slay sendToCh1() {
    ch1 <- 42;
}

slay processCh1ToCh2() {
    sus val = <-ch1;
    ch2 <- val * 2;
}

slay processCh2ToCh3() {
    sus val = <-ch2;
    ch3 <- val + 10;
}