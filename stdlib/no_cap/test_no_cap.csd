fr fr no_cap module comprehensive test suite
fr fr Tests all truth/assertion utilities, boolean logic functions, and validation helpers

yeet "testz"
yeet "no_cap"

fr fr Test truth/assertion utilities
test_start("assert_truth with true")
assert_true(assert_truth(based))

test_start("assert_truth with false")
assert_false(assert_truth(cap))

test_start("assert_fact with true condition")
assert_true(assert_fact(based, "Test assertion"))

test_start("assert_fact with false condition")
assert_false(assert_fact(cap, "Test assertion"))

test_start("verify_claim with true")
assert_true(verify_claim(based))

test_start("verify_claim with false")
assert_false(verify_claim(cap))

test_start("confirm_reality with true")
assert_true(confirm_reality(based))

test_start("confirm_reality with false")
assert_false(confirm_reality(cap))

fr fr Test boolean logic functions
test_start("logic_and - true and true")
assert_true(logic_and(based, based))

test_start("logic_and - true and false")
assert_false(logic_and(based, cap))

test_start("logic_and - false and true")
assert_false(logic_and(cap, based))

test_start("logic_and - false and false")
assert_false(logic_and(cap, cap))

test_start("logic_or - true or true")
assert_true(logic_or(based, based))

test_start("logic_or - true or false")
assert_true(logic_or(based, cap))

test_start("logic_or - false or true")
assert_true(logic_or(cap, based))

test_start("logic_or - false or false")
assert_false(logic_or(cap, cap))

test_start("logic_not - not true")
assert_false(logic_not(based))

test_start("logic_not - not false")
assert_true(logic_not(cap))

test_start("logic_xor - true xor true")
assert_false(logic_xor(based, based))

test_start("logic_xor - true xor false")
assert_true(logic_xor(based, cap))

test_start("logic_xor - false xor true")
assert_true(logic_xor(cap, based))

test_start("logic_xor - false xor false")
assert_false(logic_xor(cap, cap))

test_start("logic_nand - true nand true")
assert_false(logic_nand(based, based))

test_start("logic_nand - true nand false")
assert_true(logic_nand(based, cap))

test_start("logic_nor - false nor false")
assert_true(logic_nor(cap, cap))

test_start("logic_nor - true nor false")
assert_false(logic_nor(based, cap))

test_start("logic_implies - true implies true")
assert_true(logic_implies(based, based))

test_start("logic_implies - false implies true")
assert_true(logic_implies(cap, based))

test_start("logic_implies - true implies false")
assert_false(logic_implies(based, cap))

test_start("logic_biconditional - true biconditional true")
assert_true(logic_biconditional(based, based))

test_start("logic_biconditional - true biconditional false")
assert_false(logic_biconditional(based, cap))

fr fr Test validation helpers
test_start("validate_true with true")
assert_true(validate_true(based))

test_start("validate_true with false")
assert_false(validate_true(cap))

test_start("validate_false with false")
assert_true(validate_false(cap))

test_start("validate_false with true")
assert_false(validate_false(based))

test_start("validate_not_null with true")
assert_true(validate_not_null(based))

test_start("validate_not_null with false")
assert_false(validate_not_null(cap))

test_start("validate_equals with same values")
assert_true(validate_equals(based, based))

test_start("validate_equals with different values")
assert_false(validate_equals(based, cap))

test_start("validate_not_equals with different values")
assert_true(validate_not_equals(based, cap))

test_start("validate_not_equals with same values")
assert_false(validate_not_equals(based, based))

fr fr Test fact checking utilities
test_start("check_consistency with all true")
sus all_true_facts []lit = [based, based, based]
assert_true(check_consistency(all_true_facts))

test_start("check_consistency with mixed")
sus mixed_facts []lit = [based, cap, based]
assert_false(check_consistency(mixed_facts))

test_start("check_contradiction - true case")
assert_true(check_contradiction(based, cap))

test_start("check_contradiction - false case")
assert_false(check_contradiction(based, based))

test_start("check_tautology with all true")
sus tautology_facts []lit = [based, based, based]
assert_true(check_tautology(tautology_facts))

test_start("check_tautology with mixed")
sus mixed_tautology []lit = [based, cap, based]
assert_false(check_tautology(mixed_tautology))

test_start("check_satisfiability with at least one true")
sus satisfiable []lit = [cap, based, cap]
assert_true(check_satisfiability(satisfiable))

test_start("check_satisfiability with all false")
sus unsatisfiable []lit = [cap, cap, cap]
assert_false(check_satisfiability(unsatisfiable))

fr fr Test advanced truth operations
test_start("truth_table_and with all true")
sus and_inputs []lit = [based, based, based]
assert_true(truth_table_and(and_inputs))

test_start("truth_table_and with mixed")
sus and_mixed []lit = [based, cap, based]
assert_false(truth_table_and(and_mixed))

test_start("truth_table_or with all false")
sus or_inputs []lit = [cap, cap, cap]
assert_false(truth_table_or(or_inputs))

test_start("truth_table_or with mixed")
sus or_mixed []lit = [cap, based, cap]
assert_true(truth_table_or(or_mixed))

test_start("majority_vote with majority true")
sus majority_true []lit = [based, based, cap]
assert_true(majority_vote(majority_true))

test_start("majority_vote with majority false")
sus majority_false []lit = [cap, cap, based]
assert_false(majority_vote(majority_false))

fr fr Test utility functions
test_start("count_truths")
sus count_test []lit = [based, cap, based, based]
assert_eq_int(count_truths(count_test), 3)

test_start("count_falsehoods")
assert_eq_int(count_falsehoods(count_test), 1)

test_start("truth_ratio")
sus ratio_result meal = truth_ratio(count_test)
assert_true(ratio_result > 0.7 && ratio_result < 0.8)

test_start("all_true with all true")
sus all_true_test []lit = [based, based, based]
assert_true(all_true(all_true_test))

test_start("all_true with mixed")
sus all_true_mixed []lit = [based, cap, based]
assert_false(all_true(all_true_mixed))

test_start("any_true with at least one true")
sus any_true_test []lit = [cap, based, cap]
assert_true(any_true(any_true_test))

test_start("any_true with all false")
sus any_true_false []lit = [cap, cap, cap]
assert_false(any_true(any_true_false))

test_start("none_true with all false")
sus none_true_test []lit = [cap, cap, cap]
assert_true(none_true(none_true_test))

test_start("none_true with at least one true")
sus none_true_mixed []lit = [cap, based, cap]
assert_false(none_true(none_true_mixed))

fr fr Test empty arrays
test_start("truth_ratio with empty array")
sus empty_array []lit = []
assert_eq_int(truth_ratio(empty_array).(normie), 0)

test_start("all_true with empty array")
assert_true(all_true(empty_array))

test_start("any_true with empty array")
assert_false(any_true(empty_array))

test_start("none_true with empty array")
assert_true(none_true(empty_array))

print_test_summary()
