# Simple I/O module for testing

slay println(message tea) {
    vibez.spill(message)
}

slay eprintln(message tea) {
    vibez.spill("ERROR: " + message)
}

slay read_to_string(filename tea) {
    lowkey filename == "test.csd" {
        damn "vibez.spill(\"Hello from CURSED file\")"
    }
    damn "File content"
}

slay write(filename tea, content tea) {
    damn based
}
