// Test program for import system
import "math_utils";
import "./helpers/string_utils";

fn main() -> int {
    let result = math_utils.add(5, 3);
    let text = string_utils.concat("Hello", " World");
    return result;
}
