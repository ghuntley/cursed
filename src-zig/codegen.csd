fr fr CURSED Code Generation Implementation
fr fr Migrated from codegen.zig to pure CURSED

yeet "ast"
yeet "testz"

fr fr Code generation context and state
squad CodeGen {
    spill functions map[tea]normie
    spill variables map[tea]normie
    spill struct_types map[tea]normie
    spill interface_types map[tea]InterfaceInfo
    spill goroutines map[normie]GoroutineInfo
    spill channels map[tea]ChannelInfo
    spill loop_stack []LoopContext
    spill defer_stack []DeferInfo
    spill current_function normie
    spill goroutine_counter normie
    spill runtime_functions map[tea]normie
    spill output_code tea
    spill register_counter normie
    spill label_counter normie
}

squad InterfaceInfo {
    spill name tea
    spill methods []InterfaceMethod
    spill vtable_type normie
}

squad GoroutineInfo {
    spill function normie
    spill stack_size normie
    spill id normie
}

squad ChannelInfo {
    spill element_type normie
    spill channel_type normie
    spill buffer_size normie
}

squad LoopContext {
    spill continue_label tea
    spill break_label tea
}

squad DeferInfo {
    spill cleanup_function normie
    spill cleanup_label tea
}

fr fr Code generation errors
squad CodeGenError {
    spill message tea
    spill location normie
}

fr fr Code generation functions
slay createCodeGen() CodeGen {
    damn CodeGen{
        functions: map{},
        variables: map{},
        struct_types: map{},
        interface_types: map{},
        goroutines: map{},
        channels: map{},
        loop_stack: [],
        defer_stack: [],
        current_function: 0,
        goroutine_counter: 0,
        runtime_functions: map{},
        output_code: "",
        register_counter: 0,
        label_counter: 0
    }
}

slay generateProgram(codegen *CodeGen, program Program) tea {
    fr fr Initialize code generation
    initializeCodeGen(codegen)
    
    fr fr Generate external declarations
    generateExternalDeclarations(codegen)
    
    fr fr Generate imports
    bestie i := 0; i < program.imports.length; i = i + 1 {
        generateImport(codegen, program.imports[i])
    }
    
    fr fr Generate statements
    bestie i := 0; i < program.statements.length; i = i + 1 {
        generateStatement(codegen, program.statements[i])
    }
    
    fr fr Add main function if it doesn't exist
    if !codegen.functions.contains("main") {
        generateMainWrapper(codegen)
    }
    
    fr fr Finalize and return generated code
    damn finalizeCodeGen(codegen)
}

slay initializeCodeGen(codegen *CodeGen) {
    codegen.output_code = ""
    codegen.register_counter = 0
    codegen.label_counter = 0
    
    fr fr Add header comments
    codegen.output_code = codegen.output_code + "; Generated CURSED code\n"
    codegen.output_code = codegen.output_code + "; Target: LLVM IR\n\n"
}

slay generateExternalDeclarations(codegen *CodeGen) {
    fr fr Declare printf for vibez.spill
    codegen.output_code = codegen.output_code + "declare i32 @printf(i8*, ...)\n"
    codegen.functions["printf"] = 1
    
    fr fr Declare malloc and free for memory management
    codegen.output_code = codegen.output_code + "declare i8* @malloc(i64)\n"
    codegen.output_code = codegen.output_code + "declare void @free(i8*)\n"
    codegen.functions["malloc"] = 2
    codegen.functions["free"] = 3
    
    fr fr Declare CURSED runtime functions
    generateRuntimeDeclarations(codegen)
}

