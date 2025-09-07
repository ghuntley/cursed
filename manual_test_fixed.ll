; Hand-written test with null terminator
declare void @cursed_runtime_spill_string(ptr)

define i32 @main() {
entry:
  call void @cursed_runtime_spill_string(ptr @.str.0)  
  ret i32 0
}

@.str.0 = private unnamed_addr constant [17 x i8] c"Manual LLVM test\00", align 1
