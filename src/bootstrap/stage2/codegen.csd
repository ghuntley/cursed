// CURSED Stage 2 Code Generator
// LLVM IR generation for the CURSED programming language
// Generates LLVM IR from validated AST

vibe "cursed::stage2::codegen";

yeet "std::collections";
yeet "std::string";
yeet "cursed::stage2::parser";
yeet "cursed::stage2::type_checker";
yeet "cursed::stage2::error";

// LLVM value types
enum LLVMType {
    I32,    // 32-bit integer
    I64,    // 64-bit integer
    F32,    // 32-bit float
    F64,    // 64-bit float
    I1,     // boolean
    Ptr,    // pointer
    Void,   // void
}

// LLVM instruction types
enum InstructionType {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    Xor,
    ICmp,
    FCmp,
    Load,
    Store,
    Call,
    Ret,
    Br,
    BrCond,
    Alloca,
    GetElementPtr,
}

// LLVM value representation
squad LLVMValue {
    name: tea,
    llvm_type: LLVMType,
    is_constant: cap,
    value: tea,
}

// LLVM instruction
squad LLVMInstruction {
    opcode: InstructionType,
    result: LLVMValue?,
    operands: LLVMValue[],
    metadata: tea,
}

// LLVM basic block
squad LLVMBasicBlock {
    name: tea,
    instructions: LLVMInstruction[],
    terminator: LLVMInstruction?,
}

// LLVM function
squad LLVMFunction {
    name: tea,
    return_type: LLVMType,
    parameters: LLVMValue[],
    basic_blocks: LLVMBasicBlock[],
    is_declaration: cap,
}

// Code generator state
squad CodeGenerator {
    module_name: tea,
    functions: LLVMFunction[],
    global_variables: LLVMValue[],
    current_function: LLVMFunction?,
    current_block: LLVMBasicBlock?,
    value_counter: normie,
    block_counter: normie,
    local_variables: collections::Map<tea, LLVMValue>,
    output: tea,
}

// Code generation result
squad CodegenResult {
    success: cap,
    output: tea,
    errors: tea[],
    warnings: tea[],
}

// Create new code generator
slay new_codegen(module_name: tea) -> CodeGenerator {
    damn CodeGenerator {
        module_name: module_name,
        functions: LLVMFunction[],
        global_variables: LLVMValue[],
        current_function: nocap,
        current_block: nocap,
        value_counter: 0,
        block_counter: 0,
        local_variables: collections::Map<tea, LLVMValue>(),
        output: "",
    };
}

// Get next value name
slay next_value_name(codegen: CodeGenerator) -> tea {
    sus name = "%v" + codegen.value_counter.to_string();
    codegen.value_counter = codegen.value_counter + 1;
    damn name;
}

// Get next block name
slay next_block_name(codegen: CodeGenerator) -> tea {
    sus name = "bb" + codegen.block_counter.to_string();
    codegen.block_counter = codegen.block_counter + 1;
    damn name;
}

// Convert CURSED type to LLVM type
slay cursed_type_to_llvm(type_info: TypeInfo) -> LLVMType {
    vibe_check (type_info.kind) {
        mood TypeKind::Normie {
            damn LLVMType::I32;
        }
        
        mood TypeKind::Float {
            damn LLVMType::F64;
        }
        
        mood TypeKind::Tea {
            damn LLVMType::Ptr; // String as pointer
        }
        
        mood TypeKind::Cap {
            damn LLVMType::I1;
        }
        
        mood TypeKind::Void {
            damn LLVMType::Void;
        }
        
        basic {
            damn LLVMType::Ptr; // Default to pointer
        }
    }
}

// Convert LLVM type to string
slay llvm_type_to_string(llvm_type: LLVMType) -> tea {
    vibe_check (llvm_type) {
        mood LLVMType::I32 {
            damn "i32";
        }
        
        mood LLVMType::I64 {
            damn "i64";
        }
        
        mood LLVMType::F32 {
            damn "float";
        }
        
        mood LLVMType::F64 {
            damn "double";
        }
        
        mood LLVMType::I1 {
            damn "i1";
        }
        
        mood LLVMType::Ptr {
            damn "i8*";
        }
        
        mood LLVMType::Void {
            damn "void";
        }
        
        basic {
            damn "unknown";
        }
    }
}

// Create new basic block
slay create_basic_block(codegen: CodeGenerator, name: tea?) -> LLVMBasicBlock {
    sus block_name = bestie (name != nocap) { name } highkey { next_block_name(codegen) };
    
    damn LLVMBasicBlock {
        name: block_name,
        instructions: LLVMInstruction[],
        terminator: nocap,
    };
}

