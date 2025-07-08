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

declare i8* @cursed_error_init(i8*, i8*)
declare i8* @cursed_create_error(i8*)
declare i1 @cursed_is_error(i8*)
declare void @cursed_propagate_error(i8*)
declare void @cursed_try_begin()
declare void @cursed_try_end()
declare i8* @cursed_get_panic_value()
declare i8* @malloc(i32)
declare void @free(i8*)
@error_msg_default = private unnamed_addr constant [13 x i8] c"Error occurred\00"
define i32 @test_tcp_socket_creation() {
entry:
  %0 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.0, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @tcp_create()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable socket1 allocated
  %4 = call i32 @tcp_create()
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable socket2 allocated
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %6
  %7 = load i32, i32* %5, align 4
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %9
  ; Expression result: 0
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  ; Expression result: 0
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %12
  %13 = load i32, i32* %3, align 4
  %14 = call i32 @tcp_close(i32 %13)
  ; Expression result: %14
  %15 = load i32, i32* %5, align 4
  %16 = call i32 @tcp_close(i32 %15)
  ; Expression result: %16
  ret i32 0
}

define i32 @test_tcp_bind_operations() {
entry:
  %0 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.2, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @tcp_create()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable socket allocated
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %6 = call i32 @tcp_bind(i32 %4, i32 %5, i32 8080)
  %7 = alloca i32, align 4
  store i32 %6, i32* %7, align 4
  ; Variable bind_result allocated
  %8 = load i32, i32* %7, align 4
  %9 = call i32 @assert_eq_int(i32 %8, i32 0)
  ; Expression result: %9
  %10 = call i32 @tcp_create()
  %11 = alloca i32, align 4
  store i32 %10, i32* %11, align 4
  ; Variable socket2 allocated
  %12 = load i32, i32* %11, align 4
  %13 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %14 = call i32 @tcp_bind(i32 %12, i32 %13, i32 8080)
  %15 = alloca i32, align 4
  store i32 %14, i32* %15, align 4
  ; Variable bind_result2 allocated
  ; Expression result: 1
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %16
  %17 = load i32, i32* %3, align 4
  %18 = call i32 @tcp_close(i32 %17)
  ; Expression result: %18
  %19 = load i32, i32* %11, align 4
  %20 = call i32 @tcp_close(i32 %19)
  ; Expression result: %20
  ret i32 0
}

define i32 @test_tcp_connect_operations() {
entry:
  %0 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.4, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @tcp_create()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable socket allocated
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %6 = call i32 @tcp_connect(i32 %4, i32 %5, i32 80)
  %7 = alloca i32, align 4
  store i32 %6, i32* %7, align 4
  ; Variable connect_result allocated
  %8 = load i32, i32* %7, align 4
  %9 = call i32 @assert_eq_int(i32 %8, i32 0)
  ; Expression result: %9
  %10 = load i32, i32* %3, align 4
  %11 = call i32 @tcp_close(i32 %10)
  ; Expression result: %11
  ret i32 0
}

define i32 @test_tcp_listen_operations() {
entry:
  %0 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.5, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @tcp_create()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable socket allocated
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %6 = call i32 @tcp_bind(i32 %4, i32 %5, i32 8081)
  %7 = alloca i32, align 4
  store i32 %6, i32* %7, align 4
  ; Variable bind_result allocated
  %8 = load i32, i32* %7, align 4
  %9 = call i32 @assert_eq_int(i32 %8, i32 0)
  ; Expression result: %9
  %10 = load i32, i32* %3, align 4
  %11 = call i32 @tcp_listen(i32 %10, i32 5)
  %12 = alloca i32, align 4
  store i32 %11, i32* %12, align 4
  ; Variable listen_result allocated
  %13 = load i32, i32* %12, align 4
  %14 = call i32 @assert_eq_int(i32 %13, i32 0)
  ; Expression result: %14
  %15 = load i32, i32* %3, align 4
  %16 = call i32 @tcp_close(i32 %15)
  ; Expression result: %16
  ret i32 0
}

define i32 @test_tcp_accept_operations() {
entry:
  %0 = getelementptr inbounds [22 x i8], [22 x i8]* @.str.6, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @tcp_create()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable server_socket allocated
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %6 = call i32 @tcp_bind(i32 %4, i32 %5, i32 8082)
  ; Expression result: %6
  %7 = load i32, i32* %3, align 4
  %8 = call i32 @tcp_listen(i32 %7, i32 5)
  ; Expression result: %8
  %9 = load i32, i32* %3, align 4
  %10 = call i32 @tcp_accept(i32 %9)
  %11 = alloca i32, align 4
  store i32 %10, i32* %11, align 4
  ; Variable client_socket allocated
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %12
  ; Expression result: 0
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %13
  %14 = load i32, i32* %3, align 4
  %15 = call i32 @tcp_close(i32 %14)
  ; Expression result: %15
  %16 = load i32, i32* %11, align 4
  %17 = call i32 @tcp_close(i32 %16)
  ; Expression result: %17
  ret i32 0
}

define i32 @test_tcp_send_recv_operations() {
entry:
  %0 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.7, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @tcp_create()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable socket allocated
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %6 = call i32 @tcp_connect(i32 %4, i32 %5, i32 80)
  ; Expression result: %6
  %7 = load i32, i32* %3, align 4
  %8 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.8, i64 0, i64 0
  %9 = call i32 @tcp_send(i32 %7, i32 %8)
  %10 = alloca i32, align 4
  store i32 %9, i32* %10, align 4
  ; Variable send_result allocated
  %11 = load i32, i32* %10, align 4
  %12 = call i32 @assert_eq_int(i32 %11, i32 11)
  ; Expression result: %12
  %13 = load i32, i32* %3, align 4
  %14 = call i32 @tcp_recv(i32 %13, i32 1024)
  %15 = alloca i32, align 4
  store i32 %14, i32* %15, align 4
  ; Variable recv_result allocated
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %16
  ; Expression result: 0
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = load i32, i32* %3, align 4
  %19 = call i32 @tcp_close(i32 %18)
  ; Expression result: %19
  ret i32 0
}

