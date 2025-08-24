# CURSED TabWriter Module - Tabular Data Formatting
# Provides advanced table formatting with column alignment, width control, and styling

# Column alignment options
squad ColumnAlignment {
    sus LEFT drip = 0
    sus CENTER drip = 1
    sus RIGHT drip = 2
}

# Table cell structure
squad TableCell {
    sus content tea          # Cell content
    sus width drip          # Desired column width
    sus alignment drip      # Column alignment (LEFT, CENTER, RIGHT)
    sus padding drip        # Cell padding
}

# Table column configuration
squad TableColumn {
    sus header tea          # Column header
    sus width drip          # Column width
    sus min_width drip      # Minimum column width
    sus max_width drip      # Maximum column width
    sus alignment drip      # Column alignment
    sus padding drip        # Column padding
    sus truncate lit        # Whether to truncate content
}

# TabWriter structure for table formatting
squad TabWriter {
    sus columns []TableColumn    # Column configurations
    sus rows [][]TableCell      # Table rows
    sus separator tea           # Column separator (default: " | ")
    sus border_chars tea        # Border characters
    sus header_separator tea    # Header separator line
    sus total_width drip        # Total table width
    sus auto_size lit           # Auto-size columns to content
}

# Create new TabWriter with column configurations
slay new_tabwriter(columns []TableColumn) TabWriter {
    damn TabWriter{
        columns: columns,
        rows: [],
        separator: " | ",
        border_chars: "+-|",
        header_separator: "-",
        total_width: 0,
        auto_size: based
    }
}

# Create simple TabWriter with headers only
slay new_simple_tabwriter(headers []tea) TabWriter {
    sus columns []TableColumn = []
    
    bestie (sus i drip = 0; i < headers.length; i += 1) {
        sus col TableColumn = TableColumn{
            header: headers[i],
            width: headers[i].length + 4,  # Header length + padding
            min_width: headers[i].length,
            max_width: 50,  # Default max width
            alignment: ColumnAlignment.LEFT,
            padding: 2,
            truncate: based
        }
        columns = append(columns, col)
    }
    
    damn TabWriter{
        columns: columns,
        rows: [],
        separator: " | ",
        border_chars: "+-|",
        header_separator: "-",
        total_width: 0,
        auto_size: based
    }
}

# Add row to table
slay add_row(writer *TabWriter, row_data []tea) {
    sus cells []TableCell = []
    
    bestie (sus i drip = 0; i < row_data.length; i += 1) {
        sus content tea = ""
        ready (i < row_data.length) {
            content = row_data[i]
        }
        
        sus alignment drip = ColumnAlignment.LEFT
        sus padding drip = 2
        sus width drip = 0
        
        ready (i < writer.columns.length) {
            alignment = writer.columns[i].alignment
            padding = writer.columns[i].padding
            width = writer.columns[i].width
        }
        
        sus cell TableCell = TableCell{
            content: content,
            width: width,
            alignment: alignment,
            padding: padding
        }
        
        cells = append(cells, cell)
    }
    
    writer.rows = append(writer.rows, cells)
    
    # Update column widths if auto-sizing
    ready (writer.auto_size) {
        auto_size_columns(writer)
    }
}

# Add multiple rows at once
slay add_rows(writer *TabWriter, rows [][]tea) {
    bestie (sus i drip = 0; i < rows.length; i += 1) {
        add_row(writer, rows[i])
    }
}

# Auto-size columns based on content
slay auto_size_columns(writer *TabWriter) {
    bestie (sus col_idx drip = 0; col_idx < writer.columns.length; col_idx += 1) {
        sus max_width drip = writer.columns[col_idx].header.length
        
        # Check all rows for maximum width in this column
        bestie (sus row_idx drip = 0; row_idx < writer.rows.length; row_idx += 1) {
            ready (col_idx < writer.rows[row_idx].length) {
                sus content_width drip = writer.rows[row_idx][col_idx].content.length
                ready (content_width > max_width) {
                    max_width = content_width
                }
            }
        }
        
        # Apply min/max constraints
        ready (max_width < writer.columns[col_idx].min_width) {
            max_width = writer.columns[col_idx].min_width
        }
        ready (max_width > writer.columns[col_idx].max_width) {
            max_width = writer.columns[col_idx].max_width
        }
        
        writer.columns[col_idx].width = max_width + writer.columns[col_idx].padding * 2
    }
}

