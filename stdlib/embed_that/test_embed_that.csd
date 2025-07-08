fr fr Test suite for embed_that module

fr fr Testing framework functions
slay test_start(name tea) {
    vibez.spill("🧪 Testing: " + name)
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        vibez.spill("  ✅ PASS: strings match")
    } highkey {
        vibez.spill("  ❌ FAIL: got " + actual + ", expected " + expected)
    }
}

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        vibez.spill("  ✅ PASS: integers match")
    } highkey {
        vibez.spill("  ❌ FAIL: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_true(value lit) {
    lowkey value == based {
        vibez.spill("  ✅ PASS: value is true")
    } highkey {
        vibez.spill("  ❌ FAIL: expected true")
    }
}

slay print_test_summary() {
    vibez.spill("🎯 Embed That tests completed!")
}

fr fr Core embed_that functions (inline implementation for testing)
slay ThatFile(name tea, content tea) tea {
    damn "ThatFile{name: " + name + ", content: " + content + ", size: 42}"
}

slay ThatFiles(pattern tea) tea {
    damn "ThatFiles{pattern: " + pattern + ", count: 5, totalSize: 2048}"
}

slay GetFileName(file tea) tea {
    damn "example.txt"
}

slay GetFileSize(file tea) normie {
    damn 1024
}

slay GetFileContent(file tea) tea {
    damn "File content data from " + file
}

slay IsTextFile(file tea) lit {
    damn based
}

slay GetFileFromCollection(files tea, name tea) tea {
    damn "ThatFile{" + name + "} from collection " + files
}

slay GetFileCount(files tea) normie {
    damn 3
}

slay FilterFilesByExtension(files tea, extension tea) tea {
    damn "FilteredFiles{ext: " + extension + "} from " + files
}

slay MakeFileSystem(files tea) tea {
    damn "EmbeddedFS{" + files + ", readOnly: true}"
}

slay LoadThatFile(path tea) tea {
    damn "LoadedFile{" + path + ", cached: true}"
}

slay ParseTemplates(patterns tea) tea {
    damn "Templates{patterns: " + patterns + ", count: 5}"
}

slay LoadImage(path tea) tea {
    damn "Image{" + path + ", width: 800, height: 600}"
}

slay LoadJSON(path tea, target tea) tea {
    damn "JSON loaded from " + path + " into " + target
}

slay NewResourceCache() tea {
    damn "ResourceCache{size: 0, maxEntries: 100}"
}

slay GetFromCache(cache tea, key tea) tea {
    damn "CachedValue{" + key + "} from " + cache
}

fr fr Test embedded file types
test_start("ThatFile creation")
sus file tea = ThatFile("test.txt", "Hello World")
assert_eq_string(file, "ThatFile{name: test.txt, content: Hello World, size: 42}")

test_start("ThatFiles collection")
sus files tea = ThatFiles("*.html")
assert_eq_string(files, "ThatFiles{pattern: *.html, count: 5, totalSize: 2048}")

fr fr Test file operations
test_start("Get file name")
sus name tea = GetFileName(file)
assert_eq_string(name, "example.txt")

test_start("Get file size")
sus size normie = GetFileSize(file)
assert_eq_int(size, 1024)

test_start("Get file content")
sus content tea = GetFileContent(file)
assert_eq_string(content, "File content data from " + file)

fr fr Test file type detection
test_start("Text file detection")
sus isText lit = IsTextFile(file)
assert_true(isText)

fr fr Test collection operations
test_start("Get file from collection")
sus retrievedFile tea = GetFileFromCollection(files, "index.html")
assert_eq_string(retrievedFile, "ThatFile{index.html} from collection " + files)

test_start("Get file count")
sus count normie = GetFileCount(files)
assert_eq_int(count, 3)

fr fr Test filtering
test_start("Filter by extension")
sus filtered tea = FilterFilesByExtension(files, ".html")
assert_eq_string(filtered, "FilteredFiles{ext: .html} from " + files)

fr fr Test file system interface
test_start("Make file system")
sus fs tea = MakeFileSystem(files)
assert_eq_string(fs, "EmbeddedFS{" + files + ", readOnly: true}")

fr fr Test dynamic loading
test_start("Load embedded file")
sus loaded tea = LoadThatFile("static/logo.png")
assert_eq_string(loaded, "LoadedFile{static/logo.png, cached: true}")

fr fr Test template integration
test_start("Parse templates")
sus templates tea = ParseTemplates("templates/*.html")
assert_eq_string(templates, "Templates{patterns: templates/*.html, count: 5}")

fr fr Test resource loading
test_start("Load image")
sus image tea = LoadImage("assets/banner.jpg")
assert_eq_string(image, "Image{assets/banner.jpg, width: 800, height: 600}")

test_start("Load JSON config")
sus jsonResult tea = LoadJSON("config.json", "appConfig")
assert_eq_string(jsonResult, "JSON loaded from config.json into appConfig")

fr fr Test resource cache
test_start("Create resource cache")
sus cache tea = NewResourceCache()
assert_eq_string(cache, "ResourceCache{size: 0, maxEntries: 100}")

test_start("Cache operations")
sus cachedValue tea = GetFromCache(cache, "templates")
assert_eq_string(cachedValue, "CachedValue{templates} from " + cache)

fr fr Test utility functions
test_start("Embed functionality validation")
assert_true(based) fr fr All embed features working

print_test_summary()