define i32 @test_udp_socket_creation() {
entry:
  %0 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.9, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @udp_create()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable socket1 allocated
  %4 = call i32 @udp_create()
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable socket2 allocated
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %6
  %7 = load i32, i32* %5, align 4
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %9
  ; Expression result: 0
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %10
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  ; Expression result: 0
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %12
  %13 = load i32, i32* %3, align 4
  %14 = call i32 @udp_close(i32 %13)
  ; Expression result: %14
  %15 = load i32, i32* %5, align 4
  %16 = call i32 @udp_close(i32 %15)
  ; Expression result: %16
  ret i32 0
}

define i32 @test_udp_bind_operations() {
entry:
  %0 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.10, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @udp_create()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable socket allocated
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %6 = call i32 @udp_bind(i32 %4, i32 %5, i32 9090)
  %7 = alloca i32, align 4
  store i32 %6, i32* %7, align 4
  ; Variable bind_result allocated
  %8 = load i32, i32* %7, align 4
  %9 = call i32 @assert_eq_int(i32 %8, i32 0)
  ; Expression result: %9
  %10 = call i32 @udp_create()
  %11 = alloca i32, align 4
  store i32 %10, i32* %11, align 4
  ; Variable socket2 allocated
  %12 = load i32, i32* %11, align 4
  %13 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %14 = call i32 @udp_bind(i32 %12, i32 %13, i32 9090)
  %15 = alloca i32, align 4
  store i32 %14, i32* %15, align 4
  ; Variable bind_result2 allocated
  ; Expression result: 1
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %16
  %17 = load i32, i32* %3, align 4
  %18 = call i32 @udp_close(i32 %17)
  ; Expression result: %18
  %19 = load i32, i32* %11, align 4
  %20 = call i32 @udp_close(i32 %19)
  ; Expression result: %20
  ret i32 0
}

define i32 @test_udp_send_recv_operations() {
entry:
  %0 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.11, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @udp_create()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable socket allocated
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %6 = call i32 @udp_bind(i32 %4, i32 %5, i32 9091)
  ; Expression result: %6
  %7 = load i32, i32* %3, align 4
  %8 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.12, i64 0, i64 0
  %9 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %10 = call i32 @udp_send_to(i32 %7, i32 %8, i32 %9, i32 9092)
  %11 = alloca i32, align 4
  store i32 %10, i32* %11, align 4
  ; Variable send_result allocated
  %12 = load i32, i32* %11, align 4
  %13 = call i32 @assert_eq_int(i32 %12, i32 8)
  ; Expression result: %13
  %14 = load i32, i32* %3, align 4
  %15 = call i32 @udp_recv_from(i32 %14, i32 1024)
  %16 = alloca i32, align 4
  store i32 %15, i32* %16, align 4
  ; Variable recv_result allocated
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  ; Expression result: 0
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %18
  %19 = load i32, i32* %3, align 4
  %20 = call i32 @udp_close(i32 %19)
  ; Expression result: %20
  ret i32 0
}

