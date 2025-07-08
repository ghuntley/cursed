fr fr Minimal debug test
slay simple_function(param tea) tea {
    damn "result"
}

vibez.spill("Testing simple function")
sus result tea = simple_function("test")
vibez.spill("Result: " + result)

slay test_params(name tea, value tea) tea {
    if name == "ping" {
        damn "pong"
    }
    damn "default"
}

vibez.spill("Testing parameter function")
sus test_result tea = test_params("ping", "test")
vibez.spill("Test result: " + test_result)
