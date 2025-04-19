vibe rizztemplate_parsefile_test

yeet "vibez"
yeet "dropz"
yeet "rizztemplate"

slay main() {
    vibez.spill("Testing rizztemplate ParseFiles and ParseGlob functionality")
    
    fr fr Test ParseFiles function
    test_parse_files()
    
    fr fr Test ParseGlob function
    test_parse_glob()
    
    vibez.spill("All rizztemplate file parsing tests passed!")
}

slay test_parse_files() {
    vibez.spill("Testing ParseFiles functionality")
    
    fr fr Create some test template files
    create_test_templates()
    
    fr fr Parse multiple template files
    sus tmpl, err := rizztemplate.ParseFiles(["./tests/temp/template1.tmpl", "./tests/temp/template2.tmpl"])
    lowkey err != cap {
        panic("ParseFiles failed: " + err)
    }
    
    fr fr Check that we can execute each template
    sus data := squad{
        "Name": "bestie",
    }
    
    fr fr Test template1
    sus buf1 := dropz.NewBuffer(cap)
    err = tmpl.ExecuteTemplate(buf1, "template1", data)
    lowkey err != cap {
        panic("ExecuteTemplate for template1 failed: " + err)
    }
    
    lowkey buf1.String() != "Hello, bestie from template1!" {
        panic("Unexpected output from template1: " + buf1.String())
    }
    
    fr fr Test template2
    sus buf2 := dropz.NewBuffer(cap)
    err = tmpl.ExecuteTemplate(buf2, "template2", data)
    lowkey err != cap {
        panic("ExecuteTemplate for template2 failed: " + err)
    }
    
    lowkey buf2.String() != "Greetings, bestie from template2!" {
        panic("Unexpected output from template2: " + buf2.String())
    }
    
    fr fr Cleanup test files
    cleanup_test_templates()
}

slay test_parse_glob() {
    vibez.spill("Testing ParseGlob functionality")
    
    fr fr Create some test template files
    create_test_templates()
    
    fr fr Parse template files using glob pattern
    sus tmpl, err := rizztemplate.ParseGlob("./tests/temp/*.tmpl")
    lowkey err != cap {
        panic("ParseGlob failed: " + err)
    }
    
    fr fr Check that we can execute each template
    sus data := squad{
        "Name": "bestie",
    }
    
    fr fr Test template1
    sus buf1 := dropz.NewBuffer(cap)
    err = tmpl.ExecuteTemplate(buf1, "template1", data)
    lowkey err != cap {
        panic("ExecuteTemplate for template1 failed: " + err)
    }
    
    lowkey buf1.String() != "Hello, bestie from template1!" {
        panic("Unexpected output from template1: " + buf1.String())
    }
    
    fr fr Test template2
    sus buf2 := dropz.NewBuffer(cap)
    err = tmpl.ExecuteTemplate(buf2, "template2", data)
    lowkey err != cap {
        panic("ExecuteTemplate for template2 failed: " + err)
    }
    
    lowkey buf2.String() != "Greetings, bestie from template2!" {
        panic("Unexpected output from template2: " + buf2.String())
    }
    
    fr fr Test specific template lookup
    lowkey tmpl.Lookup("template1") == cap {
        panic("Lookup for template1 failed")
    }
    
    fr fr Test missing template lookup
    lowkey tmpl.Lookup("non_existent") != cap {
        panic("Lookup for non_existent template should return nil")
    }
    
    fr fr Cleanup test files
    cleanup_test_templates()
}

fr fr Helper function to create test template files
slay create_test_templates() {
    fr fr Create temp directory if it doesn't exist
    dropz.CreateDirectory("./tests/temp")
    
    fr fr Create test template files
    dropz.WriteFile("./tests/temp/template1.tmpl", "Hello, {{ .Name }} from template1!")
    dropz.WriteFile("./tests/temp/template2.tmpl", "Greetings, {{ .Name }} from template2!")
}

fr fr Helper function to clean up test template files
slay cleanup_test_templates() {
    fr fr Remove test template files
    dropz.RemoveFile("./tests/temp/template1.tmpl")
    dropz.RemoveFile("./tests/temp/template2.tmpl")
}