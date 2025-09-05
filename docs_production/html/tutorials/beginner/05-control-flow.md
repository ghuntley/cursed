# Control Flow

Control flow determines how your program executes. This tutorial covers conditionals, loops, and pattern matching in CURSED.

## Learning Objectives

By the end of this tutorial, you'll be able to:
- Use conditional statements with `lowkey` and `highkey`
- Write loops with `bestie` and `periodt`
- Pattern match with `vibe_check`
- Control loop execution with `ghosted` and `simp`
- Handle complex decision-making scenarios

## Conditional Statements

### Basic If-Else with `lowkey`/`highkey`

```cursed
# conditionals.💀 - Basic conditional statements

vibe main

yeet "vibez"

slay main() {
    sus age := 20
    sus has_license := based
    
    # Simple if statement
    lowkey age >= 18 {
        vibez.spill("You are an adult!")
    }
    
    # If-else statement
    lowkey has_license {
        vibez.spill("You can drive!")
    } highkey {
        vibez.spill("You need a license to drive.")
    }
    
    # Nested if-else
    lowkey age >= 21 {
        vibez.spill("You can drink alcohol.")
    } highkey lowkey age >= 18 {
        vibez.spill("You can vote but not drink.")
    } highkey {
        vibez.spill("You're still a minor.")
    }
}
```

### Conditional Expressions

```cursed
# conditional_expressions.💀 - Using conditions in expressions

vibe main

yeet "vibez"

slay get_grade_message(score normie) tea {
    # Conditional logic to determine message
    lowkey score >= 90 {
        damn "Excellent! A grade!"
    } highkey lowkey score >= 80 {
        damn "Good job! B grade!"
    } highkey lowkey score >= 70 {
        damn "Not bad! C grade!"
    } highkey lowkey score >= 60 {
        damn "Passing! D grade!"
    } highkey {
        damn "Need to study more! F grade!"
    }
}

slay main() {
    sus scores := []normie{95, 82, 67, 45}
    
    # Process each score
    vibez.spill("Grade Report:")
    vibez.spill("Score 95: " + get_grade_message(95))
    vibez.spill("Score 82: " + get_grade_message(82))
    vibez.spill("Score 67: " + get_grade_message(67))
    vibez.spill("Score 45: " + get_grade_message(45))
}
```

### Complex Conditions

```cursed
# complex_conditions.💀 - Complex boolean expressions

vibe main

yeet "vibez"

slay check_eligibility(age normie, has_id lit, has_insurance lit, income normie) {
    # Complex boolean conditions
    lowkey age >= 18 && has_id && has_insurance && income >= 25000 {
        vibez.spill("✅ Eligible for premium service")
    } highkey lowkey age >= 16 && has_id && income >= 15000 {
        vibez.spill("✅ Eligible for basic service")
    } highkey lowkey age >= 13 && has_id {
        vibez.spill("✅ Eligible for youth service")
    } highkey {
        vibez.spill("❌ Not eligible for any service")
    }
}

slay main() {
    vibez.spill("=== Service Eligibility Check ===")
    
    check_eligibility(25, based, based, 35000)
    check_eligibility(17, based, cap, 16000)
    check_eligibility(14, based, cap, 0)
    check_eligibility(12, cap, cap, 0)
}
```

## Pattern Matching with `vibe_check`

### Basic Switch Statements

```cursed
# pattern_matching.💀 - Pattern matching with vibe_check

vibe main

yeet "vibez"

slay describe_day(day tea) {
    vibe_check day {
        mood "Monday":
            vibez.spill("Start of the work week... ugh 😫")
        mood "Tuesday":
            vibez.spill("Tuesday blues continue...")
        mood "Wednesday":
            vibez.spill("Hump day! We're getting there! 🐪")
        mood "Thursday":
            vibez.spill("Almost Friday! Hang in there!")
        mood "Friday":
            vibez.spill("TGIF! Weekend is here! 🎉")
        mood "Saturday", "Sunday":
            vibez.spill("Weekend vibes! Time to relax! 😎")
        basic:
            vibez.spill("That's not a valid day...")
    }
}

slay process_grade(letter tea) {
    vibe_check letter {
        mood "A":
            vibez.spill("Outstanding! 🌟")
        mood "B":
            vibez.spill("Great work! 👍")
        mood "C":
            vibez.spill("Good effort! 👌")
        mood "D":
            vibez.spill("Needs improvement... 😐")
        mood "F":
            vibez.spill("Time to hit the books! 📚")
        basic:
            vibez.spill("Invalid grade!")
    }
}

slay main() {
    describe_day("Monday")
    describe_day("Friday")
    describe_day("Saturday")
    describe_day("InvalidDay")
    
    vibez.spill("---")
    
    process_grade("A")
    process_grade("C")
    process_grade("F")
}
```

