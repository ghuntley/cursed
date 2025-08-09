sus score drip = 95

ready (score) {
    0..59 => vibez.spill("Grade: F")
    60..69 => vibez.spill("Grade: D") 
    70..79 => vibez.spill("Grade: C")
    80..89 => vibez.spill("Grade: B")
    90..100 => vibez.spill("Grade: A")
    _ => vibez.spill("Grade: Invalid")
}
