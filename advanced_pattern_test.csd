sus name tea = "Alice"
ready (name) {
    "Bob" => vibez.spill("Hello Bob!")
    "Alice" => vibez.spill("Welcome Alice!")
    _ => vibez.spill("Unknown person")
}

sus flag lit = based
ready (flag) {
    based => vibez.spill("It's true!")
    cringe => vibez.spill("It's false!")
    _ => vibez.spill("Unknown boolean")
}

sus score drip = 85
ready (score) {
    90..100 => vibez.spill("Grade A")
    80..89 => vibez.spill("Grade B") 
    70..79 => vibez.spill("Grade C")
    _ => vibez.spill("Below grade C")
}
