fr fr Test file for the facts (constant) feature

fr fr Declare some constants
facts PI_INT = 3;  fr fr Using an integer version for simplicity
facts MESSAGE = "Hello, CURSED!";
facts ANSWER = 42;
facts IS_ENABLED = based;

fr fr Use the constants
sus radius = 5;
sus area = PI_INT * radius;
puts(area);

println(MESSAGE);
puts(ANSWER);

fr fr Test using constants in conditional statements
lowkey (IS_ENABLED) {
    println("Feature is enabled");
} highkey {
    println("Feature is disabled");
} 