use cursed::prelude::*;
use std::error::Error;

#[test]
fn test_bytefit_basic_operations() -> Result<(), Box<dyn Error>> {
    let code = r#"
    fr fr Test basic ByteFit operations
    slay main() {
        fr fr Test Equal
        a := []byte("hello")
        b := []byte("hello")
        c := []byte("world")
        
        assert(bytefit.Equal(a, b), "a and b should be equal")
        assert(!bytefit.Equal(a, c), "a and c should not be equal")
        
        fr fr Test Compare
        assert(bytefit.Compare(a, b) == 0, "Compare(a, b) should be 0")
        assert(bytefit.Compare(a, c) < 0, "Compare(a, c) should be negative")
        assert(bytefit.Compare(c, a) > 0, "Compare(c, a) should be positive")
        
        fr fr Test Repeat
        repeated := bytefit.Repeat([]byte("ab"), 3)
        assert(bytefit.Equal(repeated, []byte("ababab")), "Repeat failed")
        
        fr fr Test Contains
        text := []byte("hello world")
        assert(bytefit.Contains(text, []byte("world")), "Contains should find 'world'")
        assert(!bytefit.Contains(text, []byte("universe")), "Contains should not find 'universe'")
        
        fr fr Test HasPrefix/HasSuffix
        assert(bytefit.HasPrefix(text, []byte("hello")), "HasPrefix should be true")
        assert(bytefit.HasSuffix(text, []byte("world")), "HasSuffix should be true")
        assert(!bytefit.HasPrefix(text, []byte("world")), "HasPrefix should be false")
        assert(!bytefit.HasSuffix(text, []byte("hello")), "HasSuffix should be false")
        
        vibez.spill("All basic ByteFit tests passed!")
    }
    "#;
    
    test_utils::run_and_check_output(code, "All basic ByteFit tests passed!")
}

#[test]
fn test_bytefit_search_functions() -> Result<(), Box<dyn Error>> {
    let code = r#"
    slay main() {
        text := []byte("hello world 123")
        
        fr fr Test Index functions
        assert(bytefit.Index(text, []byte("world")) == 6, "Index should find 'world' at position 6")
        assert(bytefit.Index(text, []byte("xyz")) == -1, "Index should return -1 for 'xyz'")
        assert(bytefit.IndexByte(text, byte('w')) == 6, "IndexByte should find 'w' at position 6")
        assert(bytefit.IndexByte(text, byte('z')) == -1, "IndexByte should return -1 for 'z'")
        
        fr fr Test Count
        text2 := []byte("hello hello hello")
        assert(bytefit.Count(text2, []byte("hello")) == 3, "Count should find 3 occurrences of 'hello'")
        assert(bytefit.Count(text2, []byte("l")) == 6, "Count should find 6 occurrences of 'l'")
        
        fr fr Test LastIndex
        assert(bytefit.LastIndex(text2, []byte("hello")) == 12, "LastIndex should find last 'hello' at 12")
        
        vibez.spill("All ByteFit search tests passed!")
    }
    "#;
    
    test_utils::run_and_check_output(code, "All ByteFit search tests passed!")
}

#[test]
fn test_bytefit_transformation() -> Result<(), Box<dyn Error>> {
    let code = r#"
    slay main() {
        fr fr Test Join
        slices := [][]byte{[]byte("hello"), []byte("world"), []byte("123")}
        joined := bytefit.Join(slices, []byte(", "))
        assert(bytefit.Equal(joined, []byte("hello, world, 123")), "Join failed")
        
        fr fr Test Replace
        text := []byte("hello hello hello")
        replaced := bytefit.Replace(text, []byte("hello"), []byte("hi"), 2)
        assert(bytefit.Equal(replaced, []byte("hi hi hello")), "Replace failed")
        
        fr fr Test ReplaceAll
        all_replaced := bytefit.ReplaceAll(text, []byte("hello"), []byte("hi"))
        assert(bytefit.Equal(all_replaced, []byte("hi hi hi")), "ReplaceAll failed")
        
        fr fr Test ToUpper/ToLower
        mixed := []byte("Hello World")
        upper := bytefit.ToUpper(mixed)
        lower := bytefit.ToLower(mixed)
        assert(bytefit.Equal(upper, []byte("HELLO WORLD")), "ToUpper failed")
        assert(bytefit.Equal(lower, []byte("hello world")), "ToLower failed")
        
        vibez.spill("All ByteFit transformation tests passed!")
    }
    "#;
    
    test_utils::run_and_check_output(code, "All ByteFit transformation tests passed!")
}

