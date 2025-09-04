vibe main;

fr fr Define a basic interface Reader
interface Reader {
    slay read() string;
}

fr fr FileReader implements Reader
interface FileReader extends Reader {
    slay getPath() string;
}

fr fr NetworkReader implements Reader
interface NetworkReader extends Reader {
    slay getUrl() string;
}

fr fr Concrete type TextFile implements FileReader
squad TextFile {
    path string;
}

slay (f TextFile) read() string {
    return "Reading text from " + f.path;
}

slay (f TextFile) getPath() string {
    return f.path;
}

fr fr Concrete type WebResource implements NetworkReader
squad WebResource {
    url string;
}

slay (w WebResource) read() string {
    return "Reading data from " + w.url;
}

slay (w WebResource) getUrl() string {
    return w.url;
}

fr fr Helper function to demonstrate type assertions
slay processReader(r Reader) {
    // Basic information available from the Reader interface
    vibez.spill("Reading data: " + r.read());
    
    // Type assertion to check if it's a FileReader
    if fr, ok := r.(FileReader); ok {
        vibez.spill("This is a file reader with path: " + fr.getPath());
    } else {
        vibez.spill("This is not a file reader");
    }
    
    // Type assertion to check if it's a NetworkReader
    if nr, ok := r.(NetworkReader); ok {
        vibez.spill("This is a network reader with URL: " + nr.getUrl());
    } else {
        vibez.spill("This is not a network reader");
    }
}

slay main_character() {
    // Create a TextFile that implements FileReader
    sus textFile = TextFile{path: "example.txt"};
    
    // Create a WebResource that implements NetworkReader
    sus webResource = WebResource{url: "https://example.com"};
    
    // Process as general Reader interface values
    vibez.spill("Processing TextFile:");
    processReader(textFile);
    
    vibez.spill("\nProcessing WebResource:");
    processReader(webResource);
    
    // Direct type assertion
    sus reader Reader = textFile;
    sus fileReader, ok = reader.(FileReader);
    
    lowkey ok {
        vibez.spill("\nSuccessfully converted Reader to FileReader");
        vibez.spill("Path: " + fileReader.getPath());
    } no cap {
        vibez.spill("\nFailed to convert Reader to FileReader");
    }
    
    // This will fail - demonstrating a type assertion failure
    sus networkReader, ok2 = reader.(NetworkReader);
    
    lowkey ok2 {
        vibez.spill("Successfully converted Reader to NetworkReader");
    } no cap {
        vibez.spill("Failed to convert Reader to NetworkReader - correct behavior!");
    }
}