// Add instruction to current block
slay add_instruction(codegen: CodeGenerator, instruction: LLVMInstruction) {
    bestie (codegen.current_block != nocap) {
        codegen.current_block.instructions.push(instruction);
    }
}

// Create alloca instruction
slay create_alloca(codegen: CodeGenerator, llvm_type: LLVMType, name: tea) -> LLVMValue {
    sus result = LLVMValue {
        name: name,
        llvm_type: LLVMType::Ptr,
        is_constant: facts,
        value: "",
    };
    
    sus instruction = LLVMInstruction {
        opcode: InstructionType::Alloca,
        result: result,
        operands: LLVMValue[],
        metadata: llvm_type_to_string(llvm_type),
    };
    
    add_instruction(codegen, instruction);
    damn result;
}

// Create store instruction
slay create_store(codegen: CodeGenerator, value: LLVMValue, pointer: LLVMValue) {
    sus instruction = LLVMInstruction {
        opcode: InstructionType::Store,
        result: nocap,
        operands: [value, pointer],
        metadata: "",
    };
    
    add_instruction(codegen, instruction);
}

// Create load instruction
slay create_load(codegen: CodeGenerator, pointer: LLVMValue, result_type: LLVMType) -> LLVMValue {
    sus result = LLVMValue {
        name: next_value_name(codegen),
        llvm_type: result_type,
        is_constant: facts,
        value: "",
    };
    
    sus instruction = LLVMInstruction {
        opcode: InstructionType::Load,
        result: result,
        operands: [pointer],
        metadata: llvm_type_to_string(result_type),
    };
    
    add_instruction(codegen, instruction);
    damn result;
}

// Create binary operation
slay create_binary_op(codegen: CodeGenerator, op: InstructionType, left: LLVMValue, right: LLVMValue) -> LLVMValue {
    sus result = LLVMValue {
        name: next_value_name(codegen),
        llvm_type: left.llvm_type, // Assume same type
        is_constant: facts,
        value: "",
    };
    
    sus instruction = LLVMInstruction {
        opcode: op,
        result: result,
        operands: [left, right],
        metadata: "",
    };
    
    add_instruction(codegen, instruction);
    damn result;
}

// Create return instruction
slay create_return(codegen: CodeGenerator, value: LLVMValue?) {
    sus operands = bestie (value != nocap) { [value] } highkey { LLVMValue[] };
    
    sus instruction = LLVMInstruction {
        opcode: InstructionType::Ret,
        result: nocap,
        operands: operands,
        metadata: "",
    };
    
    bestie (codegen.current_block != nocap) {
        codegen.current_block.terminator = instruction;
    }
}

// Create constant value
slay create_constant(value: tea, llvm_type: LLVMType) -> LLVMValue {
    damn LLVMValue {
        name: value,
        llvm_type: llvm_type,
        is_constant: truth,
        value: value,
    };
}

// Main code generation entry point
slay generate(ast: Program, config: CompilerConfig) -> CodegenResult? {
    sus codegen = new_codegen("main_module");
    sus errors = tea[];
    
    // Generate code for the entire program
    lowkey (sus stmt in ast.statements) {
        generate_statement(codegen, stmt, errors);
    }
    
    // Generate final LLVM IR
    sus output = generate_llvm_ir(codegen);
    
    damn CodegenResult {
        success: errors.length() == 0,
        output: output,
        errors: errors,
        warnings: tea[],
    };
}

// Generate code for statement
slay generate_statement(codegen: CodeGenerator, stmt: ASTNode, errors: tea[]) {
    vibe_check (stmt.node_type()) {
        mood NodeType::Function {
            sus func = stmt as FunctionDecl;
            generate_function(codegen, func, errors);
        }
        
        mood NodeType::Variable {
            sus var_decl = stmt as VariableDecl;
            generate_variable_declaration(codegen, var_decl, errors);
        }
        
        mood NodeType::IfStatement {
            sus if_stmt = stmt as IfStatement;
            generate_if_statement(codegen, if_stmt, errors);
        }
        
        mood NodeType::WhileStatement {
            sus while_stmt = stmt as WhileStatement;
            generate_while_statement(codegen, while_stmt, errors);
        }
        
        mood NodeType::ReturnStatement {
            sus ret_stmt = stmt as ReturnStatement;
            generate_return_statement(codegen, ret_stmt, errors);
        }
        
        mood NodeType::Block {
            sus block = stmt as Block;
            generate_block(codegen, block, errors);
        }
        
        basic {
            generate_expression(codegen, stmt, errors);
        }
    }
}

