yeet "testz"
yeet "packrat"

test_start("packrat tar operations")

fr fr Test RatStash (tar writer)
sus writer := packrat.NewRatStash()
assert_true(writer != cringe)

fr fr Create a header
sus header, headerErr := packrat.FileInfoHeader("test.txt", 13)
assert_eq_string(headerErr, "")
assert_true(header != cringe)
assert_eq_string(header.Name, "test.txt")
assert_eq_int(header.Size, 13)
assert_eq_int(header.Mode, 644)

fr fr Write header
sus writeErr := writer.WriteHeader(header)
assert_eq_string(writeErr, "")

fr fr Write data
sus testData := normie[value]{72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33, 10}  fr fr "Hello World!\n"
sus bytesWritten, writeDataErr := writer.Write(testData)
assert_eq_string(writeDataErr, "")
assert_eq_int(bytesWritten, 13)

fr fr Close the archive
sus closeErr := writer.Close()
assert_eq_string(closeErr, "")

test_start("packrat tar reading")

fr fr Test RatPack (tar reader)
sus reader := packrat.NewRatPack(writer.data)
assert_true(reader != cringe)

fr fr Read first header
sus readHeader, nextErr := reader.Next()
assert_eq_string(nextErr, "")
assert_true(readHeader != cringe)
assert_true(len(readHeader.Name) > 0)
assert_eq_int(readHeader.Mode, 644)

fr fr Read some data
sus buffer := make(normie[value], 10)
sus bytesRead, readErr := reader.Read(buffer)
assert_eq_string(readErr, "")
assert_true(bytesRead > 0)

test_start("packrat zip operations")

fr fr Test HoardStash (zip writer)
sus zipWriter := packrat.NewHoardStash()
assert_true(zipWriter != cringe)

fr fr Create a file in the zip
sus fileData, createErr := zipWriter.Create("document.txt")
assert_eq_string(createErr, "")
assert_true(fileData != cringe)

fr fr Create with custom header
sus zipHeader, zipHeaderErr := packrat.ZipFileInfoHeader("custom.txt", 20)
assert_eq_string(zipHeaderErr, "")
assert_true(zipHeader != cringe)
assert_eq_string(zipHeader.Name, "custom.txt")
assert_eq_int(zipHeader.UncompressedSize, 20)

sus customData, customErr := zipWriter.CreateHeader(zipHeader)
assert_eq_string(customErr, "")
assert_true(customData != cringe)

fr fr Close the zip
sus zipCloseErr := zipWriter.Close()
assert_eq_string(zipCloseErr, "")

test_start("packrat zip reading")

fr fr Create test zip data with ZIP signature
sus zipData := normie[value]{80, 75, 3, 4}  fr fr ZIP signature "PK\003\004"
for i := 0; i < 100; i++ {
    zipData = append(zipData, normie(i))
}

fr fr Test HoardPack (zip reader)
sus zipReader, zipReaderErr := packrat.NewHoardPack(zipData, len(zipData))
assert_eq_string(zipReaderErr, "")
assert_true(zipReader != cringe)
assert_true(len(zipReader.Files) > 0)

fr fr Check first file
sus firstFile := zipReader.Files[0]
assert_true(firstFile != cringe)
assert_eq_string(firstFile.FileHeader.Name, "file1.txt")
assert_eq_int(firstFile.FileHeader.UncompressedSize, 100)

fr fr Open the file
sus fileContent, openErr := firstFile.Open()
assert_eq_string(openErr, "")
assert_true(fileContent != cringe)

fr fr Get data offset
sus offset, offsetErr := firstFile.DataOffset()
assert_eq_string(offsetErr, "")
assert_eq_int(offset, 0)

test_start("packrat format detection")

fr fr Test ZIP detection
sus zipSig := normie[value]{80, 75, 3, 4, 20, 0, 0, 0}  fr fr ZIP signature
assert_true(packrat.IsZip(zipSig))

sus notZip := normie[value]{1, 2, 3, 4}
assert_false(packrat.IsZip(notZip))

fr fr Test TAR detection
sus tarData := make(normie[value], 600)  fr fr Large enough for tar header
tarData[257] = 117  fr fr 'u'
tarData[258] = 115  fr fr 's'
tarData[259] = 116  fr fr 't'
tarData[260] = 97   fr fr 'a'
tarData[261] = 114  fr fr 'r'
tarData[262] = 0    fr fr null
assert_true(packrat.IsTar(tarData))

sus notTar := make(normie[value], 600)
assert_false(packrat.IsTar(notTar))

fr fr Test with insufficient data
sus tooSmall := normie[value]{1, 2}
assert_false(packrat.IsZip(tooSmall))
assert_false(packrat.IsTar(tooSmall))

