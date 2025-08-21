const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const unicode = std.unicode;

/// CURSED Panic Message Internationalization (i18n) System
/// Provides Unicode-aware panic message formatting with language pack support

/// Supported locale codes following RFC 5646 (BCP 47)
pub const Locale = enum {
    en_US,    // English (United States)
    en_GB,    // English (United Kingdom)
    es_ES,    // Spanish (Spain)
    es_MX,    // Spanish (Mexico)
    fr_FR,    // French (France)
    de_DE,    // German (Germany)
    it_IT,    // Italian (Italy)
    pt_BR,    // Portuguese (Brazil)
    ru_RU,    // Russian (Russia)
    zh_CN,    // Chinese (Simplified)
    zh_TW,    // Chinese (Traditional)
    ja_JP,    // Japanese (Japan)
    ko_KR,    // Korean (South Korea)
    ar_SA,    // Arabic (Saudi Arabia)
    hi_IN,    // Hindi (India)
    th_TH,    // Thai (Thailand)
    vi_VN,    // Vietnamese (Vietnam)
    nl_NL,    // Dutch (Netherlands)
    sv_SE,    // Swedish (Sweden)
    da_DK,    // Danish (Denmark)
    no_NO,    // Norwegian (Norway)
    fi_FI,    // Finnish (Finland)
    pl_PL,    // Polish (Poland)
    cs_CZ,    // Czech (Czech Republic)
    hu_HU,    // Hungarian (Hungary)
    tr_TR,    // Turkish (Turkey)
    he_IL,    // Hebrew (Israel)
    fa_IR,    // Persian (Iran)
    ur_PK,    // Urdu (Pakistan)
    bn_BD,    // Bengali (Bangladesh)
    ta_IN,    // Tamil (India)

    pub fn toString(self: Locale) []const u8 {
        return switch (self) {
            .en_US => "en-US",
            .en_GB => "en-GB",
            .es_ES => "es-ES",
            .es_MX => "es-MX",
            .fr_FR => "fr-FR",
            .de_DE => "de-DE",
            .it_IT => "it-IT",
            .pt_BR => "pt-BR",
            .ru_RU => "ru-RU",
            .zh_CN => "zh-CN",
            .zh_TW => "zh-TW",
            .ja_JP => "ja-JP",
            .ko_KR => "ko-KR",
            .ar_SA => "ar-SA",
            .hi_IN => "hi-IN",
            .th_TH => "th-TH",
            .vi_VN => "vi-VN",
            .nl_NL => "nl-NL",
            .sv_SE => "sv-SE",
            .da_DK => "da-DK",
            .no_NO => "no-NO",
            .fi_FI => "fi-FI",
            .pl_PL => "pl-PL",
            .cs_CZ => "cs-CZ",
            .hu_HU => "hu-HU",
            .tr_TR => "tr-TR",
            .he_IL => "he-IL",
            .fa_IR => "fa-IR",
            .ur_PK => "ur-PK",
            .bn_BD => "bn-BD",
            .ta_IN => "ta-IN",
        };
    }

    pub fn fromString(locale_str: []const u8) ?Locale {
        const locales = [_]struct{ str: []const u8, locale: Locale }{
            .{ .str = "en-US", .locale = .en_US },
            .{ .str = "en-GB", .locale = .en_GB },
            .{ .str = "es-ES", .locale = .es_ES },
            .{ .str = "es-MX", .locale = .es_MX },
            .{ .str = "fr-FR", .locale = .fr_FR },
            .{ .str = "de-DE", .locale = .de_DE },
            .{ .str = "it-IT", .locale = .it_IT },
            .{ .str = "pt-BR", .locale = .pt_BR },
            .{ .str = "ru-RU", .locale = .ru_RU },
            .{ .str = "zh-CN", .locale = .zh_CN },
            .{ .str = "zh-TW", .locale = .zh_TW },
            .{ .str = "ja-JP", .locale = .ja_JP },
            .{ .str = "ko-KR", .locale = .ko_KR },
            .{ .str = "ar-SA", .locale = .ar_SA },
            .{ .str = "hi-IN", .locale = .hi_IN },
            .{ .str = "th-TH", .locale = .th_TH },
            .{ .str = "vi-VN", .locale = .vi_VN },
            .{ .str = "nl-NL", .locale = .nl_NL },
            .{ .str = "sv-SE", .locale = .sv_SE },
            .{ .str = "da-DK", .locale = .da_DK },
            .{ .str = "no-NO", .locale = .no_NO },
            .{ .str = "fi-FI", .locale = .fi_FI },
            .{ .str = "pl-PL", .locale = .pl_PL },
            .{ .str = "cs-CZ", .locale = .cs_CZ },
            .{ .str = "hu-HU", .locale = .hu_HU },
            .{ .str = "tr-TR", .locale = .tr_TR },
            .{ .str = "he-IL", .locale = .he_IL },
            .{ .str = "fa-IR", .locale = .fa_IR },
            .{ .str = "ur-PK", .locale = .ur_PK },
            .{ .str = "bn-BD", .locale = .bn_BD },
            .{ .str = "ta-IN", .locale = .ta_IN },
        };

        for (locales) |entry| {
            if (std.mem.eql(u8, locale_str, entry.str)) {
                return entry.locale;
            }
        }
        return null;
    }

    pub fn getLanguage(self: Locale) []const u8 {
        return switch (self) {
            .en_US, .en_GB => "en",
            .es_ES, .es_MX => "es",
            .fr_FR => "fr",
            .de_DE => "de",
            .it_IT => "it",
            .pt_BR => "pt",
            .ru_RU => "ru",
            .zh_CN, .zh_TW => "zh",
            .ja_JP => "ja",
            .ko_KR => "ko",
            .ar_SA => "ar",
            .hi_IN => "hi",
            .th_TH => "th",
            .vi_VN => "vi",
            .nl_NL => "nl",
            .sv_SE => "sv",
            .da_DK => "da",
            .no_NO => "no",
            .fi_FI => "fi",
            .pl_PL => "pl",
            .cs_CZ => "cs",
            .hu_HU => "hu",
            .tr_TR => "tr",
            .he_IL => "he",
            .fa_IR => "fa",
            .ur_PK => "ur",
            .bn_BD => "bn",
            .ta_IN => "ta",
        };
    }

    pub fn isRTL(self: Locale) bool {
        return switch (self) {
            .ar_SA, .he_IL, .fa_IR, .ur_PK => true,
            else => false,
        };
    }
};

