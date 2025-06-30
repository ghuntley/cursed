// Test Rule 30 functions
fn apply_rule30_bit(left, center, right) {
    return left != (center || right);
}

println("Rule 30 truth table:");
println("000 ->", apply_rule30_bit(0, 0, 0));
println("001 ->", apply_rule30_bit(0, 0, 1));
println("010 ->", apply_rule30_bit(0, 1, 0));
println("011 ->", apply_rule30_bit(0, 1, 1));
println("100 ->", apply_rule30_bit(1, 0, 0));
println("101 ->", apply_rule30_bit(1, 0, 1));
println("110 ->", apply_rule30_bit(1, 1, 0));
println("111 ->", apply_rule30_bit(1, 1, 1));
