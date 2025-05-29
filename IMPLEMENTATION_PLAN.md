# Test Failure Resolution Plan

## Priority 1: Critical Syntax and Definition Errors

These errors prevent the codebase from compiling and must be addressed first.

*   **RESOLVED: `error: expected ',', found ':'` in `src/core/constraint_recovery.rs:340:17`**
    *   Description: A syntax error in a `panic!` macro call.
    *   Action: Fixed by correctly escaping the quotes in the panic macro string literal.
*   **RESOLVED: `error[E0592]: duplicate definitions with name 'is_string_type'`** (multiple occurrences)
    *   Description: The function `is_string_type` is defined in multiple locations (`src/codegen/llvm/string_switch.rs` and `src/codegen/llvm/range_clause_fixed_extension.rs`).
    *   Action: Resolved by using the implementation provided by `StringUtilsExtension` trait in `src/codegen/llvm/string_utils.rs`. In `range_clause_fixed_extension.rs`, the function is now properly imported and used from the trait.
*   **RESOLVED: `error[E0592]: duplicate definitions with name 'create_string_constant'`** (multiple occurrences)
    *   Description: The function `create_string_constant` has multiple definitions across different files.
    *   Action: Resolved by using the implementation provided by `StringUtilsExtension` trait in `src/codegen/llvm/string_utils.rs`. In other files that need this functionality, `create_string_constant_from_codegen` is now used from `interface_type_assertion_common.rs`.
*   **RESOLVED: `error[E0592]: duplicate definitions with name 'get_result_type'`**
    *   Description: `get_result_type` defined in both `interface_type_assertion_error_propagation.rs` and `interface_type_assertion_result_implementation.rs`.
    *   Action: Resolved by moving the function to `interface_type_assertion_common.rs` and importing it where needed.
*   **RESOLVED: `error[E0592]: duplicate definitions with name 'get_source_location_type'`**
    *   Description: `get_source_location_type` defined in both `interface_type_assertion_error_propagation.rs` and `interface_type_assertion_result_implementation.rs`.
    *   Action: Resolved by moving the function to `interface_type_assertion_common.rs` and importing it where needed.
*   **RESOLVED: `error[E0592]: duplicate definitions with name 'call_error_propagation_function'`**
    *   Description: `call_error_propagation_function` defined in both `interface_type_assertion_error_propagation.rs` and `interface_type_assertion_result_implementation.rs`.
    *   Action: Resolved by moving the function to `interface_type_assertion_common.rs` and importing it where needed.
*   **RESOLVED: `error[E0592]: duplicate definitions with name 'build_struct_value'`**
    *   Description: `build_struct_value` defined in both `interface_type_assertion_error_propagation.rs` and `interface_type_assertion_result_implementation.rs`.
    *   Action: Resolved by moving the function to `interface_type_assertion_common.rs` and importing it where needed.
*   **RESOLVED: `error[E0592]: duplicate definitions with name 'find_inheritance_path'`** (multiple occurrences)
    *   Description: `find_inheritance_path` is defined in multiple files with slightly different signatures/purposes.
    *   Action: Resolved by moving the core functionality to `interface_type_assertion_common.rs` with the common `InterfaceRegistry` trait. Different implementations now make use of this centralized function.
*   **RESOLVED: `error[E0592]: duplicate definitions with name 'get_type_name_by_id'`**
    *   Description: `get_type_name_by_id` defined in both `interface_type_assertion_error_propagation_filesystem.rs` and `interface_type_assertion_diamond_inheritance.rs`.
    *   Action: Resolved by using the common implementation `get_type_name_by_id_impl` from `interface_type_registry_common.rs` in both files. Removed the duplicated implementation in `interface_type_assertion_diamond_inheritance.rs` and modified `interface_type_assertion_error_propagation_filesystem.rs` to use the common implementation.
