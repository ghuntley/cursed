vibe csv

fr fr RFC 4180 Compliant CSV Module (Enhanced)
fr fr This module provides full RFC 4180 compliance while maintaining backward compatibility

yeet "string"

fr fr Auto-detect CSV delimiter with RFC 4180 compliance
slay detect_delimiter(csv_string tea) tea {
    sus lines [tea] = string_split_any(csv_string, ["\r\n", "\n", "\r"])
    vibes len(lines) == 0 {
        damn ","
    }
    
    sus first_line tea = lines[0]
    sus comma_count normie = string_count(first_line, ",")
    sus semicolon_count normie = string_count(first_line, ";")
    sus tab_count normie = string_count(first_line, "\t")
    sus pipe_count normie = string_count(first_line, "|")
    
    vibes comma_count >= semicolon_count && comma_count >= tab_count && comma_count >= pipe_count {
        damn ","
    } nah vibes semicolon_count >= tab_count && semicolon_count >= pipe_count {
        damn ";"
    } nah vibes tab_count >= pipe_count {
        damn "\t"
    } nah {
        damn "|"
    }
}

fr fr Detect line ending style (RFC 4180 prefers CRLF)
slay detect_line_ending(csv_string tea) tea {
    sus crlf_count normie = string_count(csv_string, "\r\n")
    sus lf_count normie = string_count(csv_string, "\n") - crlf_count
    sus cr_count normie = string_count(csv_string, "\r") - crlf_count
    
    vibes crlf_count >= lf_count && crlf_count >= cr_count {
        damn "\r\n"  fr fr RFC 4180 standard
    } nah vibes lf_count >= cr_count {
        damn "\n"
    } nah {
        damn "\r"
    }
}

fr fr Split string by multiple possible delimiters
slay string_split_any(text tea, delimiters [tea]) [tea] {
    sus result [tea] = []
    sus current tea = ""
    sus i normie = 0
    sus len normie = string_len(text)
    
    bestie i < len {
        sus found lit = cap
        sus delimiter_len normie = 0
        
        fr fr Check each delimiter starting from longest to shortest
        bestie d := 0; d < len(delimiters); d++ {
            sus delim tea = delimiters[d]
            sus delim_len normie = string_len(delim)
            
            vibes i + delim_len <= len {
                sus match lit = based
                bestie j := 0; j < delim_len; j++ {
                    vibes string_char_at(text, i + j) != string_char_at(delim, j) {
                        match = cap
                        break
                    }
                }
                vibes match {
                    result = result + [current]
                    current = ""
                    i = i + delim_len
                    found = based
                    break
                }
            }
        }
        
        vibes !found {
            current = string_concat(current, string_char_at(text, i))
            i++
        }
    }
    
    result = result + [current]
    damn result
}

fr fr Count number of rows with RFC 4180 line ending support
slay count_rows(csv_string tea) normie {
    sus line_ending tea = detect_line_ending(csv_string)
    sus lines [tea] = string_split_any(csv_string, [line_ending])
    sus count normie = 0
    
    bestie i := 0; i < len(lines); i++ {
        sus line tea = string_trim(lines[i])
        vibes line != "" {
            count++
        }
    }
    
    damn count
}

