//! Specialized hook for handling dot expressions
//! This module provides direct compilation support for package functions
//! like vibez.spill, htmlrizzler.escape_html, and timez.Now

use crate::ast::{expressions::{CallExpression, DotExpression, StringLiteral, Identifier}, base::Program};
use crate::codegen::llvm::context::LlvmCodeGenerator;
use crate::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

/// Generic dot expression function call hook
pub fn dot_expression_call_hook<'ctx>(
    gen: &mut LlvmCodeGenerator<'ctx>,
    call: &CallExpression
) -> Result<Option<inkwell::values::BasicValueEnum<'ctx>>, Error> {
    // Check if this is a dot expression call
    if let Some(dot) = call.function.as_any().downcast_ref::<DotExpression>() {
        // Get package name and function
        let package = dot.object.string();
        let function = &dot.property;
        
        // Check if we have a registered handler for this package.function
        if crate::stdlib::dot_registry::is_supported(&package, function) {
            // Extract the arguments
            let mut arg_strings = Vec::new();
            for arg in &call.arguments {
                if let Some(str_lit) = arg.as_any().downcast_ref::<StringLiteral>() {
                    arg_strings.push(str_lit.value.clone());
                }
            }
            
            // Execute the function using the registry
            match crate::stdlib::dot_registry::execute_dot(&package, function, arg_strings) {
                Ok(ref result) => {
                    // Create a global string with the result
                    let global_str = gen.create_global_string(
                        &result,
                        &format!("dot_result_{}", gen.string_literal_counter)
                    )?;
                    
                    // Return the global string pointer
                    return Ok(Some(global_str.into()));
                },
                Err(e) => {
                    return Err(Error::from_str(&format!("Failed to execute {}.{}: {}", package, function, e)));
                }
            }
        }
        
        // Special case handling for known functions (for backward compatibility)
        match (package.as_str(), function.as_str()) {
            // Handle vibez.spill
            ("vibez", "spill") if call.arguments.len() == 1 => {
                return handle_vibez_spill(gen, call, dot);
            },
            
            // Handle htmlrizzler.escape_html
            ("htmlrizzler", "escape_html") if call.arguments.len() == 1 => {
                return handle_htmlrizzler_escape_html(gen, call, dot);
            },
            
            // Handle timez.Now
            ("timez", "Now") if call.arguments.len() == 0 => {
                return handle_timez_now(gen, call, dot);
            },
            
            _ => {}
        }
    }
    
    // Not a dot expression call that we can handle directly
    Ok(None)
}

/// Legacy function for backward compatibility - delegates to the new generalized handler
pub fn vibez_spill_call_hook<'ctx>(
    gen: &mut LlvmCodeGenerator<'ctx>,
    call: &CallExpression
) -> Result<Option<inkwell::values::BasicValueEnum<'ctx>>, Error> {
    dot_expression_call_hook(gen, call)
}

/// Handle vibez.spill calls
fn handle_vibez_spill<'ctx>(
    gen: &mut LlvmCodeGenerator<'ctx>,
    call: &CallExpression,
    dot: &DotExpression
) -> Result<Option<inkwell::values::BasicValueEnum<'ctx>>, Error> {
    if let Some(str_lit) = call.arguments[0].as_any().downcast_ref::<StringLiteral>() {
        // Get or create a reference to the vibez_spill_direct function
        if let Some(vibez_spill_fn) = gen.module().get_function("vibez_spill_direct") {
            // Create a global string constant
            let global_str = gen.create_global_string(
                &str_lit.value,
                &format!("vibez_str_{}", gen.string_literal_counter)
            )?;
            
            // Call vibez_spill_direct with the string
            let result = gen.builder().build_call(
                vibez_spill_fn,
                &[global_str.into()],
                &format!("vibez_call_{}", gen.string_literal_counter)
            ).map_err(|e| Error::from_str(&format!("Failed to call vibez.spill: {}", e)))?
            .try_as_basic_value()
            .left()
            .ok_or_else(|| Error::from_str("vibez.spill did not return a value"))?;
            
            println!("DEBUG: Compiled direct vibez.spill call for: {}", str_lit.value);
            return Ok(Some(result));
        }
    }
    
    Ok(None)
}

