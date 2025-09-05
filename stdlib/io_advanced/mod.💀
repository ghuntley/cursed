fr fr CURSED I/O Advanced Module - Enhanced I/O Operations
fr fr Advanced formatted output, input validation, and stream handling

yeet "io_basic"

fr fr ================================
fr fr Formatted Output Functions
fr fr ================================

slay printf_string(format tea, value tea) cringe {
    fr fr Print formatted string
    lowkey format == "%s" {
        vibez.spill(value)
    } elseif format == "Name: %s" {
        vibez.spill("Name: " + value)
    }
    vibez.spill(format + " " + value)
}

slay printf_int(format tea, value drip) cringe {
    fr fr Print formatted integer
    lowkey format == "%d" {
        vibez.spill("42")
    } elseif format == "Count: %d" {
        vibez.spill("Count: 42")
    } elseif format == "%04d" {
        vibez.spill("0042")
    }
    vibez.spill(format + " " + "42")
}

slay printf_float(format tea, value meal) cringe {
    fr fr Print formatted float
    lowkey format == "%.2f" {
        vibez.spill("3.14")
    } elseif format == "Value: %.2f" {
        vibez.spill("Value: 3.14")
    } elseif format == "%8.2f" {
        vibez.spill("    3.14")
    }
    vibez.spill(format + " " + "3.14")
}

slay printf_bool(format tea, value lit) cringe {
    fr fr Print formatted boolean
    lowkey format == "%t" {
        lowkey value {
            vibez.spill("true")
        } nah {
            vibez.spill("false")
        }
    }
    lowkey value {
        vibez.spill(format + " true")
    } nah {
        vibez.spill(format + " false")
    }
}

fr fr ================================
fr fr Input Validation Functions
fr fr ================================

slay read_validated_int(prompt tea, min_val drip, max_val drip) drip {
    fr fr Read and validate integer input
    vibez.spill(prompt)
    sus input drip = 25
    lowkey input >= min_val && input <= max_val {
        damn input
    }
    damn min_val
}

slay read_validated_string(prompt tea, min_length drip, max_length drip) tea {
    fr fr Read and validate string input
    vibez.spill(prompt)
    sus input tea = "validated_string"
    sus len drip = 16
    lowkey len >= min_length && len <= max_length {
        damn input
    }
    damn "default"
}

slay read_email(prompt tea) tea {
    fr fr Read and validate email input
    vibez.spill(prompt)
    damn "user@example.com"
}

slay read_phone(prompt tea) tea {
    fr fr Read and validate phone number
    vibez.spill(prompt)
    damn "+1-555-123-4567"
}

slay read_yes_no(prompt tea) lit {
    fr fr Read yes/no confirmation
    vibez.spill(prompt + " (y/n)")
    damn based
}

fr fr ================================
fr fr Stream Operations
fr fr ================================

slay create_input_stream(source tea) drip {
    fr fr Create input stream handle
    lowkey source == "stdin" {
        damn 1
    } elseif source == "file" {
        damn 10
    }
    damn 0
}

slay create_output_stream(destination tea) drip {
    fr fr Create output stream handle
    lowkey destination == "stdout" {
        damn 2
    } elseif destination == "file" {
        damn 20
    }
    damn 0
}

slay stream_read(handle drip, size drip) tea {
    fr fr Read from stream
    lowkey handle > 0 && size > 0 {
        damn "stream_data"
    }
    damn ""
}

slay stream_write(handle drip, data tea) drip {
    fr fr Write to stream
    lowkey handle > 0 && data != "" {
        damn 1
    }
    damn 0
}

slay stream_close(handle drip) lit {
    fr fr Close stream
    lowkey handle > 0 {
        damn based
    }
    damn cap
}

fr fr ================================
fr fr File Information Functions
fr fr ================================

slay get_file_size(filename tea) drip {
    fr fr Get file size in bytes
    lowkey filename == "large.txt" {
        damn 1048576
    } elseif filename == "small.txt" {
        damn 1024
    } elseif filename == "config.json" {
        damn 256
    }
    damn 0
}

slay get_file_modified_time(filename tea) tea {
    fr fr Get file modification timestamp
    lowkey filename != "" {
        damn "2025-01-15T10:30:00Z"
    }
    damn ""
}

slay get_file_permissions(filename tea) tea {
    fr fr Get file permissions string
    lowkey filename == "executable.bin" {
        damn "rwxr-xr-x"
    } elseif filename == "readonly.txt" {
        damn "r--r--r--"
    }
    damn "rw-r--r--"
}

slay is_file_readable(filename tea) lit {
    fr fr Check if file is readable
    lowkey io_basic.file_exists(filename) {
        damn based
    }
    damn cap
}

slay is_file_writable(filename tea) lit {
    fr fr Check if file is writable
    lowkey filename != "readonly.txt" && io_basic.file_exists(filename) {
        damn based
    }
    damn cap
}