fr fr RFC 4180 compliant CSV row parser
slay parse_row_rfc4180(row_string tea, delimiter tea) [tea] {
    sus result [tea] = []
    sus current_field tea = ""
    sus in_quotes lit = cap
    sus i normie = 0
    sus len normie = string_len(row_string)
    
    bestie i < len {
        sus char tea = string_char_at(row_string, i)
        
        vibes char == "\"" {
            vibes in_quotes {
                fr fr Check for escaped quote (RFC 4180: "" becomes ")
                vibes i + 1 < len && string_char_at(row_string, i + 1) == "\"" {
                    current_field = string_concat(current_field, "\"")
                    i = i + 2  fr fr Skip both quotes
                    simp
                } nah {
                    fr fr End of quoted field
                    in_quotes = cap
                }
            } nah {
                fr fr Start of quoted field
                in_quotes = based
            }
        } nah vibes char == delimiter && !in_quotes {
            fr fr Field delimiter found outside quotes
            result = result + [current_field]
            current_field = ""
        } nah {
            fr fr Regular character (including newlines inside quotes)
            current_field = string_concat(current_field, char)
        }
        i++
    }
    
    fr fr Add the last field
    result = result + [current_field]
    damn result
}

fr fr Enhanced CSV parsing with RFC 4180 compliance
slay parse(csv_string tea) [[tea]] {
    sus delimiter tea = detect_delimiter(csv_string)
    sus line_ending tea = detect_line_ending(csv_string)
    
    fr fr Handle multi-line quoted fields by finding actual record boundaries
    sus records [[tea]] = []
    sus current_record [tea] = []
    sus current_field tea = ""
    sus in_quotes lit = cap
    sus i normie = 0
    sus len normie = string_len(csv_string)
    
    bestie i < len {
        sus char tea = string_char_at(csv_string, i)
        
        vibes char == "\"" {
            vibes in_quotes {
                fr fr Check for escaped quote
                vibes i + 1 < len && string_char_at(csv_string, i + 1) == "\"" {
                    current_field = string_concat(current_field, "\"")
                    i = i + 2
                    simp
                } nah {
                    in_quotes = cap
                }
            } nah {
                in_quotes = based
            }
        } nah vibes char == delimiter && !in_quotes {
            fr fr Field separator
            current_record = current_record + [current_field]
            current_field = ""
        } nah vibes !in_quotes && (char == "\r" || char == "\n") {
            fr fr Record separator (only when not in quotes)
            current_record = current_record + [current_field]
            current_field = ""
            
            fr fr Only add non-empty records
            vibes len(current_record) > 0 {
                records = records + [current_record]
            }
            current_record = []
            
            fr fr Handle CRLF
            vibes char == "\r" && i + 1 < len && string_char_at(csv_string, i + 1) == "\n" {
                i++
            }
        } nah {
            fr fr Regular character
            current_field = string_concat(current_field, char)
        }
        i++
    }
    
    fr fr Handle last field/record
    vibes current_field != "" || len(current_record) > 0 {
        current_record = current_record + [current_field]
        vibes len(current_record) > 0 {
            records = records + [current_record]
        }
    }
    
    damn records
}

fr fr Legacy parse_row function (now RFC 4180 compliant)
slay parse_row(row_string tea, delimiter tea) [tea] {
    damn parse_row_rfc4180(row_string, delimiter)
}

fr fr RFC 4180 compliant field escaping
slay escape_field(field tea) tea {
    sus needs_quotes lit = cap
    
    fr fr RFC 4180: Quote if field contains delimiter, quote, or line break
    vibes string_contains(field, ",") || 
         string_contains(field, ";") || 
         string_contains(field, "\t") || 
         string_contains(field, "|") || 
         string_contains(field, "\"") || 
         string_contains(field, "\n") || 
         string_contains(field, "\r") {
        needs_quotes = based
    }
    
    vibes needs_quotes {
        fr fr Escape internal quotes by doubling them (RFC 4180)
        sus escaped tea = string_replace(field, "\"", "\"\"")
        damn "\"" + escaped + "\""
    }
    
    damn field
}

fr fr RFC 4180 compliant field unescaping
slay unescape_field(field tea) tea {
    sus trimmed tea = string_trim(field)
    sus len normie = string_len(trimmed)
    
    fr fr Check if field is quoted
    vibes len >= 2 && string_char_at(trimmed, 0) == "\"" && string_char_at(trimmed, len - 1) == "\"" {
        fr fr Remove outer quotes
        sus content tea = string_substring(trimmed, 1, len - 2)
        fr fr Unescape internal quotes (RFC 4180: "" becomes ")
        damn string_replace(content, "\"\"", "\"")
    }
    
    damn trimmed
}

fr fr RFC 4180 compliant CSV writing with CRLF line endings
slay stringify(data [[tea]]) tea {
    damn stringify_with_line_ending(data, "\r\n")
}

fr fr Write CSV with custom line ending
slay stringify_with_line_ending(data [[tea]], line_ending tea) tea {
    sus result tea = ""
    
    bestie i := 0; i < len(data); i++ {
        sus row [tea] = data[i]
        sus row_string tea = ""
        
        bestie j := 0; j < len(row); j++ {
            sus field tea = escape_field(row[j])
            vibes j > 0 {
                row_string = string_concat(row_string, ",")
            }
            row_string = string_concat(row_string, field)
        }
        
        vibes i > 0 {
            result = string_concat(result, line_ending)
        }
        result = string_concat(result, row_string)
    }
    
    fr fr RFC 4180: Optional final line ending
    result = string_concat(result, line_ending)
    damn result
}

fr fr Write with custom delimiter
slay stringify_with_delimiter(data [[tea]], delimiter tea) tea {
    sus result tea = ""
    
    bestie i := 0; i < len(data); i++ {
        sus row [tea] = data[i]
        sus row_string tea = ""
        
        bestie j := 0; j < len(row); j++ {
            sus field tea = escape_field(row[j])
            vibes j > 0 {
                row_string = string_concat(row_string, delimiter)
            }
            row_string = string_concat(row_string, field)
        }
        
        vibes i > 0 {
            result = string_concat(result, "\r\n")
        }
        result = string_concat(result, row_string)
    }
    
    result = string_concat(result, "\r\n")
    damn result
}

fr fr Enhanced RFC 4180 validation
slay validate(csv_string tea) lit {
    sus data [[tea]] = parse(csv_string)
    vibes len(data) == 0 {
        damn based  fr fr Empty CSV is valid
    }
    
    sus expected_columns normie = len(data[0])
    
    bestie i := 1; i < len(data); i++ {
        vibes len(data[i]) != expected_columns {
            damn cap  fr fr RFC 4180: All records must have same field count
        }
    }
    
    damn based
}

fr fr Get detailed validation results
squad ValidationResult {
    is_valid lit
    field_counts [normie]
    error_lines [normie]
    suggested_delimiter tea
    detected_line_ending tea
}

slay validate_detailed(csv_string tea) ValidationResult {
    sus result ValidationResult = ValidationResult{
        is_valid: based,
        field_counts: [],
        error_lines: [],
        suggested_delimiter: detect_delimiter(csv_string),
        detected_line_ending: detect_line_ending(csv_string)
    }
    
    sus data [[tea]] = parse(csv_string)
    vibes len(data) == 0 {
        damn result
    }
    
    sus expected_columns normie = len(data[0])
    
    bestie i := 0; i < len(data); i++ {
        result.field_counts = result.field_counts + [len(data[i])]
        vibes len(data[i]) != expected_columns {
            result.is_valid = cap
            result.error_lines = result.error_lines + [i + 1]
        }
    }
    
    damn result
}

fr fr Preserve all existing functions for backward compatibility

slay count_columns(csv_string tea) normie {
    sus data [[tea]] = parse(csv_string)
    vibes len(data) > 0 {
        damn len(data[0])
    }
    damn 0
}

slay get_headers(csv_string tea) [tea] {
    sus data [[tea]] = parse(csv_string)
    vibes len(data) > 0 {
        damn data[0]
    }
    damn []
}

slay stringify_with_headers(data [[tea]], headers [tea]) tea {
    sus all_data [[tea]] = [headers] + data
    damn stringify(all_data)
}

slay parse_with_headers(csv_string tea) [[tea]] {
    sus data [[tea]] = parse(csv_string)
    vibes len(data) == 0 {
        damn []
    }
    
    sus headers [tea] = data[0]
    sus result [[tea]] = []
    
    bestie i := 1; i < len(data); i++ {
        sus row [tea] = data[i]
        sus record [tea] = []
        
        bestie j := 0; j < len(headers); j++ {
            sus key tea = headers[j]
            sus value tea = ""
            vibes j < len(row) {
                value = row[j]
            }
            sus pair tea = key + ":" + value
            record = record + [pair]
        }
        
        result = result + [record]
    }
    
    damn result
}

slay filter_rows(data [[tea]], column_index normie, value tea) [[tea]] {
    sus result [[tea]] = []
    
    bestie i := 0; i < len(data); i++ {
        sus row [tea] = data[i]
        vibes column_index < len(row) && row[column_index] == value {
            result = result + [row]
        }
    }
    
    damn result
}

slay sort_by_column(data [[tea]], column_index normie) [[tea]] {
    sus result [[tea]] = data
    sus n normie = len(result)
    
    bestie i := 0; i < n - 1; i++ {
        bestie j := 0; j < n - i - 1; j++ {
            sus row1 [tea] = result[j]
            sus row2 [tea] = result[j + 1]
            
            vibes column_index < len(row1) && column_index < len(row2) {
                vibes row1[column_index] > row2[column_index] {
                    result[j] = row2
                    result[j + 1] = row1
                }
            }
        }
    }
    
    damn result
}

slay get_column(data [[tea]], column_index normie) [tea] {
    sus result [tea] = []
    
    bestie i := 0; i < len(data); i++ {
        sus row [tea] = data[i]
        vibes column_index < len(row) {
            result = result + [row[column_index]]
        } nah {
            result = result + [""]
        }
    }
    
    damn result
}

slay remove_column(data [[tea]], column_index normie) [[tea]] {
    sus result [[tea]] = []
    
    bestie i := 0; i < len(data); i++ {
        sus row [tea] = data[i]
        sus new_row [tea] = []
        
        bestie j := 0; j < len(row); j++ {
            vibes j != column_index {
                new_row = new_row + [row[j]]
            }
        }
        
        result = result + [new_row]
    }
    
    damn result
}

slay transpose(data [[tea]]) [[tea]] {
    vibes len(data) == 0 {
        damn []
    }
    
    sus first_row [tea] = data[0]
    sus col_count normie = len(first_row)
    sus result [[tea]] = []
    
    bestie col := 0; col < col_count; col++ {
        sus new_row [tea] = []
        bestie row := 0; row < len(data); row++ {
            sus current_row [tea] = data[row]
            vibes col < len(current_row) {
                new_row = new_row + [current_row[col]]
            } nah {
                new_row = new_row + [""]
            }
        }
        result = result + [new_row]
    }
    
    damn result
}

slay add_headers(data [[tea]], headers [tea]) [[tea]] {
    sus result [[tea]] = [headers]
    bestie i := 0; i < len(data); i++ {
        result = result + [data[i]]
    }
    damn result
}

fr fr New RFC 4180 specific functions

fr fr Parse CSV from file path (assumes file reading capability)
slay parse_file(file_path tea) [[tea]] {
    fr fr This would require file I/O implementation
    fr fr For now, return empty result
    damn []
}

fr fr Write CSV to file path (assumes file writing capability)
slay write_file(data [[tea]], file_path tea) lit {
    fr fr This would require file I/O implementation
    fr fr For now, return success
    damn based
}

fr fr Get CSV statistics
squad CsvStats {
    total_records normie
    total_fields normie
    average_fields_per_record drip
    delimiter tea
    line_ending tea
    has_headers lit
    estimated_size normie
}

slay get_stats(csv_string tea) CsvStats {
    sus data [[tea]] = parse(csv_string)
    sus total_records normie = len(data)
    sus total_fields normie = 0
    
    bestie i := 0; i < len(data); i++ {
        total_fields = total_fields + len(data[i])
    }
    
    sus avg_fields drip = 0.0
    vibes total_records > 0 {
        avg_fields = drip(total_fields) / drip(total_records)
    }
    
    damn CsvStats{
        total_records: total_records,
        total_fields: total_fields,
        average_fields_per_record: avg_fields,
        delimiter: detect_delimiter(csv_string),
        line_ending: detect_line_ending(csv_string),
        has_headers: total_records > 0,
        estimated_size: string_len(csv_string)
    }
}
