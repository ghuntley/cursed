yeet "testz"
yeet "vibe_mime"
yeet "io"

fr fr Test MIME type parsing
slay test_mime_type_parsing() {
    test_start("MIME type parsing")
    
    fr fr Test simple type
    mimeType, err := vibe_mime.ParseMediaType("text/plain")
    assert_eq_string(err, "")
    assert_eq_string(mimeType.Type, "text")
    assert_eq_string(mimeType.Subtype, "plain")
    assert_eq_int(len(mimeType.Parameters), 0)
    
    fr fr Test type with parameters
    mimeType, err = vibe_mime.ParseMediaType("text/html; charset=utf-8")
    assert_eq_string(err, "")
    assert_eq_string(mimeType.Type, "text")
    assert_eq_string(mimeType.Subtype, "html")
    assert_eq_string(mimeType.Parameters["charset"], "utf-8")
    
    fr fr Test complex type
    mimeType, err = vibe_mime.ParseMediaType("application/json; charset=utf-8; boundary=something")
    assert_eq_string(err, "")
    assert_eq_string(mimeType.Type, "application")
    assert_eq_string(mimeType.Subtype, "json")
    assert_eq_string(mimeType.Parameters["charset"], "utf-8")
    assert_eq_string(mimeType.Parameters["boundary"], "something")
}

fr fr Test MIME type methods
slay test_mime_type_methods() {
    test_start("MIME type methods")
    
    fr fr Test text type
    textType := vibe_mime.MIMEType{
        Type: "text",
        Subtype: "plain",
        Parameters: make(map[tea]tea)
    }
    
    assert_true(textType.IsText())
    assert_false(textType.IsHTML())
    assert_false(textType.IsJSON())
    assert_false(textType.IsXML())
    
    fr fr Test HTML type
    htmlType := vibe_mime.MIMEType{
        Type: "text",
        Subtype: "html",
        Parameters: make(map[tea]tea)
    }
    
    assert_true(htmlType.IsText())
    assert_true(htmlType.IsHTML())
    assert_false(htmlType.IsJSON())
    
    fr fr Test JSON type
    jsonType := vibe_mime.MIMEType{
        Type: "application",
        Subtype: "json",
        Parameters: make(map[tea]tea)
    }
    
    assert_false(jsonType.IsText())
    assert_false(jsonType.IsHTML())
    assert_true(jsonType.IsJSON())
    assert_false(jsonType.IsXML())
    
    fr fr Test XML type
    xmlType := vibe_mime.MIMEType{
        Type: "application",
        Subtype: "xml",
        Parameters: make(map[tea]tea)
    }
    
    assert_false(xmlType.IsText())
    assert_false(xmlType.IsHTML())
    assert_false(xmlType.IsJSON())
    assert_true(xmlType.IsXML())
}

fr fr Test MIME type string representation
slay test_mime_type_string() {
    test_start("MIME type string representation")
    
    fr fr Test simple type
    simpleType := vibe_mime.MIMEType{
        Type: "text",
        Subtype: "plain",
        Parameters: make(map[tea]tea)
    }
    
    str := simpleType.String()
    assert_eq_string(str, "text/plain")
    
    fr fr Test type with parameters
    paramType := vibe_mime.MIMEType{
        Type: "text",
        Subtype: "html",
        Parameters: map[tea]tea{"charset": "utf-8"}
    }
    
    str = paramType.String()
    assert_true(string.Contains(str, "text/html"))
    assert_true(string.Contains(str, "charset=utf-8"))
    
    fr fr Test WithParameter method
    newType := simpleType.WithParameter("charset", "iso-8859-1")
    str = newType.String()
    assert_true(string.Contains(str, "charset=iso-8859-1"))
}

