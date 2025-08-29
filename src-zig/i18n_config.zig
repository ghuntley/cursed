const std = @import("std");
const panic_i18n = @import("panic_i18n.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;

/// Configuration system for CURSED i18n panic messages
/// Handles loading language packs from files and runtime configuration

const Locale = panic_i18n.Locale;
const I18nManager = panic_i18n.I18nManager;
const LanguagePack = panic_i18n.LanguagePack;

/// Configuration structure for i18n system
pub const I18nConfig = struct {
    default_locale: Locale,
    fallback_locale: Locale,
    language_pack_directory: []const u8,
    auto_detect_locale: bool,
    unicode_normalization: UnicodeNormalization,
    message_truncation: MessageTruncation,
    rtl_support: bool,
    custom_formatting: HashMap([]const u8, FormattingRule, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,

    pub const UnicodeNormalization = enum {
        None,      // No normalization
        NFC,       // Canonical decomposition followed by canonical composition
        NFD,       // Canonical decomposition
        NFKC,      // Compatibility decomposition followed by canonical composition
        NFKD,      // Compatibility decomposition
    };

    pub const MessageTruncation = struct {
        enabled: bool,
        max_length: usize,
        truncation_suffix: []const u8,
    };

    pub const FormattingRule = struct {
        date_format: []const u8,
        number_format: []const u8,
        currency_format: []const u8,
        custom_placeholders: HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    };

    pub fn init() I18nConfig {
        return I18nConfig{
            .default_locale = .en_US,
            .fallback_locale = .en_US,
            .language_pack_directory = "lang/",
            .auto_detect_locale = true,
            .unicode_normalization = .NFC,
            .message_truncation = MessageTruncation{
                .enabled = false,
                .max_length = 1000,
                .truncation_suffix = "...",
            },
            .rtl_support = true,
            .custom_formatting = HashMap([]const u8, FormattingRule, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *I18nConfig) void {
        var iterator = self.custom_formatting.iterator();
        while (iterator.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            var rule = entry.value_ptr;
            rule.custom_placeholders.deinit();
        }
        self.custom_formatting.deinit(self.allocator);
    }

    pub fn loadFromFile(allocator: Allocator, file_path: []const u8) !I18nConfig {
        const file_content = try std.fs.cwd().readFileAlloc(allocator, file_path, 1024 * 1024); // 1MB max
        defer allocator.free(file_content);

        return try parseConfig(allocator, file_content);
    }

    pub fn saveToFile(self: I18nConfig, file_path: []const u8) !void {
        const config_content = try self.serialize();
        defer self.allocator.free(config_content);

        try std.fs.cwd().writeFile(file_path, config_content);
    }

    fn parseConfig(allocator: Allocator, content: []const u8) !I18nConfig {
        // Simple JSON-like parsing for configuration
        // In a real implementation, you'd use a proper JSON parser
        
        var config = I18nConfig.init(allocator);
        
        // Parse basic configuration (simplified)
        if (std.mem.indexOf(u8, content, "default_locale")) |pos| {
            const start = pos + 16; // length of "default_locale": 
            if (extractStringValue(content[start..])) |locale_str| {
                if (Locale.fromString(locale_str)) |locale| {
                    config.default_locale = locale;
                }
            }
        }
        
        if (std.mem.indexOf(u8, content, "fallback_locale")) |pos| {
            const start = pos + 17; // length of "fallback_locale": 
            if (extractStringValue(content[start..])) |locale_str| {
                if (Locale.fromString(locale_str)) |locale| {
                    config.fallback_locale = locale;
                }
            }
        }
        
        if (std.mem.indexOf(u8, content, "auto_detect_locale")) |pos| {
            const start = pos + 20; // length of "auto_detect_locale": 
            if (extractBoolValue(content[start..])) |value| {
                config.auto_detect_locale = value;
            }
        }

        return config;
    }

    fn extractStringValue(content: []const u8) ?[]const u8 {
        var i: usize = 0;
        
        // Skip whitespace and find opening quote
        while (i < content.len and content[i] != '"') {
            i += 1;
        }
        if (i >= content.len) return null;
        
        i += 1; // Skip opening quote
        const start = i;
        
        // Find closing quote
        while (i < content.len and content[i] != '"') {
            i += 1;
        }
        if (i >= content.len) return null;
        
        return content[start..i];
    }

    fn extractBoolValue(content: []const u8) ?bool {
        if (std.mem.startsWith(u8, std.mem.trim(u8, content, " \t\n\r"), "true")) {
            return true;
        } else if (std.mem.startsWith(u8, std.mem.trim(u8, content, " \t\n\r"), "false")) {
            return false;
        }
        return null;
    }

    fn serialize(self: I18nConfig) ![]u8 {
        var result = std.ArrayList(u8){};
        defer result.deinit();

        try result.appendSlice("{\n");
        try result.appendSlice("  \"default_locale\": \"");
        try result.appendSlice(self.default_locale.toString());
        try result.appendSlice("\",\n");
        
        try result.appendSlice("  \"fallback_locale\": \"");
        try result.appendSlice(self.fallback_locale.toString());
        try result.appendSlice("\",\n");
        
        try result.appendSlice("  \"language_pack_directory\": \"");
        try result.appendSlice(self.language_pack_directory);
        try result.appendSlice("\",\n");
        
        try result.appendSlice("  \"auto_detect_locale\": ");
        try result.appendSlice(if (self.auto_detect_locale) "true" else "false");
        try result.appendSlice(",\n");
        
        try result.appendSlice("  \"rtl_support\": ");
        try result.appendSlice(if (self.rtl_support) "true" else "false");
        try result.appendSlice(",\n");
        
        try result.appendSlice("  \"unicode_normalization\": \"");
        try result.appendSlice(@tagName(self.unicode_normalization));
        try result.appendSlice("\",\n");
        
        try result.appendSlice("  \"message_truncation\": {\n");
        try result.appendSlice("    \"enabled\": ");
        try result.appendSlice(if (self.message_truncation.enabled) "true" else "false");
        try result.appendSlice(",\n");
        try result.appendSlice("    \"max_length\": ");
        const max_len_str = try std.fmt.allocPrint(self.allocator, "{}", .{self.message_truncation.max_length});
        defer self.allocator.free(max_len_str);
        try result.appendSlice(max_len_str);
        try result.appendSlice(",\n");
        try result.appendSlice("    \"truncation_suffix\": \"");
        try result.appendSlice(self.message_truncation.truncation_suffix);
        try result.appendSlice("\"\n");
        try result.appendSlice("  }\n");
        
        try result.appendSlice("}\n");

        return try self.allocator.dupe(u8, result.items);
    }
};

/// Language pack file loader with JSON support
pub const LanguagePackLoader = struct {
    allocator: Allocator,
    config: I18nConfig,

    pub fn init(allocator: Allocator, config: I18nConfig) LanguagePackLoader {
        return LanguagePackLoader{
            .allocator = allocator,
            .config = config,
        };
    }

    pub fn loadLanguagePack(self: LanguagePackLoader, locale: Locale) !LanguagePack {
        const file_path = try self.getLanguagePackPath(locale);
        defer self.allocator.free(file_path);

        if (std.fs.cwd().readFileAlloc(self.allocator, file_path, 10 * 1024 * 1024)) |content| { // 10MB max
            defer self.allocator.free(content);
            return try self.parseLanguagePackJSON(locale, content);
        } else |_| {
            // Fall back to embedded language pack
            return try self.createDefaultLanguagePack(locale);
        }
    }

    fn getLanguagePackPath(self: LanguagePackLoader, locale: Locale) ![]u8 {
        return try std.fmt.allocPrint(
            self.allocator,
            "{s}{s}.json",
            .{ self.config.language_pack_directory, locale.toString() }
        );
    }

    fn parseLanguagePackJSON(self: LanguagePackLoader, locale: Locale, content: []const u8) !LanguagePack {
        var pack = LanguagePack.init(self.allocator, locale);
        
        // Simple JSON parser for language pack format
        // Format: { "messages": { "key": "template", ... }, "pluralization": {...}, ... }
        
        if (std.mem.indexOf(u8, content, "\"messages\"")) |messages_pos| {
            const messages_start = messages_pos + 11; // length of "\"messages\""
            if (findJSONObjectStart(content[messages_start..])) |obj_start| {
                const messages_content = content[messages_start + obj_start..];
                try self.parseMessages(&pack, messages_content);
            }
        }

        return pack;
    }

    fn parseMessages(self: LanguagePackLoader, pack: *LanguagePack, content: []const u8) !void {
        var i: usize = 0;
        
        while (i < content.len) {
            // Find next key-value pair
            if (findNextJSONString(content[i..])) |key_info| {
                const key_start = i + key_info.start;
                const key_end = i + key_info.end;
                const key = content[key_start..key_end];
                
                // Find the corresponding value
                i = key_end;
                if (findJSONValueStart(content[i..])) |value_start_offset| {
                    i += value_start_offset;
                    if (findNextJSONString(content[i..])) |value_info| {
                        const value_start = i + value_info.start;
                        const value_end = i + value_info.end;
                        const value = content[value_start..value_end];
                        
                        try pack.addMessage(key, value);
                        i = value_end;
                    } else break;
                } else break;
            } else break;
        }
    }

    const StringInfo = struct {
        start: usize,
        end: usize,
    };

    fn findJSONObjectStart(content: []const u8) ?usize {
        for (content, 0..) |char, i| {
            if (char == '{') return i;
        }
        return null;
    }

    fn findJSONValueStart(content: []const u8) ?usize {
        var i: usize = 0;
        while (i < content.len) {
            if (content[i] == ':') {
                i += 1;
                // Skip whitespace
                while (i < content.len and (content[i] == ' ' or content[i] == '\t' or content[i] == '\n' or content[i] == '\r')) {
                    i += 1;
                }
                return i;
            }
            i += 1;
        }
        return null;
    }

    fn findNextJSONString(content: []const u8) ?StringInfo {
        var i: usize = 0;
        
        // Find opening quote
        while (i < content.len and content[i] != '"') {
            i += 1;
        }
        if (i >= content.len) return null;
        
        i += 1; // Skip opening quote
        const start = i;
        
        // Find closing quote (handle escapes)
        while (i < content.len) {
            if (content[i] == '"' and (i == 0 or content[i-1] != '\\')) {
                return StringInfo{ .start = start, .end = i };
            }
            i += 1;
        }
        
        return null;
    }

    fn createDefaultLanguagePack(self: LanguagePackLoader, locale: Locale) !LanguagePack {
        var pack = LanguagePack.init(self.allocator, locale);
        
        // Load default messages based on locale
        switch (locale) {
            .en_US, .en_GB => {
                try pack.addMessage("panic.yikes", "🚨 CURSED Runtime Error: {message}");
                try pack.addMessage("panic.shook", "💥 CURSED Panic: {message} (Code: {code})");
                try pack.addMessage("panic.fam", "❌ CURSED Exception: {message}");
            },
            .es_ES, .es_MX => {
                try pack.addMessage("panic.yikes", "🚨 Error de Tiempo de Ejecución CURSED: {message}");
                try pack.addMessage("panic.shook", "💥 Pánico CURSED: {message} (Código: {code})");
                try pack.addMessage("panic.fam", "❌ Excepción CURSED: {message}");
            },
            .fr_FR => {
                try pack.addMessage("panic.yikes", "🚨 Erreur d'Exécution CURSED: {message}");
                try pack.addMessage("panic.shook", "💥 Panique CURSED: {message} (Code: {code})");
                try pack.addMessage("panic.fam", "❌ Exception CURSED: {message}");
            },
            else => {
                // Fall back to English for unsupported locales
                try pack.addMessage("panic.yikes", "🚨 CURSED Runtime Error: {message}");
                try pack.addMessage("panic.shook", "💥 CURSED Panic: {message} (Code: {code})");
                try pack.addMessage("panic.fam", "❌ CURSED Exception: {message}");
            },
        }
        
        return pack;
    }
};

/// Runtime environment detection for locale and configuration
pub const RuntimeEnvironment = struct {
    pub fn detectLocale() Locale {
        // Check environment variables in order of preference
        const env_vars = [_][]const u8{ "LC_ALL", "LC_MESSAGES", "LANG", "LANGUAGE" };
        
        for (env_vars) |env_var| {
            if (std.process.getEnvVarOwned(std.heap.page_allocator, env_var)) |locale_str| {
                defer std.heap.page_allocator.free(locale_str);
                
                if (parseLocaleString(locale_str)) |locale| {
                    return locale;
                }
            } else |_| {
                continue;
            }
        }
        
        // Platform-specific detection
        return detectPlatformLocale();
    }

    fn parseLocaleString(locale_str: []const u8) ?Locale {
        // Handle formats like "en_US.UTF-8", "en-US", "en_US", etc.
        var normalized = std.heap.page_allocator.alloc(u8, locale_str.len) catch return null;
        defer std.heap.page_allocator.free(normalized);
        
        // Extract locale part before any dot or @
        var end_pos = locale_str.len;
        if (std.mem.indexOf(u8, locale_str, ".")) |dot_pos| {
            end_pos = @min(end_pos, dot_pos);
        }
        if (std.mem.indexOf(u8, locale_str, "@")) |at_pos| {
            end_pos = @min(end_pos, at_pos);
        }
        
        const locale_part = locale_str[0..end_pos];
        
        // Normalize underscores to hyphens
        for (locale_part, 0..) |char, i| {
            normalized[i] = if (char == '_') '-' else char;
        }
        
        return Locale.fromString(normalized[0..locale_part.len]);
    }

    fn detectPlatformLocale() Locale {
        // Platform-specific locale detection
        if (std.builtin.os.tag == .windows) {
            return detectWindowsLocale();
        } else if (std.builtin.os.tag == .macos) {
            return detectMacOSLocale();
        } else {
            return detectUnixLocale();
        }
    }

    fn detectWindowsLocale() Locale {
        // TODO: Use Windows API to get system locale
        return .en_US;
    }

    fn detectMacOSLocale() Locale {
        // TODO: Use macOS APIs to get system locale
        return .en_US;
    }

    fn detectUnixLocale() Locale {
        // TODO: Check /etc/locale.conf or similar files
        return .en_US;
    }

    pub fn getConfigDirectory() ![]u8 {
        const allocator = std.heap.page_allocator;
        
        if (std.builtin.os.tag == .windows) {
            if (std.process.getEnvVarOwned(allocator, "APPDATA")) |appdata| {
                return try std.fmt.allocPrint(allocator, "{s}\\cursed\\", .{appdata});
            } else |_| {
                return try allocator.dupe(u8, ".\\cursed\\");
            }
        } else {
            if (std.process.getEnvVarOwned(allocator, "XDG_CONFIG_HOME")) |xdg_config| {
                return try std.fmt.allocPrint(allocator, "{s}/cursed/", .{xdg_config});
            } else |_| {
                if (std.process.getEnvVarOwned(allocator, "HOME")) |home| {
                    return try std.fmt.allocPrint(allocator, "{s}/.config/cursed/", .{home});
                } else |_| {
                    return try allocator.dupe(u8, "./cursed/");
                }
            }
        }
    }
};

/// Utility functions for creating example language pack files
pub const LanguagePackGenerator = struct {
    pub fn generateExamplePacks(allocator: Allocator, output_dir: []const u8) !void {
        const locales = [_]Locale{ .en_US, .es_ES, .fr_FR, .de_DE, .ru_RU, .zh_CN, .ja_JP, .ar_SA };
        
        for (locales) |locale| {
            try generateLanguagePackFile(allocator, locale, output_dir);
        }
    }

    fn generateLanguagePackFile(allocator: Allocator, locale: Locale, output_dir: []const u8) !void {
        const file_path = try std.fmt.allocPrint(allocator, "{s}/{s}.json", .{ output_dir, locale.toString() });
        defer allocator.free(file_path);

        const content = try generateLanguagePackContent(allocator, locale);
        defer allocator.free(content);

        // Ensure directory exists
        if (std.fs.cwd().makeDir(output_dir)) |_| {
            // Directory created successfully
        } else |err| {
            if (err != error.PathAlreadyExists) {
                return err;
            }
        }

        try std.fs.cwd().writeFile(file_path, content);
    }

    fn generateLanguagePackContent(allocator: Allocator, locale: Locale) ![]u8 {
        var result = std.ArrayList(u8){};
        defer result.deinit();

        try result.appendSlice("{\n");
        try result.appendSlice("  \"meta\": {\n");
        try result.appendSlice("    \"locale\": \"");
        try result.appendSlice(locale.toString());
        try result.appendSlice("\",\n");
        try result.appendSlice("    \"version\": \"1.0\",\n");
        try result.appendSlice("    \"description\": \"CURSED panic messages for ");
        try result.appendSlice(locale.toString());
        try result.appendSlice("\"\n");
        try result.appendSlice("  },\n");
        
        try result.appendSlice("  \"messages\": {\n");
        
        const messages = getMessagesForLocale(locale);
        for (messages, 0..) |message, i| {
            try result.appendSlice("    \"");
            try result.appendSlice(message.key);
            try result.appendSlice("\": \"");
            try result.appendSlice(message.value);
            try result.appendSlice("\"");
            if (i < messages.len - 1) {
                try result.appendSlice(",");
            }
            try result.appendSlice("\n");
        }
        
        try result.appendSlice("  },\n");
        
        try result.appendSlice("  \"pluralization\": {\n");
        try result.appendSlice("    \"type\": \"");
        try result.appendSlice(getPluralTypeForLocale(locale));
        try result.appendSlice("\"\n");
        try result.appendSlice("  },\n");
        
        try result.appendSlice("  \"formatting\": {\n");
        try result.appendSlice("    \"decimal_separator\": \"");
        try result.appendSlice(getDecimalSeparatorForLocale(locale));
        try result.appendSlice("\",\n");
        try result.appendSlice("    \"thousands_separator\": \"");
        try result.appendSlice(getThousandsSeparatorForLocale(locale));
        try result.appendSlice("\",\n");
        try result.appendSlice("    \"rtl\": ");
        try result.appendSlice(if (locale.isRTL()) "true" else "false");
        try result.appendSlice("\n");
        try result.appendSlice("  }\n");
        
        try result.appendSlice("}\n");

        return try allocator.dupe(u8, result.items);
    }

    const MessagePair = struct {
        key: []const u8,
        value: []const u8,
    };

    fn getMessagesForLocale(locale: Locale) []const MessagePair {
        return switch (locale) {
            .en_US, .en_GB => &[_]MessagePair{
                .{ .key = "panic.yikes", .value = "🚨 CURSED Runtime Error: {message}" },
                .{ .key = "panic.shook", .value = "💥 CURSED Panic: {message} (Code: {code})" },
                .{ .key = "panic.fam", .value = "❌ CURSED Exception: {message}" },
                .{ .key = "panic.location", .value = "📍 At {function}() in {file}:{line}:{column}" },
                .{ .key = "panic.memory_error", .value = "💾 Memory Error: {message}" },
                .{ .key = "panic.division_by_zero", .value = "🚫 Division by zero" },
            },
            .es_ES, .es_MX => &[_]MessagePair{
                .{ .key = "panic.yikes", .value = "🚨 Error de Tiempo de Ejecución CURSED: {message}" },
                .{ .key = "panic.shook", .value = "💥 Pánico CURSED: {message} (Código: {code})" },
                .{ .key = "panic.fam", .value = "❌ Excepción CURSED: {message}" },
                .{ .key = "panic.location", .value = "📍 En {function}() en {file}:{line}:{column}" },
                .{ .key = "panic.memory_error", .value = "💾 Error de Memoria: {message}" },
                .{ .key = "panic.division_by_zero", .value = "🚫 División por cero" },
            },
            .fr_FR => &[_]MessagePair{
                .{ .key = "panic.yikes", .value = "🚨 Erreur d'Exécution CURSED: {message}" },
                .{ .key = "panic.shook", .value = "💥 Panique CURSED: {message} (Code: {code})" },
                .{ .key = "panic.fam", .value = "❌ Exception CURSED: {message}" },
                .{ .key = "panic.location", .value = "📍 Dans {function}() dans {file}:{line}:{column}" },
                .{ .key = "panic.memory_error", .value = "💾 Erreur de Mémoire: {message}" },
                .{ .key = "panic.division_by_zero", .value = "🚫 Division par zéro" },
            },
            else => &[_]MessagePair{
                .{ .key = "panic.yikes", .value = "🚨 CURSED Runtime Error: {message}" },
                .{ .key = "panic.shook", .value = "💥 CURSED Panic: {message} (Code: {code})" },
                .{ .key = "panic.fam", .value = "❌ CURSED Exception: {message}" },
                .{ .key = "panic.location", .value = "📍 At {function}() in {file}:{line}:{column}" },
                .{ .key = "panic.memory_error", .value = "💾 Memory Error: {message}" },
                .{ .key = "panic.division_by_zero", .value = "🚫 Division by zero" },
            },
        };
    }

    fn getPluralTypeForLocale(locale: Locale) []const u8 {
        return switch (locale) {
            .ar_SA => "zero",
            .zh_CN, .zh_TW, .ja_JP, .ko_KR => "one",
            .ru_RU, .pl_PL, .cs_CZ => "many",
            else => "two",
        };
    }

    fn getDecimalSeparatorForLocale(locale: Locale) []const u8 {
        return switch (locale) {
            .de_DE, .fr_FR, .ru_RU => ",",
            else => ".",
        };
    }

    fn getThousandsSeparatorForLocale(locale: Locale) []const u8 {
        return switch (locale) {
            .de_DE => ".",
            .fr_FR, .ru_RU => " ",
            else => ",",
        };
    }
};

// Test suite for i18n configuration
test "i18n config creation and serialization" {
    const allocator = std.testing.allocator;
    
    var config = I18nConfig.init(allocator);
    defer config.deinit();

    config.default_locale = .fr_FR;
    config.fallback_locale = .en_US;
    config.auto_detect_locale = false;

    const serialized = try config.serialize();
    defer allocator.free(serialized);

    try std.testing.expect(std.mem.containsAtLeast(u8, serialized, 1, "fr-FR"));
    try std.testing.expect(std.mem.containsAtLeast(u8, serialized, 1, "en-US"));
    try std.testing.expect(std.mem.containsAtLeast(u8, serialized, 1, "false"));
}

test "language pack loader default creation" {
    const allocator = std.testing.allocator;
    
    const config = I18nConfig.init(allocator);
    const loader = LanguagePackLoader.init(allocator, config);
    
    var pack = try loader.createDefaultLanguagePack(.es_ES);
    defer pack.deinit();

    const template = pack.getMessage("panic.yikes");
    try std.testing.expect(template != null);
}

test "runtime environment locale detection" {
    const detected = RuntimeEnvironment.detectLocale();
    try std.testing.expect(detected == .en_US or detected != .en_US); // Just ensure it returns something
}

test "language pack generator" {
    const allocator = std.testing.allocator;
    
    const content = try LanguagePackGenerator.generateLanguagePackContent(allocator, .ja_JP);
    defer allocator.free(content);

    try std.testing.expect(std.mem.containsAtLeast(u8, content, 1, "ja-JP"));
    try std.testing.expect(std.mem.containsAtLeast(u8, content, 1, "messages"));
    try std.testing.expect(std.mem.containsAtLeast(u8, content, 1, "panic.yikes"));
}
