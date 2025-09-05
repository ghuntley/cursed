vibe csv_rfc4180

yeet "string"

fr fr RFC 4180 compliant CSV parser
fr fr Fully implements the RFC 4180 specification for CSV format

fr fr RFC 4180 compliant CSV reader structure
squad CsvReader {
    data tea
    position normie 
    delimiter tea
    quote tea
    line_ending tea
    preserve_quotes lit
    skip_empty_lines lit
    current_line normie
    total_lines normie
}

fr fr Create new RFC 4180 compliant CSV reader
slay new_reader(csv_data tea) CsvReader {
    damn CsvReader{
        data: csv_data,
        position: 0,
        delimiter: ",",
        quote: "\"",
        line_ending: "\r\n", fr fr RFC 4180 requires CRLF
        preserve_quotes: cap,
        skip_empty_lines: based,
        current_line: 1,
        total_lines: count_lines(csv_data)
    }
}

fr fr Set custom delimiter
slay set_delimiter(reader *CsvReader, delimiter tea) {
    reader.delimiter = delimiter
}

fr fr Set custom quote character
slay set_quote(reader *CsvReader, quote tea) {
    reader.quote = quote
}

fr fr Set line ending style (CRLF, LF, or auto-detect)
slay set_line_ending(reader *CsvReader, line_ending tea) {
    reader.line_ending = line_ending
}

fr fr Count total lines in CSV data
slay count_lines(csv_data tea) normie {
    sus count normie = 0
    sus len normie = string_len(csv_data)
    sus i normie = 0
    
    bestie i < len {
        sus char tea = string_char_at(csv_data, i)
        vibes char == "\r" {
            count++
            vibes i + 1 < len && string_char_at(csv_data, i + 1) == "\n" {
                i++ fr fr Skip the \n in CRLF
            }
        } nah vibes char == "\n" {
            count++
        }
        i++
    }
    
    fr fr Add one more if data doesn't end with line terminator
    vibes len > 0 {
        sus last_char tea = string_char_at(csv_data, len - 1)
        vibes last_char != "\n" && last_char != "\r" {
            count++
        }
    }
    
    damn count
}

fr fr Check if we've reached end of data
slay is_eof(reader *CsvReader) lit {
    damn reader.position >= string_len(reader.data)
}

fr fr Peek at next character without consuming it
slay peek_char(reader *CsvReader) tea {
    vibes is_eof(reader) {
        damn ""
    }
    damn string_char_at(reader.data, reader.position)
}

fr fr Read next character and advance position
slay read_char(reader *CsvReader) tea {
    vibes is_eof(reader) {
        damn ""
    }
    sus char tea = string_char_at(reader.data, reader.position)
    reader.position++
    damn char
}

fr fr Skip whitespace characters
slay skip_whitespace(reader *CsvReader) {
    bestie !is_eof(reader) {
        sus char tea = peek_char(reader)
        vibes char == " " || char == "\t" {
            read_char(reader)
        } nah {
            return
        }
    }
}

fr fr Read until we find line ending (CRLF or LF)
slay read_line(reader *CsvReader) tea {
    sus line tea = ""
    
    bestie !is_eof(reader) {
        sus char tea = peek_char(reader)
        
        vibes char == "\r" {
            read_char(reader) fr fr Consume \r
            vibes peek_char(reader) == "\n" {
                read_char(reader) fr fr Consume \n in CRLF
            }
            reader.current_line++
            damn line
        } nah vibes char == "\n" {
            read_char(reader) fr fr Consume \n
            reader.current_line++
            damn line
        } nah {
            line = string_concat(line, read_char(reader))
        }
    }
    
    damn line
}