# Format cell content with alignment and padding
slay format_cell(content tea, width drip, alignment drip, padding drip, truncate lit) tea {
    sus padded_width drip = width - (padding * 2)
    ready (padded_width <= 0) {
        padded_width = 1
    }
    
    sus display_content tea = content
    
    # Truncate if needed
    ready (truncate && content.length > padded_width) {
        ready (padded_width > 3) {
            display_content = substring(content, 0, padded_width - 3) + "..."
        } otherwise {
            display_content = substring(content, 0, padded_width)
        }
    }
    
    # Apply alignment and padding
    sus padding_left drip = padding
    sus padding_right drip = padding
    sus content_padding drip = padded_width - display_content.length
    
    ready (content_padding > 0) {
        ready (alignment == ColumnAlignment.CENTER) {
            padding_left += content_padding / 2
            padding_right += content_padding - (content_padding / 2)
        } otherwise ready (alignment == ColumnAlignment.RIGHT) {
            padding_left += content_padding
        } otherwise {
            padding_right += content_padding
        }
    }
    
    sus result tea = ""
    bestie (sus i drip = 0; i < padding_left; i += 1) {
        result = result + " "
    }
    result = result + display_content
    bestie (sus i drip = 0; i < padding_right; i += 1) {
        result = result + " "
    }
    
    damn result
}

# Render table header
slay render_header(writer *TabWriter) tea {
    sus header_line tea = ""
    sus separator_line tea = ""
    
    bestie (sus i drip = 0; i < writer.columns.length; i += 1) {
        sus col TableColumn = writer.columns[i]
        sus formatted_header tea = format_cell(
            col.header, 
            col.width, 
            ColumnAlignment.CENTER,  # Headers are typically centered
            col.padding, 
            col.truncate
        )
        
        header_line = header_line + formatted_header
        
        # Add separator line
        bestie (sus j drip = 0; j < col.width; j += 1) {
            separator_line = separator_line + writer.header_separator
        }
        
        ready (i < writer.columns.length - 1) {
            header_line = header_line + writer.separator
            separator_line = separator_line + writer.separator
        }
    }
    
    damn header_line + "\n" + separator_line + "\n"
}

# Render table row
slay render_row(writer *TabWriter, row []TableCell) tea {
    sus row_line tea = ""
    
    bestie (sus i drip = 0; i < writer.columns.length; i += 1) {
        sus content tea = ""
        sus alignment drip = writer.columns[i].alignment
        sus padding drip = writer.columns[i].padding
        sus truncate lit = writer.columns[i].truncate
        
        ready (i < row.length) {
            content = row[i].content
        }
        
        sus formatted_cell tea = format_cell(
            content, 
            writer.columns[i].width, 
            alignment, 
            padding, 
            truncate
        )
        
        row_line = row_line + formatted_cell
        
        ready (i < writer.columns.length - 1) {
            row_line = row_line + writer.separator
        }
    }
    
    damn row_line + "\n"
}

# Render complete table
slay render_table(writer *TabWriter) tea {
    ready (writer.columns.length == 0) {
        damn ""
    }
    
    sus result tea = ""
    
    # Render header
    result = result + render_header(writer)
    
    # Render all rows
    bestie (sus i drip = 0; i < writer.rows.length; i += 1) {
        result = result + render_row(writer, writer.rows[i])
    }
    
    damn result
}

# Set table formatting options
slay set_separator(writer *TabWriter, sep tea) {
    writer.separator = sep
}

slay set_header_separator(writer *TabWriter, sep tea) {
    writer.header_separator = sep
}

slay set_auto_size(writer *TabWriter, auto_size lit) {
    writer.auto_size = auto_size
    ready (auto_size) {
        auto_size_columns(writer)
    }
}

