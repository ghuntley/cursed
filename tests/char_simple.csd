fr fr Simple test for character methods

fr fr Character literals
sus ch1='A';
sus ch2='a';
sus ch3='5';

fr fr Test functions instead of methods
sus is_upper1=is_uppercase(ch1); fr fr should be based (true)
sus is_upper2=is_uppercase(ch2); fr fr should be sus (false)

fr fr Test to_uppercase and to_lowercase
sus upper=to_uppercase(ch2);
sus lower=to_lowercase(ch1);

fr fr Test char to int conversion
sus code1=Normie(ch1); fr fr Should be 65