slay generateRuntimeDeclarations(codegen *CodeGen) {
    fr fr Channel operations
    codegen.output_code = codegen.output_code + "declare i8* @cursed_channel_create(i64, i64)\n"
    codegen.output_code = codegen.output_code + "declare void @cursed_channel_send(i8*, i8*)\n"
    codegen.output_code = codegen.output_code + "declare i8* @cursed_channel_receive(i8*)\n"
    codegen.output_code = codegen.output_code + "declare void @cursed_channel_close(i8*)\n"
    
    fr fr Goroutine operations
    codegen.output_code = codegen.output_code + "declare void @cursed_goroutine_start(i8*, i8*)\n"
    codegen.output_code = codegen.output_code + "declare void @cursed_goroutine_yield()\n"
    
    fr fr Memory management
    codegen.output_code = codegen.output_code + "declare void @cursed_gc_collect()\n"
    codegen.output_code = codegen.output_code + "declare i8* @cursed_gc_alloc(i64)\n"
    
    fr fr Error handling
    codegen.output_code = codegen.output_code + "declare void @cursed_panic(i8*)\n"
    codegen.output_code = codegen.output_code + "declare i8* @cursed_recover()\n"
    
    codegen.output_code = codegen.output_code + "\n"
}

slay generateImport(codegen *CodeGen, import_stmt ImportStatement) {
    fr fr Generate import directive as comment
    codegen.output_code = codegen.output_code + "; Import: " + import_stmt.path + "\n"
    
    fr fr In full implementation, would link external modules
}

slay generateStatement(codegen *CodeGen, stmt Statement) {
    match stmt.tag {
        "Function" => generateFunctionStatement(codegen, stmt),
        "Let" => generateLetStatement(codegen, stmt),
        "Assignment" => generateAssignmentStatement(codegen, stmt),
        "Expression" => generateExpressionStatement(codegen, stmt),
        "Return" => generateReturnStatement(codegen, stmt),
        "If" => generateIfStatement(codegen, stmt),
        "While" => generateWhileStatement(codegen, stmt),
        "Break" => generateBreakStatement(codegen),
        "Continue" => generateContinueStatement(codegen),
        "Struct" => generateStructStatement(codegen, stmt),
        "Interface" => generateInterfaceStatement(codegen, stmt),
        "Goroutine" => generateGoroutineStatement(codegen, stmt),
        "ShortDeclaration" => generateShortDeclarationStatement(codegen, stmt),
        _ => yikes "Unknown statement type: " + stmt.tag
    }
}

slay generateFunctionStatement(codegen *CodeGen, stmt Statement) {
    fr fr In full implementation, would extract FunctionStatement from stmt.data
    fr fr For now, generate a placeholder function
    
    sus func_name tea = "placeholder_function"
    sus func_id normie = codegen.functions.size() + 1
    codegen.functions[func_name] = func_id
    
    fr fr Generate function signature
    codegen.output_code = codegen.output_code + "define i32 @" + func_name + "() {\n"
    codegen.output_code = codegen.output_code + "entry:\n"
    
    fr fr Set current function
    codegen.current_function = func_id
    
    fr fr Generate function body
    fr fr In full implementation, would iterate through function.body
    codegen.output_code = codegen.output_code + "  ret i32 0\n"
    
    codegen.output_code = codegen.output_code + "}\n\n"
    
    fr fr Reset current function
    codegen.current_function = 0
}

slay generateLetStatement(codegen *CodeGen, stmt Statement) {
    fr fr Generate variable allocation
    sus var_name tea = "temp_var"
    sus register_id normie = getNextRegister(codegen)
    
    codegen.output_code = codegen.output_code + "  %" + toString(register_id) + " = alloca i32\n"
    codegen.variables[var_name] = register_id
}

slay generateAssignmentStatement(codegen *CodeGen, stmt Statement) {
    fr fr Generate assignment code
    sus register_id normie = getNextRegister(codegen)
    codegen.output_code = codegen.output_code + "  %" + toString(register_id) + " = load i32, i32* %1\n"
}

slay generateExpressionStatement(codegen *CodeGen, stmt Statement) {
    fr fr Generate expression evaluation
    sus result_reg normie = generateExpression(codegen, createNullExpression())
    fr fr Expression result is computed but not stored
}

slay generateReturnStatement(codegen *CodeGen, stmt Statement) {
    fr fr Generate return instruction
    codegen.output_code = codegen.output_code + "  ret i32 0\n"
}

