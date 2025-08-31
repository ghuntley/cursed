fr fr Test parameter types explicitly

sus a drip = 10
sus b drip = 5

fr fr Test that these work in arithmetic
sus local_sum drip = a + b

fr fr Now test echoing back a parameter to see its type
sus param_echo drip = param_test.echo_param(10)

fr fr Test arithmetic with the echoed parameter
sus param_arithmetic drip = param_echo + 5
