// Test for-in loop - working version
vibe main

slay main() {
    sus people = ["Alice", "Bob", "Charlie"]
    
    bestie person in people {
        sus message = "Hello " + person
    }
}