/// Message templates with placeholders for dynamic content
pub const MessageTemplate = struct {
    template: []const u8,
    placeholders: ArrayList([]const u8),
    allocator: Allocator,

    pub fn init(allocator: Allocator, template: []const u8) !MessageTemplate {
        var placeholders = .empty;
        
        // Extract placeholders from template (format: {placeholder_name})
        var i: usize = 0;
        while (i < template.len) {
            if (template[i] == '{') {
                const start = i + 1;
                i += 1;
                while (i < template.len and template[i] != '}') {
                    i += 1;
                }
                if (i < template.len and template[i] == '}') {
                    const placeholder = template[start..i];
                    try placeholders.append(try allocator.dupe(u8, placeholder));
                }
            }
            i += 1;
        }

        return MessageTemplate{
            .template = try allocator.dupe(u8, template),
            .placeholders = placeholders,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *MessageTemplate) void {
        for (self.placeholders.items) |placeholder| {
            self.allocator.free(placeholder);
        }
        self.placeholders.deinit();
        self.allocator.free(self.template);
    }

    pub fn format(self: MessageTemplate, values: HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)) ![]u8 {
        var result = .empty;
        defer result.deinit();

        var i: usize = 0;
        while (i < self.template.len) {
            if (self.template[i] == '{') {
                const start = i + 1;
                i += 1;
                while (i < self.template.len and self.template[i] != '}') {
                    i += 1;
                }
                if (i < self.template.len and self.template[i] == '}') {
                    const placeholder = self.template[start..i];
                    if (values.get(placeholder)) |value| {
                        try result.appendSlice(value);
                    } else {
                        // Keep placeholder if no value provided
                        try result.append('{');
                        try result.appendSlice(placeholder);
                        try result.append('}');
                    }
                    i += 1;
                } else {
                    try result.append(self.allocator, self.template[i - 1]);
                }
            } else {
                try result.append(self.allocator, self.template[i]);
                i += 1;
            }
        }

        return try self.allocator.dupe(u8, result.items);
    }
};

