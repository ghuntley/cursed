yeet "testz"
yeet "mime_vibe"

test_start("mime_vibe basic MIME type creation")

fr fr Test basic MIME type creation
sus mimeType := mime_vibe.NewVibeType("text", "plain", cringe)
assert_eq_string(mimeType.Type, "text")
assert_eq_string(mimeType.Subtype, "plain")
assert_eq_string(mimeType.FullType(), "text/plain")

fr fr Test with parameters
sus params := make(map[tea]tea)
params["charset"] = "utf-8"
sus htmlType := mime_vibe.NewVibeType("text", "html", params)
assert_eq_string(htmlType.GetParameter("charset"), "utf-8")

test_start("mime_vibe MIME type parsing")

fr fr Test MIME type parsing
sus parsed, err := mime_vibe.ParseVibeType("text/plain")
assert_eq_string(err, "")
assert_eq_string(parsed.Type, "text")
assert_eq_string(parsed.Subtype, "plain")

sus parsed2, err2 := mime_vibe.ParseVibeType("application/json")
assert_eq_string(err2, "")
assert_eq_string(parsed2.Type, "application")
assert_eq_string(parsed2.Subtype, "json")

fr fr Test invalid MIME type
sus invalid, errInvalid := mime_vibe.ParseVibeType("invalid")
assert_true(errInvalid != "")

test_start("mime_vibe MIME type constants")

fr fr Test predefined constants
assert_eq_string(mime_vibe.TypeTextPlain.Type, "text")
assert_eq_string(mime_vibe.TypeTextPlain.Subtype, "plain")
assert_eq_string(mime_vibe.TypeTextHTML.Subtype, "html")
assert_eq_string(mime_vibe.TypeApplicationJSON.Type, "application")
assert_eq_string(mime_vibe.TypeApplicationJSON.Subtype, "json")

fr fr Test image types
assert_eq_string(mime_vibe.TypeImagePNG.Type, "image")
assert_eq_string(mime_vibe.TypeImagePNG.Subtype, "png")
assert_eq_string(mime_vibe.TypeImageJPEG.Subtype, "jpeg")

fr fr Test audio/video types
assert_eq_string(mime_vibe.TypeAudioMP3.Type, "audio")
assert_eq_string(mime_vibe.TypeVideoMP4.Type, "video")

test_start("mime_vibe type methods")

fr fr Test type checking methods
assert_true(mime_vibe.TypeTextPlain.IsText())
assert_false(mime_vibe.TypeTextPlain.IsImage())
assert_false(mime_vibe.TypeTextPlain.IsAudio())
assert_false(mime_vibe.TypeTextPlain.IsVideo())
assert_false(mime_vibe.TypeTextPlain.IsApplication())
assert_false(mime_vibe.TypeTextPlain.IsMultipart())

assert_true(mime_vibe.TypeImagePNG.IsImage())
assert_false(mime_vibe.TypeImagePNG.IsText())

assert_true(mime_vibe.TypeApplicationJSON.IsApplication())
assert_true(mime_vibe.TypeMultipartMixed.IsMultipart())

test_start("mime_vibe parameter methods")

fr fr Test parameter methods
sus mimeWithCharset := mime_vibe.TypeTextPlain.WithCharset("iso-8859-1")
assert_eq_string(mimeWithCharset.GetParameter("charset"), "iso-8859-1")

sus mimeWithCustom := mime_vibe.TypeTextHTML.WithParameter("custom", "value")
assert_eq_string(mimeWithCustom.GetParameter("custom"), "value")

fr fr Test getting non-existent parameter
sus empty := mime_vibe.TypeTextPlain.GetParameter("nonexistent")
assert_eq_string(empty, "")

test_start("mime_vibe pattern matching")

fr fr Test pattern matching
assert_true(mime_vibe.TypeTextPlain.Match("*/*"))
assert_true(mime_vibe.TypeTextPlain.Match("text/*"))
assert_true(mime_vibe.TypeTextPlain.Match("text/plain"))
assert_false(mime_vibe.TypeTextPlain.Match("image/*"))
assert_false(mime_vibe.TypeTextPlain.Match("text/html"))

test_start("mime_vibe string representation")

fr fr Test string representation
sus str := mime_vibe.TypeTextPlain.String()
assert_true(len(str) > 0)

sus htmlStr := mime_vibe.TypeTextHTML.String()
assert_true(len(htmlStr) > 0)

fr fr Test with parameters
sus withParams := mime_vibe.TypeApplicationJSON.WithParameter("version", "1.0")
sus paramStr := withParams.String()
assert_true(len(paramStr) > len(mime_vibe.TypeApplicationJSON.String()))

test_start("mime_vibe extension detection")

fr fr Test extension-based detection
sus txtType := mime_vibe.TypeByExtension(".txt")
assert_eq_string(txtType.Type, "text")
assert_eq_string(txtType.Subtype, "plain")

sus htmlType2 := mime_vibe.TypeByExtension(".html")
assert_eq_string(htmlType2.Type, "text")
assert_eq_string(htmlType2.Subtype, "html")

sus pngType := mime_vibe.TypeByExtension(".png")
assert_eq_string(pngType.Type, "image")
assert_eq_string(pngType.Subtype, "png")

sus unknownType := mime_vibe.TypeByExtension(".unknown")
assert_eq_string(unknownType.Type, "application")
assert_eq_string(unknownType.Subtype, "octet-stream")

test_start("mime_vibe filename detection")

fr fr Test filename-based detection
sus fileType1 := mime_vibe.TypeByFilename("document.txt")
assert_eq_string(fileType1.Type, "text")

sus fileType2 := mime_vibe.TypeByFilename("image.png")
assert_eq_string(fileType2.Type, "image")

