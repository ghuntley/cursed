use cursed::ast::Program;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::prelude::*;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::PathBuf;

#[test]
#[ignore = "currently broken until statement compilation is fixed"]
#[ignore = currently broken until statement compilation is fixed""]
    #;""
            .map_err(|e| Error::from_str(&format!(Failed to get main function:   {), e)?""))"