slay generateIfStatement(codegen *CodeGen, stmt Statement) {
    fr fr Generate if statement
    sus condition_reg normie = generateExpression(codegen, createNullExpression())
    sus then_label tea = getNextLabel(codegen, "if_then")
    sus else_label tea = getNextLabel(codegen, "if_else")
    sus end_label tea = getNextLabel(codegen, "if_end")
    
    codegen.output_code = codegen.output_code + "  br i1 %" + toString(condition_reg) + ", label %" + then_label + ", label %" + else_label + "\n"
    
    fr fr Then block
    codegen.output_code = codegen.output_code + then_label + ":\n"
    fr fr Generate then body
    codegen.output_code = codegen.output_code + "  br label %" + end_label + "\n"
    
    fr fr Else block
    codegen.output_code = codegen.output_code + else_label + ":\n"
    fr fr Generate else body
    codegen.output_code = codegen.output_code + "  br label %" + end_label + "\n"
    
    fr fr End block
    codegen.output_code = codegen.output_code + end_label + ":\n"
}

slay generateWhileStatement(codegen *CodeGen, stmt Statement) {
    fr fr Generate while loop
    sus loop_header tea = getNextLabel(codegen, "while_header")
    sus loop_body tea = getNextLabel(codegen, "while_body")
    sus loop_end tea = getNextLabel(codegen, "while_end")
    
    fr fr Push loop context for break/continue
    sus loop_ctx LoopContext = LoopContext{
        continue_label: loop_header,
        break_label: loop_end
    }
    codegen.loop_stack.push(loop_ctx)
    
    codegen.output_code = codegen.output_code + "  br label %" + loop_header + "\n"
    
    fr fr Loop header
    codegen.output_code = codegen.output_code + loop_header + ":\n"
    sus condition_reg normie = generateExpression(codegen, createNullExpression())
    codegen.output_code = codegen.output_code + "  br i1 %" + toString(condition_reg) + ", label %" + loop_body + ", label %" + loop_end + "\n"
    
    fr fr Loop body
    codegen.output_code = codegen.output_code + loop_body + ":\n"
    fr fr Generate body statements
    codegen.output_code = codegen.output_code + "  br label %" + loop_header + "\n"
    
    fr fr Loop end
    codegen.output_code = codegen.output_code + loop_end + ":\n"
    
    fr fr Pop loop context
    codegen.loop_stack.pop()
}

slay generateBreakStatement(codegen *CodeGen) {
    if codegen.loop_stack.length > 0 {
        sus current_loop LoopContext = codegen.loop_stack[codegen.loop_stack.length - 1]
        codegen.output_code = codegen.output_code + "  br label %" + current_loop.break_label + "\n"
    } else {
        yikes "Break statement outside of loop"
    }
}

slay generateContinueStatement(codegen *CodeGen) {
    if codegen.loop_stack.length > 0 {
        sus current_loop LoopContext = codegen.loop_stack[codegen.loop_stack.length - 1]
        codegen.output_code = codegen.output_code + "  br label %" + current_loop.continue_label + "\n"
    } else {
        yikes "Continue statement outside of loop"
    }
}

slay generateStructStatement(codegen *CodeGen, stmt Statement) {
    fr fr Generate struct type definition
    sus struct_name tea = "placeholder_struct"
    sus struct_id normie = codegen.struct_types.size() + 1
    codegen.struct_types[struct_name] = struct_id
    
    fr fr Generate LLVM struct type
    codegen.output_code = codegen.output_code + "%" + struct_name + " = type { i32, i8* }\n"
}

slay generateInterfaceStatement(codegen *CodeGen, stmt Statement) {
    fr fr Generate interface vtable
    sus interface_name tea = "placeholder_interface"
    sus vtable_name tea = interface_name + "_vtable"
    
    fr fr Generate vtable type
    codegen.output_code = codegen.output_code + "%" + vtable_name + " = type { i8*, i8* }\n"
    
    sus interface_info InterfaceInfo = InterfaceInfo{
        name: interface_name,
        methods: [],
        vtable_type: 1
    }
    codegen.interface_types[interface_name] = interface_info
}

