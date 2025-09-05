yeet "testz"
yeet "tab_aesthetic"

fr fr Tab Aesthetic Module Tests

test_start("tab_writer_new creates default writer")
sus writer TabWriter = tab_writer_new()
assert_eq_string(writer.alignment, "left")
assert_eq_string(writer.separator, "\t")
assert_eq_int(writer.padding, 1)

test_start("tab_writer_with_separator sets custom separator")
sus writer TabWriter = tab_writer_with_separator(",")
assert_eq_string(writer.separator, ",")

test_start("tab_writer_with_alignment sets custom alignment")
sus writer TabWriter = tab_writer_with_alignment("right")
assert_eq_string(writer.alignment, "right")

test_start("tab_writer_set_alignment updates alignment")
sus writer TabWriter = tab_writer_new()
assert_true(tab_writer_set_alignment(&writer, "center"))
assert_eq_string(writer.alignment, "center")

test_start("tab_writer_set_separator updates separator")
sus writer TabWriter = tab_writer_new()
assert_true(tab_writer_set_separator(&writer, "|"))
assert_eq_string(writer.separator, "|")

test_start("tab_writer_set_padding updates padding")
sus writer TabWriter = tab_writer_new()
assert_true(tab_writer_set_padding(&writer, 3))
assert_eq_int(writer.padding, 3)

test_start("tab_writer_join_row joins with separator")
sus row tea[value] = tea[value]{"Name", "Age", "City"}
sus result tea = tab_writer_join_row(row, "\t")
assert_eq_string(result, "Name\tAge\tCity")

test_start("tab_writer_join_row handles empty array")
sus empty_row tea[value] = tea[value]{}
sus result tea = tab_writer_join_row(empty_row, "\t")
assert_eq_string(result, "")

test_start("tab_writer_pad_string left alignment")
sus result tea = tab_writer_pad_string("test", 8, "left")
assert_eq_string(result, "test    ")

test_start("tab_writer_pad_string right alignment")
sus result tea = tab_writer_pad_string("test", 8, "right")
assert_eq_string(result, "    test")

test_start("tab_writer_pad_string center alignment")
sus result tea = tab_writer_pad_string("test", 8, "center")
assert_eq_string(result, "  test  ")

test_start("tab_writer_pad_string no padding needed")
sus result tea = tab_writer_pad_string("testing", 5, "left")
assert_eq_string(result, "testing")

test_start("tab_writer_add_row adds data and updates widths")
sus writer TabWriter = tab_writer_new()
sus row1 tea[value] = tea[value]{"Name", "Age"}
sus row2 tea[value] = tea[value]{"John", "25"}
assert_true(tab_writer_add_row(&writer, row1))
assert_true(tab_writer_add_row(&writer, row2))
assert_eq_int(len(writer.widths), 2)
assert_eq_int(writer.widths[0], 4) fr fr "Name" length
assert_eq_int(writer.widths[1], 3) fr fr "Age" length

test_start("tab_writer_add_row updates column widths correctly")
sus writer TabWriter = tab_writer_new()
sus row1 tea[value] = tea[value]{"A", "BB"}
sus row2 tea[value] = tea[value]{"CCC", "D"}
assert_true(tab_writer_add_row(&writer, row1))
assert_true(tab_writer_add_row(&writer, row2))
assert_eq_int(writer.widths[0], 3) fr fr Max of "A" and "CCC"
assert_eq_int(writer.widths[1], 2) fr fr Max of "BB" and "D"

test_start("tab_writer_clear resets writer")
sus writer TabWriter = tab_writer_new()
sus row tea[value] = tea[value]{"test", "data"}
tab_writer_add_row(&writer, row)
assert_true(tab_writer_clear(&writer))
assert_eq_int(len(writer.columns), 0)
assert_eq_int(len(writer.widths), 0)

test_start("tab_writer_flush formats simple table")
sus writer TabWriter = tab_writer_new()
sus row1 tea[value] = tea[value]{"Name", "Age"}
sus row2 tea[value] = tea[value]{"John", "25"}
tab_writer_add_row(&writer, row1)
tab_writer_add_row(&writer, row2)
sus result tea = tab_writer_flush(&writer)
fr fr Should contain formatted table with proper spacing
assert_true(len(result) > 0)

test_start("tab_aesthetic_format_table creates aligned table")
sus data tea[value][value] = tea[value][value]{
    tea[value]{"Name", "Age", "City"},
    tea[value]{"John", "25", "NYC"},
    tea[value]{"Jane", "30", "LA"}
}
sus result tea = tab_aesthetic_format_table(data, "left")
assert_true(len(result) > 0)

test_start("tab_aesthetic_align_columns aligns single column")
sus columns tea[value] = tea[value]{"Short", "Medium", "Very Long Text"}
sus result tea = tab_aesthetic_align_columns(columns, "left")
assert_true(len(result) > 0)

test_start("tab_aesthetic_table_with_headers creates table with headers")
sus headers tea[value] = tea[value]{"Name", "Score"}
sus rows tea[value][value] = tea[value][value]{
    tea[value]{"Alice", "95"},
    tea[value]{"Bob", "87"}
}
sus result tea = tab_aesthetic_table_with_headers(headers, rows, "center")
assert_true(len(result) > 0)

test_start("tab_aesthetic_quick_table uses default formatting")
sus data tea[value][value] = tea[value][value]{
    tea[value]{"Item", "Price"},
    tea[value]{"Apple", "$1.50"},
    tea[value]{"Orange", "$2.00"}
}
sus result tea = tab_aesthetic_quick_table(data)
assert_true(len(result) > 0)

