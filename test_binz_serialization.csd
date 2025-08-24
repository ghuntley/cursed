// Test binary serialization (binz package)
yeet "binz"
yeet "vibez"

vibez.spill("=== Testing Binz Binary Serialization ===")

// Test basic type serialization
sus data_int drip = 42
sus serialized_int []drip = serialize_int(data_int)
sus deserialized_int drip = deserialize_int(serialized_int)

ready (deserialized_int == data_int) {
    vibez.spill("✅ Integer serialization: PASSED")
} otherwise {
    vibez.spill("❌ Integer serialization: FAILED")
}

// Test string serialization
sus data_string tea = "Hello, CURSED serialization!"
sus serialized_string []drip = serialize_string(data_string)
sus deserialized_string tea = deserialize_string(serialized_string)

ready (deserialized_string == data_string) {
    vibez.spill("✅ String serialization: PASSED")
} otherwise {
    vibez.spill("❌ String serialization: FAILED")
}

// Test array serialization
sus data_array []drip = [1, 2, 3, 4, 5]
sus serialized_array []drip = serialize_array(data_array)
sus deserialized_array []drip = deserialize_array(serialized_array)

sus arrays_match lit = based
bestie (sus i drip = 0; i < len(data_array); i++) {
    ready (data_array[i] != deserialized_array[i]) {
        arrays_match = cringe
        shook
    }
}

ready (arrays_match) {
    vibez.spill("✅ Array serialization: PASSED")
} otherwise {
    vibez.spill("❌ Array serialization: FAILED")
}

// Test struct serialization
squad Person {
    name tea
    age drip
    active lit
}

sus person Person = Person{name: "Alice", age: 30, active: based}
sus serialized_struct []drip = serialize_struct(person)
sus deserialized_struct Person = deserialize_struct<Person>(serialized_struct)

ready (deserialized_struct.name == person.name && 
       deserialized_struct.age == person.age && 
       deserialized_struct.active == person.active) {
    vibez.spill("✅ Struct serialization: PASSED")
} otherwise {
    vibez.spill("❌ Struct serialization: FAILED")
}

vibez.spill("=== Binz Testing Complete ===")