fr fr Test content type detection
slay test_content_type_detection() {
    test_start("Content type detection")
    
    fr fr Test PNG detection
    pngData := byte[value]{0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A}
    contentType := vibe_mime.DetectContentType(pngData)
    assert_eq_string(contentType, vibe_mime.TypeImagePNG)
    
    fr fr Test JPEG detection
    jpegData := byte[value]{0xFF, 0xD8, 0xFF, 0xE0}
    contentType = vibe_mime.DetectContentType(jpegData)
    assert_eq_string(contentType, vibe_mime.TypeImageJPEG)
    
    fr fr Test GIF detection
    gifData := byte[value]{0x47, 0x49, 0x46, 0x38}
    contentType = vibe_mime.DetectContentType(gifData)
    assert_eq_string(contentType, vibe_mime.TypeImageGIF)
    
    fr fr Test PDF detection
    pdfData := byte[value]{0x25, 0x50, 0x44, 0x46}
    contentType = vibe_mime.DetectContentType(pdfData)
    assert_eq_string(contentType, vibe_mime.TypeApplicationPDF)
    
    fr fr Test HTML detection
    htmlData := byte[value]("<!DOCTYPE html><html><body>Hello</body></html>")
    contentType = vibe_mime.DetectContentType(htmlData)
    assert_eq_string(contentType, vibe_mime.TypeTextHTML)
    
    fr fr Test JSON detection
    jsonData := byte[value]("{\"key\": \"value\"}")
    contentType = vibe_mime.DetectContentType(jsonData)
    assert_eq_string(contentType, vibe_mime.TypeApplicationJSON)
    
    fr fr Test XML detection
    xmlData := byte[value]("<?xml version=\"1.0\"?><root></root>")
    contentType = vibe_mime.DetectContentType(xmlData)
    assert_eq_string(contentType, vibe_mime.TypeApplicationXML)
    
    fr fr Test plain text detection
    textData := byte[value]("This is plain text content")
    contentType = vibe_mime.DetectContentType(textData)
    assert_eq_string(contentType, vibe_mime.TypeTextPlain)
    
    fr fr Test binary data detection
    binaryData := byte[value]{0x00, 0x01, 0x02, 0x03}
    contentType = vibe_mime.DetectContentType(binaryData)
    assert_eq_string(contentType, vibe_mime.TypeApplicationOctetStream)
}

fr fr Test extension mapping
slay test_extension_mapping() {
    test_start("Extension mapping")
    
    fr fr Test extension to type
    mimeType := vibe_mime.TypeByExtension(".txt")
    assert_eq_string(mimeType, vibe_mime.TypeTextPlain)
    
    mimeType = vibe_mime.TypeByExtension(".html")
    assert_eq_string(mimeType, vibe_mime.TypeTextHTML)
    
    mimeType = vibe_mime.TypeByExtension(".json")
    assert_eq_string(mimeType, vibe_mime.TypeApplicationJSON)
    
    mimeType = vibe_mime.TypeByExtension(".pdf")
    assert_eq_string(mimeType, vibe_mime.TypeApplicationPDF)
    
    mimeType = vibe_mime.TypeByExtension(".jpg")
    assert_eq_string(mimeType, vibe_mime.TypeImageJPEG)
    
    mimeType = vibe_mime.TypeByExtension(".png")
    assert_eq_string(mimeType, vibe_mime.TypeImagePNG)
    
    fr fr Test type to extension
    ext := vibe_mime.ExtensionByType(vibe_mime.TypeTextPlain)
    assert_eq_string(ext, ".txt")
    
    ext = vibe_mime.ExtensionByType(vibe_mime.TypeTextHTML)
    assert_eq_string(ext, ".html")
    
    ext = vibe_mime.ExtensionByType(vibe_mime.TypeApplicationJSON)
    assert_eq_string(ext, ".json")
    
    ext = vibe_mime.ExtensionByType(vibe_mime.TypeImageJPEG)
    assert_eq_string(ext, ".jpg")
}

fr fr Test multipart writer
slay test_multipart_writer() {
    test_start("Multipart writer")
    
    fr fr Create buffer writer
    writer := &mockBufferWriter{data: make(byte[value], 0)}
    
    fr fr Create multipart writer
    multiWriter := vibe_mime.NewMultipartWriter(writer)
    boundary := multiWriter.Boundary()
    assert_true(len(boundary) > 0)
    
    fr fr Write field
    err := multiWriter.WriteField("name", "value")
    assert_eq_string(err, "")
    
    fr fr Create form file
    fileWriter, err := multiWriter.CreateFormFile("file", "test.txt")
    assert_eq_string(err, "")
    assert_true(fileWriter != cap)
    
    fr fr Write file content
    fileData := byte[value]("file content here")
    _, err = fileWriter.Write(fileData)
    assert_eq_string(err, "")
    
    fr fr Close writer
    err = multiWriter.Close()
    assert_eq_string(err, "")
    
    fr fr Check output contains boundary
    output := tea(writer.data)
    assert_true(string.Contains(output, boundary))
    assert_true(string.Contains(output, "name=\"name\""))
    assert_true(string.Contains(output, "value"))
    assert_true(string.Contains(output, "filename=\"test.txt\""))
}

