; CURSED Language - Advanced LLVM Compilation
target triple = "x86_64-unknown-linux-gnu"


; Runtime function declarations
declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)
declare i8* @malloc(i64)
declare void @free(i8*)
declare i64 @strlen(i8*)
declare i8* @strcpy(i8*, i8*)
declare i8* @i32_to_string(i32)
declare i8* @char_to_string(i8)
declare i8* @string_concat(i8*, i8*)
declare i8* @tea(i64)
declare i8* @tea_float(double)
declare i8* @tea_bool(i32)

; CURSED runtime functions
declare void @cursed_panic(i8*, i64)
declare i8* @cursed_alloc(i64)
declare void @cursed_free(i8*)
declare i32 @cursed_goroutine_spawn(i8*)
declare void @cursed_channel_send(i8*, i8*)
declare i8* @cursed_channel_receive(i8*)

; Exception handling declarations
declare i32 @__gxx_personality_v0(...)
declare i8* @__cxa_begin_catch(i8*)
declare void @__cxa_end_catch()
declare void @__cxa_rethrow()
declare i8* @__cxa_allocate_exception(i64)
declare void @__cxa_throw(i8*, i8*, i8*)
declare i8* @_Unwind_GetLanguageSpecificData(i8*)
declare i32 @_Unwind_GetRegionStart(i8*)
declare i32 @_Unwind_GetDataRelBase(i8*)
declare i32 @_Unwind_GetTextRelBase(i8*)

; CURSED exception type info
@_ZTI11CursedError = constant { i8*, i8* } { i8* null, i8* bitcast ([14 x i8]* @_ZTS11CursedError to i8*) }
@_ZTS11CursedError = constant [14 x i8] c"11CursedError\00"



