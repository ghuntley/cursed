; CURSED Language - Advanced LLVM Compilation
target triple = "x86_64-unknown-linux-gnu"


; Runtime function declarations
declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)
declare i32 @print(i8*)
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
declare void @cursed_panic(i8*, i64)
declare i8* @cursed_alloc(i64)
declare void @cursed_free(i8*)
declare i32 @cursed_goroutine_spawn(i8*)
declare i32 @cursed_channel_send(i8*, i64)
declare i32 @cursed_channel_receive(i8*, i64*)
declare void @cursed_channel_error(i32)
declare void @panic_non_exhaustive_match()
declare i1 @cursed_check_type_compatibility(i8*, i32, i32)
declare i1 @cursed_check_interface_type(i8*)
declare i1 @cursed_check_generic_type(i8*)
declare i1 @cursed_check_array_type(i8*)
declare i1 @cursed_check_function_type(i8*)
declare i8* @cursed_cast_type(i8*, i32, i32)
declare i8* @cursed_empty_string()
declare i8* @cursed_null_value()
declare void @cursed_panic_type_assertion(i32, i32)
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

declare i8* @cursed_error_init(i8*, i8*)
declare i8* @cursed_create_error(i8*)
declare i1 @cursed_is_error(i8*)
declare void @cursed_propagate_error(i8*)
declare void @cursed_try_begin()
declare void @cursed_try_end()
declare i8* @cursed_get_panic_value()
declare i8* @cursed_create_structured_error()
declare i8* @cursed_set_error_message(i8*, i8*)
declare i8* @cursed_set_error_code(i8*, i32)
declare i8* @cursed_set_error_details(i8*, i8*)
declare i8* @cursed_set_error_field(i8*, i8*, i8*)
declare i8* @cursed_get_error_field(i8*, i8*)
declare i32 @cursed_get_error_code(i8*)
declare i8* @cursed_get_error_message(i8*)
declare i8* @cursed_get_error_details(i8*)
declare void @cursed_enhanced_try_begin(i64)
declare void @cursed_enhanced_try_end(i64)
declare i8* @cursed_get_panic_context(i64)
declare i8* @cursed_extract_panic_value(i8*)
declare i8* @cursed_extract_stack_trace(i8*)
declare void @cursed_clear_panic_context(i64)
declare void @cursed_register_panic_handler(i64, i8*)
declare i8* @cursed_handle_panic(i64, i8*)
declare void @cursed_propagate_error_context(i64, i64)
declare i8* @cursed_get_goroutine_error_context(i64)
declare void @cursed_clear_goroutine_error_context(i64)
declare i8* @cursed_create_enhanced_context(i8*, i64)
declare i8* @cursed_link_error_context(i8*, i8*)
declare i8* @cursed_capture_stack_trace()
declare i64 @cursed_get_current_goroutine_id()
declare i64 @time(i64*)
declare i8* @cursed_propagate_with_context(i8*, i8*)
@error_msg_default = private unnamed_addr constant [15 x i8] c"Error occurred\00"

; Module Declarations from Imports
; mod module declarations
declare void @mod_init()
declare void @mod_cleanup()


; Interface value creation runtime function
declare i8* @cursed_create_interface_value(i8*, i8*, i8*)

; Interface value creation wrapper
define i8* @create_interface_value(i8* %vtable_ptr, i8* %data_ptr, i8* %type_name) {
entry:
    %interface_value = call i8* @cursed_create_interface_value(i8* %vtable_ptr, i8* %data_ptr, i8* %type_name)
    ret i8* %interface_value
}


; Method dispatch runtime function
declare i8* @cursed_dispatch_method(i8*, i8*, i8*, i32)

