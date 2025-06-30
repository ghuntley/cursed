// Test control flow with lowkey/highkey
sus x = 15;
sus y = 25;

lowkey (x > 10) {
    print("x is greater than 10");
} highkey {
    print("x is not greater than 10");
}

lowkey (y < 20) {
    print("y is less than 20");
} highkey {
    print("y is greater than or equal to 20");
}