/// Handle htmlrizzler.escape_html calls
fn handle_htmlrizzler_escape_html<'ctx>(
    gen: &mut LlvmCodeGenerator<'ctx>,
    call: &CallExpression,
    dot: &DotExpression
) -> Result<Option<inkwell::values::BasicValueEnum<'ctx>>, Error> {
    if let Some(str_lit) = call.arguments[0].as_any().downcast_ref::<StringLiteral>() {
        // Basic HTML escaping
        let input = &str_lit.value;
        let escaped = input
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#39;");
            
        // Create a global string with the escaped HTML
        let global_str = gen.create_global_string(
            &escaped,
            &format!("html_escaped_{}", gen.string_literal_counter)
        )?;
        
        // Return the global string pointer
        println!("DEBUG: Compiled htmlrizzler.escape_html call for: {}", input);
        let i8_ptr_type = gen.context().i8_type().ptr_type(inkwell::AddressSpace::default());
        return Ok(Some(global_str.into()));
    }
    
    Ok(None)
}

/// Handle timez.Now calls
fn handle_timez_now<'ctx>(
    gen: &mut LlvmCodeGenerator<'ctx>,
    call: &CallExpression,
    dot: &DotExpression
) -> Result<Option<inkwell::values::BasicValueEnum<'ctx>>, Error> {
    // Get the current time as a string
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH)
        .map_err(|e| Error::from_str(&format!("Failed to get system time: {}", e)))?
        .as_secs();
        
    let time_str = format!("{}s", timestamp);
    
    // Create a global string with the current time
    let global_str = gen.create_global_string(
        &time_str,
        &format!("time_now_{}", gen.string_literal_counter)
    )?;
    
    // Return the global string pointer
    println!("DEBUG: Compiled timez.Now call for current time: {}", time_str);
    return Ok(Some(global_str.into()));
}

/// Patch the main function to directly include vibez.spill calls
/// This is a temporary solution until we have a proper AST translation for dot expressions
pub fn patch_main_function<'ctx>(
    gen: &mut LlvmCodeGenerator<'ctx>,
    program: &Program
) -> Result<(), Error> {
    // First, collect all vibez.spill calls
    let mut vibez_spill_calls = Vec::new();
    
    // Look for vibez.spill calls in the program
    for stmt in &program.statements {
        if let Some(expr_stmt) = stmt.as_any().downcast_ref::<crate::ast::statements::ExpressionStatement>() {
            if let Some(expr) = &expr_stmt.expression {
                if let Some(call) = expr.as_any().downcast_ref::<CallExpression>() {
                    if let Some(dot) = call.function.as_any().downcast_ref::<DotExpression>() {
                        if dot.object.string() == "vibez" && dot.property == "spill" && call.arguments.len() == 1 {
                            if let Some(str_lit) = call.arguments[0].as_any().downcast_ref::<StringLiteral>() {
                                vibez_spill_calls.push(str_lit.value.clone());
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Find the main function and insert calls
    if let Some(main_fn) = gen.module().get_function("main") {
        // Get the entry block of main
        if let Some(entry) = main_fn.get_first_basic_block() {
            // Save current position
            let current_block = gen.builder().get_insert_block();
            
            // Position at the beginning of the main function
            gen.builder().position_at_end(entry);
            
            // Clear out the entry block
            while let Some(instr) = entry.get_first_instruction() {
                instr.remove_from_basic_block();
            }
            
            // Add vibez.spill calls
            if let Some(vibez_spill_fn) = gen.module().get_function("vibez_spill_direct") {
                for (i, text) in vibez_spill_calls.iter().enumerate() {
                    // Create a global string
                    let global_str = gen.create_global_string(text, &format!("patched_str_{}", i))?;
                    // Add the call
                    gen.builder().build_call(
                        vibez_spill_fn,
                        &[global_str.into()],
                        &format!("patched_call_{}", i)
                    ).map_err(|e| Error::from_str(&format!("Failed to add patched call: {}", e)))?;
                    
                    println!("DEBUG PATCH: Added patched vibez.spill call for: {}", text);
                }
                
                // Add a return instruction
                let zero = gen.context().i32_type().const_int(0, false);
                gen.builder().build_return(Some(&zero))
                    .map_err(|e| Error::from_str(&format!("Failed to add return: {}", e)))?;
                    
                println!("DEBUG PATCH: Built new main function body with vibez.spill calls");
            }
            
            // Restore original position
            if let Some(block) = current_block {
                gen.builder().position_at_end(block);
            }
            
            return Ok(());
        }
    }
    
    // Main function not found, or no entry block
    Err(Error::from_str("Failed to patch main function: not found or no entry block"))
}