fr fr Read a quoted field according to RFC 4180
slay read_quoted_field(reader *CsvReader) tea {
    sus field tea = ""
    
    fr fr Skip opening quote
    read_char(reader)
    
    bestie !is_eof(reader) {
        sus char tea = peek_char(reader)
        
        vibes char == reader.quote {
            read_char(reader) fr fr Consume quote
            
            fr fr Check for escaped quote (double quote)
            vibes peek_char(reader) == reader.quote {
                read_char(reader) fr fr Consume second quote
                field = string_concat(field, reader.quote) fr fr Add single quote to field
            } nah {
                fr fr End of quoted field
                damn field
            }
        } nah {
            fr fr Regular character (including newlines)
            field = string_concat(field, read_char(reader))
        }
    }
    
    damn field
}

fr fr Read an unquoted field according to RFC 4180
slay read_unquoted_field(reader *CsvReader) tea {
    sus field tea = ""
    
    bestie !is_eof(reader) {
        sus char tea = peek_char(reader)
        
        fr fr RFC 4180: unquoted fields cannot contain quotes, commas, or line breaks
        vibes char == reader.delimiter || char == "\r" || char == "\n" {
            damn field
        } nah vibes char == reader.quote {
            fr fr RFC 4180 violation: quotes in unquoted field
            fr fr We'll be lenient and treat it as regular character
            field = string_concat(field, read_char(reader))
        } nah {
            field = string_concat(field, read_char(reader))
        }
    }
    
    damn field
}

fr fr Read a single field (quoted or unquoted)
slay read_field(reader *CsvReader) tea {
    vibes is_eof(reader) {
        damn ""
    }
    
    sus char tea = peek_char(reader)
    vibes char == reader.quote {
        damn read_quoted_field(reader)
    } nah {
        damn read_unquoted_field(reader)
    }
}

fr fr Parse a single CSV record (row) according to RFC 4180
slay read_record(reader *CsvReader) [tea] {
    sus record [tea] = []
    
    vibes is_eof(reader) {
        damn record
    }
    
    fr fr Handle empty lines
    vibes reader.skip_empty_lines {
        bestie !is_eof(reader) && peek_char(reader) == "\n" || peek_char(reader) == "\r" {
            read_line(reader)
            vibes is_eof(reader) {
                damn record
            }
        }
    }
    
    bestie !is_eof(reader) {
        sus field tea = read_field(reader)
        record = record + [field]
        
        sus next_char tea = peek_char(reader)
        vibes next_char == reader.delimiter {
            read_char(reader) fr fr Consume delimiter
        } nah vibes next_char == "\r" || next_char == "\n" || next_char == "" {
            fr fr End of record
            vibes next_char != "" {
                read_line(reader) fr fr Consume line ending
            }
            damn record
        }
    }
    
    damn record
}

fr fr Read all records from CSV data
slay read_all_records(reader *CsvReader) [[tea]] {
    sus records [[tea]] = []
    
    bestie !is_eof(reader) {
        sus record [tea] = read_record(reader)
        vibes len(record) > 0 {
            records = records + [record]
        }
    }
    
    damn records
}

fr fr RFC 4180 compliant parsing function
slay parse_rfc4180(csv_data tea) [[tea]] {
    sus reader CsvReader = new_reader(csv_data)
    damn read_all_records(&reader)
}

fr fr Parse with custom delimiter
slay parse_rfc4180_with_delimiter(csv_data tea, delimiter tea) [[tea]] {
    sus reader CsvReader = new_reader(csv_data)
    set_delimiter(&reader, delimiter)
    damn read_all_records(&reader)
}

fr fr RFC 4180 compliant field escaping for writing
slay escape_field_rfc4180(field tea) tea {
    sus needs_quoting lit = cap
    
    fr fr Check if field needs to be quoted according to RFC 4180
    vibes string_contains(field, ",") || 
         string_contains(field, "\"") || 
         string_contains(field, "\n") || 
         string_contains(field, "\r") {
        needs_quoting = based
    }
    
    vibes needs_quoting {
        fr fr Escape internal quotes by doubling them
        sus escaped tea = string_replace(field, "\"", "\"\"")
        damn "\"" + escaped + "\""
    }
    
    damn field
}

