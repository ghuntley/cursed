; Generated LLVM IR for CURSED program
; slay main_character() {
;     vibez.spill("Hello from CURSED Zig!")
;     sus x drip = 42
;     vibez.spill(x)
; }

target triple = "x86_64-pc-linux-gnu"

declare i32 @puts(i8*)
declare i32 @printf(i8*, ...)

@.str = private unnamed_addr constant [23 x i8] c"Hello from CURSED Zig!\00", align 1
@.int_fmt = private unnamed_addr constant [6 x i8] c"%lld\0A\00", align 1

define void @main_character() {
entry:
  ; vibez.spill("Hello from CURSED Zig!")
  %hello_str = getelementptr [23 x i8], [23 x i8]* @.str, i32 0, i32 0
  %call1 = call i32 @puts(i8* %hello_str)
  
  ; sus x drip = 42
  %x = alloca i64, align 8
  store i64 42, i64* %x, align 8
  
  ; vibez.spill(x)
  %x_load = load i64, i64* %x, align 8
  %fmt = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0
  %call2 = call i32 (i8*, ...) @printf(i8* %fmt, i64 %x_load)
  
  ret void
}

define i32 @main() {
entry:
  call void @main_character()
  ret i32 0
}
