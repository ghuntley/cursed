; Generated LLVM IR for CURSED program with DWARF debug information
target triple = "x86_64-pc-linux-gnu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

; Debug Information
!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!1, !2, !3}
!llvm.ident = !{!4}

; Debug metadata
!0 = distinct !DICompileUnit(language: DW_LANG_C, file: !5, producer: "CURSED Compiler v1.0", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug)
!1 = !{i32 7, !"Dwarf Version", i32 4}
!2 = !{i32 2, !"Debug Info Version", i32 3}
!3 = !{i32 1, !"wchar_size", i32 4}
!4 = !{!"CURSED Compiler v1.0 with DWARF debug info"}
!5 = !DIFile(filename: "simple_debug_test.csd", directory: "/home/ghuntley/cursed")

; External function declarations
declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

; Function type debug info
!10 = !DISubroutineType(types: !11)
!11 = !{!12, !12}  ; (drip) -> drip
!12 = !DIBasicType(name: "drip", size: 64, encoding: DW_ATE_signed)

; Function debug_function
define i64 @debug_function(i64 %x) !dbg !20 {
entry:
  %x_addr = alloca i64, align 8
  %local_var = alloca i64, align 8
  
  ; Store parameter with debug info
  store i64 %x, i64* %x_addr, align 8
  call void @llvm.dbg.declare(metadata i64* %x_addr, metadata !21, metadata !DIExpression()), !dbg !22
  
  ; Compute local_var = x + 10
  %x_val = load i64, i64* %x_addr, align 8, !dbg !23
  %add_result = add nsw i64 %x_val, 10, !dbg !24
  store i64 %add_result, i64* %local_var, align 8, !dbg !25
  call void @llvm.dbg.declare(metadata i64* %local_var, metadata !26, metadata !DIExpression()), !dbg !25
  
  ; Print the local variable (simplified)
  %local_val = load i64, i64* %local_var, align 8, !dbg !27
  
  ; Return local_var
  ret i64 %local_val, !dbg !28
}

; Main function
define i32 @main() !dbg !30 {
entry:
  %global_var = alloca i64, align 8
  %result = alloca i64, align 8
  
  ; Initialize global_var = 42
  store i64 42, i64* %global_var, align 8, !dbg !31
  call void @llvm.dbg.declare(metadata i64* %global_var, metadata !32, metadata !DIExpression()), !dbg !31
  
  ; Call debug_function
  %global_val = load i64, i64* %global_var, align 8, !dbg !33
  %func_result = call i64 @debug_function(i64 %global_val), !dbg !34
  store i64 %func_result, i64* %result, align 8, !dbg !35
  call void @llvm.dbg.declare(metadata i64* %result, metadata !36, metadata !DIExpression()), !dbg !35
  
  ret i32 0, !dbg !37
}

; Debug intrinsic declarations
declare void @llvm.dbg.declare(metadata, metadata, metadata)

; Function debug info
!20 = distinct !DISubprogram(name: "debug_function", linkageName: "debug_function", scope: !5, file: !5, line: 1, type: !10, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0)
!21 = !DILocalVariable(name: "x", arg: 1, scope: !20, file: !5, line: 1, type: !12)
!22 = !DILocation(line: 1, column: 21, scope: !20)
!23 = !DILocation(line: 2, column: 25, scope: !20)
!24 = !DILocation(line: 2, column: 27, scope: !20)
!25 = !DILocation(line: 2, column: 5, scope: !20)
!26 = !DILocalVariable(name: "local_var", scope: !20, file: !5, line: 2, type: !12)
!27 = !DILocation(line: 4, column: 10, scope: !20)
!28 = !DILocation(line: 4, column: 5, scope: !20)

; Main function debug info
!30 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !5, file: !5, line: 7, type: !DISubroutineType(types: !{!DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)}), scopeLine: 7, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0)
!31 = !DILocation(line: 7, column: 1, scope: !30)
!32 = !DILocalVariable(name: "global_var", scope: !30, file: !5, line: 7, type: !12)
!33 = !DILocation(line: 8, column: 18, scope: !30)
!34 = !DILocation(line: 8, column: 14, scope: !30)
!35 = !DILocation(line: 8, column: 1, scope: !30)
!36 = !DILocalVariable(name: "result", scope: !30, file: !5, line: 8, type: !12)
!37 = !DILocation(line: 9, column: 1, scope: !30)