slay generateGoroutineStatement(codegen *CodeGen, stmt Statement) {
    fr fr Generate goroutine launch
    sus goroutine_id normie = codegen.goroutine_counter
    codegen.goroutine_counter = codegen.goroutine_counter + 1
    
    sus func_name tea = "goroutine_" + toString(goroutine_id)
    
    fr fr Create goroutine function
    codegen.output_code = codegen.output_code + "define void @" + func_name + "(i8*) {\n"
    codegen.output_code = codegen.output_code + "entry:\n"
    fr fr Generate goroutine body
    codegen.output_code = codegen.output_code + "  ret void\n"
    codegen.output_code = codegen.output_code + "}\n\n"
    
    fr fr Start goroutine
    codegen.output_code = codegen.output_code + "  call void @cursed_goroutine_start(i8* bitcast (void (i8*)* @" + func_name + " to i8*), i8* null)\n"
}

slay generateShortDeclarationStatement(codegen *CodeGen, stmt Statement) {
    fr fr Generate short variable declaration
    sus register_id normie = getNextRegister(codegen)
    codegen.output_code = codegen.output_code + "  %" + toString(register_id) + " = alloca i32\n"
    
    fr fr Generate assignment
    sus value_reg normie = generateExpression(codegen, createNullExpression())
    codegen.output_code = codegen.output_code + "  store i32 %" + toString(value_reg) + ", i32* %" + toString(register_id) + "\n"
}

fr fr Expression generation
slay generateExpression(codegen *CodeGen, expr Expression) normie {
    match expr.tag {
        "Binary" => damn generateBinaryExpression(codegen, expr),
        "Unary" => damn generateUnaryExpression(codegen, expr),
        "Call" => damn generateCallExpression(codegen, expr),
        "MemberAccess" => damn generateMemberAccessExpression(codegen, expr),
        "ArrayAccess" => damn generateArrayAccessExpression(codegen, expr),
        "Identifier" => damn generateIdentifierExpression(codegen, expr),
        "Number" => damn generateNumberExpression(codegen, expr),
        "String" => damn generateStringExpression(codegen, expr),
        "Boolean" => damn generateBooleanExpression(codegen, expr),
        "Array" => damn generateArrayExpression(codegen, expr),
        "Map" => damn generateMapExpression(codegen, expr),
        "Tuple" => damn generateTupleExpression(codegen, expr),
        "Match" => damn generateMatchExpression(codegen, expr),
        _ => {
            yikes "Unknown expression type: " + expr.tag
            damn 0
        }
    }
}

slay generateBinaryExpression(codegen *CodeGen, expr Expression) normie {
    fr fr Generate binary operation
    sus left_reg normie = generateExpression(codegen, createNullExpression())
    sus right_reg normie = generateExpression(codegen, createNullExpression())
    sus result_reg normie = getNextRegister(codegen)
    
    fr fr Default to addition for now
    codegen.output_code = codegen.output_code + "  %" + toString(result_reg) + " = add i32 %" + toString(left_reg) + ", %" + toString(right_reg) + "\n"
    
    damn result_reg
}

slay generateUnaryExpression(codegen *CodeGen, expr Expression) normie {
    fr fr Generate unary operation
    sus operand_reg normie = generateExpression(codegen, createNullExpression())
    sus result_reg normie = getNextRegister(codegen)
    
    fr fr Default to negation for now
    codegen.output_code = codegen.output_code + "  %" + toString(result_reg) + " = sub i32 0, %" + toString(operand_reg) + "\n"
    
    damn result_reg
}

slay generateCallExpression(codegen *CodeGen, expr Expression) normie {
    fr fr Generate function call
    sus result_reg normie = getNextRegister(codegen)
    
    fr fr Default to printf call for vibez.spill
    sus string_reg normie = generateStringLiteral(codegen, "Hello, world!\n")
    codegen.output_code = codegen.output_code + "  %" + toString(result_reg) + " = call i32 (i8*, ...) @printf(i8* %" + toString(string_reg) + ")\n"
    
    damn result_reg
}

