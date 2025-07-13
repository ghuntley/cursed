yeet "dropz"

vibez.spill("Testing dropz without test framework")

# Test basic dropz functions
sus result tea = dropz.init_dropz()
vibez.spill("Init result: " + result)

# Test constants
vibez.spill("O_RDONLY: " + dropz.O_RDONLY)
vibez.spill("EOF: " + dropz.EOF)

# Test read file
sus (content, err) = dropz.read_text_file("main.csd")
vibez.spill("Read error: " + err)
vibez.spill("Content length > 0: " + (len(content) > 0))

vibez.spill("Basic dropz functionality test complete")