*   **RESOLVED: `error[E0592]: duplicate definitions with name 'get_interface_path_finder'`** (multiple occurrences)
    *   Description: `get_interface_path_finder` has multiple definitions.
    *   Action: Fixed by updating benchmark code to use the common implementation from `interface_type_registry_common`.
*   **RESOLVED: `error[E0592]: duplicate definitions with name 'type_implements'`** (multiple occurrences)
    *   Description: `type_implements` has multiple definitions.
    *   Action: Fixed by using the common implementation `type_implements_impl` from `interface_type_registry_common`.
*   **RESOLVED: `error[E0592]: duplicate definitions with name 'get_interface_registry'`** (multiple occurrences)
    *   Description: `get_interface_registry` has multiple definitions.
    *   Action: Fixed by using the common implementation `get_interface_registry_impl` from `interface_type_registry_common`.
*   **RESOLVED: `error[E0592]: duplicate definitions with name 'get_interface_registry_mut'`**
    *   Description: `get_interface_registry_mut` defined in `interface_type_assertion_benchmark.rs` and `interface_type_assertion_benchmark_enhanced.rs`.
    *   Action: Fixed by using the common implementation `get_interface_registry_mut_impl` from `interface_type_registry_common`.
*   **RESOLVED: `error[E0592]: duplicate definitions with name 'detect_diamond_inheritance'`**
    *   Description: `detect_diamond_inheritance` defined in `interface_type_assertion_benchmark.rs` and `interface_type_assertion_benchmark_enhanced.rs`.
    *   Action: Fixed by using the common implementation `detect_diamond_inheritance_impl` from `interface_type_registry_common`.
*   **RESOLVED: Missing `util::LlvmCodeGeneratorExtension` import in `range_clause_fixed_extension.rs`**
    *   Description: Import error for non-existent `LlvmCodeGeneratorExtension` trait.
    *   Action: Removed the erroneous import since the trait doesn't exist in the util module.
*   **RESOLVED: Invalid trait implementation `LlvmCodeGenerator` for `InterfaceTypeAssertionBenchmark`**
    *   Description: Attempting to implement a struct as a trait in `interface_type_assertion_benchmark_enhanced.rs`.
    *   Action: Removed the entire erroneous impl block since it was implementing a struct instead of a trait.
*   **RESOLVED: Missing `Error::InvalidArguments` variant usage**
    *   Description: Code attempting to use non-existent `InvalidArguments` variant of `Error` enum.
    *   Action: Replaced `Error::InvalidArguments()` calls with `Error::new("InvalidArguments", message, None)` in stdlib modules.
*   **RESOLVED: Missing `Object::Nil` variant usage**
    *   Description: Code attempting to use non-existent `Nil` variant of `Object` enum.
    *   Action: Replaced `Object::Nil` with `Object::Null` in stdlib/vibez.rs.
*   **RESOLVED: Duplicate function definitions across error propagation modules**
    *   Description: Functions like `get_result_type`, `get_source_location_type`, `call_error_propagation_function`, and `build_struct_value` were duplicated.
    *   Action: Removed duplicate implementations from `interface_type_assertion_error_propagation.rs` that were calling common implementations.
*   **RESOLVED: Duplicate function definitions in benchmark modules**
    *   Description: Functions like `find_inheritance_path`, `get_interface_path_finder`, `type_implements`, `detect_diamond_inheritance`, and `get_interface_registry` were duplicated.
    *   Action: Removed duplicate implementations from `interface_type_assertion_benchmark.rs` that were calling common implementations.
*   **RESOLVED: `error[E0592]: duplicate definitions with name 'from_seconds'` in `src/stdlib/timez/mod.rs`**
    *   Description: `from_seconds` is defined twice within the same module.
    *   Action: Upon investigation, no duplicate was found. This error may have been previously resolved.
*   **`error[E0034]: multiple applicable items in scope` for `is_string_type`** (multiple occurrences)
    *   Description: The compiler finds multiple `is_string_type` methods when one is called. This is a consequence of the duplicate definitions.
    *   Action: This will likely be resolved by fixing the `E0592` errors for `is_string_type`.