; String constants
@.str.51 = private unnamed_addr constant [35 x i8] c"char_is_alphanumeric - punctuation\00", align 1
@.str.5 = private unnamed_addr constant [13 x i8] c"Hello world.\00", align 1
@.str.19 = private unnamed_addr constant [31 x i8] c"has_balanced_braces - balanced\00", align 1
@.str.39 = private unnamed_addr constant [16 x i8] c"Hello123 world!\00", align 1
@.str.14 = private unnamed_addr constant [42 x i8] c"has_balanced_parentheses - no parentheses\00", align 1
@.str.31 = private unnamed_addr constant [32 x i8] c"validate_rule_structure - valid\00", align 1
@.str.27 = private unnamed_addr constant [35 x i8] c"calculate_complexity_score - empty\00", align 1
@.str.57 = private unnamed_addr constant [29 x i8] c"char_is_punctuation - letter\00", align 1
@.str.49 = private unnamed_addr constant [30 x i8] c"char_is_alphanumeric - letter\00", align 1
@.str.12 = private unnamed_addr constant [36 x i8] c"has_balanced_parentheses - balanced\00", align 1
@.str.32 = private unnamed_addr constant [36 x i8] c"validate_rule_structure - too short\00", align 1
@.str.55 = private unnamed_addr constant [29 x i8] c"char_is_punctuation - period\00", align 1
@.str.29 = private unnamed_addr constant [6 x i8] c"world\00", align 1
@.str.42 = private unnamed_addr constant [30 x i8] c"char_is_uppercase - lowercase\00", align 1
@.str.1 = private unnamed_addr constant [11 x i8] c"S -> NP VP\00", align 1
@.str.30 = private unnamed_addr constant [33 x i8] c"contains_pattern - empty pattern\00", align 1
@.str.16 = private unnamed_addr constant [33 x i8] c"has_balanced_brackets - balanced\00", align 1
@.str.4 = private unnamed_addr constant [36 x i8] c"is_valid_sentence - proper sentence\00", align 1
@.str.54 = private unnamed_addr constant [28 x i8] c"char_is_whitespace - letter\00", align 1
@.str.20 = private unnamed_addr constant [14 x i8] c"{hello world}\00", align 1
@.str.0 = private unnamed_addr constant [35 x i8] c"validate_grammar_rule - valid rule\00", align 1
@.str.6 = private unnamed_addr constant [33 x i8] c"is_valid_sentence - empty string\00", align 1
@.str.8 = private unnamed_addr constant [12 x i8] c"Hello world\00", align 1
@.str.10 = private unnamed_addr constant [30 x i8] c"count_sentences - simple text\00", align 1
@.str.41 = private unnamed_addr constant [30 x i8] c"char_is_uppercase - uppercase\00", align 1
@.str.23 = private unnamed_addr constant [32 x i8] c"has_balanced_quotes - no quotes\00", align 1
@.str.45 = private unnamed_addr constant [24 x i8] c"char_is_letter - letter\00", align 1
@.str.43 = private unnamed_addr constant [30 x i8] c"char_is_lowercase - lowercase\00", align 1
@.str.7 = private unnamed_addr constant [30 x i8] c"count_words - simple sentence\00", align 1
@.str.35 = private unnamed_addr constant [34 x i8] c"has_proper_capitalization - empty\00", align 1
@.str.28 = private unnamed_addr constant [25 x i8] c"contains_pattern - found\00", align 1
@.str.9 = private unnamed_addr constant [27 x i8] c"count_words - empty string\00", align 1
@.str.38 = private unnamed_addr constant [35 x i8] c"count_character_types - mixed text\00", align 1
@.str.47 = private unnamed_addr constant [22 x i8] c"char_is_digit - digit\00", align 1
@.str.26 = private unnamed_addr constant [36 x i8] c"calculate_complexity_score - simple\00", align 1
@.str.13 = private unnamed_addr constant [14 x i8] c"(hello world)\00", align 1
@.str.21 = private unnamed_addr constant [32 x i8] c"has_balanced_braces - no braces\00", align 1
@.str.34 = private unnamed_addr constant [35 x i8] c"has_proper_capitalization - proper\00", align 1
@.str.36 = private unnamed_addr constant [30 x i8] c"parse_production_rule - valid\00", align 1
@.str.24 = private unnamed_addr constant [32 x i8] c"has_proper_punctuation - proper\00", align 1
@.str.33 = private unnamed_addr constant [2 x i8] c"S\00", align 1
@.str.48 = private unnamed_addr constant [23 x i8] c"char_is_digit - letter\00", align 1
@.str.40 = private unnamed_addr constant [30 x i8] c"count_character_types - empty\00", align 1
@.str.11 = private unnamed_addr constant [31 x i8] c"count_sentences - empty string\00", align 1
@.str.53 = private unnamed_addr constant [25 x i8] c"char_is_whitespace - tab\00", align 1
@.str.56 = private unnamed_addr constant [28 x i8] c"char_is_punctuation - comma\00", align 1
@.str.2 = private unnamed_addr constant [35 x i8] c"validate_grammar_rule - empty rule\00", align 1
@.str.44 = private unnamed_addr constant [30 x i8] c"char_is_lowercase - uppercase\00", align 1
@.str.15 = private unnamed_addr constant [12 x i8] c"hello world\00", align 1
@.str.22 = private unnamed_addr constant [31 x i8] c"has_balanced_quotes - balanced\00", align 1
@.str.17 = private unnamed_addr constant [14 x i8] c"[hello world]\00", align 1
@.str.52 = private unnamed_addr constant [27 x i8] c"char_is_whitespace - space\00", align 1
@.str.37 = private unnamed_addr constant [30 x i8] c"parse_production_rule - empty\00", align 1
@.str.25 = private unnamed_addr constant [31 x i8] c"has_proper_punctuation - empty\00", align 1
@.str.50 = private unnamed_addr constant [29 x i8] c"char_is_alphanumeric - digit\00", align 1
@.str.18 = private unnamed_addr constant [36 x i8] c"has_balanced_brackets - no brackets\00", align 1
@.str.46 = private unnamed_addr constant [23 x i8] c"char_is_letter - digit\00", align 1
@.str.3 = private unnamed_addr constant [1 x i8] c"\00", align 1
define i32 @main() {
  %0 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  %2 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.1, i64 0, i64 0
  %3 = call i32 @validate_grammar_rule(i32 %2)
  %4 = alloca i1, align 4
  store i1 %3, i1* %4, align 4
  ; Variable valid_rule allocated at %4
  %5 = load i32, i32* %4, align 4
  %6 = call i32 @assert_true(i32 %5)
  %7 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.2, i64 0, i64 0
  %8 = call i32 @test_start(i32 %7)
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %10 = call i32 @validate_grammar_rule(i32 %9)
  %11 = alloca i1, align 4
  store i1 %10, i1* %11, align 4
  ; Variable empty_rule allocated at %11
  %12 = load i32, i32* %11, align 4
  %13 = call i32 @assert_false(i32 %12)
  %14 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.4, i64 0, i64 0
  %15 = call i32 @test_start(i32 %14)
  %16 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.5, i64 0, i64 0
  %17 = call i32 @is_valid_sentence(i32 %16)
  %18 = alloca i1, align 4
  store i1 %17, i1* %18, align 4
  ; Variable valid_sentence allocated at %18
  %19 = load i32, i32* %18, align 4
  %20 = call i32 @assert_true(i32 %19)
  %21 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.6, i64 0, i64 0
  %22 = call i32 @test_start(i32 %21)
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %24 = call i32 @is_valid_sentence(i32 %23)
  %25 = alloca i1, align 4
  store i1 %24, i1* %25, align 4
  ; Variable empty_sentence allocated at %25
  %26 = load i32, i32* %25, align 4
  %27 = call i32 @assert_false(i32 %26)
  %28 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.7, i64 0, i64 0
  %29 = call i32 @test_start(i32 %28)
  %30 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %31 = call i32 @count_words(i32 %30)
  %32 = alloca i32, align 4
  store i32 %31, i32* %32, align 4
  ; Variable word_count allocated at %32
  %33 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %34 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %35 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.9, i64 0, i64 0
  %36 = call i32 @test_start(i32 %35)
  %37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %38 = call i32 @count_words(i32 %37)
  %39 = alloca i32, align 4
  store i32 %38, i32* %39, align 4
  ; Variable empty_word_count allocated at %39
  %40 = load i32, i32* %39, align 4
  %41 = call i32 @assert_eq_int(i32 %40, i32 0)
  %42 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.10, i64 0, i64 0
  %43 = call i32 @test_start(i32 %42)
  %44 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.5, i64 0, i64 0
  %45 = call i32 @count_sentences(i32 %44)
  %46 = alloca i32, align 4
  store i32 %45, i32* %46, align 4
  ; Variable sentence_count allocated at %46
  %47 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %48 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %49 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.11, i64 0, i64 0
  %50 = call i32 @test_start(i32 %49)
  %51 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %52 = call i32 @count_sentences(i32 %51)
  %53 = alloca i32, align 4
  store i32 %52, i32* %53, align 4
  ; Variable empty_sentence_count allocated at %53
  %54 = load i32, i32* %53, align 4
  %55 = call i32 @assert_eq_int(i32 %54, i32 0)
  %56 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.12, i64 0, i64 0
  %57 = call i32 @test_start(i32 %56)
  %58 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.13, i64 0, i64 0
  %59 = call i32 @has_balanced_parentheses(i32 %58)
  %60 = alloca i1, align 4
  store i1 %59, i1* %60, align 4
  ; Variable balanced_parens allocated at %60
  %61 = load i32, i32* %60, align 4
  %62 = call i32 @assert_true(i32 %61)
  %63 = getelementptr inbounds [42 x i8], [42 x i8]* @.str.14, i64 0, i64 0
  %64 = call i32 @test_start(i32 %63)
  %65 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.15, i64 0, i64 0
  %66 = call i32 @has_balanced_parentheses(i32 %65)
  %67 = alloca i1, align 4
  store i1 %66, i1* %67, align 4
  ; Variable no_parens allocated at %67
  %68 = load i32, i32* %67, align 4
  %69 = call i32 @assert_true(i32 %68)
  %70 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.16, i64 0, i64 0
  %71 = call i32 @test_start(i32 %70)
  %72 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.17, i64 0, i64 0
  %73 = call i32 @has_balanced_brackets(i32 %72)
  %74 = alloca i1, align 4
  store i1 %73, i1* %74, align 4
  ; Variable balanced_brackets allocated at %74
  %75 = load i32, i32* %74, align 4
  %76 = call i32 @assert_true(i32 %75)
  %77 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.18, i64 0, i64 0
  %78 = call i32 @test_start(i32 %77)
  %79 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.15, i64 0, i64 0
  %80 = call i32 @has_balanced_brackets(i32 %79)
  %81 = alloca i1, align 4
  store i1 %80, i1* %81, align 4
  ; Variable no_brackets allocated at %81
  %82 = load i32, i32* %81, align 4
  %83 = call i32 @assert_true(i32 %82)
  %84 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.19, i64 0, i64 0
  %85 = call i32 @test_start(i32 %84)
  %86 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.20, i64 0, i64 0
  %87 = call i32 @has_balanced_braces(i32 %86)
  %88 = alloca i1, align 4
  store i1 %87, i1* %88, align 4
  ; Variable balanced_braces allocated at %88
  %89 = load i32, i32* %88, align 4
  %90 = call i32 @assert_true(i32 %89)
  %91 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.21, i64 0, i64 0
  %92 = call i32 @test_start(i32 %91)
  %93 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.15, i64 0, i64 0
  %94 = call i32 @has_balanced_braces(i32 %93)
  %95 = alloca i1, align 4
  store i1 %94, i1* %95, align 4
  ; Variable no_braces allocated at %95
  %96 = load i32, i32* %95, align 4
  %97 = call i32 @assert_true(i32 %96)
  %98 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.22, i64 0, i64 0
  %99 = call i32 @test_start(i32 %98)
  %100 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.15, i64 0, i64 0
  %101 = call i32 @has_balanced_quotes(i32 %100)
  %102 = alloca i1, align 4
  store i1 %101, i1* %102, align 4
  ; Variable balanced_quotes allocated at %102
  %103 = load i32, i32* %102, align 4
  %104 = call i32 @assert_true(i32 %103)
  %105 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.23, i64 0, i64 0
  %106 = call i32 @test_start(i32 %105)
  %107 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.15, i64 0, i64 0
  %108 = call i32 @has_balanced_quotes(i32 %107)
  %109 = alloca i1, align 4
  store i1 %108, i1* %109, align 4
  ; Variable no_quotes allocated at %109
  %110 = load i32, i32* %109, align 4
  %111 = call i32 @assert_true(i32 %110)
  %112 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.24, i64 0, i64 0
  %113 = call i32 @test_start(i32 %112)
  %114 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.5, i64 0, i64 0
  %115 = call i32 @has_proper_punctuation(i32 %114)
  %116 = alloca i1, align 4
  store i1 %115, i1* %116, align 4
  ; Variable proper_punctuation allocated at %116
  %117 = load i32, i32* %116, align 4
  %118 = call i32 @assert_true(i32 %117)
  %119 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.25, i64 0, i64 0
  %120 = call i32 @test_start(i32 %119)
  %121 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %122 = call i32 @has_proper_punctuation(i32 %121)
  %123 = alloca i1, align 4
  store i1 %122, i1* %123, align 4
  ; Variable empty_punctuation allocated at %123
  %124 = load i32, i32* %123, align 4
  %125 = call i32 @assert_false(i32 %124)
  %126 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.26, i64 0, i64 0
  %127 = call i32 @test_start(i32 %126)
  %128 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.5, i64 0, i64 0
  %129 = call i32 @calculate_complexity_score(i32 %128)
  %130 = alloca i32, align 4
  store i32 %129, i32* %130, align 4
  ; Variable complexity_score allocated at %130
  %131 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %132 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %133 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.27, i64 0, i64 0
  %134 = call i32 @test_start(i32 %133)
  %135 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %136 = call i32 @calculate_complexity_score(i32 %135)
  %137 = alloca i32, align 4
  store i32 %136, i32* %137, align 4
  ; Variable empty_complexity allocated at %137
  %138 = load i32, i32* %137, align 4
  %139 = call i32 @assert_eq_int(i32 %138, i32 0)
  %140 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.28, i64 0, i64 0
  %141 = call i32 @test_start(i32 %140)
  %142 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %143 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.29, i64 0, i64 0
  %144 = call i32 @contains_pattern(i32 %142, i32 %143)
  %145 = alloca i1, align 4
  store i1 %144, i1* %145, align 4
  ; Variable pattern_found allocated at %145
  %146 = load i32, i32* %145, align 4
  %147 = call i32 @assert_true(i32 %146)
  %148 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.30, i64 0, i64 0
  %149 = call i32 @test_start(i32 %148)
  %150 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %151 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %152 = call i32 @contains_pattern(i32 %150, i32 %151)
  %153 = alloca i1, align 4
  store i1 %152, i1* %153, align 4
  ; Variable empty_pattern allocated at %153
  %154 = load i32, i32* %153, align 4
  %155 = call i32 @assert_true(i32 %154)
  %156 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.31, i64 0, i64 0
  %157 = call i32 @test_start(i32 %156)
  %158 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.1, i64 0, i64 0
  %159 = call i32 @validate_rule_structure(i32 %158)
  %160 = alloca i1, align 4
  store i1 %159, i1* %160, align 4
  ; Variable valid_structure allocated at %160
  %161 = load i32, i32* %160, align 4
  %162 = call i32 @assert_true(i32 %161)
  %163 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.32, i64 0, i64 0
  %164 = call i32 @test_start(i32 %163)
  %165 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.33, i64 0, i64 0
  %166 = call i32 @validate_rule_structure(i32 %165)
  %167 = alloca i1, align 4
  store i1 %166, i1* %167, align 4
  ; Variable short_structure allocated at %167
  %168 = load i32, i32* %167, align 4
  %169 = call i32 @assert_false(i32 %168)
  %170 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.34, i64 0, i64 0
  %171 = call i32 @test_start(i32 %170)
  %172 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.5, i64 0, i64 0
  %173 = call i32 @has_proper_capitalization(i32 %172)
  %174 = alloca i1, align 4
  store i1 %173, i1* %174, align 4
  ; Variable proper_capitalization allocated at %174
  %175 = load i32, i32* %174, align 4
  %176 = call i32 @assert_true(i32 %175)
  %177 = getelementptr inbounds [34 x i8], [34 x i8]* @.str.35, i64 0, i64 0
  %178 = call i32 @test_start(i32 %177)
  %179 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %180 = call i32 @has_proper_capitalization(i32 %179)
  %181 = alloca i1, align 4
  store i1 %180, i1* %181, align 4
  ; Variable empty_capitalization allocated at %181
  %182 = load i32, i32* %181, align 4
  %183 = call i32 @assert_false(i32 %182)
  %184 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.36, i64 0, i64 0
  %185 = call i32 @test_start(i32 %184)
  %186 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.1, i64 0, i64 0
  %187 = call i32 @parse_production_rule(i32 %186)
  %188 = alloca i1, align 4
  store i1 %187, i1* %188, align 4
  ; Variable parsed_rule allocated at %188
  %189 = load i32, i32* %188, align 4
  %190 = call i32 @assert_true(i32 %189)
  %191 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.37, i64 0, i64 0
  %192 = call i32 @test_start(i32 %191)
  %193 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %194 = call i32 @parse_production_rule(i32 %193)
  %195 = alloca i1, align 4
  store i1 %194, i1* %195, align 4
  ; Variable empty_parsed_rule allocated at %195
  %196 = load i32, i32* %195, align 4
  %197 = call i32 @assert_false(i32 %196)
  %198 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.38, i64 0, i64 0
  %199 = call i32 @test_start(i32 %198)
  %200 = getelementptr inbounds [16 x i8], [16 x i8]* @.str.39, i64 0, i64 0
  %201 = call i32 @count_character_types(i32 %200)
  %202 = alloca i32, align 4
  store i32 %201, i32* %202, align 4
  ; Variable char_count allocated at %202
  %203 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %204 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %205 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.40, i64 0, i64 0
  %206 = call i32 @test_start(i32 %205)
  %207 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.3, i64 0, i64 0
  %208 = call i32 @count_character_types(i32 %207)
  %209 = alloca i32, align 4
  store i32 %208, i32* %209, align 4
  ; Variable empty_char_count allocated at %209
  %210 = load i32, i32* %209, align 4
  %211 = call i32 @assert_eq_int(i32 %210, i32 0)
  %212 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.41, i64 0, i64 0
  %213 = call i32 @test_start(i32 %212)
  %214 = call i32 @char_is_uppercase(i32 65)
  %215 = alloca i1, align 4
  store i1 %214, i1* %215, align 4
  ; Variable uppercase_test allocated at %215
  %216 = load i32, i32* %215, align 4
  %217 = call i32 @assert_true(i32 %216)
  %218 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.42, i64 0, i64 0
  %219 = call i32 @test_start(i32 %218)
  %220 = call i32 @char_is_uppercase(i32 97)
  %221 = alloca i1, align 4
  store i1 %220, i1* %221, align 4
  ; Variable lowercase_test allocated at %221
  %222 = load i32, i32* %221, align 4
  %223 = call i32 @assert_false(i32 %222)
  %224 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.43, i64 0, i64 0
  %225 = call i32 @test_start(i32 %224)
  %226 = call i32 @char_is_lowercase(i32 97)
  %227 = alloca i1, align 4
  store i1 %226, i1* %227, align 4
  ; Variable lowercase_test2 allocated at %227
  %228 = load i32, i32* %227, align 4
  %229 = call i32 @assert_true(i32 %228)
  %230 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.44, i64 0, i64 0
  %231 = call i32 @test_start(i32 %230)
  %232 = call i32 @char_is_lowercase(i32 65)
  %233 = alloca i1, align 4
  store i1 %232, i1* %233, align 4
  ; Variable uppercase_test2 allocated at %233
  %234 = load i32, i32* %233, align 4
  %235 = call i32 @assert_false(i32 %234)
  %236 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.45, i64 0, i64 0
  %237 = call i32 @test_start(i32 %236)
  %238 = call i32 @char_is_letter(i32 97)
  %239 = alloca i1, align 4
  store i1 %238, i1* %239, align 4
  ; Variable letter_test allocated at %239
  %240 = load i32, i32* %239, align 4
  %241 = call i32 @assert_true(i32 %240)
  %242 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.46, i64 0, i64 0
  %243 = call i32 @test_start(i32 %242)
  %244 = call i32 @char_is_letter(i32 49)
  %245 = alloca i1, align 4
  store i1 %244, i1* %245, align 4
  ; Variable digit_test allocated at %245
  %246 = load i32, i32* %245, align 4
  %247 = call i32 @assert_false(i32 %246)
  %248 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.47, i64 0, i64 0
  %249 = call i32 @test_start(i32 %248)
  %250 = call i32 @char_is_digit(i32 53)
  %251 = alloca i1, align 4
  store i1 %250, i1* %251, align 4
  ; Variable digit_test2 allocated at %251
  %252 = load i32, i32* %251, align 4
  %253 = call i32 @assert_true(i32 %252)
  %254 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.48, i64 0, i64 0
  %255 = call i32 @test_start(i32 %254)
  %256 = call i32 @char_is_digit(i32 97)
  %257 = alloca i1, align 4
  store i1 %256, i1* %257, align 4
  ; Variable letter_test2 allocated at %257
  %258 = load i32, i32* %257, align 4
  %259 = call i32 @assert_false(i32 %258)
  %260 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.49, i64 0, i64 0
  %261 = call i32 @test_start(i32 %260)
  %262 = call i32 @char_is_alphanumeric(i32 97)
  %263 = alloca i1, align 4
  store i1 %262, i1* %263, align 4
  ; Variable alnum_test allocated at %263
  %264 = load i32, i32* %263, align 4
  %265 = call i32 @assert_true(i32 %264)
  %266 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.50, i64 0, i64 0
  %267 = call i32 @test_start(i32 %266)
  %268 = call i32 @char_is_alphanumeric(i32 53)
  %269 = alloca i1, align 4
  store i1 %268, i1* %269, align 4
  ; Variable alnum_test2 allocated at %269
  %270 = load i32, i32* %269, align 4
  %271 = call i32 @assert_true(i32 %270)
  %272 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.51, i64 0, i64 0
  %273 = call i32 @test_start(i32 %272)
  %274 = call i32 @char_is_alphanumeric(i32 46)
  %275 = alloca i1, align 4
  store i1 %274, i1* %275, align 4
  ; Variable punct_test allocated at %275
  %276 = load i32, i32* %275, align 4
  %277 = call i32 @assert_false(i32 %276)
  %278 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.52, i64 0, i64 0
  %279 = call i32 @test_start(i32 %278)
  %280 = call i32 @char_is_whitespace(i32 32)
  %281 = alloca i1, align 4
  store i1 %280, i1* %281, align 4
  ; Variable space_test allocated at %281
  %282 = load i32, i32* %281, align 4
  %283 = call i32 @assert_true(i32 %282)
  %284 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.53, i64 0, i64 0
  %285 = call i32 @test_start(i32 %284)
  %286 = call i32 @char_is_whitespace(i32 9)
  %287 = alloca i1, align 4
  store i1 %286, i1* %287, align 4
  ; Variable tab_test allocated at %287
  %288 = load i32, i32* %287, align 4
  %289 = call i32 @assert_true(i32 %288)
  %290 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.54, i64 0, i64 0
  %291 = call i32 @test_start(i32 %290)
  %292 = call i32 @char_is_whitespace(i32 97)
  %293 = alloca i1, align 4
  store i1 %292, i1* %293, align 4
  ; Variable letter_test3 allocated at %293
  %294 = load i32, i32* %293, align 4
  %295 = call i32 @assert_false(i32 %294)
  %296 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.55, i64 0, i64 0
  %297 = call i32 @test_start(i32 %296)
  %298 = call i32 @char_is_punctuation(i32 46)
  %299 = alloca i1, align 4
  store i1 %298, i1* %299, align 4
  ; Variable period_test allocated at %299
  %300 = load i32, i32* %299, align 4
  %301 = call i32 @assert_true(i32 %300)
  %302 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.56, i64 0, i64 0
  %303 = call i32 @test_start(i32 %302)
  %304 = call i32 @char_is_punctuation(i32 44)
  %305 = alloca i1, align 4
  store i1 %304, i1* %305, align 4
  ; Variable comma_test allocated at %305
  %306 = load i32, i32* %305, align 4
  %307 = call i32 @assert_true(i32 %306)
  %308 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.57, i64 0, i64 0
  %309 = call i32 @test_start(i32 %308)
  %310 = call i32 @char_is_punctuation(i32 97)
  %311 = alloca i1, align 4
  store i1 %310, i1* %311, align 4
  ; Variable letter_test4 allocated at %311
  %312 = load i32, i32* %311, align 4
  %313 = call i32 @assert_false(i32 %312)
  %314 = call i32 @print_test_summary()
  ret i32 0
}
