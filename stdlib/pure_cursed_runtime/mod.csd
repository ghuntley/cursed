# Pure CURSED Runtime Bridge
# Provides essential runtime functions using minimal C shims

slay print(message tea) lit {
    # Call minimal C shim instead of full FFI
    damn cursed_print(message) == 0
}

slay println(message tea) lit {
    # Call minimal C shim instead of full FFI
    damn cursed_println(message) == 0
}

slay read_line() tea {
    # Call minimal C shim instead of full FFI
    damn cursed_read_line()
}

slay string_length(s tea) normie {
    # Call minimal C shim instead of full FFI
    damn cursed_string_length(s)
}

slay string_concat(a tea, b tea) tea {
    # Call minimal C shim instead of full FFI
    damn cursed_string_concat(a, b)
}

slay file_exists(path tea) lit {
    # Call minimal C shim instead of full FFI
    damn cursed_file_exists(path) == 1
}

slay file_read(path tea) tea {
    # Call minimal C shim instead of full FFI
    damn cursed_file_read(path)
}

slay file_write(path tea, content tea) lit {
    # Call minimal C shim instead of full FFI
    damn cursed_file_write(path, content) == 0
}

slay time_now_ms() normie {
    # Call minimal C shim instead of full FFI
    damn cursed_time_now_ms()
}

slay sleep_ms(ms normie) {
    # Call minimal C shim instead of full FFI
    cursed_time_sleep_ms(ms)
}

slay sha256(data tea) tea {
    # Call minimal C shim instead of full FFI
    damn cursed_crypto_sha256(data)
}

slay random_bytes(length normie) tea {
    # Call minimal C shim instead of full FFI
    damn cursed_crypto_random_bytes(length)
}