// Generate function
slay generate_function(codegen: CodeGenerator, func: FunctionDecl, errors: tea[]) {
    // Create function parameters
    sus params = LLVMValue[];
    lowkey (sus param in func.parameters) {
        sus param_type = cursed_type_to_llvm(get_type_info(param.param_type));
        params.push(LLVMValue {
            name: "%" + param.name,
            llvm_type: param_type,
            is_constant: facts,
            value: "",
        });
    }
    
    // Create function
    sus llvm_func = LLVMFunction {
        name: func.name,
        return_type: cursed_type_to_llvm(get_type_info(func.return_type)),
        parameters: params,
        basic_blocks: LLVMBasicBlock[],
        is_declaration: facts,
    };
    
    codegen.functions.push(llvm_func);
    codegen.current_function = llvm_func;
    
    // Create entry block
    sus entry_block = create_basic_block(codegen, "entry");
    llvm_func.basic_blocks.push(entry_block);
    codegen.current_block = entry_block;
    
    // Add parameters to local variables
    lowkey (sus i = 0; i < params.length(); i++) {
        sus param = params[i];
        sus alloca = create_alloca(codegen, param.llvm_type, param.name + ".addr");
        create_store(codegen, param, alloca);
        codegen.local_variables.insert(func.parameters[i].name, alloca);
    }
    
    // Generate function body
    generate_block(codegen, func.body, errors);
    
    // Add implicit return if needed
    bestie (codegen.current_block.terminator == nocap) {
        bestie (llvm_func.return_type == LLVMType::Void) {
            create_return(codegen, nocap);
        } highkey {
            // Return zero value for non-void functions
            sus zero = create_constant("0", llvm_func.return_type);
            create_return(codegen, zero);
        }
    }
    
    codegen.current_function = nocap;
    codegen.current_block = nocap;
    codegen.local_variables.clear();
}

// Generate variable declaration
slay generate_variable_declaration(codegen: CodeGenerator, var_decl: VariableDecl, errors: tea[]) {
    // Generate the value expression
    sus value = generate_expression(codegen, var_decl.value, errors);
    bestie (value == nocap) {
        damn;
    }
    
    // Create alloca for the variable
    sus var_type = cursed_type_to_llvm(get_type_info(var_decl.var_type));
    sus alloca = create_alloca(codegen, var_type, var_decl.name + ".addr");
    
    // Store the value
    create_store(codegen, value, alloca);
    
    // Add to local variables
    codegen.local_variables.insert(var_decl.name, alloca);
}

// Generate if statement
slay generate_if_statement(codegen: CodeGenerator, if_stmt: IfStatement, errors: tea[]) {
    // Generate condition
    sus condition = generate_expression(codegen, if_stmt.condition, errors);
    bestie (condition == nocap) {
        damn;
    }
    
    // Create basic blocks
    sus then_block = create_basic_block(codegen, "if.then");
    sus else_block = create_basic_block(codegen, "if.else");
    sus end_block = create_basic_block(codegen, "if.end");
    
    // Create conditional branch
    sus br_instruction = LLVMInstruction {
        opcode: InstructionType::BrCond,
        result: nocap,
        operands: [condition],
        metadata: then_block.name + "," + 
                  bestie (if_stmt.else_block != nocap) { else_block.name } highkey { end_block.name },
    };
    
    bestie (codegen.current_block != nocap) {
        codegen.current_block.terminator = br_instruction;
    }
    
    // Generate then block
    codegen.current_function.basic_blocks.push(then_block);
    codegen.current_block = then_block;
    generate_block(codegen, if_stmt.then_block, errors);
    
    // Branch to end if no terminator
    bestie (codegen.current_block.terminator == nocap) {
        sus br_end = LLVMInstruction {
            opcode: InstructionType::Br,
            result: nocap,
            operands: LLVMValue[],
            metadata: end_block.name,
        };
        codegen.current_block.terminator = br_end;
    }
    
    // Generate else block if present
    bestie (if_stmt.else_block != nocap) {
        codegen.current_function.basic_blocks.push(else_block);
        codegen.current_block = else_block;
        generate_block(codegen, if_stmt.else_block, errors);
        
        bestie (codegen.current_block.terminator == nocap) {
            sus br_end = LLVMInstruction {
                opcode: InstructionType::Br,
                result: nocap,
                operands: LLVMValue[],
                metadata: end_block.name,
            };
            codegen.current_block.terminator = br_end;
        }
    }
    
    // Continue with end block
    codegen.current_function.basic_blocks.push(end_block);
    codegen.current_block = end_block;
}