sus fileType3 := mime_vibe.TypeByFilename("data.json")
assert_eq_string(fileType3.Type, "application")
assert_eq_string(fileType3.Subtype, "json")

test_start("mime_vibe content detection")

fr fr Test content-based detection with magic bytes
fr fr PNG signature: 137, 80, 78, 71
sus pngData := normie[value]{137, 80, 78, 71, 13, 10, 26, 10}
sus detectedPNG := mime_vibe.TypeByContent(pngData)
assert_eq_string(detectedPNG.Type, "image")
assert_eq_string(detectedPNG.Subtype, "png")

fr fr JPEG signature: 255, 216, 255
sus jpegData := normie[value]{255, 216, 255, 224}
sus detectedJPEG := mime_vibe.TypeByContent(jpegData)
assert_eq_string(detectedJPEG.Type, "image")
assert_eq_string(detectedJPEG.Subtype, "jpeg")

fr fr Text content
sus textData := normie[value]{72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100}  fr fr "Hello World"
sus detectedText := mime_vibe.TypeByContent(textData)
assert_eq_string(detectedText.Type, "text")

fr fr Empty data
sus emptyData := normie[value]{}
sus detectedEmpty := mime_vibe.TypeByContent(emptyData)
assert_eq_string(detectedEmpty.Type, "application")

test_start("mime_vibe enhanced detection")

fr fr Test combined detection
sus combinedType1 := mime_vibe.DetectVibeType("test.png", normie[value]{137, 80, 78, 71})
assert_eq_string(combinedType1.Type, "image")
assert_eq_string(combinedType1.Subtype, "png")

fr fr Fallback to filename when content is unknown
sus unknownContent := normie[value]{1, 2, 3, 4}
sus combinedType2 := mime_vibe.DetectVibeType("document.pdf", unknownContent)
assert_eq_string(combinedType2.Type, "application")
assert_eq_string(combinedType2.Subtype, "pdf")

test_start("mime_vibe content encoding")

fr fr Test content encoding
sus originalData := normie[value]{72, 101, 108, 108, 111}  fr fr "Hello"
sus encoded, encErr := mime_vibe.EncodeContent(originalData, mime_vibe.EncodingBase64)
assert_eq_string(encErr, "")
assert_true(len(encoded) > 0)

fr fr Test content decoding
sus decoded, decErr := mime_vibe.DecodeContent(encoded, mime_vibe.EncodingBase64)
assert_eq_string(decErr, "")
assert_true(len(decoded) > 0)

fr fr Test binary encoding (pass-through)
sus binaryEncoded, binErr := mime_vibe.EncodeContent(originalData, mime_vibe.EncodingBinary)
assert_eq_string(binErr, "")
assert_eq_int(len(binaryEncoded), len(originalData))

test_start("mime_vibe message creation")

fr fr Test message creation
sus msg := mime_vibe.NewVibeMessage()
assert_true(msg != cringe)
assert_eq_string(msg.ContentType.Type, "multipart")
assert_true(len(msg.Boundary) > 0)

fr fr Test adding text part
sus textPart := msg.AddTextPart("Hello, World!", mime_vibe.TypeTextPlain)
assert_true(textPart != cringe)
assert_eq_string(textPart.ContentType.Type, "text")
assert_eq_int(textPart.Size, 13)

fr fr Test adding binary part
sus binaryData := normie[value]{1, 2, 3, 4, 5}
sus binaryPart := msg.AddBinaryPart(binaryData, mime_vibe.TypeApplicationOctetStream, "data.bin")
assert_true(binaryPart != cringe)
assert_eq_string(binaryPart.Filename, "data.bin")
assert_eq_int(binaryPart.Size, 5)

test_start("mime_vibe message string representation")

fr fr Test message string representation
sus msgStr := msg.String()
assert_true(len(msgStr) > 0)
assert_true(len(msgStr) > 100)  fr fr Should be a substantial message

test_start("mime_vibe GenZ features")

fr fr Test VibeCheck
sus vibeData := normie[value]{137, 80, 78, 71}
sus vibeResult := mime_vibe.VibeCheck(vibeData)
assert_eq_string(vibeResult.Type, "image")

fr fr Test NoCapDetect
sus noCapResult := mime_vibe.NoCapDetect("test.jpg", normie[value]{255, 216, 255})
assert_eq_string(noCapResult.Type, "image")

fr fr Test EmojiType
sus emojiText := mime_vibe.EmojiType(mime_vibe.TypeTextPlain)
assert_true(len(emojiText) > len(mime_vibe.TypeTextPlain.String()))

sus emojiImage := mime_vibe.EmojiType(mime_vibe.TypeImagePNG)
assert_true(len(emojiImage) > len(mime_vibe.TypeImagePNG.String()))

sus emojiAudio := mime_vibe.EmojiType(mime_vibe.TypeAudioMP3)
assert_true(len(emojiAudio) > len(mime_vibe.TypeAudioMP3.String()))

sus emojiVideo := mime_vibe.EmojiType(mime_vibe.TypeVideoMP4)
assert_true(len(emojiVideo) > len(mime_vibe.TypeVideoMP4.String()))

test_start("mime_vibe modern web types")

fr fr Test modern web MIME types
assert_eq_string(mime_vibe.TypeApplicationGraphQL.Type, "application")
assert_eq_string(mime_vibe.TypeApplicationGraphQL.Subtype, "graphql")

assert_eq_string(mime_vibe.TypeApplicationProtobuf.Subtype, "protobuf")
assert_eq_string(mime_vibe.TypeApplicationGRPC.Subtype, "grpc")
assert_eq_string(mime_vibe.TypeApplicationMsgpack.Subtype, "msgpack")
assert_eq_string(mime_vibe.TypeApplicationYAML.Subtype, "yaml")

print_test_summary()