## Priority 2: Type Mismatches and Trait Implementation Errors

These errors indicate issues with type compatibility, trait bounds, or incorrect method calls.

*   **RESOLVED: `error[E0308]: mismatched types` in `src/codegen/llvm/context.rs:197:17` and `src/codegen/llvm/context.rs:206:120`**
    *   Description: Arguments passed to `InterfaceRegistryAdapter::new` and `InterfaceTypeRegistry::with_extension_registry` do not match the expected types (related to `Arc<RwLock<...>>` wrapping).
    *   Action: Fixed by updating type signatures to use `Arc<RwLock<ThreadSafeInterfaceExtensionRegistry>>` consistently across the codebase and adding trait implementations for the Arc<RwLock> wrapper.
*   **RESOLVED: `error[E0061]: this method takes 2 arguments but 3 arguments were supplied` in `src/codegen/llvm/context.rs:807:37`**
    *   Description: `get_context_lines` is called with an incorrect number of arguments.
    *   Action: Fixed by converting the parameters to a SourceLocation struct and updating the return type conversion from HashMap to Vec.
*   **RESOLVED: `error[E0308]: mismatched types` in `src/codegen/llvm/context.rs:807:9` (return type)**
    *   Description: The return type of `get_context_lines` does not match the expected `io::Result<Vec<String>>`.
    *   Action: Fixed by converting the HashMap<usize, String> result to Vec<String> for API compatibility.
*   **RESOLVED: `error[E0609]: no field 'type_parameter' on type '&GenericConstraint'` in `src/codegen/llvm/enhanced_monomorphization.rs:106:42`**
    *   Description: Accessing a non-existent field `type_parameter` on `GenericConstraint`.
    *   Action: Fixed by using the correct field name `parameter_name`.
*   **RESOLVED: `error[E0609]: no field 'trait_name' on type '&GenericConstraint'` in `src/codegen/llvm/enhanced_monomorphization.rs:107:46`**
    *   Description: Accessing a non-existent field `trait_name` on `GenericConstraint`.
    *   Action: Fixed by using the correct field name `interface_name`.
*   **RESOLVED: `error[E0277]: the trait bound \`BasicMetadataTypeEnum<'_>: From<&BasicTypeEnum<'_>>\` is not satisfied`** (multiple occurrences in `interface_field_accessors_lru.rs`)
    *   Description: Attempting to convert `&BasicTypeEnum` to `BasicMetadataTypeEnum` where the `From` trait is not implemented.
    *   Action: Fixed by dereferencing the reference (`(*t).into()`) as suggested by the compiler.
*   **RESOLVED: `error[E0599]: no method named 'string' found for reference '&type_checker::Type'` in `src/codegen/llvm/function_monomorphization.rs:63:42`**
    *   Description: Calling a non-existent method `string()`.
    *   Action: Fixed by using `to_string()` method.
*   **RESOLVED: `error[E0609]: no field 'type_name' on type '&Parameter'` in `src/codegen/llvm/function_monomorphization.rs:145:41`**
    *   Description: Accessing a non-existent field `type_name` on `Parameter`.
    *   Action: Fixed by using the correct field name `param_type` and calling `string()` method on the Expression trait.
*   **`error[E0308]: mismatched types` in `src/codegen/llvm/function_monomorphization.rs:510:47`, `514:42`, `516:36`**
    *   Description: Incorrect types used when constructing `TypeParameter`, `Parameter`, and `Token`.
    *   Action: Ensure the correct types are used for initialization, possibly wrapping values in appropriate enums or structs.
*   **`error[E0599]: no variant or associated item named 'NotFound' found for enum 'error::Error'`** (multiple occurrences in `interface_registry.rs`)
    *   Description: Attempting to use a non-existent `NotFound` variant for `error::Error`.
    *   Action: Use the correct error variants or error creation methods (e.g., `Error::new(...)`).