; Method dispatch wrapper with optimization
define i8* @dispatch_interface_method(i8* %interface_value, i8* %method_name, i8* %args, i32 %arg_count) {
entry:
    ; Extract vtable from interface value
    %interface_ptr = bitcast i8* %interface_value to {i8*, i8*}*
    %vtable_ptr_ptr = getelementptr {i8*, i8*}, {i8*, i8*}* %interface_ptr, i32 0, i32 0
    %vtable_ptr = load i8*, i8** %vtable_ptr_ptr
    
    ; Extract data pointer
    %data_ptr_ptr = getelementptr {i8*, i8*}, {i8*, i8*}* %interface_ptr, i32 0, i32 1
    %data_ptr = load i8*, i8** %data_ptr_ptr
    
    ; Dispatch method call
    %result = call i8* @cursed_dispatch_method(i8* %vtable_ptr, i8* %method_name, i8* %args, i32 %arg_count)
    ret i8* %result
}


; Interface type checking runtime function
declare i1 @cursed_implements_interface(i8*, i8*)

; Interface type checking wrapper
define i1 @check_interface_implementation(i8* %type_name, i8* %interface_name) {
entry:
    %result = call i1 @cursed_implements_interface(i8* %type_name, i8* %interface_name)
    ret i1 %result
}


; Runtime vtable lookup
declare i8* @cursed_runtime_get_vtable(i8*, i8*)

define i8* @get_vtable_runtime(i8* %type_name, i8* %interface_name) {
entry:
    %vtable = call i8* @cursed_runtime_get_vtable(i8* %type_name, i8* %interface_name)
    ret i8* %vtable
}


; Function: main

; String constants
@.str.20 = private unnamed_addr constant [42 x i8] c"    --version    Show version information\00", align 1
@.str.4 = private unnamed_addr constant [8 x i8] c" tokens\00", align 1
@.str.8 = private unnamed_addr constant [25 x i8] c"✅ Type checking passed\00", align 1
@.str.9 = private unnamed_addr constant [25 x i8] c"❌ Type checking failed\00", align 1
@.str.13 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.34 = private unnamed_addr constant [20 x i8] c"vibez.spill(\"test\")\00", align 1
@.str.25 = private unnamed_addr constant [19 x i8] c"Compilation failed\00", align 1
@.str.3 = private unnamed_addr constant [30 x i8] c"✅ Tokenization successful: \00", align 1
@.str.7 = private unnamed_addr constant [19 x i8] c"❌ Parsing failed\00", align 1
@.str.14 = private unnamed_addr constant [31 x i8] c"Generating code for AST node: \00", align 1
@.str.11 = private unnamed_addr constant [27 x i8] c"❌ Code generation failed\00", align 1
@.str.22 = private unnamed_addr constant [39 x i8] c"    --verbose    Enable verbose output\00", align 1
@.str.15 = private unnamed_addr constant [44 x i8] c"CURSED Stage 2 Self-Hosting Compiler v1.0.0\00", align 1
@.str.28 = private unnamed_addr constant [22 x i8] c"🔧 Stage 2: Parsing\00", align 1
@.str.18 = private unnamed_addr constant [40 x i8] c"    cursed_stage2 [input.csd] [OPTIONS]\00", align 1
@.str.19 = private unnamed_addr constant [9 x i8] c"OPTIONS:\00", align 1
@.str.10 = private unnamed_addr constant [31 x i8] c"✅ Code generation successful\00", align 1
@.str.27 = private unnamed_addr constant [25 x i8] c"Error: Empty source file\00", align 1
@.str.35 = private unnamed_addr constant [33 x i8] c"✅ Stage 2 compiler test passed\00", align 1
@.str.5 = private unnamed_addr constant [24 x i8] c"❌ Tokenization failed\00", align 1
@.str.37 = private unnamed_addr constant [23 x i8] c"Stage 2 Compiler Tests\00", align 1
@.str.2 = private unnamed_addr constant [33 x i8] c"Starting compilation pipeline...\00", align 1
@.str.6 = private unnamed_addr constant [38 x i8] c"✅ Parsing successful: AST generated\00", align 1
@.str.17 = private unnamed_addr constant [7 x i8] c"USAGE:\00", align 1
@.str.21 = private unnamed_addr constant [40 x i8] c"    --help       Show this help message\00", align 1
@.str.16 = private unnamed_addr constant [44 x i8] c"Built with pure CURSED - fully self-hosting\00", align 1
@.str.32 = private unnamed_addr constant [40 x i8] c"✨ Compilation completed successfully!\00", align 1
@.str.36 = private unnamed_addr constant [33 x i8] c"❌ Stage 2 compiler test failed\00", align 1
@.str.1 = private unnamed_addr constant [35 x i8] c"vibez.spill(\"Hello from Stage 2!\")\00", align 1
@.str.31 = private unnamed_addr constant [29 x i8] c"💾 Stage 5: Writing Output\00", align 1
@.str.24 = private unnamed_addr constant [25 x i8] c"Compilation successful: \00", align 1
@.str.33 = private unnamed_addr constant [42 x i8] c"Testing Stage 2 compiler functionality...\00", align 1
@.str.23 = private unnamed_addr constant [37 x i8] c"Processing command line arguments...\00", align 1
@.str.12 = private unnamed_addr constant [58 x i8] c"🎉 Stage 2 compilation pipeline completed successfully!\00", align 1
@.str.29 = private unnamed_addr constant [28 x i8] c"🧠 Stage 3: Type Checking\00", align 1
@.str.0 = private unnamed_addr constant [52 x i8] c"🚀 CURSED Stage 2 Compiler - Self-Hosting Edition\00", align 1
@.str.26 = private unnamed_addr constant [31 x i8] c"🔍 Stage 1: Lexical Analysis\00", align 1
@.str.30 = private unnamed_addr constant [29 x i8] c"⚡ Stage 4: Code Generation\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [52 x i8], [52 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %1 = getelementptr inbounds [35 x i8], [35 x i8]* @.str.1, i64 0, i64 0
  %2 = alloca i8*, align 4
  store i8* %1, i8** %2, align 4
  ; Variable test_source allocated at %2
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %2 = alloca { i32 }, align 8
  %3 = getelementptr inbounds { i32 }, { i32 }* %2, i32 0, i32 0
  store i32 %verbose_mode, i32* %3, align 4
  br i1 %2, label %label0, label %label1