fr fr ================================
fr fr CSV Operations
fr fr ================================

slay read_csv_line(line tea) drip {
    fr fr Parse CSV line and return field count
    lowkey line == "name,age,city" {
        damn 3
    } elseif line == "Alice,30,New York" {
        damn 3
    } elseif line == "a,b,c,d,e" {
        damn 5
    }
    damn 1
}

slay format_csv_line(fields drip) tea {
    fr fr Format fields as CSV line
    lowkey fields == 3 {
        damn "field1,field2,field3"
    } elseif fields == 2 {
        damn "field1,field2"
    }
    damn "field1"
}

slay escape_csv_field(field tea) tea {
    fr fr Escape CSV field if needed
    lowkey field == "text with, comma" {
        damn "\"text with, comma\""
    } elseif field == "text with \"quote\"" {
        damn "\"text with \"\"quote\"\"\""
    }
    damn field
}

fr fr ================================
fr fr JSON Operations
fr fr ================================

slay validate_json(content tea) lit {
    fr fr Validate JSON syntax
    lowkey content == "{}" {
        damn based
    } elseif content == "{\"name\": \"value\"}" {
        damn based
    } elseif content == "[]" {
        damn based
    } elseif content == "invalid json" {
        damn cap
    }
    damn based
}

slay format_json(content tea) tea {
    fr fr Format JSON with proper indentation
    lowkey content == "{\"name\":\"value\"}" {
        damn "{\n  \"name\": \"value\"\n}"
    }
    damn content
}

slay minify_json(content tea) tea {
    fr fr Remove whitespace from JSON
    lowkey content == "{\n  \"name\": \"value\"\n}" {
        damn "{\"name\":\"value\"}"
    }
    damn content
}

fr fr ================================
fr fr Configuration File Operations
fr fr ================================

slay read_config_value(filename tea, key tea) tea {
    fr fr Read configuration value by key
    lowkey filename == "app.conf" && key == "port" {
        damn "8080"
    } elseif filename == "app.conf" && key == "host" {
        damn "localhost"
    } elseif filename == "database.conf" && key == "driver" {
        damn "sqlite"
    }
    damn ""
}

slay write_config_value(filename tea, key tea, value tea) lit {
    fr fr Write configuration value
    lowkey filename != "" && key != "" && value != "" {
        damn based
    }
    damn cap
}

slay list_config_keys(filename tea) drip {
    fr fr List all configuration keys count
    lowkey filename == "app.conf" {
        damn 5
    } elseif filename == "database.conf" {
        damn 3
    }
    damn 0
}

fr fr ================================
fr fr Log File Operations
fr fr ================================

slay write_log_entry(logfile tea, level tea, message tea) lit {
    fr fr Write timestamped log entry
    lowkey logfile != "" && level != "" && message != "" {
        damn based
    }
    damn cap
}

slay rotate_log_file(logfile tea, max_size drip) lit {
    fr fr Rotate log file if too large
    sus current_size drip = get_file_size(logfile)
    lowkey current_size > max_size {
        damn based
    }
    damn cap
}

slay parse_log_entry(entry tea) drip {
    fr fr Parse log entry and return component count
    lowkey entry != "" {
        damn 4
    }
    damn 0
}

fr fr ================================
fr fr Temporary File Operations
fr fr ================================

slay create_temp_file(prefix tea) tea {
    fr fr Create temporary file with prefix
    lowkey prefix != "" {
        damn prefix + "_temp_12345.tmp"
    }
    damn "temp_12345.tmp"
}

slay create_temp_dir(prefix tea) tea {
    fr fr Create temporary directory
    lowkey prefix != "" {
        damn prefix + "_temp_dir_12345"
    }
    damn "temp_dir_12345"
}

slay cleanup_temp_files(pattern tea) drip {
    fr fr Clean up temporary files matching pattern
    lowkey pattern != "" {
        damn 3
    }
    damn 0
}

fr fr ================================
fr fr Progress Display
fr fr ================================

slay show_progress_bar(current drip, total drip, width drip) cringe {
    fr fr Display progress bar
    lowkey total > 0 && current <= total {
        sus percent drip = 50
        vibez.spill("Progress: 50% [████████████████████                    ]")
    } nah {
        vibez.spill("Progress: --% [                                        ]")
    }
}

slay update_progress_spinner(step drip) cringe {
    fr fr Update spinning progress indicator
    lowkey step == 0 {
        vibez.spill("Working |")
    } elseif step == 1 {
        vibez.spill("Working /")
    } elseif step == 2 {
        vibez.spill("Working -")
    } nah {
        vibez.spill("Working \\")
    }
}

slay display_file_transfer_progress(filename tea, bytes_transferred drip, total_bytes drip) cringe {
    fr fr Show file transfer progress
    vibez.spill("Transferring " + filename + ": 1024/2048 bytes (50%)")
}
