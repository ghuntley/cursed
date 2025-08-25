yeet "slay_io"

fr fr Simple scanning test
vibez.spill("Testing token scanning...")

sus scanner tea = NewSlayScanner("hello world")
vibez.spill("Created scanner: " + scanner)

sus result lit = ScanNext(scanner)
vibez.spill("First scan result: " + tea(result))

sus empty_scanner tea = NewSlayScanner("")
sus empty_result lit = ScanNext(empty_scanner)
vibez.spill("Empty scan result: " + tea(empty_result))

vibez.spill("Scanning test complete!")