test_start("tab_aesthetic_key_value_table formats key-value pairs")
sus keys tea[value] = tea[value]{"Name", "Version", "Author"}
sus values tea[value] = tea[value]{"CURSED", "1.0", "Developer"}
sus result tea = tab_aesthetic_key_value_table(keys, values)
assert_true(len(result) > 0)

test_start("tab_aesthetic_key_value_table handles mismatched arrays")
sus keys tea[value] = tea[value]{"Key1", "Key2"}
sus values tea[value] = tea[value]{"Value1"}
sus result tea = tab_aesthetic_key_value_table(keys, values)
assert_eq_string(result, "Error: keys and values arrays must have same length")

test_start("tab_aesthetic_csv_style creates CSV formatting")
sus data tea[value][value] = tea[value][value]{
    tea[value]{"Name", "Age", "Email"},
    tea[value]{"John", "25", "john@example.com"},
    tea[value]{"Jane", "30", "jane@example.com"}
}
sus result tea = tab_aesthetic_csv_style(data)
assert_true(len(result) > 0)

test_start("tab_aesthetic_markdown_table creates markdown table")
sus headers tea[value] = tea[value]{"Feature", "Status"}
sus rows tea[value][value] = tea[value][value]{
    tea[value]{"Parser", "Complete"},
    tea[value]{"Compiler", "Beta"}
}
sus result tea = tab_aesthetic_markdown_table(headers, rows)
assert_true(len(result) > 0)
fr fr Should contain markdown table borders
assert_true(stringz.contains(result, "|"))

test_start("tab_writer handles different alignments")
sus writer_left TabWriter = tab_writer_with_alignment("left")
sus writer_right TabWriter = tab_writer_with_alignment("right")
sus writer_center TabWriter = tab_writer_with_alignment("center")

sus row tea[value] = tea[value]{"Test", "Data"}
tab_writer_add_row(&writer_left, row)
tab_writer_add_row(&writer_right, row)
tab_writer_add_row(&writer_center, row)

sus result_left tea = tab_writer_flush(&writer_left)
sus result_right tea = tab_writer_flush(&writer_right)
sus result_center tea = tab_writer_flush(&writer_center)

assert_true(len(result_left) > 0)
assert_true(len(result_right) > 0)
assert_true(len(result_center) > 0)

test_start("tab_writer handles custom separators")
sus writer_tab TabWriter = tab_writer_with_separator("\t")
sus writer_pipe TabWriter = tab_writer_with_separator("|")
sus writer_comma TabWriter = tab_writer_with_separator(",")

sus row tea[value] = tea[value]{"Col1", "Col2"}
tab_writer_add_row(&writer_tab, row)
tab_writer_add_row(&writer_pipe, row)
tab_writer_add_row(&writer_comma, row)

sus result_tab tea = tab_writer_flush(&writer_tab)
sus result_pipe tea = tab_writer_flush(&writer_pipe)
sus result_comma tea = tab_writer_flush(&writer_comma)

assert_true(len(result_tab) > 0)
assert_true(len(result_pipe) > 0)
assert_true(len(result_comma) > 0)

test_start("tab_writer handles varying column counts")
sus writer TabWriter = tab_writer_new()
sus row1 tea[value] = tea[value]{"A"}
sus row2 tea[value] = tea[value]{"B", "C"}
sus row3 tea[value] = tea[value]{"D", "E", "F"}

tab_writer_add_row(&writer, row1)
tab_writer_add_row(&writer, row2)
tab_writer_add_row(&writer, row3)

sus result tea = tab_writer_flush(&writer)
assert_true(len(result) > 0)

test_start("tab_aesthetic module comprehensive test")
fr fr Test complex table with multiple formatting options
sus complex_data tea[value][value] = tea[value][value]{
    tea[value]{"Language", "Type", "Year", "Popular"},
    tea[value]{"CURSED", "Compiled", "2024", "Growing"},
    tea[value]{"Go", "Compiled", "2009", "High"},
    tea[value]{"Python", "Interpreted", "1991", "Very High"},
    tea[value]{"Rust", "Compiled", "2010", "Growing"}
}

sus left_aligned tea = tab_aesthetic_format_table(complex_data, "left")
sus right_aligned tea = tab_aesthetic_format_table(complex_data, "right")
sus center_aligned tea = tab_aesthetic_format_table(complex_data, "center")

assert_true(len(left_aligned) > 0)
assert_true(len(right_aligned) > 0)
assert_true(len(center_aligned) > 0)

fr fr Test key-value formatting
sus config_keys tea[value] = tea[value]{"Compiler", "Version", "Target", "Optimization"}
sus config_values tea[value] = tea[value]{"CURSED", "1.0.0", "x86_64", "Enabled"}
sus config_table tea = tab_aesthetic_key_value_table(config_keys, config_values)
assert_true(len(config_table) > 0)

fr fr Test markdown table generation
sus feature_headers tea[value] = tea[value]{"Module", "Status", "Tests", "Coverage"}
sus feature_rows tea[value][value] = tea[value][value]{
    tea[value]{"Parser", "Complete", "125", "98%"},
    tea[value]{"Compiler", "Complete", "89", "95%"},
    tea[value]{"Runtime", "Complete", "67", "92%"},
    tea[value]{"Stdlib", "Complete", "156", "97%"}
}
sus markdown_table tea = tab_aesthetic_markdown_table(feature_headers, feature_rows)
assert_true(len(markdown_table) > 0)
assert_true(stringz.contains(markdown_table, "|"))

print_test_summary()