### Numeric Pattern Matching

```cursed
# numeric_patterns.💀 - Pattern matching with numbers

vibe main

yeet "vibez"

slay describe_number(n normie) {
    vibe_check n {
        mood 0:
            vibez.spill("Zero - the beginning of everything!")
        mood 1:
            vibez.spill("One - the loneliest number")
        mood 2, 3, 5, 7:
            vibez.spill("Prime number!")
        mood 4, 6, 8, 9, 10:
            vibez.spill("Composite number!")
        basic:
            vibez.spill("Some other number: " + string(n))
    }
}

slay get_month_name(month normie) tea {
    vibe_check month {
        mood 1:
            damn "January"
        mood 2:
            damn "February"
        mood 3:
            damn "March"
        mood 4:
            damn "April"
        mood 5:
            damn "May"
        mood 6:
            damn "June"
        mood 7:
            damn "July"
        mood 8:
            damn "August"
        mood 9:
            damn "September"
        mood 10:
            damn "October"
        mood 11:
            damn "November"
        mood 12:
            damn "December"
        basic:
            damn "Invalid month"
    }
}

slay main() {
    sus numbers := []normie{0, 1, 2, 7, 9, 42}
    
    vibez.spill("=== Number Analysis ===")
    vibez.spill("0: "); describe_number(0)
    vibez.spill("1: "); describe_number(1)
    vibez.spill("7: "); describe_number(7)
    vibez.spill("9: "); describe_number(9)
    vibez.spill("42: "); describe_number(42)
    
    vibez.spill("\n=== Months ===")
    vibez.spill("Month 1: " + get_month_name(1))
    vibez.spill("Month 6: " + get_month_name(6))
    vibez.spill("Month 12: " + get_month_name(12))
    vibez.spill("Month 13: " + get_month_name(13))
}
```

## Loops

### For Loops with `bestie`

```cursed
# for_loops.💀 - For loop examples

vibe main

yeet "vibez"

slay main() {
    # C-style for loop
    vibez.spill("=== Countdown ===")
    bestie i := 5; i > 0; i-- {
        vibez.spill(string(i) + "...")
    }
    vibez.spill("Blast off! 🚀")
    
    # For loop with increment
    vibez.spill("\n=== Multiplication Table ===")
    bestie i := 1; i <= 10; i++ {
        sus result := i * 5
        vibez.spill(string(i) + " × 5 = " + string(result))
    }
    
    # Nested for loops
    vibez.spill("\n=== Grid Pattern ===")
    bestie row := 1; row <= 3; row++ {
        bestie col := 1; col <= 4; col++ {
            vibez.spill("[" + string(row) + "," + string(col) + "] ")
        }
        vibez.spill("")  # New line after each row
    }
}
```

### While Loops with `periodt`

```cursed
# while_loops.💀 - While loop examples

vibe main

yeet "vibez"

slay main() {
    # Basic while loop
    vibez.spill("=== Number Guessing Game ===")
    sus secret := 7
    sus guess := 1
    
    periodt guess != secret {
        vibez.spill("Guess " + string(guess) + " is wrong!")
        guess++
    }
    vibez.spill("Correct! The number was " + string(secret))
    
    # While loop with complex condition
    vibez.spill("\n=== Fibonacci Sequence ===")
    sus a, b := 0, 1
    sus count := 0
    
    periodt count < 10 {
        vibez.spill(string(a))
        sus next := a + b
        a = b
        b = next
        count++
    }
}
```

### Range-based Loops

```cursed
# range_loops.💀 - Iterating over collections

vibe main

yeet "vibez"

slay main() {
    # Array iteration (simplified - actual syntax may vary)
    sus fruits := []tea{"apple", "banana", "cherry", "date"}
    
    vibez.spill("=== Fruits List ===")
    # Note: This is conceptual - actual range syntax may be different
    bestie i := 0; i < 4; i++ {
        vibez.spill(string(i + 1) + ". " + fruits[i])
    }
    
    # Number ranges
    vibez.spill("\n=== Even Numbers ===")
    bestie i := 2; i <= 20; i += 2 {
        vibez.spill(string(i))
    }
    
    # Reverse iteration
    vibez.spill("\n=== Reverse Countdown ===")
    bestie i := 10; i >= 1; i-- {
        vibez.spill(string(i))
    }
}
```