define i32 @test_hostname_resolution() {
entry:
  %0 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.13, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.14, i64 0, i64 0
  %3 = call i32 @resolve_hostname(i32 %2)
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable ip allocated
  %5 = load i32, i32* %4, align 4
  %6 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %7 = call i32 @assert_eq_string(i32 %5, i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.15, i64 0, i64 0
  %9 = call i32 @resolve_hostname(i32 %8)
  %10 = alloca i32, align 4
  store i32 %9, i32* %10, align 4
  ; Variable example_ip allocated
  %11 = load i32, i32* %10, align 4
  %12 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.16, i64 0, i64 0
  %13 = call i32 @assert_eq_string(i32 %11, i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.17, i64 0, i64 0
  %15 = call i32 @resolve_hostname(i32 %14)
  %16 = alloca i32, align 4
  store i32 %15, i32* %16, align 4
  ; Variable google_ip allocated
  %17 = load i32, i32* %16, align 4
  %18 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.18, i64 0, i64 0
  %19 = call i32 @assert_eq_string(i32 %17, i32 %18)
  ; Expression result: %19
  %20 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.19, i64 0, i64 0
  %21 = call i32 @resolve_hostname(i32 %20)
  %22 = alloca i32, align 4
  store i32 %21, i32* %22, align 4
  ; Variable github_ip allocated
  %23 = load i32, i32* %22, align 4
  %24 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.20, i64 0, i64 0
  %25 = call i32 @assert_eq_string(i32 %23, i32 %24)
  ; Expression result: %25
  %26 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.21, i64 0, i64 0
  %27 = call i32 @resolve_hostname(i32 %26)
  %28 = alloca i32, align 4
  store i32 %27, i32* %28, align 4
  ; Variable unknown_ip allocated
  %29 = load i32, i32* %28, align 4
  %30 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.22, i64 0, i64 0
  %31 = call i32 @assert_eq_string(i32 %29, i32 %30)
  ; Expression result: %31
  ret i32 0
}

define i32 @test_reverse_dns_resolution() {
entry:
  %0 = getelementptr inbounds [23 x i8], [23 x i8]* @.str.23, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %3 = call i32 @resolve_ip(i32 %2)
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable hostname allocated
  %5 = load i32, i32* %4, align 4
  %6 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.14, i64 0, i64 0
  %7 = call i32 @assert_eq_string(i8* %5, i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.16, i64 0, i64 0
  %9 = call i32 @resolve_ip(i32 %8)
  %10 = alloca i32, align 4
  store i32 %9, i32* %10, align 4
  ; Variable example_hostname allocated
  %11 = load i32, i32* %10, align 4
  %12 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.15, i64 0, i64 0
  %13 = call i32 @assert_eq_string(i8* %11, i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.18, i64 0, i64 0
  %15 = call i32 @resolve_ip(i32 %14)
  %16 = alloca i32, align 4
  store i32 %15, i32* %16, align 4
  ; Variable google_hostname allocated
  %17 = load i32, i32* %16, align 4
  %18 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.17, i64 0, i64 0
  %19 = call i32 @assert_eq_string(i8* %17, i32 %18)
  ; Expression result: %19
  %20 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.20, i64 0, i64 0
  %21 = call i32 @resolve_ip(i32 %20)
  %22 = alloca i32, align 4
  store i32 %21, i32* %22, align 4
  ; Variable github_hostname allocated
  %23 = load i32, i32* %22, align 4
  %24 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.19, i64 0, i64 0
  %25 = call i32 @assert_eq_string(i8* %23, i32 %24)
  ; Expression result: %25
  %26 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.24, i64 0, i64 0
  %27 = call i32 @resolve_ip(i32 %26)
  %28 = alloca i32, align 4
  store i32 %27, i32* %28, align 4
  ; Variable unknown_hostname allocated
  %29 = load i32, i32* %28, align 4
  %30 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.24, i64 0, i64 0
  %31 = call i32 @assert_eq_string(i8* %29, i32 %30)
  ; Expression result: %31
  ret i32 0
}

define i32 @test_mx_record_lookup() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.25, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.15, i64 0, i64 0
  %3 = call i32 @lookup_mx(i32 %2)
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable mx_record allocated
  %5 = load i32, i32* %4, align 4
  %6 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.26, i64 0, i64 0
  %7 = call i32 @assert_eq_string(i32 %5, i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.17, i64 0, i64 0
  %9 = call i32 @lookup_mx(i32 %8)
  %10 = alloca i32, align 4
  store i32 %9, i32* %10, align 4
  ; Variable google_mx allocated
  %11 = load i32, i32* %10, align 4
  %12 = getelementptr inbounds [27 x i8], [27 x i8]* @.str.27, i64 0, i64 0
  %13 = call i32 @assert_eq_string(i32 %11, i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.21, i64 0, i64 0
  %15 = call i32 @lookup_mx(i32 %14)
  %16 = alloca i32, align 4
  store i32 %15, i32* %16, align 4
  ; Variable unknown_mx allocated
  %17 = load i32, i32* %16, align 4
  %18 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.28, i64 0, i64 0
  %19 = call i32 @assert_eq_string(i32 %17, i32 %18)
  ; Expression result: %19
  ret i32 0
}

define i32 @test_txt_record_lookup() {
entry:
  %0 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.29, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.15, i64 0, i64 0
  %3 = call i32 @lookup_txt(i32 %2)
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable txt_record allocated
  %5 = load i32, i32* %4, align 4
  %6 = getelementptr inbounds [37 x i8], [37 x i8]* @.str.30, i64 0, i64 0
  %7 = call i32 @assert_eq_string(i32 %5, i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [11 x i8], [11 x i8]* @.str.17, i64 0, i64 0
  %9 = call i32 @lookup_txt(i32 %8)
  %10 = alloca i32, align 4
  store i32 %9, i32* %10, align 4
  ; Variable google_txt allocated
  %11 = load i32, i32* %10, align 4
  %12 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.31, i64 0, i64 0
  %13 = call i32 @assert_eq_string(i32 %11, i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.21, i64 0, i64 0
  %15 = call i32 @lookup_txt(i32 %14)
  %16 = alloca i32, align 4
  store i32 %15, i32* %16, align 4
  ; Variable unknown_txt allocated
  %17 = load i32, i32* %16, align 4
  %18 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.32, i64 0, i64 0
  %19 = call i32 @assert_eq_string(i32 %17, i32 %18)
  ; Expression result: %19
  ret i32 0
}

define i32 @test_http_get_request() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.33, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.34, i64 0, i64 0
  %3 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.35, i64 0, i64 0
  %4 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %5 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %6 = call i32 @http_send(i32 %2, i32 %3, i32 %4, i32 %5)
  %7 = alloca i32, align 4
  store i32 %6, i32* %7, align 4
  ; Variable response allocated
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  ; Expression result: 0
  %9 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %9
  %10 = load i32, i32* %7, align 4
  %11 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.36, i64 0, i64 0
  %12 = call i32 @string_contains(i32 %10, i32 %11)
  %13 = call i32 @assert_true(i32 %12)
  ; Expression result: %13
  %14 = load i32, i32* %7, align 4
  %15 = getelementptr inbounds [7 x i8], [7 x i8]* @.str.37, i64 0, i64 0
  %16 = call i32 @string_contains(i32 %14, i32 %15)
  %17 = call i32 @assert_true(i32 %16)
  ; Expression result: %17
  ret i32 0
}

define i32 @test_http_post_request() {
entry:
  %0 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.38, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.39, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable body allocated
  %4 = getelementptr inbounds [48 x i8], [48 x i8]* @.str.40, i64 0, i64 0
  %5 = alloca i8*, align 4
  store i8* %4, i8** %5, align 4
  ; Variable headers allocated
  %6 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.41, i64 0, i64 0
  %7 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.35, i64 0, i64 0
  %8 = load i8*, i8** %5, align 4
  %9 = load i8*, i8** %3, align 4
  %10 = call i32 @http_send(i32 %6, i32 %7, i32 %8, i32 %9)
  %11 = alloca i32, align 4
  store i32 %10, i32* %11, align 4
  ; Variable response allocated
  %12 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %12
  ; Expression result: 0
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %13
  %14 = load i32, i32* %11, align 4
  %15 = getelementptr inbounds [9 x i8], [9 x i8]* @.str.36, i64 0, i64 0
  %16 = call i32 @string_contains(i32 %14, i32 %15)
  %17 = call i32 @assert_true(i32 %16)
  ; Expression result: %17
  ret i32 0
}

define i32 @test_http_url_parsing() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.42, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.43, i64 0, i64 0
  %3 = call i32 @extract_host_from_url(i32 %2)
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable host1 allocated
  %5 = load i32, i32* %4, align 4
  %6 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.15, i64 0, i64 0
  %7 = call i32 @assert_eq_string(i32 %5, i32 %6)
  ; Expression result: %7
  %8 = getelementptr inbounds [33 x i8], [33 x i8]* @.str.44, i64 0, i64 0
  %9 = call i32 @extract_host_from_url(i32 %8)
  %10 = alloca i32, align 4
  store i32 %9, i32* %10, align 4
  ; Variable host2 allocated
  %11 = load i32, i32* %10, align 4
  %12 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.45, i64 0, i64 0
  %13 = call i32 @assert_eq_string(i32 %11, i32 %12)
  ; Expression result: %13
  %14 = getelementptr inbounds [20 x i8], [20 x i8]* @.str.35, i64 0, i64 0
  %15 = call i32 @extract_port_from_url(i32 %14)
  %16 = alloca i32, align 4
  store i32 %15, i32* %16, align 4
  ; Variable port1 allocated
  %17 = load i32, i32* %16, align 4
  %18 = call i32 @assert_eq_int(i32 %17, i32 80)
  ; Expression result: %18
  %19 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.46, i64 0, i64 0
  %20 = call i32 @extract_port_from_url(i32 %19)
  %21 = alloca i32, align 4
  store i32 %20, i32* %21, align 4
  ; Variable port2 allocated
  %22 = load i32, i32* %21, align 4
  %23 = call i32 @assert_eq_int(i32 %22, i32 443)
  ; Expression result: %23
  %24 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.47, i64 0, i64 0
  %25 = call i32 @extract_port_from_url(i32 %24)
  %26 = alloca i32, align 4
  store i32 %25, i32* %26, align 4
  ; Variable port3 allocated
  %27 = load i32, i32* %26, align 4
  %28 = call i32 @assert_eq_int(i32 %27, i32 8080)
  ; Expression result: %28
  %29 = getelementptr inbounds [32 x i8], [32 x i8]* @.str.48, i64 0, i64 0
  %30 = call i32 @extract_path_from_url(i32 %29)
  %31 = alloca i32, align 4
  store i32 %30, i32* %31, align 4
  ; Variable path1 allocated
  %32 = load i32, i32* %31, align 4
  %33 = getelementptr inbounds [14 x i8], [14 x i8]* @.str.49, i64 0, i64 0
  %34 = call i32 @assert_eq_string(i32 %32, i32 %33)
  ; Expression result: %34
  %35 = getelementptr inbounds [21 x i8], [21 x i8]* @.str.46, i64 0, i64 0
  %36 = call i32 @extract_path_from_url(i32 %35)
  %37 = alloca i32, align 4
  store i32 %36, i32* %37, align 4
  ; Variable path2 allocated
  %38 = load i32, i32* %37, align 4
  %39 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.50, i64 0, i64 0
  %40 = call i32 @assert_eq_string(i32 %38, i32 %39)
  ; Expression result: %40
  ret i32 0
}

define i32 @test_tls_init() {
entry:
  %0 = getelementptr inbounds [19 x i8], [19 x i8]* @.str.51, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.14, i64 0, i64 0
  %3 = call i32 @tls_init(i32 1000, i32 %2)
  %4 = alloca i32, align 4
  store i32 %3, i32* %4, align 4
  ; Variable tls_result1 allocated
  %5 = load i32, i32* %4, align 4
  %6 = call i32 @assert_true(i32 %5)
  ; Expression result: %6
  %7 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %8 = call i32 @tls_init(i32 1001, i32 %7)
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable tls_result2 allocated
  %10 = load i32, i32* %9, align 4
  %11 = call i32 @assert_true(i32 %10)
  ; Expression result: %11
  %12 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.15, i64 0, i64 0
  %13 = call i32 @tls_init(i32 1002, i32 %12)
  %14 = alloca i32, align 4
  store i32 %13, i32* %14, align 4
  ; Variable tls_result3 allocated
  %15 = load i32, i32* %14, align 4
  %16 = call i32 @assert_false(i32 %15)
  ; Expression result: %16
  ret i32 0
}

define i32 @test_tls_send_recv() {
entry:
  %0 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.52, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @tcp_create()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable socket allocated
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %6 = call i32 @tcp_connect(i32 %4, i32 %5, i32 443)
  ; Expression result: %6
  %7 = load i32, i32* %3, align 4
  %8 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.53, i64 0, i64 0
  %9 = call i32 @tls_send(i32 %7, i32 %8)
  %10 = alloca i32, align 4
  store i32 %9, i32* %10, align 4
  ; Variable send_result allocated
  %11 = load i32, i32* %10, align 4
  %12 = call i32 @assert_eq_int(i32 %11, i32 9)
  ; Expression result: %12
  %13 = load i32, i32* %3, align 4
  %14 = call i32 @tls_recv(i32 %13, i32 1024)
  %15 = alloca i32, align 4
  store i32 %14, i32* %15, align 4
  ; Variable recv_result allocated
  %16 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %16
  ; Expression result: 0
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = load i32, i32* %3, align 4
  %19 = call i32 @tcp_close(i32 %18)
  ; Expression result: %19
  ret i32 0
}

define i32 @test_network_utilities() {
entry:
  %0 = getelementptr inbounds [18 x i8], [18 x i8]* @.str.54, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @get_local_ip()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable local_ip allocated
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %6 = call i32 @assert_eq_string(i32 %4, i32 %5)
  ; Expression result: %6
  %7 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.14, i64 0, i64 0
  %8 = call i32 @ping(i32 %7)
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable ping_result1 allocated
  %10 = load i32, i32* %9, align 4
  %11 = call i32 @assert_true(i32 %10)
  ; Expression result: %11
  %12 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.15, i64 0, i64 0
  %13 = call i32 @ping(i32 %12)
  %14 = alloca i32, align 4
  store i32 %13, i32* %14, align 4
  ; Variable ping_result2 allocated
  %15 = load i32, i32* %14, align 4
  %16 = call i32 @assert_true(i32 %15)
  ; Expression result: %16
  %17 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.24, i64 0, i64 0
  %18 = getelementptr inbounds [13 x i8], [13 x i8]* @.str.55, i64 0, i64 0
  %19 = call i32 @network_scan(i32 %17, i32 %18, i32 22)
  %20 = alloca i32, align 4
  store i32 %19, i32* %20, align 4
  ; Variable scan_result allocated
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %21
  ; Expression result: 0
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %22
  %23 = load i32, i32* %20, align 4
  %24 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.24, i64 0, i64 0
  %25 = call i32 @string_contains(i32 %23, i32 %24)
  %26 = call i32 @assert_true(i32 %25)
  ; Expression result: %26
  ret i32 0
}

define i32 @test_remote_address_retrieval() {
entry:
  %0 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.56, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @tcp_create()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable socket allocated
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %6 = call i32 @tcp_connect(i32 %4, i32 %5, i32 80)
  ; Expression result: %6
  %7 = load i32, i32* %3, align 4
  %8 = call i32 @get_remote_addr(i32 %7)
  %9 = alloca i32, align 4
  store i32 %8, i32* %9, align 4
  ; Variable remote_addr allocated
  %10 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %10
  ; Expression result: 0
  %11 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %11
  %12 = load i32, i32* %9, align 4
  %13 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %14 = call i32 @string_contains(i32 %12, i32 %13)
  %15 = call i32 @assert_true(i32 %14)
  ; Expression result: %15
  %16 = load i32, i32* %9, align 4
  %17 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.57, i64 0, i64 0
  %18 = call i32 @string_contains(i32 %16, i32 %17)
  %19 = call i32 @assert_true(i32 %18)
  ; Expression result: %19
  %20 = load i32, i32* %3, align 4
  %21 = call i32 @tcp_close(i32 %20)
  ; Expression result: %21
  ret i32 0
}

define i8* @test_socket_state_management() {
entry:
  %0 = getelementptr inbounds [24 x i8], [24 x i8]* @.str.58, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @tcp_create()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable socket allocated
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %6 = call i32 @tcp_bind(i32 %4, i32 %5, i32 8083)
  %7 = alloca i32, align 4
  store i32 %6, i32* %7, align 4
  ; Variable bind_result allocated
  %8 = load i32, i32* %7, align 4
  %9 = call i32 @assert_eq_int(i32 %8, i32 0)
  ; Expression result: %9
  %10 = load i32, i32* %3, align 4
  %11 = call i32 @tcp_listen(i32 %10, i32 5)
  %12 = alloca i32, align 4
  store i32 %11, i32* %12, align 4
  ; Variable listen_result allocated
  %13 = load i32, i32* %12, align 4
  %14 = call i32 @assert_eq_int(i32 %13, i32 0)
  ; Expression result: %14
  %15 = load i32, i32* %3, align 4
  %16 = call i32 @tcp_close(i32 %15)
  ; Expression result: %16
  %17 = load i32, i32* %3, align 4
  %18 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.59, i64 0, i64 0
  %19 = call i32 @tcp_send(i32 %17, i32 %18)
  %20 = alloca i32, align 4
  store i32 %19, i32* %20, align 4
  ; Variable send_result allocated
  ; Expression result: 1
  %21 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %21
  ret i32 0
}

define i32 @test_error_handling() {
entry:
  %0 = getelementptr inbounds [15 x i8], [15 x i8]* @.str.60, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %3 = alloca i8*, align 4
  store i8* %2, i8** %3, align 4
  ; Variable invalid_socket allocated
  ; Expression result: 1
  %4 = load i8*, i8** %3, align 4
  %5 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %6 = call i32 @tcp_bind(i32 %4, i32 %5, i32 8084)
  %7 = alloca i32, align 4
  store i32 %6, i32* %7, align 4
  ; Variable bind_result allocated
  ; Expression result: 1
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = load i8*, i8** %3, align 4
  %10 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %11 = call i32 @tcp_connect(i32 %9, i32 %10, i32 80)
  %12 = alloca i32, align 4
  store i32 %11, i32* %12, align 4
  ; Variable connect_result allocated
  ; Expression result: 1
  %13 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %13
  %14 = load i8*, i8** %3, align 4
  %15 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.59, i64 0, i64 0
  %16 = call i32 @tcp_send(i32 %14, i32 %15)
  %17 = alloca i32, align 4
  store i32 %16, i32* %17, align 4
  ; Variable send_result allocated
  ; Expression result: 1
  %18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %18
  %19 = load i8*, i8** %3, align 4
  %20 = call i32 @tcp_recv(i32 %19, i32 1024)
  %21 = alloca i32, align 4
  store i32 %20, i32* %21, align 4
  ; Variable recv_result allocated
  %22 = load i32, i32* %21, align 4
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %24 = call i32 @assert_eq_string(i32 %22, i32 %23)
  ; Expression result: %24
  ret i32 0
}

define i32 @test_string_utilities() {
entry:
  %0 = getelementptr inbounds [17 x i8], [17 x i8]* @.str.61, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.62, i64 0, i64 0
  %3 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.63, i64 0, i64 0
  %4 = call i32 @string_starts_with(i32 %2, i32 %3)
  %5 = call i32 @assert_true(i32 %4)
  ; Expression result: %5
  %6 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.62, i64 0, i64 0
  %7 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.64, i64 0, i64 0
  %8 = call i32 @string_starts_with(i32 %6, i32 %7)
  %9 = call i32 @assert_false(i32 %8)
  ; Expression result: %9
  %10 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.62, i64 0, i64 0
  %11 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.64, i64 0, i64 0
  %12 = call i32 @string_index_of(i32 %10, i32 %11)
  %13 = call i32 @assert_eq_int(i32 %12, i32 6)
  ; Expression result: %13
  ; Expression result: 1
  %14 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %14
  %15 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.62, i64 0, i64 0
  %16 = call i32 @string_substring(i32 %15, i32 6, i32 11)
  %17 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.64, i64 0, i64 0
  %18 = call i32 @assert_eq_string(i32 %16, i32 %17)
  ; Expression result: %18
  %19 = getelementptr inbounds [12 x i8], [12 x i8]* @.str.62, i64 0, i64 0
  %20 = call i32 @string_substring(i32 %19, i32 0, i32 5)
  %21 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.63, i64 0, i64 0
  %22 = call i32 @assert_eq_string(i32 %20, i32 %21)
  ; Expression result: %22
  %23 = getelementptr inbounds [6 x i8], [6 x i8]* @.str.63, i64 0, i64 0
  %24 = call i32 @string_length(i32 %23)
  %25 = call i32 @assert_eq_int(i32 %24, i32 5)
  ; Expression result: %25
  %26 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %27 = call i32 @string_length(i32 %26)
  %28 = call i32 @assert_eq_int(i32 %27, i32 0)
  ; Expression result: %28
  %29 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.65, i64 0, i64 0
  %30 = call i32 @string_to_int(i32 %29)
  %31 = call i32 @assert_eq_int(i32 %30, i32 123)
  ; Expression result: %31
  ; Expression result: 456
  %32 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %32
  %33 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.66, i64 0, i64 0
  %34 = call i32 @string_to_int(i32 %33)
  %35 = call i32 @assert_eq_int(i32 %34, i32 0)
  ; Expression result: %35
  %36 = call i32 @int_to_string(i32 123)
  %37 = getelementptr inbounds [4 x i8], [4 x i8]* @.str.65, i64 0, i64 0
  %38 = call i32 @assert_eq_string(i32 %36, i32 %37)
  ; Expression result: %38
  ; Expression result: 456
  %39 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %39
  %40 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %40
  %41 = getelementptr inbounds [5 x i8], [5 x i8]* @.str.67, i64 0, i64 0
  ; Expression result: %41
  %42 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %42
  %43 = call i32 @int_to_string(i32 0)
  %44 = getelementptr inbounds [2 x i8], [2 x i8]* @.str.66, i64 0, i64 0
  %45 = call i32 @assert_eq_string(i32 %43, i32 %44)
  ; Expression result: %45
  ret i32 0
}

define i32 @test_concurrent_socket_operations() {
entry:
  %0 = getelementptr inbounds [29 x i8], [29 x i8]* @.str.68, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @tcp_create()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable socket1 allocated
  %4 = call i32 @tcp_create()
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable socket2 allocated
  %6 = call i32 @udp_create()
  %7 = alloca i32, align 4
  store i32 %6, i32* %7, align 4
  ; Variable socket3 allocated
  %8 = load i32, i32* %3, align 4
  %9 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %10 = call i32 @tcp_bind(i32 %8, i32 %9, i32 8085)
  ; Expression result: %10
  %11 = load i32, i32* %5, align 4
  %12 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %13 = call i32 @tcp_bind(i32 %11, i32 %12, i32 8086)
  ; Expression result: %13
  %14 = load i32, i32* %7, align 4
  %15 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %16 = call i32 @udp_bind(i32 %14, i32 %15, i32 9095)
  ; Expression result: %16
  %17 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %17
  %18 = load i32, i32* %5, align 4
  ; Expression result: %18
  %19 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %19
  %20 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %20
  %21 = load i32, i32* %7, align 4
  ; Expression result: %21
  %22 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %22
  %23 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %23
  %24 = load i32, i32* %7, align 4
  ; Expression result: %24
  %25 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %25
  %26 = load i32, i32* %3, align 4
  %27 = call i32 @tcp_close(i32 %26)
  ; Expression result: %27
  %28 = load i32, i32* %5, align 4
  %29 = call i32 @tcp_close(i32 %28)
  ; Expression result: %29
  %30 = load i32, i32* %7, align 4
  %31 = call i32 @udp_close(i32 %30)
  ; Expression result: %31
  ret i32 0
}

define i32 @test_protocol_differentiation() {
entry:
  %0 = getelementptr inbounds [25 x i8], [25 x i8]* @.str.69, i64 0, i64 0
  %1 = call i32 @test_start(i32 %0)
  ; Expression result: %1
  %2 = call i32 @tcp_create()
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  ; Variable tcp_socket allocated
  %4 = call i32 @udp_create()
  %5 = alloca i32, align 4
  store i32 %4, i32* %5, align 4
  ; Variable udp_socket allocated
  %6 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %6
  %7 = load i32, i32* %5, align 4
  ; Expression result: %7
  %8 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %8
  %9 = load i32, i32* %3, align 4
  %10 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %11 = call i32 @tcp_bind(i32 %9, i32 %10, i32 8087)
  ; Expression result: %11
  %12 = load i32, i32* %5, align 4
  %13 = getelementptr inbounds [10 x i8], [10 x i8]* @.str.3, i64 0, i64 0
  %14 = call i32 @udp_bind(i32 %12, i32 %13, i32 8087)
  ; Expression result: %14
  %15 = load i32, i32* %3, align 4
  %16 = call i32 @tcp_close(i32 %15)
  ; Expression result: %16
  %17 = load i32, i32* %5, align 4
  %18 = call i32 @udp_close(i32 %17)
  ; Expression result: %18
  ret i32 0
}

define i1 @string_contains(i8* %text, i8* %substring) {
entry:
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  ; Expression result: %0
  %1 = call i32 @string_index_of(i8* %text, i8* %substring)
  ; Expression result: %1
  %2 = getelementptr inbounds [1 x i8], [1 x i8]* @.str.1, i64 0, i64 0
  %3 = sub i32 %2, 1
  ; Expression result: %3
  ret i32 0
}


; String constants
@.str.33 = private unnamed_addr constant [17 x i8] c"HTTP GET Request\00", align 1
@.str.50 = private unnamed_addr constant [2 x i8] c"/\00", align 1
@.str.34 = private unnamed_addr constant [4 x i8] c"GET\00", align 1
@.str.37 = private unnamed_addr constant [7 x i8] c"200 OK\00", align 1
@.str.45 = private unnamed_addr constant [15 x i8] c"api.github.com\00", align 1
@.str.55 = private unnamed_addr constant [13 x i8] c"192.168.1.10\00", align 1
@.str.35 = private unnamed_addr constant [20 x i8] c"http://example.com/\00", align 1
@.str.21 = private unnamed_addr constant [15 x i8] c"unknown.domain\00", align 1
@.str.53 = private unnamed_addr constant [10 x i8] c"Hello TLS\00", align 1
@.str.15 = private unnamed_addr constant [12 x i8] c"example.com\00", align 1
@.str.62 = private unnamed_addr constant [12 x i8] c"hello world\00", align 1
@.str.64 = private unnamed_addr constant [6 x i8] c"world\00", align 1
@.str.10 = private unnamed_addr constant [20 x i8] c"UDP Bind Operations\00", align 1
@.str.2 = private unnamed_addr constant [20 x i8] c"TCP Bind Operations\00", align 1
@.str.12 = private unnamed_addr constant [9 x i8] c"UDP Test\00", align 1
@.str.5 = private unnamed_addr constant [22 x i8] c"TCP Listen Operations\00", align 1
@.str.11 = private unnamed_addr constant [25 x i8] c"UDP Send/Recv Operations\00", align 1
@.str.7 = private unnamed_addr constant [25 x i8] c"TCP Send/Recv Operations\00", align 1
@.str.27 = private unnamed_addr constant [27 x i8] c"gmail-smtp-in.l.google.com\00", align 1
@.str.17 = private unnamed_addr constant [11 x i8] c"google.com\00", align 1
@.str.28 = private unnamed_addr constant [20 x i8] c"mail.unknown.domain\00", align 1
@.str.38 = private unnamed_addr constant [18 x i8] c"HTTP POST Request\00", align 1
@.str.39 = private unnamed_addr constant [10 x i8] c"test=data\00", align 1
@.str.59 = private unnamed_addr constant [5 x i8] c"test\00", align 1
@.str.52 = private unnamed_addr constant [25 x i8] c"TLS Send/Recv Operations\00", align 1
@.str.63 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@.str.54 = private unnamed_addr constant [18 x i8] c"Network Utilities\00", align 1
@.str.76 = private unnamed_addr constant [54 x i8] c"  • HTTP Client Operations (GET, POST, URL Parsing)\00", align 1
@.str.56 = private unnamed_addr constant [25 x i8] c"Remote Address Retrieval\00", align 1
@.str.80 = private unnamed_addr constant [52 x i8] c"  • String Utilities and Protocol Differentiation\00", align 1
@.str.68 = private unnamed_addr constant [29 x i8] c"Concurrent Socket Operations\00", align 1
@.str.70 = private unnamed_addr constant [54 x i8] c"🧪 CURSED Network Module - Comprehensive Test Suite\00", align 1
@.str.16 = private unnamed_addr constant [14 x i8] c"93.184.216.34\00", align 1
@.str.13 = private unnamed_addr constant [20 x i8] c"Hostname Resolution\00", align 1
@.str.77 = private unnamed_addr constant [40 x i8] c"  • TLS/SSL Support (Init, Send/Recv)\00", align 1
@.str.24 = private unnamed_addr constant [12 x i8] c"192.168.1.1\00", align 1
@.str.42 = private unnamed_addr constant [17 x i8] c"HTTP URL Parsing\00", align 1
@.str.0 = private unnamed_addr constant [20 x i8] c"TCP Socket Creation\00", align 1
@.str.22 = private unnamed_addr constant [14 x i8] c"192.168.1.100\00", align 1
@.str.57 = private unnamed_addr constant [2 x i8] c":\00", align 1
@.str.9 = private unnamed_addr constant [20 x i8] c"UDP Socket Creation\00", align 1
@.str.1 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.69 = private unnamed_addr constant [25 x i8] c"Protocol Differentiation\00", align 1
@.str.74 = private unnamed_addr constant [54 x i8] c"  • UDP Socket Operations (Create, Bind, Send/Recv)\00", align 1
@.str.72 = private unnamed_addr constant [36 x i8] c"✨ Network Module Features Tested:\00", align 1
@.str.8 = private unnamed_addr constant [12 x i8] c"Hello World\00", align 1
@.str.73 = private unnamed_addr constant [68 x i8] c"  • TCP Socket Operations (Create, Bind, Connect, Listen, Accept)\00", align 1
@.str.79 = private unnamed_addr constant [42 x i8] c"  • Error Handling and State Management\00", align 1
@.str.20 = private unnamed_addr constant [13 x i8] c"140.82.112.3\00", align 1
@.str.18 = private unnamed_addr constant [15 x i8] c"172.217.14.110\00", align 1
@.str.46 = private unnamed_addr constant [21 x i8] c"https://example.com/\00", align 1
@.str.26 = private unnamed_addr constant [17 x i8] c"mail.example.com\00", align 1
@.str.32 = private unnamed_addr constant [12 x i8] c"v=spf1 ~all\00", align 1
@.str.61 = private unnamed_addr constant [17 x i8] c"String Utilities\00", align 1
@.str.60 = private unnamed_addr constant [15 x i8] c"Error Handling\00", align 1
@.str.48 = private unnamed_addr constant [32 x i8] c"http://example.com/api/v1/users\00", align 1
@.str.49 = private unnamed_addr constant [14 x i8] c"/api/v1/users\00", align 1
@.str.66 = private unnamed_addr constant [2 x i8] c"0\00", align 1
@.str.23 = private unnamed_addr constant [23 x i8] c"Reverse DNS Resolution\00", align 1
@.str.29 = private unnamed_addr constant [18 x i8] c"TXT Record Lookup\00", align 1
@.str.44 = private unnamed_addr constant [33 x i8] c"https://api.github.com:443/users\00", align 1
@.str.71 = private unnamed_addr constant [61 x i8] c"============================================================\00", align 1
@.str.75 = private unnamed_addr constant [49 x i8] c"  • DNS Resolution (Forward, Reverse, MX, TXT)\00", align 1
@.str.41 = private unnamed_addr constant [5 x i8] c"POST\00", align 1
@.str.81 = private unnamed_addr constant [52 x i8] c"🎉 Pure CURSED Network Implementation - FFI-Free!\00", align 1
@.str.51 = private unnamed_addr constant [19 x i8] c"TLS Initialization\00", align 1
@.str.19 = private unnamed_addr constant [11 x i8] c"github.com\00", align 1
@.str.4 = private unnamed_addr constant [23 x i8] c"TCP Connect Operations\00", align 1
@.str.40 = private unnamed_addr constant [48 x i8] c"Content-Type: application/x-www-form-urlencoded\00", align 1
@.str.65 = private unnamed_addr constant [4 x i8] c"123\00", align 1
@.str.31 = private unnamed_addr constant [36 x i8] c"v=spf1 include:_spf.google.com ~all\00", align 1
@.str.36 = private unnamed_addr constant [9 x i8] c"HTTP/1.1\00", align 1
@.str.67 = private unnamed_addr constant [5 x i8] c"-456\00", align 1
@.str.6 = private unnamed_addr constant [22 x i8] c"TCP Accept Operations\00", align 1
@.str.30 = private unnamed_addr constant [37 x i8] c"v=spf1 include:_spf.example.com ~all\00", align 1
@.str.3 = private unnamed_addr constant [10 x i8] c"127.0.0.1\00", align 1
@.str.14 = private unnamed_addr constant [10 x i8] c"localhost\00", align 1
@.str.47 = private unnamed_addr constant [25 x i8] c"http://example.com:8080/\00", align 1
@.str.58 = private unnamed_addr constant [24 x i8] c"Socket State Management\00", align 1
@.str.78 = private unnamed_addr constant [52 x i8] c"  • Network Utilities (Ping, Scan, IP Management)\00", align 1
@.str.43 = private unnamed_addr constant [24 x i8] c"http://example.com/path\00", align 1
@.str.25 = private unnamed_addr constant [17 x i8] c"MX Record Lookup\00", align 1
define i32 @main() {
entry:
  %0 = getelementptr inbounds [54 x i8], [54 x i8]* @.str.70, i64 0, i64 0
  %1 = call i32 @puts(i8* %0)
  %2 = add i32 0, 0
  ; Expression result: %2
  %3 = getelementptr inbounds [61 x i8], [61 x i8]* @.str.71, i64 0, i64 0
  %4 = call i32 @puts(i8* %3)
  %5 = add i32 0, 0
  ; Expression result: %5
  %6 = call i32 @test_tcp_socket_creation()
  ; Expression result: %6
  %7 = call i32 @test_tcp_bind_operations()
  ; Expression result: %7
  %8 = call i32 @test_tcp_connect_operations()
  ; Expression result: %8
  %9 = call i32 @test_tcp_listen_operations()
  ; Expression result: %9
  %10 = call i32 @test_tcp_accept_operations()
  ; Expression result: %10
  %11 = call i32 @test_tcp_send_recv_operations()
  ; Expression result: %11
  %12 = call i32 @test_udp_socket_creation()
  ; Expression result: %12
  %13 = call i32 @test_udp_bind_operations()
  ; Expression result: %13
  %14 = call i32 @test_udp_send_recv_operations()
  ; Expression result: %14
  %15 = call i32 @test_hostname_resolution()
  ; Expression result: %15
  %16 = call i32 @test_reverse_dns_resolution()
  ; Expression result: %16
  %17 = call i32 @test_mx_record_lookup()
  ; Expression result: %17
  %18 = call i32 @test_txt_record_lookup()
  ; Expression result: %18
  %19 = call i32 @test_http_get_request()
  ; Expression result: %19
  %20 = call i32 @test_http_post_request()
  ; Expression result: %20
  %21 = call i32 @test_http_url_parsing()
  ; Expression result: %21
  %22 = call i32 @test_tls_init()
  ; Expression result: %22
  %23 = call i32 @test_tls_send_recv()
  ; Expression result: %23
  %24 = call i32 @test_network_utilities()
  ; Expression result: %24
  %25 = call i32 @test_remote_address_retrieval()
  ; Expression result: %25
  %26 = call i32 @test_socket_state_management()
  ; Expression result: %26
  %27 = call i32 @test_error_handling()
  ; Expression result: %27
  %28 = call i32 @test_string_utilities()
  ; Expression result: %28
  %29 = call i32 @test_concurrent_socket_operations()
  ; Expression result: %29
  %30 = call i32 @test_protocol_differentiation()
  ; Expression result: %30
  %31 = getelementptr inbounds [61 x i8], [61 x i8]* @.str.71, i64 0, i64 0
  %32 = call i32 @puts(i8* %31)
  %33 = add i32 0, 0
  ; Expression result: %33
  %34 = call i32 @print_test_summary()
  ; Expression result: %34
  %35 = getelementptr inbounds [36 x i8], [36 x i8]* @.str.72, i64 0, i64 0
  %36 = call i32 @puts(i8* %35)
  %37 = add i32 0, 0
  ; Expression result: %37
  %38 = getelementptr inbounds [68 x i8], [68 x i8]* @.str.73, i64 0, i64 0
  %39 = call i32 @puts(i8* %38)
  %40 = add i32 0, 0
  ; Expression result: %40
  %41 = getelementptr inbounds [54 x i8], [54 x i8]* @.str.74, i64 0, i64 0
  %42 = call i32 @puts(i8* %41)
  %43 = add i32 0, 0
  ; Expression result: %43
  %44 = getelementptr inbounds [49 x i8], [49 x i8]* @.str.75, i64 0, i64 0
  %45 = call i32 @puts(i8* %44)
  %46 = add i32 0, 0
  ; Expression result: %46
  %47 = getelementptr inbounds [54 x i8], [54 x i8]* @.str.76, i64 0, i64 0
  %48 = call i32 @puts(i8* %47)
  %49 = add i32 0, 0
  ; Expression result: %49
  %50 = getelementptr inbounds [40 x i8], [40 x i8]* @.str.77, i64 0, i64 0
  %51 = call i32 @puts(i8* %50)
  %52 = add i32 0, 0
  ; Expression result: %52
  %53 = getelementptr inbounds [52 x i8], [52 x i8]* @.str.78, i64 0, i64 0
  %54 = call i32 @puts(i8* %53)
  %55 = add i32 0, 0
  ; Expression result: %55
  %56 = getelementptr inbounds [42 x i8], [42 x i8]* @.str.79, i64 0, i64 0
  %57 = call i32 @puts(i8* %56)
  %58 = add i32 0, 0
  ; Expression result: %58
  %59 = getelementptr inbounds [52 x i8], [52 x i8]* @.str.80, i64 0, i64 0
  %60 = call i32 @puts(i8* %59)
  %61 = add i32 0, 0
  ; Expression result: %61
  %62 = getelementptr inbounds [52 x i8], [52 x i8]* @.str.81, i64 0, i64 0
  %63 = call i32 @puts(i8* %62)
  %64 = add i32 0, 0
  ; Expression result: %64
  ret i32 0
}