*   **`error[E0282]: type annotations needed for 'HashMap<_, _>'` in `src/codegen/llvm/interface_registry.rs:114:13`**
    *   Description: The compiler cannot infer the types for a `HashMap`.
    *   Action: Provide explicit type annotations for the `HashMap`.
*   **`error[E0308]: mismatched types` in `src/codegen/llvm/range_clause_fixed.rs` (multiple related to `AnyTypeEnum` vs `BasicTypeEnum`)**
    *   Description: Pattern matching on `AnyTypeEnum` where `BasicTypeEnum` is expected.
    *   Action: Ensure the match is on the correct enum type or convert between them if necessary.
*   **`error[E0599]: no method named 'as_array_type' / 'as_struct_type' found for enum 'BasicTypeEnum'`** (in `range_clause_fixed.rs`)
    *   Description: Calling non-existent `as_array_type` or `as_struct_type`.
    *   Action: Use the correct methods like `is_array_type()` or `into_array_type()`.
*   **`error[E0624]: method 'ensure_runtime_container_functions' is private`** (multiple in `range_clause_error_recovery.rs`)
    *   Description: Attempting to call a private method from outside its defining module/scope.
    *   Action: Make the method public if it's intended to be part of the public API, or refactor the code to avoid the cross-module call. Similar issues for `emit_map_iterator_create_fixed`, `emit_container_length_fixed`, etc. These might require importing the correct trait (`RangeClauseFixedMethodsExtension`).
*   **`error[E0599]: no method named 'ok_or_else' found for reference '&Module<'_>'`** (multiple in `range_clause_error_recovery.rs`)
    *   Description: `self.module()` returns `&Module`, not an `Option` or `Result` that `ok_or_else` can be called on.
    *   Action: Adjust logic to correctly handle the `&Module` type. If `self.module` is intended to be optional, its return type or internal storage might need adjustment.
*   **`error[E0599]: no variant or associated item named 'Parsing' found for enum 'error::Error'`** (multiple in `range_clause_error_recovery.rs` and `parser/range_expression_error_recovery.rs`)
    *   Description: Attempting to use a non-existent `Parsing` variant for `error::Error`.
    *   Action: Use the correct error variants or error creation methods.
*   **`error[E0599]: no method named 'get_element_type' found for struct 'inkwell::types::PointerType'`** (multiple in `range_clause_fixed_extension.rs`)
    *   Description: The `PointerTypeExtension` trait providing `get_element_type` is not in scope.
    *   Action: Import `crate::codegen::llvm::pointer_type_extension::PointerTypeExtension`.
*   **`error[E0599]: no method named 'as_struct_type' found for enum 'BasicTypeEnum'` in `map_iteration_improvements.rs`**
    *   Description: Calling a non-existent `as_struct_type`.
    *   Action: Use `is_struct_type()` or `into_struct_type()`.
*   **`error[E0308]: mismatched types` and `error[E0277]: cannot multiply 'u64' by 'u32'` in `src/codegen/llvm/concurrency.rs`**
    *   Description: Type conversion issues between `u32` and `u64`, and lack of direct multiplication.
    *   Action: Ensure types are consistently `u64` or explicitly cast `u32` to `u64` before multiplication.
*   **`error[E0308]: mismatched types` in `src/codegen/llvm/property_access.rs:43:62` (expected `&str`, found `String`)**
    *   Description: Passing `String` where `&str` is expected.
    *   Action: Borrow the `String` using `&` or `.as_str()`.
*   **`error[E0599]: no method named 'get_pointee_type' found for struct 'inkwell::types::PointerType'` in `src/codegen/llvm/property_access.rs:92:43`**
    *   Description: Calling a non-existent method.
    *   Action: Investigate the correct LLVM API for getting the pointee type.