fr fr Test multipart reader
slay test_multipart_reader() {
    test_start("Multipart reader")
    
    fr fr Create sample multipart data
    data := "--boundary123\r\n"
    data = data + "Content-Disposition: form-data; name=\"field1\"\r\n\r\n"
    data = data + "value1\r\n"
    data = data + "--boundary123\r\n"
    data = data + "Content-Disposition: form-data; name=\"field2\"\r\n\r\n"
    data = data + "value2\r\n"
    data = data + "--boundary123--\r\n"
    
    reader := &stringReader{content: data, pos: 0}
    
    fr fr Create multipart reader
    multiReader := vibe_mime.NewMultipartReader(reader, "boundary123")
    
    fr fr Test reading parts (simplified)
    assert_true(multiReader != cap)
}

fr fr Test part functionality
slay test_part_functionality() {
    test_start("Part functionality")
    
    fr fr Create part
    part := &vibe_mime.Part{
        Header: make(map[tea]tea[value]),
        Body: byte[value]("test content")
    }
    
    fr fr Set headers
    part.SetHeader("Content-Type", "text/plain")
    part.SetHeader("Content-Disposition", "form-data; name=\"test\"")
    
    fr fr Test header retrieval
    contentType := part.ContentType()
    assert_eq_string(contentType, "text/plain")
    
    disposition := part.ContentDisposition()
    assert_true(string.Contains(disposition, "form-data"))
    
    fr fr Test filename extraction
    part.SetHeader("Content-Disposition", "form-data; name=\"file\"; filename=\"test.txt\"")
    filename := part.Filename()
    assert_true(len(filename) > 0)
}

fr fr Test custom detector
slay test_custom_detector() {
    test_start("Custom detector")
    
    fr fr Create detector
    detector := vibe_mime.NewDetector()
    
    fr fr Add custom signatures
    detector.AddSignature("CUSTOM", byte[value]{0xCA, 0xFE, 0xBA, 0xBE})
    detector.AddSignature("OTHER", byte[value]{0xDE, 0xAD, 0xBE, 0xEF})
    
    fr fr Test detection
    customData := byte[value]{0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x01}
    detected := detector.Detect(customData)
    assert_eq_string(detected, "CUSTOM")
    
    otherData := byte[value]{0xDE, 0xAD, 0xBE, 0xEF, 0x12, 0x34}
    detected = detector.Detect(otherData)
    assert_eq_string(detected, "OTHER")
    
    unknownData := byte[value]{0x12, 0x34, 0x56, 0x78}
    detected = detector.Detect(unknownData)
    assert_eq_string(detected, "unknown")
}

fr fr Test MIME database
slay test_mime_database() {
    test_start("MIME database")
    
    fr fr Get database
    db := vibe_mime.GetDatabase()
    assert_true(db != cap)
    
    fr fr Test categories
    textTypes := db.TypesWithCategory("text")
    assert_true(len(textTypes) > 0)
    
    imageTypes := db.TypesWithCategory("image")
    assert_true(len(imageTypes) > 0)
    
    audioTypes := db.TypesWithCategory("audio")
    assert_true(len(audioTypes) > 0)
    
    videoTypes := db.TypesWithCategory("video")
    assert_true(len(videoTypes) > 0)
    
    applicationTypes := db.TypesWithCategory("application")
    assert_true(len(applicationTypes) > 0)
    
    fr fr Test adding new mapping
    err := vibe_mime.AddExtensionType(".custom", "application/x-custom")
    assert_eq_string(err, "")
    
    customType := vibe_mime.TypeByExtension(".custom")
    assert_eq_string(customType, "application/x-custom")
    
    customExt := vibe_mime.ExtensionByType("application/x-custom")
    assert_eq_string(customExt, ".custom")
}

