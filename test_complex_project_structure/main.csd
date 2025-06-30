// Main entry point for complex multi-file CURSED program testing
import "utils/math_utils.csd" as MathUtils
import "data/models.csd" as Models  
import "services/network_service.csd" as Network
import "config/settings.csd" as Config
import crypto_pqc
import json_parser from stdlib

func main() {
    println("🚀 Starting Complex Project Structure Test")
    
    // Test configuration loading
    let config = Config.load_settings()
    println("Loaded config: " + config.app_name)
    
    // Test mathematical utilities
    let result = MathUtils.fibonacci(10)
    println("Fibonacci(10) = " + result.to_string())
    
    // Test data models
    let user = Models.User.new("Alice", 25)
    println("Created user: " + user.name)
    
    // Test network services
    let response = await Network.fetch_data("https://api.example.com/users")
    println("Network response length: " + response.len().to_string())
    
    // Test package imports
    let signature = crypto_pqc.generate_keypair()
    println("Generated PQC signature")
    
    // Test stdlib imports  
    let data = json_parser.parse("{\"test\": true}")
    println("Parsed JSON: " + data.to_string())
    
    println("✅ All imports and modules working correctly!")
}
