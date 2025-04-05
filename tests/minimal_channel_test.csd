vibe main

slay main() {
    sus ch = dm normie;
    
    fr fr Start a goroutine
    stan sendValue();
    
    fr fr Sleep a bit
    sleep(0.1);
    
    fr fr Receive the value
    sus value = <-ch;
    
    fr fr Check value
    lowkey value == 42 {
        print("Success!");
    }
    
    yolo 0;
}

fr fr Function to run in a goroutine
slay sendValue() {
    ch <- 42;
}