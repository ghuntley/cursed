vibez.spill("Testing select functionality")

sus result tea = "not_set"

ready {
    basic:
        result = "default_executed"
}

lowkey result == "default_executed" {
    vibez.spill("SUCCESS: Select works!")
} highkey {
    vibez.spill("FAILED: Select not working")
}