*   **`error[E0609]: no field '1' on type '&std::string::String'` in `src/codegen/llvm/property_access.rs:159:36`**
    *   Description: Attempting to access a tuple field on a `String`.
    *   Action: Correct the field access; `field_info` is likely a tuple like `(String, Type)`, so access should be `field_info.0` for name and `field_info.1` for type. The comparison `&field_info.1 == field_name` seems incorrect.
*   **`error[E0308]: mismatched types` in `src/codegen/llvm/interface_type_assertion.rs:120:10` (return type `()` vs `Result<(), Error>`)**
    *   Description: Function implicitly returns `()` but is declared to return `Result<(), Error>`.
    *   Action: Ensure the function returns an appropriate `Result`, likely `Ok(())`.
*   **`error[E0599]: no method named 'get_assertion_type_info' found for mutable reference '&mut LlvmCodeGenerator<'ctx>'`** (multiple in `interface_type_assertion_errors.rs`)
    *   Description: The `EnhancedTypeRegistry` trait is not in scope.
    *   Action: Import `crate::codegen::llvm::interface_type_registry_enhanced::EnhancedTypeRegistry`.
*   **`error[E0599]: no method named 'is_int_value' / 'into_int_value' found for struct 'inkwell::values::IntValue'`** (in `type_assertion_implementation.rs`)
    *   Description: Calling non-existent methods on `IntValue`.
    *   Action: Check Inkwell documentation for correct methods to check type or convert.
*   **`error[E0277]: the trait bound \`inkwell::values::PointerValue<'_>: BasicType<'_>\` is not satisfied`** (in `enhanced_dynamic_dispatch.rs`)
    *   Description: `build_ptr_diff` expects arguments that implement `BasicType`. `PointerValue` itself is a value, not a type.
    *   Action: Pass the *type* of the pointers (`PointerType`) or ensure the LLVM API is used correctly.
*   **`error[E0061]: this method takes 4 arguments but 3 arguments were supplied` for `build_ptr_diff`**
    *   Description: Incorrect number of arguments for `build_ptr_diff`.
    *   Action: Supply the correct arguments.
*   **`error[E0599]: no method named 'call_interface_method' found for mutable reference '&mut LlvmCodeGenerator<'ctx>'`** (in `enhanced_dynamic_dispatch.rs`)
    *   Description: The `InterfaceImplementation` trait is not in scope.
    *   Action: Import `crate::codegen::llvm::interface_implementation::InterfaceImplementation`. Consider `call_interface_method_enhanced`.
*   **`error[E0308]: arguments to this method are incorrect` for `call_interface_method_enhanced`** (multiple in `optimized_dynamic_dispatch.rs`)
    *   Description: Passing `PointerValue` where `BasicValueEnum` is expected, or `&[T]` where `Vec<T>` is expected.
    *   Action: Wrap `PointerValue` in `BasicValueEnum::PointerValue(...)` and convert slices `args.to_vec()`.
*   **`error[E0615]: attempted to take value of method 'monomorphization_manager'`** (multiple in `interface_field_accessors.rs`)
    *   Description: Missing `()` when calling the method.
    *   Action: Add `()` to call `self.monomorphization_manager()`.
*   **`error[E0599]: no method named 'register_implementation' found for mutable reference '&mut InterfaceManager<'ctx>'`**
    *   Description: Method not found.
    *   Action: Check `InterfaceManager` for the correct method name or implement it.
*   **`error[E0624]: method 'get_extension_hierarchy' is private`** (in `interface_type_registry.rs`)
    *   Description: Calling a private method.
    *   Action: Import `InterfaceRegistryExtensionWithVisualization` if appropriate or make method public.
*   **`error[E0277]: the size for values of type 'str' cannot be known at compilation time`** (in `interface_type_registry.rs` when iterating `interface_names`)
    *   Description: Trying to iterate over `interface_names` where `interface_name` is `str`.
    *   Action: Ensure `interface_names` is a collection of owned `String`s or `&str` that live long enough. If it's `Vec<&str>`, cloning to `Vec<String>` might be needed if ownership is an issue.
