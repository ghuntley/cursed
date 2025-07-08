# CURSED Target: LLVM IR

This document specifies the compilation target for the CURSED language. Instead of a custom virtual machine, CURSED will compile to LLVM Intermediate Representation (LLVM IR).

## Rationale

Targeting LLVM IR provides several advantages:

1.  **Leverages Existing Infrastructure**: Utilizes LLVM's mature ecosystem for optimization passes and native code generation.
2.  **Portability**: LLVM supports a wide range of target architectures.
3.  **Performance**: LLVM's optimizers can produce highly efficient machine code.
4.  **Interoperability**: Facilitates potential interaction with code written in other languages compiled via LLVM.

## Compilation Pipeline Adjustment

The final stage of the CURSED compiler pipeline ("Code Generation") will involve translating the compiler's internal representation (e.g., an optimized Abstract Syntax Tree or a custom IR) into LLVM IR.

```
[Source Code] -> Lexer -> Parser -> [AST] -> Semantic Analyzer -> [Typed AST/IR] -> Optimizer -> [Optimized IR] -> **LLVM IR Generator** -> [LLVM IR] -> LLVM Toolchain -> [Native Code]
```

## Mapping CURSED to LLVM IR

The following sections outline how core CURSED language features will map to LLVM IR constructs.

### 1. Types (`types.md`)

-   **Primitive Types**:
    -   `normie` (Integer): Mapped to LLVM integer types (e.g., `i32`).
    -   `snack/meal` (Float): Mapped to LLVM floating-point types (e.g., `float`/`double`).
    -   `tea` (String): Likely represented as a pointer to a struct containing length and character data (`{ i64, i8* }`). Requires runtime support for allocation and GC.
    -   `lit` (Boolean): Mapped to LLVM `i1`.
    -   `based` (Nil/Null): Represented by a null pointer (`ptr`).
-   **Composite Types**:
    -   `squad` (Struct): Mapped to LLVM `struct` types.
    -   Arrays/Slices: Represented similarly to strings (pointer to struct with length, capacity, data pointer). Requires runtime support.
    -   `tea[K]V` (Map): Implemented via runtime library calls, likely using hash tables. The LLVM IR will contain calls to these runtime functions.
-   **Interfaces (`collab`)**: Implemented using techniques like vtables or similar dynamic dispatch mechanisms, managed by the runtime. The LLVM IR will represent interface values as a structure containing a data pointer and a vtable pointer.

### 2. Variables and Scope (`sus`, `let`)

-   Local variables allocated on the stack using LLVM's `alloca` instruction.
-   Global variables mapped to LLVM global variables.
-   Closures will require capturing context, potentially allocating capture records on the heap managed by the GC.

### 3. Functions (`slay`)

-   CURSED functions mapped directly to LLVM `function` definitions.
-   Parameter passing and return values follow standard LLVM calling conventions.

### 4. Control Flow (`lowkey`, `highkey`, `periodt`, `bestie`)

-   Conditional statements (`lowkey`/`highkey`) mapped to LLVM conditional branch instructions (`br i1 ..., label %true, label %false`).
-   Loops (`periodt`) mapped using LLVM basic blocks and branch instructions to create loop structures.
-   While loops implemented using conditional branches with loop header, body, and exit blocks.
-   Switch-like statements (`bestie`) mapped using `switch` instructions or cascaded conditional branches.

### 5. Concurrency (`stan`, `dm`)

-   Goroutines (`stan`): Implemented via runtime library support for lightweight threads/tasks. Spawning a goroutine translates to an LLVM call to the runtime scheduler.
-   Channels (`dm`): Implemented via runtime library data structures and synchronization primitives. Channel operations translate to LLVM calls to the runtime.

### 6. Memory Management (Garbage Collection)

-   The compiler will generate LLVM IR that interacts with a Garbage Collector (GC) provided by the CURSED runtime.
-   Allocations (`make`, new objects) will involve calls to GC allocation functions.
-   The compiler needs to generate GC metadata (e.g., stack maps) to inform the GC about live pointers during collection, potentially using LLVM's GC support features (`gcroot`, statepoints).

### 7. Standard Library (`stdlib.md`)

-   Functions defined in the standard library will have corresponding LLVM function declarations.
-   Calls to standard library functions in CURSED code will be mapped to LLVM `call` instructions targeting these declared functions.
-   The standard library implementation (potentially written in CURSED itself or linked as native code) will be linked with the compiled user code.

## LLVM Toolchain Integration

The output of the CURSED compiler's LLVM IR generation stage will typically be:

1.  LLVM IR assembly (`.ll` file) or bitcode (`.bc` file).
2.  This IR is then processed by standard LLVM tools:
    -   `opt`: To apply LLVM optimization passes.
    -   `llc`: To generate native assembly or object code for the target architecture.
    -   A linker (like `clang` or `ld`) combines the object code with the CURSED runtime library and standard library to produce the final executable.

## Name Mangling and Module Compilation

### Initial Strategy: Single Module Compilation

To simplify the initial implementation, the CURSED compiler will adopt a **single-module compilation strategy**. This means:

1.  When the compiler processes the main source file and encounters `yeet` (import) statements, it recursively parses and analyzes all imported CURSED source files.
2.  All parsed code (from the main file and all imported files) is compiled into a single LLVM module (`.ll` file or in-memory representation).

### Name Mangling Scheme

To avoid naming conflicts between symbols (functions, globals) defined in different packages within this single LLVM module, a name mangling scheme is employed:

*   **Format:** `_<package_name>_<symbol_name>`
*   **Example:** A function `DoThing` in package `myutils` will have the LLVM name `_myutils_DoThing`.
*   **Private Symbols:** Private symbols (starting with lowercase) are also mangled using the same scheme (e.g., `_myutils_internalHelper`). This ensures they don't clash with symbols from other packages but doesn't imply they are accessible from outside their package at the CURSED language level.
*   **Main Package:** Symbols in the `main` package might use a slightly simpler mangling or none if guaranteed unique (e.g., `_main_MyFunc` or just `MyFunc` if no conflicts are possible, TBD).
*   **Built-ins:** Built-in functions (like `puts`) might have special, unmangled names (e.g., `@cursed_puts`) or follow a convention like `_builtin_puts`.

### Symbol Resolution

The compiler is responsible for resolving CURSED-level symbol access (e.g., `myutils.DoThing()`) to the correct mangled LLVM symbol name (`_myutils_DoThing`) during code generation.

### Future Goal: Separate Compilation

The long-term goal is to support separate compilation, where each CURSED package is compiled into its own LLVM module or object file, and these are then linked together. The single-module strategy is a stepping stone towards this more complex model.

## Runtime Considerations

The CURSED runtime will need to support the following:

1.  **Garbage Collection**: The runtime must manage memory allocation and deallocation efficiently.
2.  **Concurrency**: The runtime must handle multiple goroutines and channels effectively.
3.  **Standard Library**: The runtime must provide access to the standard library functions.
