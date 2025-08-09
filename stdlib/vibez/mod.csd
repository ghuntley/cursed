fr fr CURSED Enhanced VIBEZ Module - Essential I/O Operations
fr fr Pure CURSED implementation for maximum compatibility

fr fr ===== CORE OUTPUT FUNCTIONS =====

slay spill(msg tea) lit {
    vibez.spill(msg)
    damn based
}

slay spill_two(msg1 tea, msg2 tea) lit {
    vibez.spill(msg1, msg2)
    damn based
}

slay spill_three(msg1 tea, msg2 tea, msg3 tea) lit {
    vibez.spill(msg1, msg2, msg3)
    damn based
}

slay spillln(msg tea) lit {
    vibez.spill(msg)
    vibez.spill("")
    damn based
}

fr fr ===== CONSOLE FORMATTING =====

slay print_header(title tea) lit {
    vibez.spill("=== ", title, " ===")
    damn based
}

slay print_separator() lit {
    vibez.spill("--------------------------------")
    damn based
}

slay print_success(msg tea) lit {
    vibez.spill("✅ SUCCESS: ", msg)
    damn based
}

slay print_error(msg tea) lit {
    vibez.spill("❌ ERROR: ", msg)
    damn based
}

slay print_warning(msg tea) lit {
    vibez.spill("⚠️ WARNING: ", msg)
    damn based
}

slay print_info(msg tea) lit {
    vibez.spill("ℹ️ INFO: ", msg)
    damn based
}

fr fr ===== NUMBERED OUTPUT =====

slay print_numbered_item(number drip, item tea) lit {
    vibez.spill(number, ". ", item)
    damn based
}

slay print_result(label tea, value tea) lit {
    vibez.spill(label, ": ", value)
    damn based
}

fr fr ===== DEBUG OUTPUT =====

slay debug_print(msg tea) lit {
    vibez.spill("[DEBUG] ", msg)
    damn based
}

slay trace_print(function_name tea, msg tea) lit {
    vibez.spill("[TRACE] ", function_name, ": ", msg)
    damn based
}