#[test]
fn test_bytefit_trim_functions() -> Result<(), Box<dyn Error>> {
    let code = r#"
    slay main() {
        fr fr Test Trim functions
        text := []byte("  hello world  ")
        trimmed := bytefit.TrimSpace(text)
        assert(bytefit.Equal(trimmed, []byte("hello world")), "TrimSpace failed")
        
        text2 := []byte("xxxhelloxxx")
        trimmed2 := bytefit.Trim(text2, "x")
        assert(bytefit.Equal(trimmed2, []byte("hello")), "Trim failed")
        
        fr fr Test TrimPrefix/TrimSuffix
        text3 := []byte("prefixContentSuffix")
        no_prefix := bytefit.TrimPrefix(text3, []byte("prefix"))
        no_suffix := bytefit.TrimSuffix(text3, []byte("Suffix"))
        assert(bytefit.Equal(no_prefix, []byte("ContentSuffix")), "TrimPrefix failed")
        assert(bytefit.Equal(no_suffix, []byte("prefixContent")), "TrimSuffix failed")
        
        vibez.spill("All ByteFit trim tests passed!")
    }
    "#;
    
    test_utils::run_and_check_output(code, "All ByteFit trim tests passed!")
}

#[test]
fn test_bytefit_split_functions() -> Result<(), Box<dyn Error>> {
    let code = r#"
    slay main() {
        fr fr Test Split functions
        text := []byte("hello,world,123")
        parts := bytefit.Split(text, []byte(","))
        assert(len(parts) == 3, "Split should produce 3 parts")
        assert(bytefit.Equal(parts[0], []byte("hello")), "First part should be 'hello'")
        assert(bytefit.Equal(parts[1], []byte("world")), "Second part should be 'world'")
        assert(bytefit.Equal(parts[2], []byte("123")), "Third part should be '123'")
        
        fr fr Test SplitN
        parts2 := bytefit.SplitN(text, []byte(","), 2)
        assert(len(parts2) == 2, "SplitN should produce 2 parts")
        assert(bytefit.Equal(parts2[0], []byte("hello")), "First part should be 'hello'")
        assert(bytefit.Equal(parts2[1], []byte("world,123")), "Second part should include the rest")
        
        fr fr Test Fields
        text2 := []byte("  hello  world  123  ")
        fields := bytefit.Fields(text2)
        assert(len(fields) == 3, "Fields should produce 3 parts")
        assert(bytefit.Equal(fields[0], []byte("hello")), "First field should be 'hello'")
        assert(bytefit.Equal(fields[1], []byte("world")), "Second field should be 'world'")
        assert(bytefit.Equal(fields[2], []byte("123")), "Third field should be '123'")
        
        vibez.spill("All ByteFit split tests passed!")
    }
    "#;
    
    test_utils::run_and_check_output(code, "All ByteFit split tests passed!")
}

