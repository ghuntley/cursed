yeet "regex"
yeet "vibez"

slay main_character() {
    vibez.spillln("=== CURSED Regex Module Final Test ===")
    
    fr fr Test find_all function - should return array
    sus text_multi tea = "cat dog cat bird cat"
    sus matches tea = regex.find_all("cat", text_multi) 
    vibez.spillln("Find all 'cat' matches:")
    vibez.spillln(matches)
    
    fr fr Test split function - should return array
    sus split_text tea = "apple,banana,cherry"
    sus parts tea = regex.split(",", split_text)
    vibez.spillln("Split 'apple,banana,cherry' by ',':")
    vibez.spillln(parts)
    
    fr fr Test no match case
    sus no_match tea = regex.find("xyz", "hello world")
    vibez.spillln("Find 'xyz' in 'hello world' (should be empty):")
    vibez.spillln(no_match)
    
    vibez.spillln("=== All Regex Tests Complete ===")
}
