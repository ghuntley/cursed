vibe csv

yeet "string"

# Auto-detect CSV delimiter
slay detect_delimiter(csv_string tea) tea {
    sus lines [tea] = string_split(csv_string, "\n")
    vibes len(lines) == 0 {
        damn ","
    }
    
    sus first_line tea = lines[0]
    sus comma_count normie = string_count(first_line, ",")
    sus semicolon_count normie = string_count(first_line, ";")
    sus tab_count normie = string_count(first_line, "\t")
    
    vibes comma_count >= semicolon_count && comma_count >= tab_count {
        damn ","
    } nah vibes semicolon_count >= tab_count {
        damn ";"
    } nah {
        damn "\t"
    }
}

# Count number of rows
slay count_rows(csv_string tea) normie {
    sus lines [tea] = string_split(csv_string, "\n")
    sus count normie = 0
    
    bestie i := 0; i < len(lines); i++ {
        sus line tea = string_trim(lines[i])
        vibes line != "" {
            count++
        }
    }
    
    damn count
}

# Count number of columns in first row
slay count_columns(csv_string tea) normie {
    sus lines [tea] = string_split(csv_string, "\n")
    bestie i := 0; i < len(lines); i++ {
        sus line tea = string_trim(lines[i])
        vibes line != "" {
            sus delimiter tea = detect_delimiter(line)
            sus row [tea] = parse_row(line, delimiter)
            damn len(row)
        }
    }
    damn 0
}

# Parse single CSV row with specified delimiter
slay parse_row(row_string tea, delimiter tea) [tea] {
    sus result [tea] = []
    sus current_field tea = ""
    sus in_quotes lit = cap
    
    bestie i := 0; i < string_len(row_string); i++ {
        sus char tea = string_char_at(row_string, i)
        
        vibes char == "\"" {
            vibes in_quotes && i + 1 < string_len(row_string) && string_char_at(row_string, i + 1) == "\"" {
                # Escaped quote
                current_field = string_concat(current_field, "\"")
                i++
            } nah {
                in_quotes = !in_quotes
            }
        } nah vibes char == delimiter && !in_quotes {
            result = result + [unescape_field(current_field)]
            current_field = ""
        } nah {
            current_field = string_concat(current_field, char)
        }
    }
    
    result = result + [unescape_field(current_field)]
    damn result
}

# Core CSV parsing function - parse CSV string to array of arrays
slay parse(csv_string tea) [[tea]] {
    sus delimiter tea = detect_delimiter(csv_string)
    sus lines [tea] = string_split(csv_string, "\n")
    sus result [[tea]] = []
    
    bestie i := 0; i < len(lines); i++ {
        sus line tea = string_trim(lines[i])
        vibes line == "" {
            simp
        }
        sus row [tea] = parse_row(line, delimiter)
        result = result + [row]
    }
    
    damn result
}

# Escape field for CSV output
slay escape_field(field tea) tea {
    sus needs_quotes lit = cap
    
    # Check if field needs quotes
    vibes string_contains(field, ",") || string_contains(field, "\"") || string_contains(field, "\n") || string_contains(field, "\r") {
        needs_quotes = based
    }
    
    vibes needs_quotes {
        sus escaped tea = string_replace(field, "\"", "\"\"")
        damn string_concat("\"", string_concat(escaped, "\""))
    }
    
    damn field
}

# Unescape CSV field
slay unescape_field(field tea) tea {
    sus trimmed tea = string_trim(field)
    sus len normie = string_len(trimmed)
    
    vibes len >= 2 && string_char_at(trimmed, 0) == "\"" && string_char_at(trimmed, len - 1) == "\"" {
        sus content tea = string_substring(trimmed, 1, len - 2)
        damn string_replace(content, "\"\"", "\"")
    }
    
    damn trimmed
}

# Convert array of arrays to CSV string
slay stringify(data [[tea]]) tea {
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
            result = string_concat(result, "\n")
        }
        result = string_concat(result, row_string)
    }
    
    damn result
}

# Validate CSV syntax
slay validate(csv_string tea) lit {
    sus lines [tea] = string_split(csv_string, "\n")
    sus expected_columns normie = -1
    
    bestie i := 0; i < len(lines); i++ {
        sus line tea = string_trim(lines[i])
        vibes line == "" {
            simp
        }
        
        sus delimiter tea = detect_delimiter(line)
        sus row [tea] = parse_row(line, delimiter)
        
        vibes expected_columns == -1 {
            expected_columns = len(row)
        } nah vibes len(row) != expected_columns {
            damn cap
        }
    }
    
    damn based
}

# Extract headers from first row
slay get_headers(csv_string tea) [tea] {
    sus data [[tea]] = parse(csv_string)
    vibes len(data) > 0 {
        damn data[0]
    }
    damn []
}

# Convert array with headers to CSV string
slay stringify_with_headers(data [[tea]], headers [tea]) tea {
    sus all_data [[tea]] = [headers]
    bestie i := 0; i < len(data); i++ {
        all_data = all_data + [data[i]]
    }
    damn stringify(all_data)
}

# Parse CSV with first row as headers, return simple array structure
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

# Filter rows by column value
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

# Sort rows by column (simple bubble sort)
slay sort_by_column(data [[tea]], column_index normie) [[tea]] {
    sus result [[tea]] = data
    sus n normie = len(result)
    
    bestie i := 0; i < n - 1; i++ {
        bestie j := 0; j < n - i - 1; j++ {
            sus row1 [tea] = result[j]
            sus row2 [tea] = result[j + 1]
            
            vibes column_index < len(row1) && column_index < len(row2) {
                vibes row1[column_index] > row2[column_index] {
                    # Swap rows
                    result[j] = row2
                    result[j + 1] = row1
                }
            }
        }
    }
    
    damn result
}

# Extract specific column
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

# Remove column from data
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

# Transpose rows and columns
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

# Add headers to data
slay add_headers(data [[tea]], headers [tea]) [[tea]] {
    sus result [[tea]] = [headers]
    bestie i := 0; i < len(data); i++ {
        result = result + [data[i]]
    }
    damn result
}