*   **`error[E0616]: field 'type_ids_global' / 'type_names_global' of struct 'InterfaceTypeRegistry' is private`** (multiple in `interface_type_registry_enhanced.rs`)
    *   Description: Direct access to private fields.
    *   Action: Use public methods if available, or reconsider the design of `InterfaceTypeRegistry` if these fields need to be accessed/modified from outside.
*   **`error[E0599]: no method named 'init_source_file_cache' / 'enhance_source_location' / etc.`** (multiple in `interface_type_assertion_error_propagation.rs` and related files)
    *   Description: Calling methods that are not defined or not in scope. This points to significant refactoring or missing trait imports related to error propagation and source location handling.
    *   Action: These require a careful review of the error handling and source location modules. Ensure necessary traits are imported (e.g., `EnhancedErrorPropagationWithFilesystem`, `Node`) and methods exist or are correctly named.
*   **`error[E0277]: the trait bound \`BasicValueEnum<'_>: From<Result<PointerValue<'_>, Error>>\` is not satisfied`** (multiple in `interface_type_assertion_error_propagation.rs`)
    *   Description: Cannot directly convert `Result<PointerValue, Error>` to `BasicValueEnum` using `.into()`.
    *   Action: Handle the `Result` first (e.g., using `?` or `match`) and then convert the `PointerValue` to `BasicValueEnum` (e.g., `pointer_value.into()` or `BasicValueEnum::PointerValue(pointer_value)`).
*    **Many `E0599`: method not found / no variant or associated item named errors** across `codegen/llvm/`, `core/`, `stdlib/`, `parser/`, `runtime/` and `error/` modules.
    *   Description: These are numerous and indicate widespread issues with method calls, enum variants, or trait implementations being incorrect or missing.
    *   Action: These will need to be tackled systematically, likely module by module or by feature area, after the critical definition and type errors are resolved. Many might be cascading effects from earlier errors.

## Priority 3: Logic Errors and Deprecations

These errors are related to incorrect program logic, deprecated API usage, or minor warnings.

*   **`warning: unnecessary parentheses around block return value`** (multiple in `interface_registry_visualization.rs`)
    *   Action: Remove the unnecessary parentheses as suggested by the compiler.
*   **`warning: use of deprecated associated function \`chrono::NaiveDateTime::from_timestamp_opt\` / \`chrono::DateTime::<Tz>::from_utc\``** (in `stdlib/timez/mod.rs`)
    *   Action: Update to the suggested newer chrono APIs.
*   **`error[E0277]: the trait bound \`char: tracing::Value\` is not satisfied` in `src/runtime/jit_runtime.rs:128:9`**
    *   Description: `char` cannot be directly used as a `tracing::Value`.
    *   Action: Convert the char to a `String` or use a wrapper that implements `tracing::Value`.
*   **`error[E0614]: type \`object::Object\` cannot be dereferenced`** (multiple in `stdlib/dot_registry.rs` and `stdlib_test.rs`)
    *   Description: Attempting to dereference `&*result` where `result` is an `Rc<Object>`. This is usually done to get `&Object` from `&Rc<Object>`, but the pattern matching `Object::String(s)` expects `&Object`. The double dereference `&**obj` in `if let Object::Template(template) = &**obj` is likely correct if `obj` is `Rc<Object>`. The ones with `&*result` might be incorrect if `result` is already `&Object`.
    *   Action: Review each case. If `result` is `Rc<Object>`, then `&*result` gives `Object`, and `&(*result)` or `result.as_ref()` gives `&Object`. Ensure the pattern matches the type being referenced.
*   **`error[E0599]: no variant or associated item named 'ExternalData' / 'Template' / 'Nil' / 'Function' found for enum 'object::Object'`** (multiple in `stdlib/`)
    *   Description: Enum variants are misspelled, removed, or renamed in `object::Object`.
    *   Action: Update to use the correct variants of the `Object` enum.
