// Test basic CURSED language features
facts answer = 42;
facts greeting = "Hello, CURSED!";
facts is_valid = true;

slay calculate_sum(a, b) {
    facts result = a + b;
    vibes result;
}

slay main() {
    facts x = 10;
    facts y = 32;
    facts sum = calculate_sum(x, y);
    vibes sum;
}
