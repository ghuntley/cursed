yeet "tab_aesthetic"

vibez.spill("=== Tab Aesthetic Module Demo ===")

# Demo 1: Basic table formatting
vibez.spill("\n1. Basic Table Formatting:")
sus data [][]tea = [][]tea{
    []tea{"Language", "Type", "Year", "Popular"},
    []tea{"CURSED", "Compiled", "2024", "Growing"},
    []tea{"Go", "Compiled", "2009", "High"},
    []tea{"Python", "Interpreted", "1991", "Very High"}
}

sus table tea = tab_aesthetic_format_table(data, "left")
vibez.spill(table)

# Demo 2: Key-value configuration table
vibez.spill("\n2. Configuration Table:")
sus config_keys []tea = []tea{"Compiler", "Version", "Target", "Optimization"}
sus config_values []tea = []tea{"CURSED", "1.0.0", "x86_64", "Enabled"}
sus config_table tea = tab_aesthetic_key_value_table(config_keys, config_values)
vibez.spill(config_table)

# Demo 3: Right-aligned table
vibez.spill("\n3. Right-Aligned Table:")
sus price_data [][]tea = [][]tea{
    []tea{"Item", "Price", "Stock"},
    []tea{"Apple", "$1.50", "50"},
    []tea{"Orange", "$2.00", "25"},
    []tea{"Banana", "$0.75", "100"}
}
sus right_table tea = tab_aesthetic_format_table(price_data, "right")
vibez.spill(right_table)

# Demo 4: Markdown table
vibez.spill("\n4. Markdown Table:")
sus headers []tea = []tea{"Module", "Status", "Coverage"}
sus rows [][]tea = [][]tea{
    []tea{"Parser", "Complete", "98%"},
    []tea{"Compiler", "Complete", "95%"},
    []tea{"Runtime", "Complete", "92%"}
}
sus markdown tea = tab_aesthetic_markdown_table(headers, rows)
vibez.spill(markdown)

vibez.spill("\n=== Demo Complete ===")