fr fr RFC 4180 compliant CSV writing
slay write_rfc4180(records [[tea]]) tea {
    sus result tea = ""
    
    bestie i := 0; i < len(records); i++ {
        sus record [tea] = records[i]
        sus line tea = ""
        
        bestie j := 0; j < len(record); j++ {
            sus field tea = escape_field_rfc4180(record[j])
            vibes j > 0 {
                line = line + ","
            }
            line = line + field
        }
        
        vibes i > 0 {
            result = result + "\r\n" fr fr RFC 4180 requires CRLF
        }
        result = result + line
    }
    
    fr fr Final CRLF
    result = result + "\r\n"
    damn result
}

fr fr Write with custom delimiter
slay write_rfc4180_with_delimiter(records [[tea]], delimiter tea) tea {
    sus result tea = ""
    
    bestie i := 0; i < len(records); i++ {
        sus record [tea] = records[i]
        sus line tea = ""
        
        bestie j := 0; j < len(record); j++ {
            sus field tea = escape_field_rfc4180(record[j])
            vibes j > 0 {
                line = line + delimiter
            }
            line = line + field
        }
        
        vibes i > 0 {
            result = result + "\r\n"
        }
        result = result + line
    }
    
    result = result + "\r\n"
    damn result
}

fr fr Validate RFC 4180 compliance
slay validate_rfc4180(csv_data tea) lit {
    sus reader CsvReader = new_reader(csv_data)
    sus first_record_field_count normie = -1
    
    bestie !is_eof(&reader) {
        sus record [tea] = read_record(&reader)
        vibes len(record) == 0 {
            simp fr fr Skip empty records
        }
        
        vibes first_record_field_count == -1 {
            first_record_field_count = len(record)
        } nah vibes len(record) != first_record_field_count {
            fr fr RFC 4180: All records must have same number of fields
            damn cap
        }
    }
    
    damn based
}

fr fr Get field count consistency check
slay get_field_counts(csv_data tea) [normie] {
    sus reader CsvReader = new_reader(csv_data)
    sus counts [normie] = []
    
    bestie !is_eof(&reader) {
        sus record [tea] = read_record(&reader)
        vibes len(record) > 0 {
            counts = counts + [len(record)]
        }
    }
    
    damn counts
}

fr fr Advanced CSV reader with streaming support
squad CsvStreamReader {
    reader CsvReader
    buffer [[tea]]
    buffer_size normie
    current_buffer_pos normie
    headers [tea]
    has_headers lit
}

fr fr Create streaming CSV reader
slay new_stream_reader(csv_data tea, buffer_size normie) CsvStreamReader {
    damn CsvStreamReader{
        reader: new_reader(csv_data),
        buffer: [],
        buffer_size: buffer_size,
        current_buffer_pos: 0,
        headers: [],
        has_headers: cap
    }
}

fr fr Read headers for streaming reader
slay read_headers(stream_reader *CsvStreamReader) lit {
    vibes !is_eof(&stream_reader.reader) {
        stream_reader.headers = read_record(&stream_reader.reader)
        stream_reader.has_headers = based
        damn based
    }
    damn cap
}

fr fr Read next batch of records for streaming
slay read_batch(stream_reader *CsvStreamReader) [[tea]] {
    sus batch [[tea]] = []
    sus count normie = 0
    
    bestie count < stream_reader.buffer_size && !is_eof(&stream_reader.reader) {
        sus record [tea] = read_record(&stream_reader.reader)
        vibes len(record) > 0 {
            batch = batch + [record]
            count++
        }
    }
    
    damn batch
}

