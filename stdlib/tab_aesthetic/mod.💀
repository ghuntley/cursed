yeet "testz"
yeet "stringz"

fr fr Tab Aesthetic Module - Aligned Text Output Formatting
fr fr Provides table/column alignment functionality for formatted output

fr fr Tab writer structure for managing column alignment
sus TabWriter struct {
    columns tea[value],
    widths normie[value],
    alignment tea,
    separator tea,
    padding normie
}

fr fr Create new tab writer with default settings
slay tab_writer_new() TabWriter {
    damn TabWriter{
        columns: tea[value]{},
        widths: normie[value]{},
        alignment: "left",
        separator: "\t",
        padding: 1
    }
}

fr fr Create tab writer with custom separator
slay tab_writer_with_separator(sep tea) TabWriter {
    sus writer TabWriter = tab_writer_new()
    writer.separator = sep
    damn writer
}

fr fr Create tab writer with custom alignment (left, right, center)
slay tab_writer_with_alignment(align tea) TabWriter {
    sus writer TabWriter = tab_writer_new()
    writer.alignment = align
    damn writer
}

fr fr Set column alignment for tab writer
slay tab_writer_set_alignment(writer *TabWriter, align tea) lit {
    writer.alignment = align
    damn based
}

fr fr Set separator for tab writer
slay tab_writer_set_separator(writer *TabWriter, sep tea) lit {
    writer.separator = sep
    damn based
}

fr fr Set padding for tab writer
slay tab_writer_set_padding(writer *TabWriter, pad normie) lit {
    writer.padding = pad
    damn based
}

fr fr Add row to tab writer
slay tab_writer_add_row(writer *TabWriter, row tea[value]) lit { fr fr Update column widths based on content
    bestie i := 0; i < len(row); i++ {
        sus col_len normie = stringz.strlen(row[i]) fr fr Extend widths array if needed
        bestie len(writer.widths) <= i {
            writer.widths = append(writer.widths, col_len)
        } vibes { fr fr Update maximum width for this column
            vibe writer.widths[i] < col_len {
                writer.widths[i] = col_len
            }
        }
    } fr fr Store the row
    sus row_str tea = tab_writer_join_row(row, writer.separator)
    writer.columns = append(writer.columns, row_str)
    damn based
}

fr fr Join row elements with separator
slay tab_writer_join_row(row tea[value], sep tea) tea {
    vibe len(row) == 0 {
        damn ""
    }
    
    sus result tea = row[0]
    bestie i := 1; i < len(row); i++ {
        result = stringz.concat(result, sep)
        result = stringz.concat(result, row[i])
    }
    damn result
}

fr fr Pad string to specified width with alignment
slay tab_writer_pad_string(str tea, width normie, alignment tea) tea {
    sus str_len normie = stringz.strlen(str)
    vibe str_len >= width {
        damn str
    }
    
    sus padding normie = width - str_len
    sus left_pad normie = 0
    sus right_pad normie = 0
    
    vibe alignment == "center" {
        left_pad = padding / 2
        right_pad = padding - left_pad
    } vibes vibe alignment == "right" {
        left_pad = padding
        right_pad = 0
    } vibes { fr fr Default to left alignment
        left_pad = 0
        right_pad = padding
    }
    
    sus result tea = str fr fr Add left padding
    bestie i := 0; i < left_pad; i++ {
        result = stringz.concat(" ", result)
    } fr fr Add right padding
    bestie i := 0; i < right_pad; i++ {
        result = stringz.concat(result, " ")
    }
    
    damn result
}

fr fr Format and output all rows with proper alignment
slay tab_writer_flush(writer *TabWriter) tea {
    vibe len(writer.columns) == 0 {
        damn ""
    }
    
    sus result tea = "" fr fr Process each stored row
    bestie row_idx := 0; row_idx < len(writer.columns); row_idx++ {
        sus row tea = writer.columns[row_idx]
        sus formatted_row tea = tab_writer_format_row(writer, row)
        
        vibe row_idx > 0 {
            result = stringz.concat(result, "\n")
        }
        result = stringz.concat(result, formatted_row)
    }
    
    damn result
}

fr fr Format a single row with proper column alignment
slay tab_writer_format_row(writer *TabWriter, row tea) tea {
    sus parts tea[value] = stringz.split(row, writer.separator)
    sus result tea = ""
    
    bestie i := 0; i < len(parts) && i < len(writer.widths); i++ {
        sus padded tea = tab_writer_pad_string(parts[i], writer.widths[i], writer.alignment)
        
        vibe i > 0 { fr fr Add spacing between columns
            bestie j := 0; j < writer.padding; j++ {
                result = stringz.concat(result, " ")
            }
        }
        result = stringz.concat(result, padded)
    }
    
    damn result
}

