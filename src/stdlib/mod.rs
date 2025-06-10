/// Standard library for CURSED
pub mod dot_registry;
pub mod packages;
pub mod web_vibez;
pub mod http_core;
pub mod database;
pub mod crypto;
pub mod template;
pub mod value;
pub mod fs;
pub mod io;
pub mod string;

// Database package re-exports for easy access
pub use database::llvm_integration::{
    DatabaseLLVMIntegration, DatabaseLLVMIntegrationImpl, 
    register_database_functions
};

// Crypto package re-exports for easy access
// TODO: Re-enable when crypto packages are fully implemented
// pub use crypto::{
//     CryptoPlatform, JwtHandler, HmacAuth, TotpGenerator, TlsHandshake,
//     SecureRandom, UuidV4Generator, SaltGenerator, NonceGenerator,
//     Base64Encoder, HexEncoder, Base32Encoder, Asn1Parser,
//     CryptoLLVMIntegration, CryptoLLVMIntegrationImpl, register_crypto_functions
// };

// Package re-exports
pub use packages::{
    db_core, db_pool, sql_vibes, db_migrate, db_orm, db_nosql, db_query
};

// Template system re-exports
pub use template::{
    TemplateEngine, TemplateContext, TemplateConfig, TemplateLoader,
    TemplateRenderer, FilterRegistry, TemplateCache, WebTemplateRenderer,
    TemplateFormat, TemplateFormatRenderer, HtmlTemplateContext, HtmlEscaper
};

// File system re-exports
pub use fs::{
    FsError, FsResult, FileMetadata, DirEntry,
    read_file, write_file, append_file, delete_file, copy_file, move_file,
    create_dir, create_dir_all, remove_dir, remove_dir_all, list_dir,
    exists, is_file, is_dir, file_size, metadata,
    join_path, parent_dir, file_name, extension, absolute_path,
    read_text_file_safe, write_text_file_safe
};

// Console I/O re-exports
pub use io::{
    IoError, IoResult, system_error, io_error, invalid_input,
    Stdin, Stdout, Stderr, stdin, stdout, stderr, flush_all,
    print, println, eprint, eprintln, printf, printfln, eprintf, eprintfln,
    read_line, read_char, read_until, read_all, flush,
    prompt, confirm, select, multi_select, read_password, paginate, ProgressBar,
    BufferedReader, BufferedWriter, SharedBufferedReader, SharedBufferedWriter,
    buffered_stdin, buffered_stdout, buffered_stderr,
    shared_buffered_stdin, shared_buffered_stdout, shared_buffered_stderr
};

// String manipulation re-exports
pub use string::{
    StringError, StringResult,
    // Core operations
    length, is_empty, concat, concat_owned, repeat, reverse, char_at, chars, bytes, is_ascii,
    from_utf8, from_utf8_lossy,
    // Search and replace
    contains, starts_with, ends_with, find, find_last, find_all, replace, replace_first,
    replace_last, replace_n, count_occurrences, contains_ignore_case, find_ignore_case,
    // Transformations
    substring, substring_range, trim, trim_start, trim_end, trim_chars, trim_start_chars,
    trim_end_chars, to_lowercase, to_uppercase, to_title_case, to_camel_case, to_pascal_case,
    to_snake_case, to_kebab_case, insert_at, remove_range,
    // Splitting and joining
    split, split_n, rsplit, rsplit_n, split_lines, split_whitespace, split_any, split_by,
    join, join_owned, join_with_separators, partition, rpartition, chunk, split_into_n_parts,
    // Validation
    is_numeric, is_integer, is_alphabetic, is_alphanumeric, is_whitespace, is_uppercase,
    is_lowercase, is_title_case, is_hex, is_email, is_url, is_phone_number,
    has_balanced_parentheses, has_balanced_brackets, is_palindrome,
    // Formatting
    pad_left, pad_right, center, truncate, wrap_text, format_columns, auto_detect_column_widths,
    format_table, add_line_numbers, indent_lines, dedent, escape_html, escape_json, escape_csv
};

pub use dot_registry::DOT_REGISTRY;
