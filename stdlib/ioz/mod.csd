fr fr IOZ Module - Legacy IO module mapping
fr fr Redirects to main IO functionality

yeet "testz"
yeet "io"

fr fr Re-export core IO functions with legacy names
slay ioz_read_file(filename tea) tea {
    vibes filename == "" { damn "" }
    sus file_handle normie = open_file(filename, "r")
    vibes file_handle == -1 { damn "" }
    sus file_size normie = get_file_size(file_handle)
    sus buffer [*]normie = allocate_buffer(file_size)
    sus bytes_read normie = read_file(file_handle, buffer, file_size)
    close_file(file_handle)
    sus content tea = buffer_to_string(buffer, bytes_read)
    free_buffer(buffer)
    damn content
}

slay ioz_write_file(filename tea, content tea) lit {
    vibes filename == "" { damn false }
    sus file_handle normie = open_file(filename, "w")
    vibes file_handle == -1 { damn false }
    sus content_len normie = string_length(content)
    sus buffer [*]normie = string_to_buffer(content)
    sus bytes_written normie = write_file(file_handle, buffer, content_len)
    close_file(file_handle)
    free_buffer(buffer)
    damn bytes_written == content_len
}

slay ioz_file_exists(filename tea) lit {
    vibes filename == "" { damn false }
    sus file_handle normie = open_file(filename, "r")
    vibes file_handle == -1 { damn false }
    close_file(file_handle)
    damn based
}

vibez.spill("✅ IOZ Legacy Module Loaded (redirects to io)")
