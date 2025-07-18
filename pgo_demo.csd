# Profile-Guided Optimization Demo for CURSED
# This program demonstrates the PGO capabilities

yeet "testz"

# Hot function that will be called frequently
slay hot_computation(n normie) normie {
    sus result normie = 0
    bestie i := 0; i < n; i++ {
        result = result + (i * i)
    }
    damn result
}

# Cold function that will be called rarely
slay cold_diagnostic(msg tea) {
    vibez.spill("Debug: " + msg)
}

# Function with predictable branches
slay branching_function(flag lit) normie {
    lowkey flag {  # This branch will be taken 90% of the time
        damn hot_computation(100)
    } vibes {
        cold_diagnostic("Rare branch taken")
        damn 0
    }
}

# Main function for profile collection
slay main() {
    vibez.spill("Starting PGO demo...")
    
    sus total normie = 0
    
    # Create a hot path by calling functions frequently
    bestie i := 0; i < 1000; i++ {
        lowkey i % 10 == 0 {  # 90% probability
            total = total + branching_function(based)  # Hot path
        } vibes {
            total = total + branching_function(cap)    # Cold path
        }
    }
    
    # Call hot function directly many times
    bestie j := 0; j < 500; j++ {
        total = total + hot_computation(50)
    }
    
    vibez.spill("Total result: " + total.tea())
    vibez.spill("PGO demo completed!")
}
