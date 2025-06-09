; ModuleID = 'testpkg'
source_filename = "testpkg"

declare i32 @puts(ptr)

define i32 @vibez_spill_direct(ptr %0) {
entry:
  %puts_call = call i32 @puts(ptr %0)
  ret i32 0
}

define i32 @main() {
entry:
  ret i32 0
}

define i32 @_testpkg_main() {
entry:
  ret i32 0
}

define i32 @_main_main() {
entry:
  ret i32 0
}