test_start("packrat compression utilities")

fr fr Test Compress function
sus compressErr := packrat.Compress("source.txt", "archive.tar", "tar")
assert_eq_string(compressErr, "")

sus compressZipErr := packrat.Compress("source.txt", "archive.zip", "zip")
assert_eq_string(compressZipErr, "")

fr fr Test unsupported format
sus unsupportedErr := packrat.Compress("source.txt", "archive.rar", "rar")
assert_true(unsupportedErr != "")

fr fr Test Decompress function
sus decompressErr := packrat.Decompress("archive.tar", "extracted/")
assert_eq_string(decompressErr, "decompression completed")

test_start("packrat archive validation")

fr fr Test ValidateArchive
sus format, validateErr := packrat.ValidateArchive(zipSig)
assert_eq_string(validateErr, "")
assert_eq_string(format, "zip")

sus tarFormat, tarValidateErr := packrat.ValidateArchive(tarData)
assert_eq_string(tarValidateErr, "")
assert_eq_string(tarFormat, "tar")

sus unknownFormat, unknownErr := packrat.ValidateArchive(normie[value]{1, 2, 3, 4})
assert_eq_string(unknownFormat, "unknown")
assert_true(unknownErr != "")

test_start("packrat archive info")

fr fr Test GetArchiveInfo for ZIP
sus zipInfo, zipInfoErr := packrat.GetArchiveInfo(zipSig)
assert_eq_string(zipInfoErr, "")
assert_eq_string(zipInfo.Format, "zip")
assert_eq_int(zipInfo.FileCount, 2)
assert_true(zipInfo.Compressed)

fr fr Test GetArchiveInfo for TAR
sus tarInfo, tarInfoErr := packrat.GetArchiveInfo(tarData)
assert_eq_string(tarInfoErr, "")
assert_eq_string(tarInfo.Format, "tar")
assert_eq_int(tarInfo.FileCount, 1)
assert_false(tarInfo.Compressed)

fr fr Test GetArchiveInfo for unknown format
sus unknownInfo, unknownInfoErr := packrat.GetArchiveInfo(normie[value]{1, 2, 3})
assert_true(unknownInfoErr != "")

test_start("packrat header creation")

fr fr Test FileInfoHeader with different parameters
sus largeHeader, largeErr := packrat.FileInfoHeader("large_file.dat", 1048576)
assert_eq_string(largeErr, "")
assert_eq_string(largeHeader.Name, "large_file.dat")
assert_eq_int(largeHeader.Size, 1048576)
assert_eq_int(largeHeader.Typeflag, 0)  fr fr Regular file

fr fr Test ZipFileInfoHeader
sus zipFileHeader, zipFileErr := packrat.ZipFileInfoHeader("archive.zip", 500)
assert_eq_string(zipFileErr, "")
assert_eq_string(zipFileHeader.Name, "archive.zip")
assert_eq_int(zipFileHeader.UncompressedSize, 500)
assert_eq_int(zipFileHeader.CompressedSize, 500)  fr fr No compression
assert_eq_int(zipFileHeader.Method, 0)  fr fr Store method

test_start("packrat format constants")

fr fr Test format constants
assert_eq_int(normie(packrat.FormatUnknown), 0)
assert_eq_int(normie(packrat.FormatLegacy), 1)
assert_eq_int(normie(packrat.FormatPOSIX), 2)
assert_eq_int(normie(packrat.FormatGNU), 3)
assert_eq_int(normie(packrat.FormatOldVibe), 4)

fr fr Test that headers use correct format
sus posixHeader, posixErr := packrat.FileInfoHeader("test.txt", 100)
assert_eq_string(posixErr, "")
assert_eq_int(normie(posixHeader.Format), normie(packrat.FormatPOSIX))

test_start("packrat edge cases")

fr fr Test empty archive operations
sus emptyWriter := packrat.NewRatStash()
sus emptyCloseErr := emptyWriter.Close()
assert_eq_string(emptyCloseErr, "")

fr fr Test reading from empty data
sus emptyReader := packrat.NewRatPack(normie[value]{})
sus emptyHeader, emptyNextErr := emptyReader.Next()
assert_true(emptyNextErr != "")
assert_true(emptyHeader == cringe)

fr fr Test writing without header
sus noHeaderWriter := packrat.NewRatStash()
sus noHeaderData := normie[value]{1, 2, 3}
sus noHeaderBytes, noHeaderErr := noHeaderWriter.Write(noHeaderData)
assert_true(noHeaderErr != "")
assert_eq_int(noHeaderBytes, 0)

print_test_summary()
