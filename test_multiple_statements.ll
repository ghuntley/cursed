; ModuleID = 'cursed_program'
source_filename = "cursed_program"

@fmt_str = private unnamed_addr constant [14 x i8] c"Value x: %ld\0A\00", align 1
@fmt_str.1 = private unnamed_addr constant [14 x i8] c"Value y: %ld\0A\00", align 1
@fmt_str.2 = private unnamed_addr constant [24 x i8] c"Arithmetic result: %ld\0A\00", align 1
@fmt_str.3 = private unnamed_addr constant [16 x i8] c"End of program\0A\00", align 1

define i32 @main() {
entry:
  %printf_call = call i32 (ptr, ...) @printf(ptr @fmt_str, i32 10)
  %printf_call1 = call i32 (ptr, ...) @printf(ptr @fmt_str.1, i32 5)
  %printf_call2 = call i32 (ptr, ...) @printf(ptr @fmt_str.2, i32 0)
  %printf_call3 = call i32 (ptr, ...) @printf(ptr @fmt_str.3)
  ret i32 0
}

declare i32 @printf(ptr, ...)