## Loop Control

### Break with `ghosted`

```cursed
# loop_control.💀 - Loop control with ghosted and simp

vibe main

yeet "vibez"

slay find_first_even(numbers []normie) {
    vibez.spill("Looking for first even number...")
    
    bestie i := 0; i < 10; i++ {
        sus num := numbers[i]
        vibez.spill("Checking: " + string(num))
        
        lowkey num % 2 == 0 {
            vibez.spill("Found even number: " + string(num))
            ghosted  # Break out of loop
        }
    }
    
    vibez.spill("No even number found")
}

slay skip_negative_numbers(numbers []normie) {
    vibez.spill("Processing numbers, skipping negatives...")
    
    bestie i := 0; i < 8; i++ {
        sus num := numbers[i]
        
        lowkey num < 0 {
            vibez.spill("Skipping negative: " + string(num))
            simp  # Continue to next iteration
        }
        
        vibez.spill("Processing: " + string(num) + " → " + string(num * 2))
    }
}

slay main() {
    sus test_numbers := []normie{1, 3, 5, 8, 9, 11, 14, 17}
    find_first_even(test_numbers)
    
    vibez.spill("\n---")
    
    sus mixed_numbers := []normie{5, -3, 10, -7, 2, -1, 8, 4}
    skip_negative_numbers(mixed_numbers)
}
```

### Labeled Breaks and Continues

```cursed
# labeled_control.💀 - Labeled loop control

vibe main

yeet "vibez"

slay find_in_matrix() {
    vibez.spill("=== Matrix Search ===")
    
    # Simulated 2D array search
    outer_loop: bestie row := 1; row <= 3; row++ {
        inner_loop: bestie col := 1; col <= 4; col++ {
            sus value := row * col
            
            lowkey value == 6 {
                vibez.spill("Found target value " + string(value) + " at [" + string(row) + "," + string(col) + "]")
                ghosted outer_loop  # Break out of both loops
            }
            
            vibez.spill("Checking [" + string(row) + "," + string(col) + "] = " + string(value))
        }
    }
}

slay process_batches() {
    vibez.spill("\n=== Batch Processing ===")
    
    batch_loop: bestie batch := 1; batch <= 3; batch++ {
        vibez.spill("Processing batch " + string(batch))
        
        item_loop: bestie item := 1; item <= 5; item++ {
            lowkey item == 3 {
                vibez.spill("  Item " + string(item) + " failed, skipping to next batch")
                simp batch_loop  # Continue outer loop
            }
            
            vibez.spill("  Processing item " + string(item))
        }
        
        vibez.spill("Batch " + string(batch) + " completed")
    }
}

slay main() {
    find_in_matrix()
    process_batches()
}
```

## Exercise: Text Adventure Game

Create a program called `adventure.💀` that implements a simple text adventure game using various control flow structures:

### Solution