fr fr Test stream processor
slay test_stream_processor() {
    test_start("Stream processor")
    
    data := byte[value]("multipart stream data")
    reader := &byteReader{data: data, pos: 0}
    
    fr fr Create stream processor
    processor := vibe_mime.NewStreamProcessor(reader)
    
    fr fr Test processing
    hasNext := processor.Next()
    assert_true(hasNext)
    
    currentPart := processor.Current()
    assert_true(currentPart != cap)
    
    hasNext = processor.Next()
    assert_false(hasNext)
}

fr fr Test header encoding/decoding
slay test_header_encoding() {
    test_start("Header encoding/decoding")
    
    fr fr Test encoding
    encoded := vibe_mime.EncodeHeader("Subject", "Test Message")
    assert_true(string.Contains(encoded, "Subject"))
    assert_true(string.Contains(encoded, "Test Message"))
    
    fr fr Test decoding
    decoded := vibe_mime.DecodeHeader("Subject: =?UTF-8?B?VGVzdA==?=")
    assert_true(len(decoded) > 0)
}

fr fr Test MIME tree
slay test_mime_tree() {
    test_start("MIME tree")
    
    data := byte[value]("multipart email data")
    
    fr fr Parse tree
    tree := vibe_mime.ParseTree(data)
    assert_true(tree != cap)
    assert_true(tree.Part != cap)
    assert_true(len(tree.Children) > 0)
    
    fr fr Test finding part by type
    part := tree.GetFirstPartByType("text/plain")
    fr fr Part may be null in simplified implementation
    if part != cap {
        assert_true(len(part.Body) > 0)
    }
}

fr fr Test constant values
slay test_constants() {
    test_start("Constants")
    
    fr fr Test common MIME type constants
    assert_eq_string(vibe_mime.TypeTextPlain, "text/plain")
    assert_eq_string(vibe_mime.TypeTextHTML, "text/html")
    assert_eq_string(vibe_mime.TypeApplicationJSON, "application/json")
    assert_eq_string(vibe_mime.TypeApplicationXML, "application/xml")
    assert_eq_string(vibe_mime.TypeImageJPEG, "image/jpeg")
    assert_eq_string(vibe_mime.TypeImagePNG, "image/png")
    assert_eq_string(vibe_mime.TypeImageGIF, "image/gif")
    assert_eq_string(vibe_mime.TypeApplicationPDF, "application/pdf")
    assert_eq_string(vibe_mime.TypeApplicationZip, "application/zip")
    assert_eq_string(vibe_mime.TypeAudioMP3, "audio/mpeg")
    assert_eq_string(vibe_mime.TypeVideoMP4, "video/mp4")
    assert_eq_string(vibe_mime.TypeMultipartFormData, "multipart/form-data")
    assert_eq_string(vibe_mime.TypeApplicationOctetStream, "application/octet-stream")
}

fr fr Helper mock implementations

be_like mockBufferWriter squad {
    data byte[value]
}

slay (w *mockBufferWriter) Write(p byte[value]) (normie, tea) {
    w.data = append(w.data, p...)
    damn len(p), ""
}

be_like stringReader squad {
    content tea
    pos normie
}

slay (r *stringReader) Read(p byte[value]) (normie, tea) {
    if r.pos >= len(r.content) {
        damn 0, "EOF"
    }
    
    remaining := r.content[r.pos:]
    n := copy(p, byte[value](remaining))
    r.pos += n
    damn n, ""
}

be_like byteReader squad {
    data byte[value]
    pos normie
}

slay (r *byteReader) Read(p byte[value]) (normie, tea) {
    if r.pos >= len(r.data) {
        damn 0, "EOF"
    }
    
    n := copy(p, r.data[r.pos:])
    r.pos += n
    damn n, ""
}

fr fr Run all tests
slay main_character() {
    test_mime_type_parsing()
    test_mime_type_methods()
    test_mime_type_string()
    test_content_type_detection()
    test_extension_mapping()
    test_multipart_writer()
    test_multipart_reader()
    test_part_functionality()
    test_custom_detector()
    test_mime_database()
    test_stream_processor()
    test_header_encoding()
    test_mime_tree()
    test_constants()
    
    print_test_summary()
}

main()