slay generateMemberAccessExpression(codegen *CodeGen, expr Expression) normie {
    fr fr Generate member access
    sus object_reg normie = generateExpression(codegen, createNullExpression())
    sus result_reg normie = getNextRegister(codegen)
    
    fr fr Default to struct field access
    codegen.output_code = codegen.output_code + "  %" + toString(result_reg) + " = getelementptr inbounds i32, i32* %" + toString(object_reg) + ", i32 0\n"
    
    damn result_reg
}

slay generateArrayAccessExpression(codegen *CodeGen, expr Expression) normie {
    fr fr Generate array access
    sus array_reg normie = generateExpression(codegen, createNullExpression())
    sus index_reg normie = generateExpression(codegen, createNullExpression())
    sus result_reg normie = getNextRegister(codegen)
    
    fr fr Generate GEP instruction
    codegen.output_code = codegen.output_code + "  %" + toString(result_reg) + " = getelementptr inbounds i32, i32* %" + toString(array_reg) + ", i32 %" + toString(index_reg) + "\n"
    
    damn result_reg
}

slay generateIdentifierExpression(codegen *CodeGen, expr Expression) normie {
    fr fr Generate variable load
    sus result_reg normie = getNextRegister(codegen)
    
    fr fr Look up variable in symbol table
    sus var_reg normie = 1 fr fr Default to register 1
    codegen.output_code = codegen.output_code + "  %" + toString(result_reg) + " = load i32, i32* %" + toString(var_reg) + "\n"
    
    damn result_reg
}

slay generateNumberExpression(codegen *CodeGen, expr Expression) normie {
    fr fr Generate number constant
    sus result_reg normie = getNextRegister(codegen)
    
    fr fr Default to constant 42
    codegen.output_code = codegen.output_code + "  %" + toString(result_reg) + " = add i32 0, 42\n"
    
    damn result_reg
}

slay generateStringExpression(codegen *CodeGen, expr Expression) normie {
    fr fr Generate string constant
    damn generateStringLiteral(codegen, "string_value")
}

slay generateBooleanExpression(codegen *CodeGen, expr Expression) normie {
    fr fr Generate boolean constant
    sus result_reg normie = getNextRegister(codegen)
    
    fr fr Default to true (1)
    codegen.output_code = codegen.output_code + "  %" + toString(result_reg) + " = add i1 0, 1\n"
    
    damn result_reg
}

slay generateArrayExpression(codegen *CodeGen, expr Expression) normie {
    fr fr Generate array literal
    sus result_reg normie = getNextRegister(codegen)
    
    fr fr Allocate array memory
    codegen.output_code = codegen.output_code + "  %" + toString(result_reg) + " = call i8* @malloc(i64 40)\n"
    
    fr fr In full implementation, would initialize array elements
    
    damn result_reg
}

slay generateMapExpression(codegen *CodeGen, expr Expression) normie {
    fr fr Generate map literal
    sus result_reg normie = getNextRegister(codegen)
    
    fr fr Allocate map structure
    codegen.output_code = codegen.output_code + "  %" + toString(result_reg) + " = call i8* @malloc(i64 64)\n"
    
    damn result_reg
}

slay generateTupleExpression(codegen *CodeGen, expr Expression) normie {
    fr fr Generate tuple literal
    sus result_reg normie = getNextRegister(codegen)
    
    fr fr Allocate tuple structure
    codegen.output_code = codegen.output_code + "  %" + toString(result_reg) + " = call i8* @malloc(i64 32)\n"
    
    damn result_reg
}