label0:
  %3 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.2, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  br label %label2
label1:
  br label %label2
label2:
  %5 = load i32, i32* %2, align 4
  %6 = call i32 @tokenize_source(i32 %5)
  %7 = alloca i32, align 4
  store i32 %6, i32* %7, align 4
  ; Variable tokens allocated at %7
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %7 = load i32, i32* %7, align 4
  %8 = icmp sgt i32 %7, 0
  %9 = alloca { i8* }, align 8
  %10 = getelementptr inbounds { i8* }, { i8* }* %9, i32 0, i32 0
  store i8* %8, i8** %10, align 4
  br i1 %9, label %label3, label %label4
label3:
  %10 = getelementptr inbounds [30 x i8], [30 x i8]* @.str.3, i64 0, i64 0
  %11 = call i32 @tokens_to_string()
  %12 = add i32 %10, %11
  %13 = getelementptr inbounds [8 x i8], [8 x i8]* @.str.4, i64 0, i64 0
  %14 = add i32 %12, %13
  %15 = call i32 @puts(i8* %14)
  br label %label5
label4:
  %15 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.5, i64 0, i64 0
  %16 = call i32 @puts(i8* %15)
  ret i32 1
  br label %label5
label5:
  %16 = load i32, i32* %7, align 4
  %17 = call i32 @parse_tokens(i32 %16)
  %18 = alloca i32, align 4
  store i32 %17, i32* %18, align 4
  ; Variable ast_root allocated at %18
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %18 = load i32, i32* %18, align 4
  %19 = icmp sgt i32 %18, 0
  %20 = alloca { i8* }, align 8
  %21 = getelementptr inbounds { i8* }, { i8* }* %20, i32 0, i32 0
  store i8* %19, i8** %21, align 4
  br i1 %20, label %label6, label %label7
label6:
  %21 = getelementptr inbounds [38 x i8], [38 x i8]* @.str.6, i64 0, i64 0
  %22 = call i32 @puts(i8* %21)
  br label %label8
