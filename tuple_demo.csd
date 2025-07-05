// CURSED Tuple Demonstration
// Comprehensive showcase of tuple functionality

// Basic tuple creation
sus basic_tuple = (42, "hello", based);
vibez.spill("Basic tuple created");

// Tuple access
sus first = basic_tuple.0;
sus second = basic_tuple.1;
sus third = basic_tuple.2;

vibez.spill("First element:");
vibez.spill(first);
vibez.spill("Second element:");
vibez.spill(second);
vibez.spill("Third element:");
vibez.spill(third);

// Empty tuple
sus empty_tuple = ();
vibez.spill("Empty tuple created");

// Tuple destructuring
sus coordinates = (100, 200);
(x, y) = coordinates;
vibez.spill("Coordinates destructured:");
vibez.spill(x);
vibez.spill(y);

// Nested tuples
sus nested = ((1, 2), (3, 4));
sus inner_tuple = nested.0;
sus value = inner_tuple.1;
vibez.spill("Nested tuple access:");
vibez.spill(value);

// Complex tuple with different types
sus mixed = (314, "pi", based, 42);
vibez.spill("Mixed tuple second element:");
vibez.spill(mixed.1);

vibez.spill("Tuple demo complete!");