slay generateMatchExpression(codegen *CodeGen, expr Expression) normie {
    fr fr Generate match expression
    sus value_reg normie = generateExpression(codegen, createNullExpression())
    sus result_reg normie = getNextRegister(codegen)
    
    fr fr Generate switch-like structure
    sus default_label tea = getNextLabel(codegen, "match_default")
    sus end_label tea = getNextLabel(codegen, "match_end")
    
    codegen.output_code = codegen.output_code + "  switch i32 %" + toString(value_reg) + ", label %" + default_label + " [\n"
    codegen.output_code = codegen.output_code + "  ]\n"
    
    fr fr Default case
    codegen.output_code = codegen.output_code + default_label + ":\n"
    codegen.output_code = codegen.output_code + "  %" + toString(result_reg) + " = add i32 0, 0\n"
    codegen.output_code = codegen.output_code + "  br label %" + end_label + "\n"
    
    fr fr End label
    codegen.output_code = codegen.output_code + end_label + ":\n"
    
    damn result_reg
}

fr fr Utility functions
slay generateStringLiteral(codegen *CodeGen, value tea) normie {
    sus string_reg normie = getNextRegister(codegen)
    sus length normie = value.length + 1 fr fr Include null terminator
    
    fr fr Generate global string constant
    sus global_name tea = "@.str." + toString(string_reg)
    codegen.output_code = codegen.output_code + global_name + " = private unnamed_addr constant [" + toString(length) + " x i8] c\"" + value + "\\00\"\n"
    
    fr fr Get pointer to string
    sus ptr_reg normie = getNextRegister(codegen)
    codegen.output_code = codegen.output_code + "  %" + toString(ptr_reg) + " = getelementptr inbounds [" + toString(length) + " x i8], [" + toString(length) + " x i8]* " + global_name + ", i32 0, i32 0\n"
    
    damn ptr_reg
}

slay generateMainWrapper(codegen *CodeGen) {
    fr fr Generate main function wrapper
    codegen.output_code = codegen.output_code + "define i32 @main() {\n"
    codegen.output_code = codegen.output_code + "entry:\n"
    
    fr fr Call main_character if it exists
    if codegen.functions.contains("main_character") {
        codegen.output_code = codegen.output_code + "  call i32 @main_character()\n"
    }
    
    codegen.output_code = codegen.output_code + "  ret i32 0\n"
    codegen.output_code = codegen.output_code + "}\n\n"
}

slay getNextRegister(codegen *CodeGen) normie {
    codegen.register_counter = codegen.register_counter + 1
    damn codegen.register_counter
}

slay getNextLabel(codegen *CodeGen, prefix tea) tea {
    codegen.label_counter = codegen.label_counter + 1
    damn prefix + "." + toString(codegen.label_counter)
}

slay finalizeCodeGen(codegen *CodeGen) tea {
    fr fr Add any final code generation
    damn codegen.output_code
}

fr fr Type system functions
slay getTypeSize(type_obj Type) normie {
    match type_obj.tag {
        "Primitive" => damn 4, fr fr 32-bit integers
        "Array" => damn 8, fr fr Pointer + length
        "Map" => damn 8, fr fr Hash table pointer
        "Channel" => damn 8, fr fr Channel structure pointer
        "String" => damn 8, fr fr Pointer + length
        _ => damn 8 fr fr Default pointer size
    }
}

slay getLLVMType(type_obj Type) tea {
    match type_obj.tag {
        "Primitive" => damn "i32",
        "Array" => damn "i8*",
        "Map" => damn "i8*",
        "Channel" => damn "i8*",
        "String" => damn "i8*",
        "Boolean" => damn "i1",
        "Float" => damn "double",
        _ => damn "i8*"
    }
}

slay generateTypeConversion(codegen *CodeGen, from_type Type, to_type Type, value_reg normie) normie {
    fr fr Generate type conversion code
    sus result_reg normie = getNextRegister(codegen)
    
    fr fr Simple bitcast for now
    codegen.output_code = codegen.output_code + "  %" + toString(result_reg) + " = bitcast " + getLLVMType(from_type) + " %" + toString(value_reg) + " to " + getLLVMType(to_type) + "\n"
    
    damn result_reg
}

fr fr Memory management functions
slay generateGCAllocation(codegen *CodeGen, size normie) normie {
    sus result_reg normie = getNextRegister(codegen)
    
    fr fr Call garbage collector allocation
    codegen.output_code = codegen.output_code + "  %" + toString(result_reg) + " = call i8* @cursed_gc_alloc(i64 " + toString(size) + ")\n"
    
    damn result_reg
}

