// Test basic channel functionality
dm my_channel = dm int(5);
my_channel <- 42;
let result = <- my_channel;
print(result);
