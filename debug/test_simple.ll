; ModuleID = 'test_simple'
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

define i32 @main() !dbg !11 {
  call void @llvm.dbg.declare(metadata i8* null, metadata !12, metadata !DIExpression()), !dbg !DILocation(line: 1, column: 1, scope: !10)
  ret i32 0
}

!0 = distinct !DICompileUnit(language: DW_LANG_lo_user, file: !1, producer: "CURSED Compiler v1.0", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug)
!1 = !DIFile(filename: "test_simple.csd", directory: "")
!11 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !1, file: !1, line: 1, type: !12, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0)

; Debug intrinsics
declare void @llvm.dbg.declare(metadata, metadata, metadata) #0
declare void @llvm.dbg.value(metadata, metadata, metadata) #0
declare void @llvm.dbg.addr(metadata, metadata, metadata) #0

; Debug info version
!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!2, !3}
!llvm.ident = !{!4}
!2 = !{i32 7, !"Dwarf Version", i32 4}
!3 = !{i32 2, !"Debug Info Version", i32 3}
!4 = !{!"CURSED Compiler with Debug Support"}