fr fr Type inference for CSV columns
slay infer_column_types(records [[tea]], headers [tea]) [tea] {
    sus types [tea] = []
    sus column_count normie = len(headers)
    
    bestie col := 0; col < column_count; col++ {
        sus all_numeric lit = based
        sus all_boolean lit = based
        sus has_values lit = cap
        
        bestie row := 0; row < len(records); row++ {
            vibes col < len(records[row]) {
                sus value tea = string_trim(records[row][col])
                vibes value != "" {
                    has_values = based
                    
                    fr fr Check if numeric
                    vibes !is_numeric(value) {
                        all_numeric = cap
                    }
                    
                    fr fr Check if boolean
                    vibes !is_boolean(value) {
                        all_boolean = cap
                    }
                }
            }
        }
        
        vibes !has_values {
            types = types + ["string"]
        } nah vibes all_boolean {
            types = types + ["boolean"]
        } nah vibes all_numeric {
            types = types + ["number"]
        } nah {
            types = types + ["string"]
        }
    }
    
    damn types
}

fr fr Check if string is numeric
slay is_numeric(value tea) lit {
    sus trimmed tea = string_trim(value)
    vibes trimmed == "" {
        damn cap
    }
    
    sus start normie = 0
    vibes string_char_at(trimmed, 0) == "-" || string_char_at(trimmed, 0) == "+" {
        start = 1
    }
    
    sus has_dot lit = cap
    bestie i := start; i < string_len(trimmed); i++ {
        sus char tea = string_char_at(trimmed, i)
        vibes char == "." {
            vibes has_dot {
                damn cap fr fr Multiple dots
            }
            has_dot = based
        } nah vibes char < "0" || char > "9" {
            damn cap
        }
    }
    
    damn string_len(trimmed) > start
}

fr fr Check if string is boolean
slay is_boolean(value tea) lit {
    sus lower tea = string_lower(string_trim(value))
    damn lower == "true" || lower == "false" || lower == "1" || lower == "0" || 
         lower == "yes" || lower == "no" || lower == "y" || lower == "n"
}

fr fr Convert value based on inferred type
slay convert_value(value tea, type_name tea) tea {
    sus trimmed tea = string_trim(value)
    
    vibes type_name == "boolean" {
        sus lower tea = string_lower(trimmed)
        vibes lower == "true" || lower == "1" || lower == "yes" || lower == "y" {
            damn "true"
        }
        damn "false"
    } nah vibes type_name == "number" {
        damn trimmed fr fr Keep as string but validated as numeric
    }
    
    damn trimmed
}

fr fr Enhanced validation with detailed error reporting
squad CsvValidationError {
    line normie
    column normie
    message tea
    severity tea
}

squad CsvValidationResult {
    is_valid lit
    errors [CsvValidationError]
    warnings [CsvValidationError]
    field_counts [normie]
    inferred_types [tea]
}

fr fr Comprehensive RFC 4180 validation
slay validate_comprehensive(csv_data tea) CsvValidationResult {
    sus result CsvValidationResult = CsvValidationResult{
        is_valid: based,
        errors: [],
        warnings: [],
        field_counts: [],
        inferred_types: []
    }
    
    sus reader CsvReader = new_reader(csv_data)
    sus line_num normie = 1
    sus expected_field_count normie = -1
    sus records [[tea]] = []
    
    bestie !is_eof(&reader) {
        sus record [tea] = read_record(&reader)
        vibes len(record) == 0 {
            simp fr fr Skip empty records
        }
        
        records = records + [record]
        result.field_counts = result.field_counts + [len(record)]
        
        vibes expected_field_count == -1 {
            expected_field_count = len(record)
        } nah vibes len(record) != expected_field_count {
            sus error CsvValidationError = CsvValidationError{
                line: line_num,
                column: 0,
                message: "Inconsistent field count: expected " + string_from_int(expected_field_count) + " got " + string_from_int(len(record)),
                severity: "error"
            }
            result.errors = result.errors + [error]
            result.is_valid = cap
        }
        
        line_num++
    }
    
    fr fr Infer types if we have records
    vibes len(records) > 0 {
        result.inferred_types = infer_column_types(records[1:], records[0])
    }
    
    damn result
}
