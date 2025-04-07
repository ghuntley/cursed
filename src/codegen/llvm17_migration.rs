//! LLVM 17 Migration Documentation
//! 
//! This file documents the changes required for migration from LLVM 14 to LLVM 17 API.
//! 
//! The codebase was found to already be using LLVM 17 compatible API calls:
//! 
//! 1. build_load - Already using the format: build_load(type, ptr, name)
//! 2. build_struct_gep - Already using the format: build_struct_gep(struct_type, ptr, index, name)
//! 3. build_call - Already using the format: build_call(fn_value, args, name)
//! 
//! No adapter was needed as the codebase appears to have been prepared for
//! LLVM 17 compatibility already.