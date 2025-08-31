slay main_character() {
    /* Test multiple stdlib modules together */
    print("Testing mathz + stringz combination:");
    sus math_result drip = mathz.add_two(10, 5);
    print(math_result);
    
    sus string_len drip = stringz.length("world");
    print(string_len);
    
    sus combined drip = mathz.add_two(math_result, string_len);
    print("Combined result:");
    print(combined);
}
