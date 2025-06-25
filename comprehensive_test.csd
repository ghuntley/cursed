facts number = 42;
facts text = "Hello CURSED";
facts flag = true;
facts negative = false;

slay add(a, b) {
    facts sum = a + b;
    vibes sum;
}

slay test_variables() {
    facts x = 10;
    facts y = 20;
    facts z = add(x, y);
    vibes z;
}

slay main() {
    vibes number;
    vibes text;
    vibes flag;
    test_variables();
}
