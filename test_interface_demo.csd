// Test CURSED interface implementation
collab Speaker {
    speak(message: tea) -> void;
    greet() -> void;
}

squad Dog {
    name: tea;
    breed: tea;
}

impl Dog : Speaker {
    speak(message: tea) -> void {
        print("Woof! " + message);
    }
    
    greet() -> void {
        print("Hello, I'm " + self.name + " the " + self.breed);
    }
}

facts main() -> void {
    facts dog = Dog { name: "Buddy", breed: "Golden Retriever" };
    facts speaker: Speaker = dog as Speaker;
    speaker.speak("How are you?");
    speaker.greet();
}