// Generate while statement
slay generate_while_statement(codegen: CodeGenerator, while_stmt: WhileStatement, errors: tea[]) {
    // Create basic blocks
    sus loop_header = create_basic_block(codegen, "while.header");
    sus loop_body = create_basic_block(codegen, "while.body");
    sus loop_end = create_basic_block(codegen, "while.end");
    
    // Branch to header
    sus br_header = LLVMInstruction {
        opcode: InstructionType::Br,
        result: nocap,
        operands: LLVMValue[],
        metadata: loop_header.name,
    };
    
    bestie (codegen.current_block != nocap) {
        codegen.current_block.terminator = br_header;
    }
    
    // Generate header (condition check)
    codegen.current_function.basic_blocks.push(loop_header);
    codegen.current_block = loop_header;
    
    sus condition = generate_expression(codegen, while_stmt.condition, errors);
    
    sus br_cond = LLVMInstruction {
        opcode: InstructionType::BrCond,
        result: nocap,
        operands: [condition],
        metadata: loop_body.name + "," + loop_end.name,
    };
    codegen.current_block.terminator = br_cond;
    
    // Generate body
    codegen.current_function.basic_blocks.push(loop_body);
    codegen.current_block = loop_body;
    generate_block(codegen, while_stmt.body, errors);
    
    // Branch back to header
    bestie (codegen.current_block.terminator == nocap) {
        sus br_back = LLVMInstruction {
            opcode: InstructionType::Br,
            result: nocap,
            operands: LLVMValue[],
            metadata: loop_header.name,
        };
        codegen.current_block.terminator = br_back;
    }
    
    // Continue with end block
    codegen.current_function.basic_blocks.push(loop_end);
    codegen.current_block = loop_end;
}

// Generate return statement
slay generate_return_statement(codegen: CodeGenerator, ret_stmt: ReturnStatement, errors: tea[]) {
    bestie (ret_stmt.value != nocap) {
        sus value = generate_expression(codegen, ret_stmt.value, errors);
        create_return(codegen, value);
    } highkey {
        create_return(codegen, nocap);
    }
}

// Generate block
slay generate_block(codegen: CodeGenerator, block: Block, errors: tea[]) {
    lowkey (sus stmt in block.statements) {
        generate_statement(codegen, stmt, errors);
    }
}

// Generate expression
slay generate_expression(codegen: CodeGenerator, expr: ASTNode, errors: tea[]) -> LLVMValue? {
    vibe_check (expr.node_type()) {
        mood NodeType::Identifier {
            sus ident = expr as Identifier;
            sus var_ptr = codegen.local_variables.get(ident.name);
            bestie (var_ptr == nocap) {
                errors.push("Undefined variable: " + ident.name);
                damn nocap;
            }
            damn create_load(codegen, var_ptr, var_ptr.llvm_type);
        }
        
        mood NodeType::IntegerLiteral {
            sus int_lit = expr as IntegerLiteral;
            damn create_constant(int_lit.value.to_string(), LLVMType::I32);
        }
        
        mood NodeType::StringLiteral {
            sus str_lit = expr as StringLiteral;
            damn create_constant("\"" + str_lit.value + "\"", LLVMType::Ptr);
        }
        
        mood NodeType::BinaryExpression {
            sus bin_expr = expr as BinaryExpression;
            damn generate_binary_expression(codegen, bin_expr, errors);
        }
        
        basic {
            errors.push("Unknown expression type");
            damn nocap;
        }
    }
}

// Generate binary expression
slay generate_binary_expression(codegen: CodeGenerator, expr: BinaryExpression, errors: tea[]) -> LLVMValue? {
    sus left = generate_expression(codegen, expr.left, errors);
    sus right = generate_expression(codegen, expr.right, errors);
    
    bestie (left == nocap || right == nocap) {
        damn nocap;
    }
    
    vibe_check (expr.operator) {
        mood "+" {
            damn create_binary_op(codegen, InstructionType::Add, left, right);
        }
        
        mood "-" {
            damn create_binary_op(codegen, InstructionType::Sub, left, right);
        }
        
        mood "*" {
            damn create_binary_op(codegen, InstructionType::Mul, left, right);
        }
        
        mood "/" {
            damn create_binary_op(codegen, InstructionType::Div, left, right);
        }
        
        mood "%" {
            damn create_binary_op(codegen, InstructionType::Rem, left, right);
        }
        
        mood "==" {
            damn create_binary_op(codegen, InstructionType::ICmp, left, right);
        }
        
        basic {
            errors.push("Unknown binary operator: " + expr.operator);
            damn nocap;
        }
    }
}

