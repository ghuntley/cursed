; Hand-written test
declare void @cursed_runtime_spill_string(ptr)

define i32 @main() {
entry:
  call void @cursed_runtime_spill_string(ptr @.str.0)  
  ret i32 0
}

@.str.0 = private unnamed_addr constant [16 x i8] c"Manual LLVM test", align 1
