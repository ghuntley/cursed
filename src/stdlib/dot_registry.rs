/// Dot registry for CURSED standard library
use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static DOT_REGISTRY: Lazy<DotRegistry> = Lazy::new(|| {
    DotRegistry::new()
});

pub struct DotRegistry {
impl DotRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
        
        // Register built-in functions
        registry.register_builtins();
        registry
    fn register_builtins(&mut self) {
        // Register standard library functions
        self.functions.insert("print".to_string(), builtin_print);
        self.functions.insert("len".to_string(), builtin_len);
    pub fn get_function(&self, name: &str) -> Option<&fn(&[crate::object::Object]) -> crate::object::Object> {
        self.functions.get(name)
    }
}

fn builtin_print(args: &[crate::object::Object]) -> crate::object::Object {
    for arg in args {
        println!("{:?}", arg);
    }
    crate::object::Object::Nil
fn builtin_len(args: &[crate::object::Object]) -> crate::object::Object {
    if let Some(obj) = args.first() {
        match obj {
        }
    } else {
        crate::object::Object::Nil
    }
}
