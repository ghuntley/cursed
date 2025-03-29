fr fr This is a test program to verify the compiler implementation works
sus x = 10;
sus y = 20;
sus add = stan(a, b) {
    yolo a + b;
};

sus result = add(x, y);
yolo result;

fr fr Test array
sus numbers = crew[1, 2, 3, 4, 5];
yolo numbers[2];

fr fr Test hash
sus person = tea{
    "name": "Alice", 
    "age": 30
};
yolo person["name"];

fr fr Test control flow
lowkey x > 5 {
    yolo "x is greater than 5";
} highkey {
    yolo "x is not greater than 5";
} 