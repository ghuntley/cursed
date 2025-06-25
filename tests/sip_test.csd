fr fr Test file for the sip (character) type and methods

fr fr Test character literals
sus c_A = 'A'; fr fr sip (character) type inferred
sus c_a = 'a'; fr fr sip (character) type inferred
sus c_5 = '5'; fr fr sip (character) type inferred
sus c_newline = '\n'; fr fr sip (character) type inferred
sus c_emoji = '🙂'; fr fr sip (character) type inferred

fr fr Test is_uppercase method
sus A_is_upper = c_A.is_uppercase(); fr fr should be based (true)
sus a_is_upper = c_a.is_uppercase(); fr fr should be sus (false)
sus five_is_upper = c_5.is_uppercase(); fr fr should be sus (false)
sus emoji_is_upper = c_emoji.is_uppercase(); fr fr should be sus (false)

fr fr Test is_lowercase method
sus A_is_lower = c_A.is_lowercase(); fr fr should be sus (false)
sus a_is_lower = c_a.is_lowercase(); fr fr should be based (true)
sus five_is_lower = c_5.is_lowercase(); fr fr should be sus (false)
sus emoji_is_lower = c_emoji.is_lowercase(); fr fr should be sus (false)

fr fr Test is_digit method
sus A_is_digit = c_A.is_digit(); fr fr should be sus (false)
sus a_is_digit = c_a.is_digit(); fr fr should be sus (false)
sus five_is_digit = c_5.is_digit(); fr fr should be based (true)
sus emoji_is_digit = c_emoji.is_digit(); fr fr should be sus (false)

fr fr Test is_alpha method
sus A_is_alpha = c_A.is_alpha(); fr fr should be based (true)
sus a_is_alpha = c_a.is_alpha(); fr fr should be based (true)
sus five_is_alpha = c_5.is_alpha(); fr fr should be sus (false)

fr fr Test is_alnum method
sus A_is_alnum = c_A.is_alnum(); fr fr should be based (true)
sus a_is_alnum = c_a.is_alnum(); fr fr should be based (true)
sus five_is_alnum = c_5.is_alnum(); fr fr should be based (true)

fr fr Test to_uppercase and to_lowercase methods
sus A_to_upper = c_A.to_uppercase(); fr fr should remain 'A'
sus a_to_upper = c_a.to_uppercase(); fr fr should become 'A'
sus A_to_lower = c_A.to_lowercase(); fr fr should become 'a'
sus a_to_lower = c_a.to_lowercase(); fr fr should remain 'a'

fr fr Test char to integer conversion
sus A_as_int = Normie(c_A); fr fr should be 65
sus a_as_int = Normie(c_a); fr fr should be 97
sus five_as_int = Normie(c_5); fr fr should be 53