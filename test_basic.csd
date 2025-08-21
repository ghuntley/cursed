// Basic CURSED program for debugger testing
sus name tea = "CURSED Developer"
sus age drip = 25
sus active lit = based

vibez.spill("Hello,", name)
vibez.spill("Age:", age)

ready (active) {
    vibez.spill("Status: Active!")
} otherwise {
    vibez.spill("Status: Inactive")
}

bestie (age > 0) {
    vibez.spill("Processing user:", name)
    age = age - 1
    ready (age == 0) {
        cap
    }
}

vibez.spill("Done processing!")