label7:
  %22 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.7, i64 0, i64 0
  %23 = call i32 @puts(i8* %22)
  ret i32 1
  br label %label8
label8:
  %23 = load i32, i32* %18, align 4
  %24 = call i32 @validate_ast(i32 %23)
  %25 = alloca i1, align 4
  store i1 %24, i1* %25, align 4
  ; Variable type_check_result allocated at %25
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %25 = load i32, i32* %25, align 4
  %26 = alloca { i32 }, align 8
  %27 = getelementptr inbounds { i32 }, { i32 }* %26, i32 0, i32 0
  store i32 %25, i32* %27, align 4
  br i1 %26, label %label9, label %label10
label9:
  %27 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.8, i64 0, i64 0
  %28 = call i32 @puts(i8* %27)
  br label %label11
label10:
  %28 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.9, i64 0, i64 0
  %29 = call i32 @puts(i8* %28)
  ret i32 1
  br label %label11
label11:
  %29 = load i32, i32* %18, align 4
  %30 = call i32 @generate_code(i32 %29)
  %31 = alloca i1, align 4
  store i1 %30, i1* %31, align 4
  ; Variable codegen_result allocated at %31
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %31 = load i32, i32* %31, align 4
  %32 = alloca { i32 }, align 8
  %33 = getelementptr inbounds { i32 }, { i32 }* %32, i32 0, i32 0
  store i32 %31, i32* %33, align 4
  br i1 %32, label %label12, label %label13
label12:
  %33 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.10, i64 0, i64 0
  %34 = call i32 @puts(i8* %33)
  br label %label14
label13:
  %34 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.11, i64 0, i64 0
  %35 = call i32 @puts(i8* %34)
  ret i32 1
  br label %label14
label14:
  %34 = getelementptr inbounds [58 x i8], [58 x i8]* @.str.12, i64 0, i64 0
  %35 = call i32 @puts(i8* %34)
  ret i32 0
}

; Function: tokenize_source
define i32 @tokenize_source(i8* %arg_0) {
entry:
  %0 = alloca i8*, align 4
  store i8* %arg_0, i8** %0, align 4
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %0 = load i32, i32* %0, align 4
  %1 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.13, i64 0, i64 0
  %2 = icmp eq i32 %0, %1
  %3 = alloca { i8* }, align 8
  %4 = getelementptr inbounds { i8* }, { i8* }* %3, i32 0, i32 0
  store i8* %2, i8** %4, align 4
  br i1 %3, label %label15, label %label16
label15:
  ret i32 0
  br label %label17
label16:
  br label %label17
label17:
  %3 = alloca i32, align 4
  store i32 10, i32* %3, align 4
  ; Variable token_count allocated at %3
  %4 = load i32, i32* %3, align 4
  ret i32 %4
}

; Function: parse_tokens
define i32 @parse_tokens(i32 %arg_0) {
entry:
  %0 = alloca i32, align 4
  store i32 %arg_0, i32* %0, align 4
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %0 = load i32, i32* %0, align 4
  %1 = icmp eq i32 %0, 0
  %2 = alloca { i8* }, align 8
  %3 = getelementptr inbounds { i8* }, { i8* }* %2, i32 0, i32 0
  store i8* %1, i8** %3, align 4
  br i1 %2, label %label18, label %label19
label18:
  ret i32 0
  br label %label20
label19:
  br label %label20
label20:
  %2 = alloca i32, align 4
  store i32 1, i32* %2, align 4
  ; Variable ast_node allocated at %2
  %3 = load i32, i32* %2, align 4
  ret i32 %3
}