/// Language pack structure containing all localized messages
pub const LanguagePack = struct {
    locale: Locale,
    messages: HashMap([]const u8, MessageTemplate, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    pluralization_rules: PluralRules,
    number_format: NumberFormat,
    date_format: DateFormat,
    allocator: Allocator,

    pub const PluralRules = struct {
        rule_type: PluralRuleType,

        pub const PluralRuleType = enum {
            Zero,      // Languages with zero form (Arabic)
            One,       // Languages with only one form (Chinese, Japanese)
            Two,       // Languages with two forms (English)
            Few,       // Languages with few form (Russian, Polish)
            Many,      // Languages with many form (Russian, Polish)
            Other,     // Default form
        };

        pub fn getPluralForm(self: PluralRules, count: i64) PluralRuleType {
            return switch (self.rule_type) {
                .Zero => if (count == 0) .Zero else .Other,
                .One => if (count == 1) .One else .Other,
                .Two => if (count == 1) .One else .Other,
                .Few => blk: {
                    if (count == 1) break :blk .One;
                    if (count >= 2 and count <= 4) break :blk .Few;
                    break :blk .Other;
                },
                .Many => blk: {
                    if (count == 1) break :blk .One;
                    if (count >= 2 and count <= 4) break :blk .Few;
                    if (count >= 5) break :blk .Many;
                    break :blk .Other;
                },
                .Other => .Other,
            };
        }
    };

    pub const NumberFormat = struct {
        decimal_separator: []const u8,
        thousands_separator: []const u8,
        currency_symbol: []const u8,
        currency_position: CurrencyPosition,

        pub const CurrencyPosition = enum {
            Before,    // $100
            After,     // 100$
            BeforeSpace, // $ 100
            AfterSpace,  // 100 $
        };
    };

    pub const DateFormat = struct {
        date_format: []const u8,      // "YYYY-MM-DD", "DD/MM/YYYY", etc.
        time_format: []const u8,      // "HH:MM:SS", "hh:mm:ss AM/PM", etc.
        datetime_format: []const u8,  // Combined format
    };

    pub fn init(allocator: Allocator, locale: Locale) LanguagePack {
        return LanguagePack{
            .locale = locale,
            .messages = HashMap([]const u8, MessageTemplate, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .pluralization_rules = getDefaultPluralRules(locale),
            .number_format = getDefaultNumberFormat(locale),
            .date_format = getDefaultDateFormat(locale),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *LanguagePack) void {
        var iterator = self.messages.iterator();
        while (iterator.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit();
        }
        self.messages.deinit();
    }

    pub fn addMessage(self: *LanguagePack, key: []const u8, template: []const u8) !void {
        const owned_key = try self.allocator.dupe(u8, key);
        const message_template = try MessageTemplate.init(self.allocator, template);
        try self.messages.put(owned_key, message_template);
    }

    pub fn getMessage(self: *LanguagePack, key: []const u8) ?*MessageTemplate {
        return self.messages.getPtr(key);
    }

    fn getDefaultPluralRules(locale: Locale) PluralRules {
        return switch (locale) {
            .ar_SA => PluralRules{ .rule_type = .Zero },
            .zh_CN, .zh_TW, .ja_JP, .ko_KR, .th_TH, .vi_VN => PluralRules{ .rule_type = .One },
            .ru_RU, .pl_PL, .cs_CZ => PluralRules{ .rule_type = .Many },
            else => PluralRules{ .rule_type = .Two },
        };
    }

    fn getDefaultNumberFormat(locale: Locale) NumberFormat {
        return switch (locale) {
            .en_US => NumberFormat{
                .decimal_separator = ".",
                .thousands_separator = ",",
                .currency_symbol = "$",
                .currency_position = .Before,
            },
            .de_DE => NumberFormat{
                .decimal_separator = ",",
                .thousands_separator = ".",
                .currency_symbol = "€",
                .currency_position = .AfterSpace,
            },
            .fr_FR => NumberFormat{
                .decimal_separator = ",",
                .thousands_separator = " ",
                .currency_symbol = "€",
                .currency_position = .AfterSpace,
            },
            .ru_RU => NumberFormat{
                .decimal_separator = ",",
                .thousands_separator = " ",
                .currency_symbol = "₽",
                .currency_position = .AfterSpace,
            },
            .ja_JP => NumberFormat{
                .decimal_separator = ".",
                .thousands_separator = ",",
                .currency_symbol = "¥",
                .currency_position = .Before,
            },
            else => NumberFormat{
                .decimal_separator = ".",
                .thousands_separator = ",",
                .currency_symbol = "$",
                .currency_position = .Before,
            },
        };
    }

    fn getDefaultDateFormat(locale: Locale) DateFormat {
        return switch (locale) {
            .en_US => DateFormat{
                .date_format = "MM/DD/YYYY",
                .time_format = "hh:mm:ss AM/PM",
                .datetime_format = "MM/DD/YYYY hh:mm:ss AM/PM",
            },
            .en_GB, .de_DE, .fr_FR => DateFormat{
                .date_format = "DD/MM/YYYY",
                .time_format = "HH:mm:ss",
                .datetime_format = "DD/MM/YYYY HH:mm:ss",
            },
            .ja_JP => DateFormat{
                .date_format = "YYYY/MM/DD",
                .time_format = "HH:mm:ss",
                .datetime_format = "YYYY/MM/DD HH:mm:ss",
            },
            else => DateFormat{
                .date_format = "YYYY-MM-DD",
                .time_format = "HH:mm:ss",
                .datetime_format = "YYYY-MM-DD HH:mm:ss",
            },
        };
    }
};

/// Unicode-aware text processing utilities
pub const UnicodeUtils = struct {
    pub fn validateUTF8(text: []const u8) bool {
        return unicode.utf8ValidateSlice(text);
    }

    pub fn utf8Length(text: []const u8) !usize {
        return unicode.utf8CountCodepoints(text);
    }

    pub fn truncateUTF8(allocator: Allocator, text: []const u8, max_codepoints: usize) ![]u8 {
        if (!validateUTF8(text)) return error.InvalidUTF8;
        
        const total_codepoints = try utf8Length(text);
        if (total_codepoints <= max_codepoints) {
            return try allocator.dupe(u8, text);
        }

        var view = unicode.Utf8View.init(text) catch return error.InvalidUTF8;
        var iterator = view.iterator();
        var result = .empty;
        defer result.deinit();

        var count: usize = 0;
        while (iterator.nextCodepoint()) |codepoint| {
            if (count >= max_codepoints) break;
            
            var buffer: [4]u8 = undefined;
            const len = unicode.utf8Encode(codepoint, &buffer) catch break;
            try result.appendSlice(buffer[0..len]);
            count += 1;
        }

        return try allocator.dupe(u8, result.items);
    }

    pub fn reverseRTL(allocator: Allocator, text: []const u8) ![]u8 {
        if (!validateUTF8(text)) return error.InvalidUTF8;
        
        var view = unicode.Utf8View.init(text) catch return error.InvalidUTF8;
        var codepoints = .empty;
        defer codepoints.deinit();

        var iterator = view.iterator();
        while (iterator.nextCodepoint()) |codepoint| {
            try codepoints.append(codepoint);
        }

        var result = .empty;
        defer result.deinit();

        var i: usize = codepoints.items.len;
        while (i > 0) {
            i -= 1;
            var buffer: [4]u8 = undefined;
            const len = unicode.utf8Encode(codepoints.items[i], &buffer) catch continue;
            try result.appendSlice(buffer[0..len]);
        }

        return try allocator.dupe(u8, result.items);
    }
};

/// Main i18n manager that handles language packs and message formatting
pub const I18nManager = struct {
    language_packs: HashMap(Locale, LanguagePack, EnumContext(Locale), std.hash_map.default_max_load_percentage),
    current_locale: Locale,
    fallback_locale: Locale,
    allocator: Allocator,

    const EnumContext = struct {
        pub fn Context(comptime T: type) type {
            return struct {
                pub fn hash(self: @This(), s: T) u64 {
                    _ = self;
                    return std.hash_map.hashString(@tagName(s));
                }
                pub fn eql(self: @This(), a: T, b: T) bool {
                    _ = self;
                    return a == b;
                }
            };
        }
    };

    pub fn init(allocator: Allocator, current_locale: Locale, fallback_locale: Locale) I18nManager {
        return I18nManager{
            .language_packs = HashMap(Locale, LanguagePack, EnumContext(Locale).Context(Locale), std.hash_map.default_max_load_percentage).init(allocator),
            .current_locale = current_locale,
            .fallback_locale = fallback_locale,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *I18nManager) void {
        var iterator = self.language_packs.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.language_packs.deinit();
    }

    pub fn loadLanguagePack(self: *I18nManager, locale: Locale) !void {
        var pack = LanguagePack.init(self.allocator, locale);
        try self.loadDefaultMessages(&pack);
        try self.language_packs.put(locale, pack);
    }

    pub fn loadLanguagePackFromFile(self: *I18nManager, locale: Locale, file_path: []const u8) !void {
        var pack = LanguagePack.init(self.allocator, locale);
        try self.loadMessagesFromFile(&pack, file_path);
        try self.language_packs.put(locale, pack);
    }

    pub fn setCurrentLocale(self: *I18nManager, locale: Locale) !void {
        if (!self.language_packs.contains(locale)) {
            try self.loadLanguagePack(locale);
        }
        self.current_locale = locale;
    }

    pub fn formatPanicMessage(self: *I18nManager, message_key: []const u8, values: HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)) ![]u8 {
        // Try current locale first
        if (self.getMessageTemplate(self.current_locale, message_key)) |template| {
            return try template.format(values);
        }

        // Fall back to fallback locale
        if (self.getMessageTemplate(self.fallback_locale, message_key)) |template| {
            return try template.format(values);
        }

        // Last resort: return the key with basic formatting
        return try self.formatFallbackMessage(message_key, values);
    }

    fn getMessageTemplate(self: *I18nManager, locale: Locale, key: []const u8) ?*MessageTemplate {
        if (self.language_packs.getPtr(locale)) |pack| {
            return pack.getMessage(key);
        }
        return null;
    }

    fn formatFallbackMessage(self: *I18nManager, key: []const u8, values: HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)) ![]u8 {
        var result = .empty;
        defer result.deinit();

        try result.appendSlice("UNTRANSLATED: ");
        try result.appendSlice(key);

        if (values.count() > 0) {
            try result.appendSlice(" (");
            var first = true;
            var iterator = values.iterator();
            while (iterator.next()) |entry| {
                if (!first) try result.appendSlice(", ");
                try result.appendSlice(entry.key_ptr.*);
                try result.appendSlice("=");
                try result.appendSlice(entry.value_ptr.*);
                first = false;
            }
            try result.appendSlice(")");
        }

        return try self.allocator.dupe(u8, result.items);
    }

    fn loadDefaultMessages(self: *I18nManager, pack: *LanguagePack) !void {
        switch (pack.locale) {
            .en_US, .en_GB => {
                try pack.addMessage("panic.yikes", "🚨 CURSED Runtime Error: {message}");
                try pack.addMessage("panic.shook", "💥 CURSED Panic: {message} (Code: {code})");
                try pack.addMessage("panic.fam", "❌ CURSED Exception: {message}");
                try pack.addMessage("panic.location", "📍 At {function}() in {file}:{line}:{column}");
                try pack.addMessage("panic.stack_trace", "📚 Stack trace:");
                try pack.addMessage("panic.caused_by", "Caused by:");
                try pack.addMessage("panic.memory_error", "💾 Memory Error: {message}");
                try pack.addMessage("panic.type_error", "🔷 Type Error: {message}");
                try pack.addMessage("panic.runtime_error", "⚡ Runtime Error: {message}");
                try pack.addMessage("panic.division_by_zero", "🚫 Division by zero");
                try pack.addMessage("panic.index_out_of_bounds", "📊 Index {index} out of bounds (size: {size})");
                try pack.addMessage("panic.null_pointer", "🎯 Null pointer dereference");
                try pack.addMessage("panic.invalid_operation", "❗ Invalid operation: {operation}");
                try pack.addMessage("panic.unknown_error", "❓ Unknown error occurred");
            },
            .es_ES, .es_MX => {
                try pack.addMessage("panic.yikes", "🚨 Error de Tiempo de Ejecución CURSED: {message}");
                try pack.addMessage("panic.shook", "💥 Pánico CURSED: {message} (Código: {code})");
                try pack.addMessage("panic.fam", "❌ Excepción CURSED: {message}");
                try pack.addMessage("panic.location", "📍 En {function}() en {file}:{line}:{column}");
                try pack.addMessage("panic.stack_trace", "📚 Rastro de pila:");
                try pack.addMessage("panic.caused_by", "Causado por:");
                try pack.addMessage("panic.memory_error", "💾 Error de Memoria: {message}");
                try pack.addMessage("panic.type_error", "🔷 Error de Tipo: {message}");
                try pack.addMessage("panic.runtime_error", "⚡ Error de Tiempo de Ejecución: {message}");
                try pack.addMessage("panic.division_by_zero", "🚫 División por cero");
                try pack.addMessage("panic.index_out_of_bounds", "📊 Índice {index} fuera de límites (tamaño: {size})");
                try pack.addMessage("panic.null_pointer", "🎯 Desreferencia de puntero nulo");
                try pack.addMessage("panic.invalid_operation", "❗ Operación inválida: {operation}");
                try pack.addMessage("panic.unknown_error", "❓ Error desconocido");
            },
            .fr_FR => {
                try pack.addMessage("panic.yikes", "🚨 Erreur d'Exécution CURSED: {message}");
                try pack.addMessage("panic.shook", "💥 Panique CURSED: {message} (Code: {code})");
                try pack.addMessage("panic.fam", "❌ Exception CURSED: {message}");
                try pack.addMessage("panic.location", "📍 Dans {function}() dans {file}:{line}:{column}");
                try pack.addMessage("panic.stack_trace", "📚 Trace de pile:");
                try pack.addMessage("panic.caused_by", "Causé par:");
                try pack.addMessage("panic.memory_error", "💾 Erreur de Mémoire: {message}");
                try pack.addMessage("panic.type_error", "🔷 Erreur de Type: {message}");
                try pack.addMessage("panic.runtime_error", "⚡ Erreur d'Exécution: {message}");
                try pack.addMessage("panic.division_by_zero", "🚫 Division par zéro");
                try pack.addMessage("panic.index_out_of_bounds", "📊 Index {index} hors limites (taille: {size})");
                try pack.addMessage("panic.null_pointer", "🎯 Déréférencement de pointeur nul");
                try pack.addMessage("panic.invalid_operation", "❗ Opération invalide: {operation}");
                try pack.addMessage("panic.unknown_error", "❓ Erreur inconnue");
            },
            .de_DE => {
                try pack.addMessage("panic.yikes", "🚨 CURSED Laufzeitfehler: {message}");
                try pack.addMessage("panic.shook", "💥 CURSED Panik: {message} (Code: {code})");
                try pack.addMessage("panic.fam", "❌ CURSED Ausnahme: {message}");
                try pack.addMessage("panic.location", "📍 In {function}() in {file}:{line}:{column}");
                try pack.addMessage("panic.stack_trace", "📚 Stack-Trace:");
                try pack.addMessage("panic.caused_by", "Verursacht durch:");
                try pack.addMessage("panic.memory_error", "💾 Speicherfehler: {message}");
                try pack.addMessage("panic.type_error", "🔷 Typfehler: {message}");
                try pack.addMessage("panic.runtime_error", "⚡ Laufzeitfehler: {message}");
                try pack.addMessage("panic.division_by_zero", "🚫 Division durch Null");
                try pack.addMessage("panic.index_out_of_bounds", "📊 Index {index} außerhalb der Grenzen (Größe: {size})");
                try pack.addMessage("panic.null_pointer", "🎯 Null-Zeiger-Dereferenzierung");
                try pack.addMessage("panic.invalid_operation", "❗ Ungültige Operation: {operation}");
                try pack.addMessage("panic.unknown_error", "❓ Unbekannter Fehler");
            },
            .ru_RU => {
                try pack.addMessage("panic.yikes", "🚨 Ошибка Времени Выполнения CURSED: {message}");
                try pack.addMessage("panic.shook", "💥 Паника CURSED: {message} (Код: {code})");
                try pack.addMessage("panic.fam", "❌ Исключение CURSED: {message}");
                try pack.addMessage("panic.location", "📍 В {function}() в {file}:{line}:{column}");
                try pack.addMessage("panic.stack_trace", "📚 Трассировка стека:");
                try pack.addMessage("panic.caused_by", "Вызвано:");
                try pack.addMessage("panic.memory_error", "💾 Ошибка Памяти: {message}");
                try pack.addMessage("panic.type_error", "🔷 Ошибка Типа: {message}");
                try pack.addMessage("panic.runtime_error", "⚡ Ошибка Времени Выполнения: {message}");
                try pack.addMessage("panic.division_by_zero", "🚫 Деление на ноль");
                try pack.addMessage("panic.index_out_of_bounds", "📊 Индекс {index} вне границ (размер: {size})");
                try pack.addMessage("panic.null_pointer", "🎯 Разыменование нулевого указателя");
                try pack.addMessage("panic.invalid_operation", "❗ Недопустимая операция: {operation}");
                try pack.addMessage("panic.unknown_error", "❓ Неизвестная ошибка");
            },
            .zh_CN => {
                try pack.addMessage("panic.yikes", "🚨 CURSED 运行时错误: {message}");
                try pack.addMessage("panic.shook", "💥 CURSED 恐慌: {message} (代码: {code})");
                try pack.addMessage("panic.fam", "❌ CURSED 异常: {message}");
                try pack.addMessage("panic.location", "📍 在 {function}() 位于 {file}:{line}:{column}");
                try pack.addMessage("panic.stack_trace", "📚 堆栈跟踪:");
                try pack.addMessage("panic.caused_by", "由以下原因引起:");
                try pack.addMessage("panic.memory_error", "💾 内存错误: {message}");
                try pack.addMessage("panic.type_error", "🔷 类型错误: {message}");
                try pack.addMessage("panic.runtime_error", "⚡ 运行时错误: {message}");
                try pack.addMessage("panic.division_by_zero", "🚫 除零错误");
                try pack.addMessage("panic.index_out_of_bounds", "📊 索引 {index} 超出边界 (大小: {size})");
                try pack.addMessage("panic.null_pointer", "🎯 空指针解引用");
                try pack.addMessage("panic.invalid_operation", "❗ 无效操作: {operation}");
                try pack.addMessage("panic.unknown_error", "❓ 未知错误");
            },
            .ja_JP => {
                try pack.addMessage("panic.yikes", "🚨 CURSED ランタイムエラー: {message}");
                try pack.addMessage("panic.shook", "💥 CURSED パニック: {message} (コード: {code})");
                try pack.addMessage("panic.fam", "❌ CURSED 例外: {message}");
                try pack.addMessage("panic.location", "📍 {function}() 内の {file}:{line}:{column} で");
                try pack.addMessage("panic.stack_trace", "📚 スタックトレース:");
                try pack.addMessage("panic.caused_by", "原因:");
                try pack.addMessage("panic.memory_error", "💾 メモリエラー: {message}");
                try pack.addMessage("panic.type_error", "🔷 型エラー: {message}");
                try pack.addMessage("panic.runtime_error", "⚡ ランタイムエラー: {message}");
                try pack.addMessage("panic.division_by_zero", "🚫 ゼロ除算");
                try pack.addMessage("panic.index_out_of_bounds", "📊 インデックス {index} が範囲外 (サイズ: {size})");
                try pack.addMessage("panic.null_pointer", "🎯 ヌルポインタの参照解除");
                try pack.addMessage("panic.invalid_operation", "❗ 無効な操作: {operation}");
                try pack.addMessage("panic.unknown_error", "❓ 不明なエラー");
            },
            .ar_SA => {
                try pack.addMessage("panic.yikes", "🚨 خطأ وقت التشغيل CURSED: {message}");
                try pack.addMessage("panic.shook", "💥 ذعر CURSED: {message} (الرمز: {code})");
                try pack.addMessage("panic.fam", "❌ استثناء CURSED: {message}");
                try pack.addMessage("panic.location", "📍 في {function}() في {file}:{line}:{column}");
                try pack.addMessage("panic.stack_trace", "📚 تتبع المكدس:");
                try pack.addMessage("panic.caused_by", "سببه:");
                try pack.addMessage("panic.memory_error", "💾 خطأ في الذاكرة: {message}");
                try pack.addMessage("panic.type_error", "🔷 خطأ في النوع: {message}");
                try pack.addMessage("panic.runtime_error", "⚡ خطأ وقت التشغيل: {message}");
                try pack.addMessage("panic.division_by_zero", "🚫 القسمة على صفر");
                try pack.addMessage("panic.index_out_of_bounds", "📊 الفهرس {index} خارج الحدود (الحجم: {size})");
                try pack.addMessage("panic.null_pointer", "🎯 إلغاء مرجع مؤشر فارغ");
                try pack.addMessage("panic.invalid_operation", "❗ عملية غير صالحة: {operation}");
                try pack.addMessage("panic.unknown_error", "❓ خطأ غير معروف");
            },
            else => {
                // Default to English for unsupported locales
                try self.loadDefaultMessages(pack);
                return;
            },
        }
    }

    fn loadMessagesFromFile(self: *I18nManager, pack: *LanguagePack, file_path: []const u8) !void {
        _ = self; // Remove unused parameter warning
        _ = pack;
        _ = file_path;
        // TODO: Implement JSON/YAML file loading for language packs
        return error.NotImplemented;
    }
};

/// Global i18n instance for the CURSED runtime
var global_i18n: ?I18nManager = null;
var global_i18n_mutex: std.Thread.Mutex = .{};

/// Initialize the global i18n system
pub fn initGlobalI18n(allocator: Allocator, locale: ?Locale) !void {
    global_i18n_mutex.lock();
    defer global_i18n_mutex.unlock();

    if (global_i18n != null) {
        global_i18n.?.deinit();
    }

    const detected_locale = locale orelse detectSystemLocale();
    global_i18n = I18nManager.init(allocator, detected_locale, .en_US);
    try global_i18n.?.loadLanguagePack(detected_locale);
    try global_i18n.?.loadLanguagePack(.en_US); // Always load English as fallback
}

/// Deinitialize the global i18n system
pub fn deinitGlobalI18n() void {
    global_i18n_mutex.lock();
    defer global_i18n_mutex.unlock();

    if (global_i18n) |*i18n| {
        i18n.deinit();
        global_i18n = null;
    }
}

/// Format a panic message using the global i18n system
pub fn formatGlobalPanicMessage(message_key: []const u8, values: HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)) ![]u8 {
    global_i18n_mutex.lock();
    defer global_i18n_mutex.unlock();

    if (global_i18n) |*i18n| {
        return try i18n.formatPanicMessage(message_key, values);
    }

    // Fallback if i18n not initialized
    const allocator = std.heap.page_allocator;
    var result = .empty;
    defer result.deinit();

    try result.appendSlice("🚨 CURSED Error: ");
    try result.appendSlice(message_key);

    return try allocator.dupe(u8, result.items);
}

/// Detect system locale from environment variables
fn detectSystemLocale() Locale {
    // Check LC_ALL, LC_MESSAGES, LANG environment variables
    const env_vars = [_][]const u8{ "LC_ALL", "LC_MESSAGES", "LANG" };
    
    for (env_vars) |env_var| {
        if (std.process.getEnvVarOwned(std.heap.page_allocator, env_var)) |locale_str| {
            defer std.heap.page_allocator.free(locale_str);
            
            // Extract locale code (e.g., "en_US.UTF-8" -> "en-US")
            var locale_part = locale_str;
            if (std.mem.indexOf(u8, locale_str, ".")) |dot_pos| {
                locale_part = locale_str[0..dot_pos];
            }
            
            // Convert underscore to hyphen
            var normalized = std.heap.page_allocator.alloc(u8, locale_part.len) catch continue;
            defer std.heap.page_allocator.free(normalized);
            
            for (locale_part, 0..) |char, i| {
                normalized[i] = if (char == '_') '-' else char;
            }
            
            if (Locale.fromString(normalized)) |locale| {
                return locale;
            }
        } else |_| {
            // Continue to next environment variable if this one fails
        }
    }
    
    // Default to US English if no locale detected
    return .en_US;
}

// Test suite for i18n system
test "locale conversion" {
    const locale = Locale.en_US;
    try std.testing.expectEqualStrings("en-US", locale.toString());
    try std.testing.expectEqualStrings("en", locale.getLanguage());
    try std.testing.expect(!locale.isRTL());

    const ar_locale = Locale.ar_SA;
    try std.testing.expect(ar_locale.isRTL());
}

test "message template formatting" {
    const allocator = std.testing.allocator;
    
    var template = try MessageTemplate.init(allocator, "Error: {message} (Code: {code})");
    defer template.deinit();

    var values = HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
    defer values.deinit();

    try values.put("message", "Test error");
    try values.put("code", "42");

    const formatted = try template.format(values);
    defer allocator.free(formatted);

    try std.testing.expectEqualStrings("Error: Test error (Code: 42)", formatted);
}

test "unicode utilities" {
    const allocator = std.testing.allocator;
    
    const text = "Hello 世界 🌍";
    try std.testing.expect(UnicodeUtils.validateUTF8(text));
    
    const length = try UnicodeUtils.utf8Length(text);
    try std.testing.expect(length == 9); // H e l l o   世 界   🌍
    
    const truncated = try UnicodeUtils.truncateUTF8(allocator, text, 7);
    defer allocator.free(truncated);
    
    const truncated_length = try UnicodeUtils.utf8Length(truncated);
    try std.testing.expect(truncated_length == 7);
}

test "i18n manager basic functionality" {
    const allocator = std.testing.allocator;
    
    var i18n = I18nManager.init(allocator, .en_US, .en_US);
    defer i18n.deinit();

    try i18n.loadLanguagePack(.en_US);

    var values = HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
    defer values.deinit();
    try values.put("message", "Test panic");

    const formatted = try i18n.formatPanicMessage("panic.yikes", values);
    defer allocator.free(formatted);

    try std.testing.expect(std.mem.containsAtLeast(u8, formatted, 1, "Test panic"));
}

test "rtl text processing" {
    const allocator = std.testing.allocator;
    
    const text = "Hello";
    const reversed = try UnicodeUtils.reverseRTL(allocator, text);
    defer allocator.free(reversed);
    
    try std.testing.expectEqualStrings("olleH", reversed);
}