slay generateGCCollection(codegen *CodeGen) {
    fr fr Trigger garbage collection
    codegen.output_code = codegen.output_code + "  call void @cursed_gc_collect()\n"
}

fr fr Error handling functions
slay generatePanic(codegen *CodeGen, message tea) {
    sus string_reg normie = generateStringLiteral(codegen, message)
    codegen.output_code = codegen.output_code + "  call void @cursed_panic(i8* %" + toString(string_reg) + ")\n"
}

slay generateRecover(codegen *CodeGen) normie {
    sus result_reg normie = getNextRegister(codegen)
    codegen.output_code = codegen.output_code + "  %" + toString(result_reg) + " = call i8* @cursed_recover()\n"
    damn result_reg
}

fr fr Debug information functions
slay generateDebugInfo(codegen *CodeGen, line normie, column normie) {
    fr fr Generate debug metadata
    codegen.output_code = codegen.output_code + "  ; Debug: line " + toString(line) + ", column " + toString(column) + "\n"
}

fr fr Optimization functions
slay optimizeCode(codegen *CodeGen) {
    fr fr Apply basic optimizations
    fr fr In full implementation, would run LLVM optimization passes
    codegen.output_code = codegen.output_code + "\n; Optimizations applied\n"
}

fr fr Utility helper functions
slay toString(value normie) tea {
    fr fr Convert number to string
    fr fr Simple implementation - would use proper number formatting in full version
    if value == 0 {
        damn "0"
    } else if value == 1 {
        damn "1"
    } else if value == 2 {
        damn "2"
    } else if value == 3 {
        damn "3"
    } else if value == 4 {
        damn "4"
    } else if value == 5 {
        damn "5"
    } else {
        damn toString(value / 10) + toString(value % 10)
    }
}

fr fr Test functions for code generation
slay test_createCodeGen() {
    test_start("Create CodeGen")
    
    sus codegen CodeGen = createCodeGen()
    assert_true(codegen.register_counter == 0)
    assert_true(codegen.label_counter == 0)
    assert_true(codegen.output_code == "")
    
    test_passed()
}

slay test_generateBasicCode() {
    test_start("Generate Basic Code")
    
    sus codegen CodeGen = createCodeGen()
    initializeCodeGen(codegen)
    generateExternalDeclarations(codegen)
    
    assert_true(codegen.output_code.contains("declare i32 @printf"))
    assert_true(codegen.functions.contains("printf"))
    
    test_passed()
}

slay test_generateExpression() {
    test_start("Generate Expression")
    
    sus codegen CodeGen = createCodeGen()
    initializeCodeGen(codegen)
    
    sus expr Expression = createNumberExpression("42")
    sus result_reg normie = generateExpression(codegen, expr)
    
    assert_true(result_reg > 0)
    assert_true(codegen.output_code.contains("%"))
    
    test_passed()
}

slay test_generateFunction() {
    test_start("Generate Function")
    
    sus codegen CodeGen = createCodeGen()
    initializeCodeGen(codegen)
    
    sus stmt Statement = createExpressionStatement(createNullExpression())
    stmt.tag = "Function"
    generateStatement(codegen, stmt)
    
    assert_true(codegen.output_code.contains("define"))
    assert_true(codegen.functions.size() > 0)
    
    test_passed()
}

slay test_generateProgram() {
    test_start("Generate Program")
    
    sus codegen CodeGen = createCodeGen()
    sus program Program = createProgram()
    
    sus generated_code tea = generateProgram(codegen, program)
    
    assert_true(generated_code.contains("Generated CURSED code"))
    assert_true(generated_code.contains("declare i32 @printf"))
    
    test_passed()
}

slay runCodeGenTests() {
    test_createCodeGen()
    test_generateBasicCode()
    test_generateExpression()
    test_generateFunction()
    test_generateProgram()
    print_test_summary()
}

fr fr Entry point for testing
slay main() {
    runCodeGenTests()
}
