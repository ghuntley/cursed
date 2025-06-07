use crate::error::Error;
use crate::codegen::llvm::interface_registry::InterfaceTypeRegistry;

/// Interface registry helpers for type name lookups
pub trait TypeNameRegistry {
    /// Get a type name by its ID
    fn get_type_name_by_id(&self, type_id: u64) -> Option<String>;
    
    /// Get a type name from the registry
    fn get_type_name_from_registry(&self, type_id: u64) -> Option<String>;
}

/// Default implementation for LlvmCodeGenerator
impl<'ctx> TypeNameRegistry for crate::codegen::llvm::LlvmCodeGenerator<'ctx> {
    fn get_type_name_by_id(&self, type_id: u64) -> Option<String> {
        // First try to get from the registry
        self.get_type_name_from_registry(type_id)
            .or_else(|| {
                // Fall back to internal fields
                let key = format!("type_name_{}", type_id);
                self.internal_fields.get(&key)
                    .and_then(|boxed| boxed.downcast_ref::<String>())
                    .map(|s| s.clone())
            })
            .or_else(|| Some(format!("Type#{}", type_id)))
    }
    
    fn get_type_name_from_registry(&self, type_id: u64) -> Option<String> {
        if let Some(registry) = InterfaceRegistryAccess::get_interface_registry(self) {
            if let Ok(name) = registry.get_type_name(type_id) {
                return Some(name);
            }
        }
        None
    }
}

/// Extension trait to add interface registry access
pub trait InterfaceRegistryAccess<'ctx> {
    fn get_interface_registry(&self) -> Option<&dyn InterfaceTypeRegistry>;
}

impl<'ctx> InterfaceRegistryAccess<'ctx> for crate::codegen::llvm::LlvmCodeGenerator<'ctx> {
    fn get_interface_registry(&self) -> Option<&dyn InterfaceTypeRegistry> {
        self.interface_type_registry.as_ref().map(|r| r as &dyn InterfaceTypeRegistry)
    }
}