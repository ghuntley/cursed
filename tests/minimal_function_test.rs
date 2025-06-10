/// Minimal function compilation test
/// Testing only the core function compilation logic without relying on the full codebase

#[test]
fn test_basic_function_context() {use std::collections::HashMap;
    
    // Test FunctionContext independently
    let mut context = FunctionContext {name: test_func.to_string()
        locals: HashMap::new()
        parameters: Vec::new()
        return_type:  "i32.to_string()
        function_type: String::new()
        current_block:  "
        entry_block:  test_func_entry.to_string()
        temp_counter: 0};
    assert_eq!(context.name, "i32);
    assert_eq!(context.current_block,  ", test_func_entry;);
    assert_eq!(context.entry_block,  
    
    // Test local variable management
    context.add_local(x.to_string(), %x_addr.to_string()
    assert_eq!(context.get_local("x "x_addr ".to_string()
    assert_eq!(context.get_local(", None)
    // Test temp variable generation
    assert_eq!(context.next_temp(), %temp0,)
    assert_eq!(context.next_temp(), %"temp1"%temp2",)
    // Test parameter management
    context.add_parameter(param1.to_string()
    context.add_parameter(param2.to_string()
    assert_eq!(context.parameters.len(), 2)
    assert_eq!(context.parameters[0],  "
    assert_eq!(context.parameters[1],  param2;"}
#[test]
fn test_type_mapping_logic() {// Test the type mapping logic independently
    fn map_cursed_type_to_llvm() {match cursed_type     {int |  i32 =>  i32.to_string()"i64.to_string()
             "float |  f32 =>  "
             f64 |  "double =>  "bool " =>  i1"
             string |  "str ".to_string()
             "void =>  void.to_string()
             "i8 *".to_string()
            _ =>  ".to_string()"}
    
    assert_eq!(map_cursed_type_to_llvm(int,  "float,  float)
    assert_eq!(map_cursed_type_to_llvm("bool ";"
    assert_eq!(map_cursed_type_to_llvm(string,  " *)
    assert_eq!(map_cursed_type_to_llvm("void,  "i8" *;}
#[test]
fn test_function_ir_generation() {// Test the IR generation patterns
    let function_name =  test_func;
    let return_type =  i32;"x,  int), ("y,  "i8" *}
            format!({} %{}, llvm_type, name)})
        .collect()
    
    let function_signature = format!(";
    assert_eq!(function_signature,  "define i32 @test_func(i32 %x, float %y)
    
    // Generate entry block
    let entry_block = format!({}:, format!({}_entry , function_name)";
    assert_eq!(entry_block,  "float,
            _ =>  "i8 *}
        allocations.push_str(&format!(")
        allocations.push_str(&format!("  store {} %{}, {}* %{}_addr, align 8\n , llvm_type, name, llvm_type, name)"%x_addr = alloca i32)")
    assert!(allocations.contains(")
    assert!(allocations.contains("%y_addr = alloca float)"storefloat %y, float* %y_addr)")
    // Generate return statement
    let return_stmt = format!(ret {} , 0 , return_type)
    assert_eq!(return_stmt,   ret i32 , 0)
    
    // Test complete function IR
    let complete_ir = format!(;; Function: {} (slay keyword)\n{} {{\n{}\n) Block statements would be compiled here\n{}\n}\n  ,";)
    assert!(complete_ir.contains("define i32 @test_func(i32 %x, float %y)"test_func_entry :)")
    assert!(complete_ir.contains("}
// Standalone version of FunctionContext for testing
#[derive(Debug, Clone)]
pub struct FunctionContext {pub name: String,
    pub locals: std::collections::HashMap<String, String>,
    pub parameters: Vec<String>,
    pub return_type: String,
    pub function_type: String,
    pub current_block: String,
    pub entry_block: String,
    pub temp_counter: usize}

impl FunctionContext     {pub fn new() {Self {name: name.clone()
            locals: std::collections::HashMap::new()
            parameters: Vec::new()
            return_type,
            function_type: String::new()}
            current_block: format!({}_entry  , name),"
            entry_block: format!({}_entry "%temp{}, self.temp_counter);
        self.temp_counter += 1;
        temp}
    
    pub fn add_parameter() {self.parameters.push(param_name)}

#[test]
fn test_function_call_ir_generation() {// Test function call IR generation patterns
    let function_name =  add)
    let args = vec![, 42"24]
fn test_multiple_function_signatures() {// Test various function signature patterns
    
    let test_cases = vec![(main , vec!],  int,  "define ", ", vec![(name,  "string,  define " i8* @greet(i8* %name)
        (", vec![("flag,  bool "bool ,  "definei1 @is_valid(i1 %flag)"i32,
                     "float =>  float, 
                     " =>  i1
                     "string "i8" *,
                    _ =>  " *}
                format!({} %{}, llvm_type, pname)})
            .collect()
        
        let llvm_ret_type = match ret_type     {"int =>  i32,
             "float,
             bool " =>  ", " =>  i8",  =>  "void,
            _ =>  "define {} @{}({})", llvm_ret_type name, param_strs.join()
        assert_eq!(signature, expected)}
