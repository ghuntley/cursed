fr fr Test hybrid system: CURSED loading with Zig fallback

fr fr This should load CURSED modules but fallback to Zig functions
sus math_result drip = mathz.add(10, 5)
sus str_len drip = stringz.length("hello")

fr fr Test our proven working CURSED functions  
sus literal_result drip = explicit_return.get_number()
sus param_result drip = param_test.echo_param(99)
