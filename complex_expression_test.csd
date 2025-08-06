# Complex Expression Test Suite
# Tests operator precedence, nested expressions, and edge cases

# Test complex arithmetic with mixed types
sus a normie = 10
sus b meal = 3.5
sus c normie = 2

# Test operator precedence: should be 10 + (3.5 * 2) = 17.0
sus precedence1 meal = a.(meal) + b * c.(meal)
vibez.spill("Precedence test 1:", precedence1)

# Test parentheses override: should be (10 + 3.5) * 2 = 27.0  
sus precedence2 meal = (a.(meal) + b) * c.(meal)
vibez.spill("Precedence test 2:", precedence2)

# Test nested array access with expressions
sus matrix [[normie]] = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]

sus row_idx normie = 1
sus col_idx normie = 2
sus matrix_val normie = matrix[row_idx][col_idx]
vibez.spill("Matrix access:", matrix_val)  # Should be 6

# Test function calls as array indices
slay get_index(base normie, offset normie) normie {
    damn base + offset
}

sus dynamic_index normie = get_index(0, 2)
sus array_val normie = matrix[0][dynamic_index]
vibez.spill("Dynamic index:", array_val)  # Should be 3

# Test struct with arrays and nested access
squad Person {
    spill name tea
    spill scores [normie]
    spill age normie
}

sus john Person = Person{
    name: "John",
    scores: [85, 92, 78, 95],
    age: 25
}

sus best_score normie = john.scores[3]  # 95
sus avg_score normie = (john.scores[0] + john.scores[1] + john.scores[2] + john.scores[3]) / 4
vibez.spill("Person scores:", best_score, avg_score)

# Test function calls with complex arguments
slay process_person(p Person, bonus normie) normie {
    sus total normie = 0
    # Simplified: just return age + bonus
    damn p.age + bonus
}

sus result normie = process_person(john, get_index(5, 10))
vibez.spill("Complex function call:", result)  # Should be 25 + 15 = 40

# Test chained member access and array indexing
sus people [Person] = [john]
sus first_person_first_score normie = people[0].scores[0]
vibez.spill("Chained access:", first_person_first_score)  # Should be 85

# Test expressions in struct initialization
sus calculated_age normie = 20 + 5
sus bob Person = Person{
    name: "Bob",
    scores: [get_index(80, 5), get_index(90, 2), 88, 94],
    age: calculated_age
}

vibez.spill("Calculated person:", bob.age, bob.scores[0])  # Should be 25, 85

# Test complex boolean expressions
sus is_adult lit = john.age >= 18 && john.age < 65
sus has_good_grades lit = john.scores[0] >= 80 && john.scores[1] >= 80
vibez.spill("Boolean tests:", is_adult, has_good_grades)

# Test ternary-like expressions using match
sus grade_letter tea = match john.scores[0] {
    val when val >= 90 => "A"
    val when val >= 80 => "B"
    val when val >= 70 => "C"
    _ => "F"
}
vibez.spill("Grade letter:", grade_letter)

vibez.spill("Complex expression tests completed!")