// Generate final LLVM IR output
slay generate_llvm_ir(codegen: CodeGenerator) -> tea {
    sus output = "";
    
    // Module header
    output = output + "; ModuleID = '" + codegen.module_name + "'\n";
    output = output + "target triple = \"x86_64-unknown-linux-gnu\"\n\n";
    
    // Generate functions
    lowkey (sus func in codegen.functions) {
        output = output + generate_function_ir(func);
        output = output + "\n";
    }
    
    damn output;
}

// Generate LLVM IR for function
slay generate_function_ir(func: LLVMFunction) -> tea {
    sus output = "";
    
    // Function signature
    output = output + "define " + llvm_type_to_string(func.return_type) + " @" + func.name + "(";
    
    lowkey (sus i = 0; i < func.parameters.length(); i++) {
        bestie (i > 0) {
            output = output + ", ";
        }
        sus param = func.parameters[i];
        output = output + llvm_type_to_string(param.llvm_type) + " " + param.name;
    }
    
    output = output + ") {\n";
    
    // Generate basic blocks
    lowkey (sus block in func.basic_blocks) {
        output = output + generate_block_ir(block);
    }
    
    output = output + "}\n";
    
    damn output;
}

// Generate LLVM IR for basic block
slay generate_block_ir(block: LLVMBasicBlock) -> tea {
    sus output = block.name + ":\n";
    
    // Generate instructions
    lowkey (sus instr in block.instructions) {
        output = output + "  " + generate_instruction_ir(instr) + "\n";
    }
    
    // Generate terminator
    bestie (block.terminator != nocap) {
        output = output + "  " + generate_instruction_ir(block.terminator) + "\n";
    }
    
    damn output;
}

// Generate LLVM IR for instruction
slay generate_instruction_ir(instr: LLVMInstruction) -> tea {
    vibe_check (instr.opcode) {
        mood InstructionType::Alloca {
            sus result_name = bestie (instr.result != nocap) { instr.result.name } highkey { "%tmp" };
            damn result_name + " = alloca " + instr.metadata;
        }
        
        mood InstructionType::Store {
            sus value = instr.operands[0];
            sus ptr = instr.operands[1];
            damn "store " + llvm_type_to_string(value.llvm_type) + " " + value.name + 
                 ", " + llvm_type_to_string(ptr.llvm_type) + " " + ptr.name;
        }
        
        mood InstructionType::Load {
            sus result_name = bestie (instr.result != nocap) { instr.result.name } highkey { "%tmp" };
            sus ptr = instr.operands[0];
            damn result_name + " = load " + instr.metadata + ", " + 
                 llvm_type_to_string(ptr.llvm_type) + " " + ptr.name;
        }
        
        mood InstructionType::Add {
            sus result_name = bestie (instr.result != nocap) { instr.result.name } highkey { "%tmp" };
            sus left = instr.operands[0];
            sus right = instr.operands[1];
            damn result_name + " = add " + llvm_type_to_string(left.llvm_type) + " " + 
                 left.name + ", " + right.name;
        }
        
        mood InstructionType::Sub {
            sus result_name = bestie (instr.result != nocap) { instr.result.name } highkey { "%tmp" };
            sus left = instr.operands[0];
            sus right = instr.operands[1];
            damn result_name + " = sub " + llvm_type_to_string(left.llvm_type) + " " + 
                 left.name + ", " + right.name;
        }
        
        mood InstructionType::Mul {
            sus result_name = bestie (instr.result != nocap) { instr.result.name } highkey { "%tmp" };
            sus left = instr.operands[0];
            sus right = instr.operands[1];
            damn result_name + " = mul " + llvm_type_to_string(left.llvm_type) + " " + 
                 left.name + ", " + right.name;
        }
        
        mood InstructionType::Ret {
            bestie (instr.operands.length() > 0) {
                sus value = instr.operands[0];
                damn "ret " + llvm_type_to_string(value.llvm_type) + " " + value.name;
            } highkey {
                damn "ret void";
            }
        }
        
        mood InstructionType::Br {
            damn "br label %" + instr.metadata;
        }
        
        mood InstructionType::BrCond {
            sus condition = instr.operands[0];
            sus blocks = instr.metadata.split(",");
            damn "br " + llvm_type_to_string(condition.llvm_type) + " " + condition.name + 
                 ", label %" + blocks[0] + ", label %" + blocks[1];
        }
        
        basic {
            damn "; unknown instruction";
        }
    }
}
