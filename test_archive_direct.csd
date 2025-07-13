# Direct archive handling test without module import

# Archive Structure
sus archive_type tea = ""
sus archive_filename tea = ""
sus archive_files tea = ""
sus archive_loaded lit = cap

# Simple test function
slay archive_create_test(filename tea, format tea) lit {
    vibez.spill("Creating archive: " + filename + " format: " + format)
    archive_filename = filename
    archive_type = format
    archive_loaded = based
    damn based
}

vibez.spill("Testing direct archive functions")
sus result lit = archive_create_test("test.zip", "zip")
vibez.spill("Result: " + result)
vibez.spill("Archive filename: " + archive_filename)
vibez.spill("Archive type: " + archive_type)
