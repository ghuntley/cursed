/// Dot registry for CURSED standard library
use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static DOT_REGISTRY: Lazy<DotRegistry> = Lazy::new(|| {
    DotRegistry::new()
});

pub struct DotRegistry {
    functions: HashMap<String, fn(&[crate::object::Object]) -> crate::object::Object>,
}

impl DotRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
        };
        
        // Register built-in functions
        registry.register_builtins();
        registry
    }
    
    fn register_builtins(&mut self) {
        // Register standard library functions
        self.functions.insert("print".to_string(), builtin_print);
        self.functions.insert("len".to_string(), builtin_len);
    }
    
    pub fn get_function(&self, name: &str) -> Option<&fn(&[crate::object::Object]) -> crate::object::Object> {
        self.functions.get(name)
    }
}

fn builtin_print(args: &[crate::object::Object]) -> crate::object::Object {
    for arg in args {
        println!("{:?}", arg);
    }
    crate::object::Object::Nil
}

fn builtin_len(args: &[crate::object::Object]) -> crate::object::Object {
    if let Some(obj) = args.first() {
        match obj {
            crate::object::Object::String(s) => crate::object::Object::Integer(s.len() as i64),
            crate::object::Object::Array(arr) => crate::object::Object::Integer(arr.len() as i64),
            _ => crate::object::Object::Nil,
        }
    } else {
        crate::object::Object::Nil
    }
}
