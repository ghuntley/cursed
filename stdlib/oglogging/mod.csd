# Basic logging functions only

slay Spill(message tea) {
    vibez.spill("LOG: " + message)
}

slay Debug(message tea) {
    vibez.spill("[DEBUG] " + message)
}

slay Info(message tea) {
    vibez.spill("[INFO] " + message)
}

slay Warn(message tea) {
    vibez.spill("[WARN] " + message)
}

slay Error(message tea) {
    vibez.spill("[ERROR] " + message)
}

slay Fatal(message tea) {
    vibez.spill("[FATAL] " + message)
}

# Constants
sus DEBUG normie = 0
sus INFO normie = 1
sus WARN normie = 2
sus ERROR normie = 3
sus FATAL normie = 4