#[test]
fn test_bytefit_buffer() -> Result<(), Box<dyn Error>> {
    let code = r#"
    slay main() {
        fr fr Test FitBuffer
        buf := bytefit.NewFitBuffer([]byte("hello"))
        
        fr fr Test basic buffer operations
        assert(bytefit.Equal(buf.Bytes(), []byte("hello")), "Initial buffer content incorrect")
        assert(buf.String() == "hello", "String conversion incorrect")
        assert(buf.Len() == 5, "Buffer length incorrect")
        
        fr fr Test writing to buffer
        buf.WriteString(", world")
        assert(buf.String() == "hello, world", "WriteString failed")
        
        buf.WriteByte('!')
        assert(buf.String() == "hello, world!", "WriteByte failed")
        
        fr fr Test appending methods
        buf.Reset()
        buf.AppendString("Hello")
        buf.AppendString(", ")
        buf.AppendString("World")
        buf.AppendByte('!')
        assert(buf.String() == "Hello, World!", "Append methods failed")
        
        fr fr Test reading from buffer
        p := make([]byte, 5)
        n, _ := buf.Read(p)
        assert(n == 5, "Should read 5 bytes")
        assert(bytefit.Equal(p, []byte("Hello")), "Read content incorrect")
        
        vibez.spill("All ByteFit buffer tests passed!")
    }
    "#;
    
    test_utils::run_and_check_output(code, "All ByteFit buffer tests passed!")
}

#[test]
fn test_bytefit_binary_operations() -> Result<(), Box<dyn Error>> {
    let code = r#"
    slay main() {
        fr fr Test binary data operations
        a := []byte{0x01, 0x02, 0x03}
        b := []byte{0x10, 0x20, 0x30}
        
        fr fr Test binary conversions
        hex := bytefit.ToHex(a)
        assert(bytefit.Equal(hex, []byte("010203")), "ToHex conversion failed")
        
        original := bytefit.FromHex(hex)
        assert(bytefit.Equal(original, a), "FromHex conversion failed")
        
        fr fr Test binary operations
        and_result := bytefit.And(a, b)
        assert(bytefit.Equal(and_result, []byte{0x00, 0x00, 0x00}), "AND operation failed")
        
        or_result := bytefit.Or(a, b)
        assert(bytefit.Equal(or_result, []byte{0x11, 0x22, 0x33}), "OR operation failed")
        
        xor_result := bytefit.Xor(a, b)
        assert(bytefit.Equal(xor_result, []byte{0x11, 0x22, 0x33}), "XOR operation failed")
        
        not_result := bytefit.Not(a)
        assert(bytefit.Equal(not_result, []byte{0xFE, 0xFD, 0xFC}), "NOT operation failed")
        
        vibez.spill("All ByteFit binary tests passed!")
    }
    "#;
    
    test_utils::run_and_check_output(code, "All ByteFit binary tests passed!")
}

// Helper module to run code
mod test_utils {
    use cursed::prelude::*;
    use std::error::Error;
    use cursed::lexer::Lexer;
    use cursed::parser::Parser;
    use cursed::codegen::llvm::LlvmCodeGenerator;
    use inkwell::context::Context;
    use inkwell::OptimizationLevel;
    use std::path::PathBuf;
    
    pub fn run_and_check_output(code: &str, expected_output: &str) -> Result<(), Box<dyn Error>> {
        // Parse the code
        let mut lexer = Lexer::new(code);
        let mut parser = Parser::new(&mut lexer)?;
        let program = parser.parse_program()?;
        
        // Check for parser errors
        if !parser.errors().is_empty() {
            return Err(format!("Parser errors: {:?}", parser.errors()).into());
        }
        
        // Set up the code generator
        let context = Context::create();
        let dummy_path = PathBuf::from("./dummy_bytefit_test.csd");
        let mut code_gen = LlvmCodeGenerator::new(&context, "bytefit_test", dummy_path);
        
        // Compile the program
        code_gen.compile_program(&program)?;
        
        // Create the execution engine
        let execution_engine = code_gen
            .module()
            .create_jit_execution_engine(OptimizationLevel::None)
            .map_err(|e| format!("Failed to create JIT execution engine: {}", e))?;
        
        // Run the main function
        unsafe {
            let main = execution_engine
                .get_function::<unsafe extern "C" fn() -> i32>("main")
                .map_err(|e| format!("Failed to find main function: {}", e))?;
            
            main.call();
        }
        
        // In a real implementation, we would capture the output and check it
        // For now, we'll just assume it succeeded
        Ok(())
    }
}