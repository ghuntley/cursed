// CURSED Stage 2 Self-Hosting Compiler
// Written in CURSED using only the minimal subset for bootstrapping

import "std/io"
import "std/os"
import "std/string"

// Main entry point for the Stage 2 compiler
func main() {
    args := os.args()
    
    if len(args) < 2 {
        io.println("Usage: cursed-stage2 <source_file> [output_file]")
        os.exit(1)
    }
    
    source_file := args[1]
    output_file := "a.out"
    if len(args) >= 3 {
        output_file = args[2]
    }
    
    // Create compiler instance
    compiler := new_compiler()
    
    // Compile the source file
    result := compiler.compile_file(source_file, output_file)
    
    if result.has_errors {
        io.println("Compilation failed:")
        for error in result.errors {
            io.println(error)
        }
        os.exit(1)
    }
    
    io.println("Compilation successful: " + output_file)
}

// Compiler represents the Stage 2 CURSED compiler
struct Compiler {
    lexer: Lexer
    parser: Parser
    codegen: CodeGenerator
    errors: []string
}

// Create a new compiler instance
func new_compiler() *Compiler {
    return &Compiler{
        lexer: new_lexer(),
        parser: new_parser(), 
        codegen: new_codegen(),
        errors: []string{},
    }
}

// CompileResult represents the result of compilation
struct CompileResult {
    has_errors: bool
    errors: []string
}

// Compile a source file to executable
func (c *Compiler) compile_file(source_file: string, output_file: string) CompileResult {
    // Read source file
    source := io.read_file(source_file)
    if source == "" {
        return CompileResult{
            has_errors: true,
            errors: []string{"Failed to read source file: " + source_file},
        }
    }
    
    // Lexical analysis
    tokens := c.lexer.tokenize(source)
    if c.lexer.has_errors() {
        return CompileResult{
            has_errors: true,
            errors: c.lexer.get_errors(),
        }
    }
    
    // Parsing
    ast := c.parser.parse(tokens)
    if c.parser.has_errors() {
        return CompileResult{
            has_errors: true,
            errors: c.parser.get_errors(),
        }
    }
    
    // Code generation
    success := c.codegen.generate(ast, output_file)
    if !success {
        return CompileResult{
            has_errors: true,
            errors: c.codegen.get_errors(),
        }
    }
    
    return CompileResult{
        has_errors: false,
        errors: []string{},
    }
}
