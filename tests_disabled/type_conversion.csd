fr fr Test file for type conversions

fr fr Define a variable as normie (int32)
sus num normie = 42;

fr fr Test conversions between numeric types
fr fr Convert to smol (int8)
sus small smol = smol(num);
puts(small);

fr fr Convert to mid (int16)
sus medium mid = mid(num);
puts(medium);

fr fr Convert to thicc (int64)
sus large thicc = thicc(num);
puts(large);

fr fr Convert to snack (float32)
sus floatNum snack = snack(num);
puts(normie(floatNum));

fr fr Convert to meal (float64)
sus doubleNum meal = meal(num);
puts(normie(doubleNum));

fr fr Test float to integer conversion
sus pi meal = 3.14159;
sus piInt normie = normie(pi);
puts(piInt); fr fr Should print 3 (truncated)