```cursed
# adventure.💀 - Text adventure game with control flow

vibe main

yeet "vibez"

# Game state
sus player_health := 100
sus player_gold := 50
sus has_sword := cap
sus game_over := cap

slay display_status() {
    vibez.spill("\n=== Player Status ===")
    vibez.spill("Health: " + string(player_health))
    vibez.spill("Gold: " + string(player_gold))
    vibez.spill("Has Sword: " + string(has_sword))
}

slay encounter_goblin() {
    vibez.spill("\n🧌 A goblin appears!")
    
    lowkey has_sword {
        vibez.spill("You defeat the goblin with your sword!")
        player_gold += 20
        vibez.spill("You found 20 gold!")
    } highkey {
        vibez.spill("The goblin attacks! You take damage!")
        player_health -= 30
        
        lowkey player_health <= 0 {
            vibez.spill("💀 You died! Game Over!")
            game_over = based
        }
    }
}

slay visit_shop() {
    vibez.spill("\n🏪 Welcome to the weapon shop!")
    
    lowkey has_sword {
        vibez.spill("You already have a sword!")
    } highkey lowkey player_gold >= 50 {
        vibez.spill("You buy a sword for 50 gold!")
        player_gold -= 50
        has_sword = based
    } highkey {
        vibez.spill("You don't have enough gold for a sword (50 gold needed)")
    }
}

slay find_treasure() {
    vibez.spill("\n💎 You found a treasure chest!")
    
    sus treasure_gold := 30
    player_gold += treasure_gold
    vibez.spill("You found " + string(treasure_gold) + " gold!")
}

slay heal_at_inn() {
    vibez.spill("\n🏨 You rest at the inn!")
    
    lowkey player_gold >= 20 {
        player_gold -= 20
        player_health = 100
        vibez.spill("You pay 20 gold and restore full health!")
    } highkey {
        vibez.spill("You don't have enough gold for healing (20 gold needed)")
    }
}

slay choose_action(action normie) {
    vibe_check action {
        mood 1:
            encounter_goblin()
        mood 2:
            visit_shop()
        mood 3:
            find_treasure()
        mood 4:
            heal_at_inn()
        mood 5:
            vibez.spill("👋 Thanks for playing! Goodbye!")
            game_over = based
        basic:
            vibez.spill("Invalid choice! Try again.")
    }
}

slay main() {
    vibez.spill("🗡️ Welcome to the CURSED Adventure Game! 🗡️")
    vibez.spill("Survive encounters and collect gold!")
    
    # Main game loop
    periodt !game_over {
        display_status()
        
        # Check win condition
        lowkey player_gold >= 100 {
            vibez.spill("\n🎉 You won! You collected 100+ gold!")
            ghosted
        }
        
        # Check lose condition
        lowkey player_health <= 0 {
            vibez.spill("\n💀 Game Over! You died!")
            ghosted
        }
        
        # Show menu
        vibez.spill("\n=== What do you want to do? ===")
        vibez.spill("1. Fight a goblin")
        vibez.spill("2. Visit weapon shop")
        vibez.spill("3. Search for treasure")
        vibez.spill("4. Rest at inn")
        vibez.spill("5. Quit game")
        
        # Simulate user input (in real game, this would be actual input)
        sus turn := 1
        bestie turn <= 10 && !game_over {
            # Simulate different choices
            sus choice normie
            vibe_check turn {
                mood 1:
                    choice = 2  # Buy sword first
                mood 2:
                    choice = 3  # Find treasure
                mood 3:
                    choice = 1  # Fight goblin
                mood 4:
                    choice = 3  # More treasure
                mood 5:
                    choice = 1  # Fight another goblin
                basic:
                    choice = 5  # Quit
            }
            
            vibez.spill("Choice: " + string(choice))
            choose_action(choice)
            
            turn++
        }
    }
    
    display_status()
    vibez.spill("\n=== Final Score ===")
    vibez.spill("Gold collected: " + string(player_gold))
    vibez.spill("Health remaining: " + string(player_health))
}
```

## Common Control Flow Mistakes

### 1. Missing Braces
```cursed
# ❌ Wrong - missing braces
lowkey age >= 18
    vibez.spill("Adult")

# ✅ Correct
lowkey age >= 18 {
    vibez.spill("Adult")
}
```

### 2. Infinite Loops
```cursed
# ❌ Wrong - infinite loop
sus i := 0
periodt i < 10 {
    vibez.spill(string(i))
    # Missing i++
}

# ✅ Correct
sus i := 0
periodt i < 10 {
    vibez.spill(string(i))
    i++
}
```

### 3. Unreachable Code
```cursed
# ❌ Wrong - code after return
slay example() {
    damn "result"
    vibez.spill("This will never execute")
}

# ✅ Correct
slay example() {
    vibez.spill("This executes")
    damn "result"
}
```

## Best Practices

1. **Use meaningful conditions**: `lowkey user_is_admin` instead of `lowkey flag == based`
2. **Keep nesting shallow**: Use early returns and breaks to reduce nesting
3. **Use pattern matching**: `vibe_check` is often clearer than multiple `lowkey` statements
4. **Label complex loops**: Use labels for nested loops with breaks/continues
5. **Avoid infinite loops**: Always ensure loop conditions can become false
6. **Use appropriate loop types**: `bestie` for counting, `periodt` for conditions

## What's Next?

Now that you understand control flow, let's explore data structures in the next tutorial: [Data Structures](06-data-structures.md).

## Key Takeaways

- `lowkey`/`highkey` are used for conditional statements
- `vibe_check` provides pattern matching for multiple conditions
- `bestie` creates for loops, `periodt` creates while loops
- `ghosted` breaks out of loops, `simp` continues to next iteration
- Labels allow control of nested loops
- Choose the right control structure for each situation
- Always ensure loops can terminate
- Pattern matching often provides cleaner code than multiple if statements

Control flow is essential for creating dynamic, interactive programs in CURSED!
