slay main() {
    vibez.spill("🎉 CURSED Tuple Demo! 🎉");
    
    // Basic tuple creation
    sus coordinates = (10, 20);
    vibez.spill("Coordinates:");
    vibez.spill(coordinates.0);
    vibez.spill(coordinates.1);
    
    // Tuple with mixed types
    sus person_info = ("Alice", 25, based);
    vibez.spill("Person info:");
    vibez.spill(person_info.0);
    vibez.spill(person_info.1);
    vibez.spill(person_info.2);
    
    // Tuple destructuring
    (name, age, is_awesome) = person_info;
    vibez.spill("After destructuring:");
    vibez.spill(name);
    vibez.spill(age);
    vibez.spill(is_awesome);
    
    // Nested tuples
    sus nested = ((1, 2), (3, 4));
    sus first_pair = nested.0;
    vibez.spill("First nested element:");
    vibez.spill(first_pair.0);
    vibez.spill(first_pair.1);
    
    // Return a tuple
    damn coordinates;
}