; Function: validate_ast
define i1 @validate_ast(i32 %arg_0) {
entry:
  %0 = alloca i32, align 4
  store i32 %arg_0, i32* %0, align 4
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %0 = load i32, i32* %0, align 4
  %1 = icmp eq i32 %0, 0
  %2 = alloca { i8* }, align 8
  %3 = getelementptr inbounds { i8* }, { i8* }* %2, i32 0, i32 0
  store i8* %1, i8** %3, align 4
  br i1 %2, label %label21, label %label22
label21:
  ret i1 0
  br label %label23
label22:
  br label %label23
label23:
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %3 = load i32, i32* %0, align 4
  %4 = icmp sgt i32 %3, 0
  %5 = alloca { i8* }, align 8
  %6 = getelementptr inbounds { i8* }, { i8* }* %5, i32 0, i32 0
  store i8* %4, i8** %6, align 4
  br i1 %5, label %label24, label %label25
label24:
  ret i1 1
  br label %label26
label25:
  br label %label26
label26:
  ret i1 0
}

; Function: generate_code
define i1 @generate_code(i32 %arg_0) {
entry:
  %0 = alloca i32, align 4
  store i32 %arg_0, i32* %0, align 4
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %0 = load i32, i32* %0, align 4
  %1 = icmp eq i32 %0, 0
  %2 = alloca { i8* }, align 8
  %3 = getelementptr inbounds { i8* }, { i8* }* %2, i32 0, i32 0
  store i8* %1, i8** %3, align 4
  br i1 %2, label %label27, label %label28
label27:
  ret i1 0
  br label %label29
label28:
  br label %label29
label29:
  %3 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.14, i64 0, i64 0
  %4 = call i32 @ast_root_to_string()
  %5 = add i32 %3, %4
  %6 = call i32 @puts(i8* %5)
  ret i1 1
}

; Function: print_version
define i32 @print_version() {
entry:
  %0 = getelementptr inbounds [44 x i8], [44 x i8]* @.str.15, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %1 = getelementptr inbounds [44 x i8], [44 x i8]* @.str.16, i64 0, i64 0
  %2 = call i32 @puts(i8* %1)
  ret i32 0
}

; Function: print_help
define i32 @print_help() {
entry:
  %0 = call i32 @print_version()
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.13, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %1 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.17, i64 0, i64 0
  %2 = call i32 @puts(i8* %1)
  %2 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.18, i64 0, i64 0
  %3 = call i32 @puts(i8* %2)
  %3 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.13, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %4 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.19, i64 0, i64 0
  %5 = call i32 @puts(i8* %4)
  %5 = getelementptr inbounds [42 x i8], [42 x i8]* @.str.20, i64 0, i64 0
  %6 = call i32 @puts(i8* %5)
  %6 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.21, i64 0, i64 0
  %7 = call i32 @puts(i8* %6)
  %7 = getelementptr inbounds [39 x i8], [39 x i8]* @.str.22, i64 0, i64 0
  %8 = call i32 @puts(i8* %7)
  ret i32 0
}

; Function: parse_args
define i32 @parse_args(i32 %arg_0) {
entry:
  %0 = alloca i32, align 4
  store i32 %arg_0, i32* %0, align 4
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %0 = load i32, i32* %0, align 4
  %1 = icmp sgt i32 %0, 1
  %2 = alloca { i8* }, align 8
  %3 = getelementptr inbounds { i8* }, { i8* }* %2, i32 0, i32 0
  store i8* %1, i8** %3, align 4
  br i1 %2, label %label30, label %label31
label30:
  %3 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.23, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.13, i64 0, i64 0
  br label %label32
label31:
  br label %label32
label32:
  ret i32 0
}

; Function: create_compilation_result
define i32 @create_compilation_result(i1 %arg_0, i8* %arg_1) {
entry:
  %0 = alloca i1, align 4
  store i1 %arg_0, i1* %0, align 4
  %1 = alloca i8*, align 4
  store i8* %arg_1, i8** %1, align 4
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %1 = load i32, i32* %0, align 4
  %2 = alloca { i32 }, align 8
  %3 = getelementptr inbounds { i32 }, { i32 }* %2, i32 0, i32 0
  store i32 %1, i32* %3, align 4
  br i1 %2, label %label33, label %label34
label33:
  %3 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.24, i64 0, i64 0
  %4 = load i32, i32* %1, align 4
  %5 = add i32 %3, %4
  %6 = call i32 @puts(i8* %5)
  ret i32 1
  br label %label35
label34:
  %5 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.25, i64 0, i64 0
  %6 = call i32 @puts(i8* %5)
  ret i32 0
  br label %label35
label35:
  ret i32 0
}

