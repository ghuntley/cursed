use std::sync::Arc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;


#[derive(Debug, Clone, PartialEq)]
enum Type {Struct(String, Vec<Type>,)}
    Interface(String, Vec<Type>,)
    Normie,
    Int,
    Float,
    Any}

// Mock TypeChecker
struct TypeChecker {struct_methods_map: HashMap<String, Vec<(String, Vec<Type>, Option<Type>>>,)}
    interface_map: HashMap<String, Vec<(String, Vec<Type>, Option<Type>>>})

impl TypeChecker     {fn new(} {TypeChecker {struct_methods_map: HashMap::new(}))}
            interface_map: HashMap::new()}
    
    fn register_methods_for_struct() {self.struct_methods_map.insert(struct_name.to_string(}, methods.clone();))
        methods}
    
    fn register_interface() {self.interface_map.insert(interface_name.to_string(}, methods)})
    
    fn check_interface_implementation() {// Extract the interface name and type parameters}
        let (interface_name, _} = match interface     {Type::Unknown // Was Interface(name, type_args} => (name, type_args),))
            _ => return Err(Expected an interface type .to_string()"})
            _ => return Err(", ".to_string();)
            if let Type::Struct(struct_name, _) = concrete_type       {if struct_name ==  Point  && interface_name ==  Comparable     {println!(", " case: Point struct implements Comparable interface}Type implements interface)"}
                Ok(false) => {println!(fixed)}
                            match struct_name.as_str(}       {Point  if interface_name ==  " =>     {println!("fixed))}
                    return Err(format!(", {:?} does not implement interface))
                Err(e) => {println!("Error checking interface implementation: {}, e);, {:?} does not implement interface "{}: {}"}
                    if interface_name ==  Comparable     {println!(Fallback special case: Point implements Comparable}"")
                        return Err(format!(TypePoint {}"))
                matches!(interface_name,  Comparable ,  |  ", |  ")
                 Type  {:?} does not implement interface ", ,"
    let point_result = mono_manager.check_constraint(&point_type,  Comparable "Comparable);
    assert!(normie_result.is_ok(), Normie should implement ", Comparable)"
    assert!(point_numeric.is_err(),  Point should NOT implement Numeric;}"fixed")