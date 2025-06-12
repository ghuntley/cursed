use std::collections::HashMap;

fn main() {
    // Test the template format renderers work correctly
    println!("Testing template format renderers...");
    
    // Create a simple test to verify compilation and basic functionality
    let obj = cursed::core::CursedObject::Map({
        let mut map = HashMap::new();
        map.insert("license_type".to_string(), cursed::core::CursedObject::String("MIT".to_string()));
        map.insert("year".to_string(), cursed::core::CursedObject::String("2024".to_string()));
        map.insert("holder".to_string(), cursed::core::CursedObject::String("Test Author".to_string()));
        map
    });
    
    let formatter = cursed::stdlib::template::template_formats::TemplateFormatRenderer::new();
    
    match formatter.render_license(&obj) {
        Ok(license_text) => {
            println!("✅ License renderer works!");
            println!("Generated license contains MIT: {}", license_text.contains("MIT"));
            println!("Generated license contains 2024: {}", license_text.contains("2024"));
            println!("Generated license contains Test Author: {}", license_text.contains("Test Author"));
        }
        Err(e) => {
            println!("❌ License renderer failed: {}", e);
        }
    }
    
    println!("✅ Template format system compilation test passed!");
}
