//! Simple test to verify HTTP client compilation

use cursed::stdlib::glowup_http::client::VibeClient;
use cursed::stdlib::glowup_http::error::GlowUpResult;

fn main() -> GlowUpResult<()> {
    println!("Testing HTTP client creation...");
    
    // Test basic client creation
    let client = VibeClient::new()?;
    println!("✅ Client created successfully");
    
    // Test builder pattern
    let _builder_client = VibeClient::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("Test-Client/1.0")
        .gzip(true)
        .build()?;
    println!("✅ Builder pattern works");
    
    // Test request builder creation (without sending)
    let _request_builder = client.get("https://example.com")
        .header("X-Test", "value")
        .timeout(std::time::Duration::from_secs(5));
    println!("✅ Request builder works");
    
    println!("✅ All HTTP client tests passed!");
    Ok(())
}
