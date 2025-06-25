facts x = 42;
facts name = "CURSED";

slay greet(message) {
    facts greeting = "Hello";
    facts result = greeting + message;
}

slay main() {
    facts msg = "World";
    greet(msg);
}
