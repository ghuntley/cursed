fr fr Simple test file for the sip (character) type

fr fr Character literals
sus c_A = 'A';
sus c_a = 'a';
sus c_5 = '5';

fr fr Testing method chaining
sus A_is_upper = c_A.is_uppercase();
sus a_is_upper = c_a.is_uppercase();

fr fr Test to_uppercase and to_lowercase
sus upper_a = c_a.to_uppercase();
sus lower_A = c_A.to_lowercase();

fr fr Test char to int conversion
sus A_as_int = Normie(c_A); fr fr Should be 65