// Test comprehensive CURSED syntax
facts name = "World";
sus age = 25;
facts pi = 3.14159;

slay greet(person tea) tea {
    yolo "Hello, " + person + "!";
}

slay main() {
    facts greeting = greet(name);
    sus result = 5 + 3 * 2;
    facts isValid = true;
    
    lowkey (age > 18) {
        facts status = "adult";
    } highkey {
        facts status = "minor";
    }
}
