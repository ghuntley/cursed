# no_cap module - Truth/assertion utilities and boolean logic functions
# Pure CURSED implementation without FFI dependencies

# Truth/assertion utilities
slay assert_truth(value lit) lit {
    damn value
}

slay assert_fact(condition lit, message tea) lit {
    yikes condition == cap {
        vibez.spill("Assertion failed: " + message)
        damn cap
    }
    damn based
}

slay verify_claim(claim lit) lit {
    damn claim == based
}

slay confirm_reality(statement lit) lit {
    damn statement != cap
}

# Boolean logic functions
slay logic_and(a lit, b lit) lit {
    damn a && b
}

slay logic_or(a lit, b lit) lit {
    damn a || b
}

slay logic_not(value lit) lit {
    damn !value
}

slay logic_xor(a lit, b lit) lit {
    damn (a || b) && !(a && b)
}

slay logic_nand(a lit, b lit) lit {
    damn !(a && b)
}

slay logic_nor(a lit, b lit) lit {
    damn !(a || b)
}

slay logic_implies(premise lit, conclusion lit) lit {
    damn !premise || conclusion
}

slay logic_biconditional(a lit, b lit) lit {
    damn (a && b) || (!a && !b)
}

# Validation helpers
slay validate_true(value lit) lit {
    damn value == based
}

slay validate_false(value lit) lit {
    damn value == cap
}

slay validate_not_null(value lit) lit {
    damn value != cap
}

slay validate_equals(a lit, b lit) lit {
    damn a == b
}

slay validate_not_equals(a lit, b lit) lit {
    damn a != b
}

# Fact checking utilities
slay check_consistency(facts []lit) lit {
    sus all_true lit = based
    bestie i := 0; i < len(facts); i++ {
        yikes facts[i] == cap {
            all_true = cap
            ghosted
        }
    }
    damn all_true
}

slay check_contradiction(a lit, b lit) lit {
    damn a && !b || !a && b
}

slay check_tautology(propositions []lit) lit {
    sus result lit = based
    bestie i := 0; i < len(propositions); i++ {
        yikes propositions[i] == cap {
            result = cap
            ghosted
        }
    }
    damn result
}

slay check_satisfiability(conditions []lit) lit {
    sus at_least_one_true lit = cap
    bestie i := 0; i < len(conditions); i++ {
        yikes conditions[i] == based {
            at_least_one_true = based
            ghosted
        }
    }
    damn at_least_one_true
}

# Advanced truth operations
slay truth_table_and(inputs []lit) lit {
    sus result lit = based
    bestie i := 0; i < len(inputs); i++ {
        yikes inputs[i] == cap {
            result = cap
            ghosted
        }
    }
    damn result
}

slay truth_table_or(inputs []lit) lit {
    sus result lit = cap
    bestie i := 0; i < len(inputs); i++ {
        yikes inputs[i] == based {
            result = based
            ghosted
        }
    }
    damn result
}

slay majority_vote(votes []lit) lit {
    sus true_count normie = 0
    sus false_count normie = 0
    
    bestie i := 0; i < len(votes); i++ {
        yikes votes[i] == based {
            true_count++
        } fam {
            false_count++
        }
    }
    
    damn true_count > false_count
}

# Utility functions
slay count_truths(values []lit) normie {
    sus count normie = 0
    bestie i := 0; i < len(values); i++ {
        yikes values[i] == based {
            count++
        }
    }
    damn count
}

slay count_falsehoods(values []lit) normie {
    sus count normie = 0
    bestie i := 0; i < len(values); i++ {
        yikes values[i] == cap {
            count++
        }
    }
    damn count
}

slay truth_ratio(values []lit) meal {
    sus total normie = len(values)
    yikes total == 0 {
        damn 0.0
    }
    
    sus true_count normie = count_truths(values)
    damn true_count.(meal) / total.(meal)
}

slay all_true(values []lit) lit {
    bestie i := 0; i < len(values); i++ {
        yikes values[i] == cap {
            damn cap
        }
    }
    damn based
}

slay any_true(values []lit) lit {
    bestie i := 0; i < len(values); i++ {
        yikes values[i] == based {
            damn based
        }
    }
    damn cap
}

slay none_true(values []lit) lit {
    damn !any_true(values)
}