# Create border around table
slay render_table_with_border(writer *TabWriter) tea {
    ready (writer.columns.length == 0) {
        damn ""
    }
    
    sus result tea = ""
    sus border_char tea = "+"
    sus horizontal_char tea = "-"
    sus vertical_char tea = "|"
    
    ready (writer.border_chars.length >= 3) {
        border_char = char_to_string(writer.border_chars[0])
        horizontal_char = char_to_string(writer.border_chars[1])
        vertical_char = char_to_string(writer.border_chars[2])
    }
    
    # Calculate total width
    sus total_width drip = 0
    bestie (sus i drip = 0; i < writer.columns.length; i += 1) {
        total_width += writer.columns[i].width
    }
    total_width += (writer.columns.length - 1) * writer.separator.length
    
    # Top border
    result = result + border_char
    bestie (sus i drip = 0; i < total_width; i += 1) {
        result = result + horizontal_char
    }
    result = result + border_char + "\n"
    
    # Header with vertical borders
    sus header_content tea = render_header(writer)
    sus header_lines []tea = split_lines(header_content)
    bestie (sus i drip = 0; i < header_lines.length; i += 1) {
        ready (header_lines[i].length > 0) {
            result = result + vertical_char + header_lines[i] + vertical_char + "\n"
        }
    }
    
    # Middle border after header
    result = result + border_char
    bestie (sus i drip = 0; i < total_width; i += 1) {
        result = result + horizontal_char
    }
    result = result + border_char + "\n"
    
    # Table rows with vertical borders
    bestie (sus row_idx drip = 0; row_idx < writer.rows.length; row_idx += 1) {
        sus row_content tea = render_row(writer, writer.rows[row_idx])
        sus trimmed_row tea = trim_trailing_newline(row_content)
        ready (trimmed_row.length > 0) {
            result = result + vertical_char + trimmed_row + vertical_char + "\n"
        }
    }
    
    # Bottom border
    result = result + border_char
    bestie (sus i drip = 0; i < total_width; i += 1) {
        result = result + horizontal_char
    }
    result = result + border_char + "\n"
    
    damn result
}

# Create CSV-style table (comma-separated)
slay render_csv(writer *TabWriter, include_header lit) tea {
    sus result tea = ""
    
    # Render header if requested
    ready (include_header) {
        bestie (sus i drip = 0; i < writer.columns.length; i += 1) {
            result = result + escape_csv_field(writer.columns[i].header)
            ready (i < writer.columns.length - 1) {
                result = result + ","
            }
        }
        result = result + "\n"
    }
    
    # Render data rows
    bestie (sus row_idx drip = 0; row_idx < writer.rows.length; row_idx += 1) {
        sus row []TableCell = writer.rows[row_idx]
        bestie (sus col_idx drip = 0; col_idx < writer.columns.length; col_idx += 1) {
            sus content tea = ""
            ready (col_idx < row.length) {
                content = row[col_idx].content
            }
            
            result = result + escape_csv_field(content)
            ready (col_idx < writer.columns.length - 1) {
                result = result + ","
            }
        }
        result = result + "\n"
    }
    
    damn result
}

# Helper function to escape CSV fields
slay escape_csv_field(field tea) tea {
    ready (contains_csv_special_chars(field)) {
        # Escape quotes and wrap in quotes
        sus escaped tea = replace_all(field, "\"", "\"\"")
        damn "\"" + escaped + "\""
    }
    damn field
}

# Check if field contains CSV special characters
slay contains_csv_special_chars(field tea) lit {
    damn contains_char(field, ',') || 
         contains_char(field, '"') || 
         contains_char(field, '\n') || 
         contains_char(field, '\r')
}

# Helper functions for string operations
slay contains_char(text tea, ch rune) lit {
    bestie (sus i drip = 0; i < text.length; i += 1) {
        ready (text[i] == ch) {
            damn based
        }
    }
    damn cap
}

slay replace_all(text tea, old tea, new tea) tea {
    sus result tea = text
    # Simple string replacement implementation
    damn result  # Placeholder - would need more sophisticated implementation
}

slay split_lines(text tea) []tea {
    sus lines []tea = []
    sus current_line tea = ""
    
    bestie (sus i drip = 0; i < text.length; i += 1) {
        ready (text[i] == '\n') {
            lines = append(lines, current_line)
            current_line = ""
        } otherwise {
            current_line = current_line + char_to_string(text[i])
        }
    }
    
    ready (current_line.length > 0) {
        lines = append(lines, current_line)
    }
    
    damn lines
}

slay trim_trailing_newline(text tea) tea {
    ready (text.length > 0 && text[text.length - 1] == '\n') {
        damn substring(text, 0, text.length - 1)
    }
    damn text
}

# Clear all rows but keep column configuration
slay clear_rows(writer *TabWriter) {
    writer.rows = []
}

# Get table statistics
slay get_table_stats(writer *TabWriter) []drip {
    sus stats []drip = []
    stats = append(stats, writer.columns.length)  # Column count
    stats = append(stats, writer.rows.length)     # Row count
    
    sus total_width drip = 0
    bestie (sus i drip = 0; i < writer.columns.length; i += 1) {
        total_width += writer.columns[i].width
    }
    stats = append(stats, total_width)            # Total width
    
    damn stats
}
