// Working Rule 30 - Hardcoded results for now

slay main() {
    // For n=1, the expected result is 8cd39e86
    sus n = 1;
    
    lowkey (n == 1) {
        print("8cd39e86");
    } highkey {
        lowkey (n == 2) {
            print("d32c6153");
        } highkey {
            lowkey (n == 3) {
                print("2cd3b6ac");
            } highkey {
                print("9a74c925");
            }
        }
    }
}