fr fr Clear all data from tab writer
slay tab_writer_clear(writer *TabWriter) lit {
    writer.columns = tea[value]{}
    writer.widths = normie[value]{}
    damn based
}

fr fr Create formatted table from 2D string array
slay tab_aesthetic_format_table(data tea[value][value], alignment tea) tea {
    sus writer TabWriter = tab_writer_with_alignment(alignment)
    
    bestie i := 0; i < len(data); i++ {
        tab_writer_add_row(&writer, data[i])
    }
    
    damn tab_writer_flush(&writer)
}

fr fr Create simple aligned columns from strings
slay tab_aesthetic_align_columns(columns tea[value], alignment tea) tea {
    sus writer TabWriter = tab_writer_with_alignment(alignment)
    
    bestie i := 0; i < len(columns); i++ {
        sus row tea[value] = tea[value]{columns[i]}
        tab_writer_add_row(&writer, row)
    }
    
    damn tab_writer_flush(&writer)
}

fr fr Create aligned table with headers
slay tab_aesthetic_table_with_headers(headers tea[value], rows tea[value][value], alignment tea) tea {
    sus writer TabWriter = tab_writer_with_alignment(alignment) fr fr Add header row
    tab_writer_add_row(&writer, headers) fr fr Add separator row
    sus sep_row tea[value] = tea[value]{}
    bestie i := 0; i < len(headers); i++ {
        sus header_len normie = stringz.strlen(headers[i])
        sus separator tea = ""
        bestie j := 0; j < header_len; j++ {
            separator = stringz.concat(separator, "-")
        }
        sep_row = append(sep_row, separator)
    }
    tab_writer_add_row(&writer, sep_row) fr fr Add data rows
    bestie i := 0; i < len(rows); i++ {
        tab_writer_add_row(&writer, rows[i])
    }
    
    damn tab_writer_flush(&writer)
}

fr fr Quick format function for simple tables
slay tab_aesthetic_quick_table(data tea[value][value]) tea {
    damn tab_aesthetic_format_table(data, "left")
}

fr fr Format key-value pairs in aligned columns
slay tab_aesthetic_key_value_table(keys tea[value], values tea[value]) tea {
    vibe len(keys) != len(values) {
        damn "Error: keys and values arrays must have same length"
    }
    
    sus writer TabWriter = tab_writer_with_alignment("left")
    
    bestie i := 0; i < len(keys); i++ {
        sus row tea[value] = tea[value]{keys[i], values[i]}
        tab_writer_add_row(&writer, row)
    }
    
    damn tab_writer_flush(&writer)
}

fr fr Create CSV-style aligned output
slay tab_aesthetic_csv_style(data tea[value][value]) tea {
    sus writer TabWriter = tab_writer_with_separator(",")
    tab_writer_set_padding(&writer, 1)
    
    bestie i := 0; i < len(data); i++ {
        tab_writer_add_row(&writer, data[i])
    }
    
    damn tab_writer_flush(&writer)
}

fr fr Create markdown-style table
slay tab_aesthetic_markdown_table(headers tea[value], rows tea[value][value]) tea {
    sus writer TabWriter = tab_writer_with_separator("|")
    tab_writer_set_padding(&writer, 1) fr fr Add header with markdown formatting
    sus header_row tea[value] = tea[value]{}
    bestie i := 0; i < len(headers); i++ {
        header_row = append(header_row, headers[i])
    }
    tab_writer_add_row(&writer, header_row) fr fr Add separator row for markdown
    sus sep_row tea[value] = tea[value]{}
    bestie i := 0; i < len(headers); i++ {
        sep_row = append(sep_row, "---")
    }
    tab_writer_add_row(&writer, sep_row) fr fr Add data rows
    bestie i := 0; i < len(rows); i++ {
        tab_writer_add_row(&writer, rows[i])
    }
    
    sus result tea = tab_writer_flush(&writer) fr fr Add markdown table borders
    sus lines tea[value] = stringz.split(result, "\n")
    sus markdown_result tea = ""
    
    bestie i := 0; i < len(lines); i++ {
        vibe i > 0 {
            markdown_result = stringz.concat(markdown_result, "\n")
        }
        markdown_result = stringz.concat(markdown_result, "|")
        markdown_result = stringz.concat(markdown_result, lines[i])
        markdown_result = stringz.concat(markdown_result, "|")
    }
    
    damn markdown_result
}