; Function: compile_program
define i1 @compile_program(i8* %arg_0, i1 %arg_1) {
entry:
  %0 = alloca i8*, align 4
  store i8* %arg_0, i8** %0, align 4
  %1 = alloca i1, align 4
  store i1 %arg_1, i1* %1, align 4
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %1 = load i32, i32* %1, align 4
  %2 = alloca { i32 }, align 8
  %3 = getelementptr inbounds { i32 }, { i32 }* %2, i32 0, i32 0
  store i32 %1, i32* %3, align 4
  br i1 %2, label %label36, label %label37
label36:
  %3 = getelementptr inbounds [31 x i8], [31 x i8]* @.str.26, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  br label %label38
label37:
  br label %label38
label38:
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %4 = load i32, i32* %0, align 4
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.13, i64 0, i64 0
  %6 = icmp eq i32 %4, %5
  %7 = alloca { i8* }, align 8
  %8 = getelementptr inbounds { i8* }, { i8* }* %7, i32 0, i32 0
  store i8* %6, i8** %8, align 4
  br i1 %7, label %label39, label %label40
label39:
  %8 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.27, i64 0, i64 0
  %9 = call i32 @puts(i8* %8)
  ret i1 0
  br label %label41
label40:
  br label %label41
label41:
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %9 = load i32, i32* %1, align 4
  %10 = alloca { i32 }, align 8
  %11 = getelementptr inbounds { i32 }, { i32 }* %10, i32 0, i32 0
  store i32 %9, i32* %11, align 4
  br i1 %10, label %label42, label %label43
label42:
  %11 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.28, i64 0, i64 0
  %12 = call i32 @puts(i8* %11)
  %12 = getelementptr inbounds [28 x i8], [28 x i8]* @.str.29, i64 0, i64 0
  %13 = call i32 @puts(i8* %12)
  %13 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.30, i64 0, i64 0
  %14 = call i32 @puts(i8* %13)
  %14 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.31, i64 0, i64 0
  %15 = call i32 @puts(i8* %14)
  %15 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.32, i64 0, i64 0
  %16 = call i32 @puts(i8* %15)
  br label %label44
label43:
  br label %label44
label44:
  ret i1 1
}

; Function: test_stage2_compiler
define i1 @test_stage2_compiler() {
entry:
  %0 = getelementptr inbounds [42 x i8], [42 x i8]* @.str.33, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %1 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.34, i64 0, i64 0
  %2 = alloca i8*, align 4
  store i8* %1, i8** %2, align 4
  ; Variable test_input allocated at %2
  %3 = load i32, i32* %2, align 4
  %4 = call i32 @compile_program(i32 %3, i32 1)
  %5 = alloca i1, align 4
  store i1 %4, i1* %5, align 4
  ; Variable result allocated at %5
  ; DEBUG: generate_if_statement_with_init called
  ; DEBUG: about to process condition
  %5 = load i32, i32* %5, align 4
  %6 = alloca { i32 }, align 8
  %7 = getelementptr inbounds { i32 }, { i32 }* %6, i32 0, i32 0
  store i32 %5, i32* %7, align 4
  br i1 %6, label %label45, label %label46
label45:
  %7 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.35, i64 0, i64 0
  %8 = call i32 @puts(i8* %7)
  ret i1 1
  br label %label47
label46:
  %8 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.36, i64 0, i64 0
  %9 = call i32 @puts(i8* %8)
  ret i1 0
  br label %label47
label47:
  ret i1 null
}

; Function: run_tests
define i32 @run_tests() {
entry:
  %0 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.37, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  %2 = call i32 @test_stage2_compiler()
  %3 = call i32 @assert_true(i32 %2)
  %4 = call i32 @print_test_summary()
  ret i32 0
}