*   **`error[E0164]: expected tuple struct or tuple variant, found struct variant \`Object::Builtin\``** (in `stdlib/mod.rs`)
    *   Description: `Object::Builtin` is a struct variant, but the pattern `Object::Builtin(_)` treats it as a tuple variant.
    *   Action: Change the pattern to `Object::Builtin { name: _, function: _ }` or `Object::Builtin { .. }`.
*   **`error[E0599]: no method named \`size_estimate\` found for reference \`&object::Object\`` in `src/runtime/channel_gc.rs`**
    *   Action: Implement `size_estimate` for `Object` or find an alternative way to get its size.
*   **`error[E0308]: \`else\` clause of \`let...else\` does not diverge` in `src/runtime/container.rs`**
    *   Description: The `else` block of a `let...else` statement must diverge (e.g., `panic!`, `return`, or an infinite loop).
    *   Action: Ensure the `else` block diverges, or refactor to a standard `match` or `if/else`.
*   **`error[E0277]: \`Rc<...>\` cannot be sent/shared between threads safely`** (multiple in `runtime/container.rs` and `object_thread_safe.rs`)
    *   Description: `Rc` and `RefCell` are not thread-safe. These are likely being used in contexts where `Arc` and `Mutex`/`RwLock` are needed, especially related to GC and concurrent operations.
    *   Action: Replace `Rc` with `Arc` and `RefCell` with `Mutex` or `RwLock` where thread safety is required. This is a significant change and needs careful consideration of locking strategies.
*   **`warning: function cannot return without recursing` for `detect_diamond_inheritance`**
    *   Description: The function calls itself unconditionally.
    *   Action: Add a base case or condition to terminate the recursion.
*   **`error[E0502]: cannot borrow \`*self\` as mutable because it is also borrowed as immutable`** (in `struct_monomorphization.rs` and `optimized_dynamic_dispatch.rs`)
    *   Description: Rust's borrowing rules are violated. A mutable borrow occurs while an immutable borrow is still active, or vice-versa.
    *   Action: Refactor the code to ensure borrows do not overlap in a way that violates the rules. This might involve changing the order of operations, cloning data, or using interior mutability patterns if appropriate (though be cautious with interior mutability).
*   **`error[E0382]: use of partially moved value: \`concrete_type\`` in `struct_monomorphization.rs`**
    *   Description: A field of `concrete_type` (which is `Type::Named(name)`) was moved, and then `concrete_type` is used again.
    *   Action: Borrow the `name` in the pattern: `Type::Named(ref name)`.
*   **`error[E0004]: non-exhaustive patterns` for `TokenType::Question` and `Token::{Less, Greater, Question}`**
    *   Description: `match` statements are not covering all possible enum variants.
    *   Action: Add match arms for the missing variants or a wildcard `_` arm.

## Priority 4: Lower Priority/Test-Specific Issues

These errors are often found in test code or benchmarks and might be due to outdated test setup or minor bugs.

*   **Errors specific to benchmark files like `interface_type_assertion_benchmark.rs` and `interface_type_assertion_benchmark_enhanced.rs`**: Many of these are duplicate definition errors or using outdated/incorrect methods related to the interface registry and path finding.
    *   Action: These will likely be resolved by fixing the core issues in the `codegen` and `core` modules. Update benchmark code to use the corrected APIs.
*   **Errors in `stdlib_test.rs` related to `Object::String` dereferencing:**
    *   Action: Fix pattern matching for `Object::String` as per the `E0614` errors.
*   **Errors in `core/async_constraint_checker.rs` related to `assert_eq!` on `Result<bool, error::Error>`:**
    *   Description: `error::Error` does not implement `PartialEq`.
    *   Action: Derive `PartialEq` for `error::Error` if appropriate, or change assertions to `assert!(result.is_ok_and(|v| v == true))` or match on the `Result`.

This plan will be updated as errors are resolved and new priorities emerge.
