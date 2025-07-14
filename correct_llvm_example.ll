define i32 @main() {
  %1 = alloca i32, align 4
  store i32 42, i32* %1
  ret i32 0
}
