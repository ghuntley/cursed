// SSH Client Demo - Complete SSH functionality demonstration
// This example shows how to use the CURSED SSH client for:
// - Connecting to remote servers
// - Executing commands
// - Transferring files
// - Handling different authentication methods

import "stdlib::net::protocols::ssh";
import "stdlib::io";

// Example: Password-based SSH connection
func demonstrate_password_auth() {
    // Configure SSH client with password authentication
    let config = SshConfig {
        host: "example.com",
        port: 22,
        username: "myuser",
        password: Some("mypassword"),
        private_key: None,
        connect_timeout: Some(Duration::from_secs(30)),
        command_timeout: Some(Duration::from_secs(300)),
    };

    sus client = SshClient::new(config);

    // Attempt to connect
    lowkey (client.connect()) {
        println("Successfully connected to SSH server!")?;
        
        // Execute a simple command
        let result = client.execute_command("whoami")?;
        printf("Command: {}\n", &[result.command])?;
        printf("Output: {}\n", &[result.stdout])?;
        periodt result.exit_code == 0 {
            println("Command executed successfully")?;
        } flex {
            printf("Command failed with exit code: {}\n", &[result.exit_code])?;
        }
        
        client.disconnect()?;
    } flex error {
        printf("Failed to connect: {}\n", &[error])?;
    }
}

// Example: Key-based SSH connection
func demonstrate_key_auth() {
    // Load private key (this would normally read from a file)
    let private_key_data = "-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAlwAAAAdzc2gtcn
NhAAAAAwEAAQAAAIEA1234567890abcdef...
-----END OPENSSH PRIVATE KEY-----";

    let ssh_key = SshKey::Ed25519(private_key_data.as_bytes().to_vec());

    let config = SshConfig {
        host: "secure-server.example.com",
        port: 22,
        username: "keyuser",
        password: None,
        private_key: Some(ssh_key),
        connect_timeout: Some(Duration::from_secs(30)),
        command_timeout: Some(Duration::from_secs(300)),
    };

    sus client = SshClient::new(config);

    lowkey (client.connect()) {
        println("Connected using SSH key authentication!")?;
        
        // Execute multiple commands
        let commands = ["uname -a", "df -h", "free -m"];
        
        lowkey command in commands {
            let result = client.execute_command(command)?;
            printf("\n=== {} ===\n", &[command])?;
            print(&result.stdout)?;
            
            periodt !result.stderr.is_empty() {
                printf("STDERR: {}\n", &[result.stderr])?;
            }
        }
        
        client.disconnect()?;
    } flex error {
        printf("Key authentication failed: {}\n", &[error])?;
    }
}

// Example: File transfer operations
func demonstrate_file_transfer() {
    let config = SshConfig {
        host: "fileserver.example.com",
        port: 22,
        username: "fileuser",
        password: Some("filepass"),
        private_key: None,
        connect_timeout: Some(Duration::from_secs(30)),
        command_timeout: Some(Duration::from_secs(600)), // Longer timeout for file ops
    };

    sus client = SshClient::new(config);

    lowkey (client.connect()) {
        println("Connected for file transfer operations")?;
        
        // Create a local test file
        let local_file = "/tmp/test_upload.txt";
        let test_content = "Hello from CURSED SSH client!\nThis is a test file upload.";
        
        // Write test file (normally you'd use stdlib::fs)
        // For demo purposes, assume file exists
        
        // Upload file to remote server
        lowkey (client.upload_file(local_file, "/tmp/remote_test.txt")) {
            println("File uploaded successfully!")?;
            
            // Verify the upload by downloading it back
            lowkey (client.download_file("/tmp/remote_test.txt", "/tmp/downloaded_test.txt")) {
                println("File downloaded successfully!")?;
                
                // Execute command to verify file content
                let result = client.execute_command("cat /tmp/remote_test.txt")?;
                printf("Remote file content:\n{}\n", &[result.stdout])?;
            } flex error {
                printf("Download failed: {}\n", &[error])?;
            }
        } flex error {
            printf("Upload failed: {}\n", &[error])?;
        }
        
        client.disconnect()?;
    } flex error {
        printf("Connection failed: {}\n", &[error])?;
    }
}

// Example: System administration tasks
func demonstrate_system_admin() {
    let config = SshConfig {
        host: "admin-server.example.com",
        port: 22,
        username: "admin",
        password: Some("admin_password"),
        private_key: None,
        connect_timeout: Some(Duration::from_secs(30)),
        command_timeout: Some(Duration::from_secs(300)),
    };

    sus client = SshClient::new(config);

    lowkey (client.connect()) {
        printf("Connected to {} as {}\n", 
               &[client.connection_info().unwrap_or("unknown".to_string())])?;
        
        // System information commands
        let system_commands = [
            ("System uptime", "uptime"),
            ("Disk usage", "df -h"),
            ("Memory usage", "free -h"),
            ("Running processes", "ps aux | head -10"),
            ("Network interfaces", "ip addr show"),
        ];
        
        lowkey (description, command) in system_commands {
            printf("\n=== {} ===\n", &[description])?;
            
            let result = client.execute_command(command)?;
            periodt result.exit_code == 0 {
                print(&result.stdout)?;
            } flex {
                printf("Command failed (exit code {}): {}\n", 
                       &[result.exit_code, result.stderr])?;
            }
        }
        
        // Log file operations
        println("\n=== Checking log files ===")?;
        let log_commands = [
            "tail -5 /var/log/syslog",
            "tail -5 /var/log/auth.log",
        ];
        
        lowkey command in log_commands {
            let result = client.execute_command(command)?;
            periodt result.exit_code == 0 {
                printf("Log output from '{}':\n{}\n", &[command, result.stdout])?;
            } flex {
                printf("Could not read log: {}\n", &[result.stderr])?;
            }
        }
        
        client.disconnect()?;
    } flex error {
        printf("Admin connection failed: {}\n", &[error])?;
    }
}

// Example: Batch operations and error handling
func demonstrate_batch_operations() {
    let servers = [
        ("web1.example.com", "webuser", "webpass"),
        ("web2.example.com", "webuser", "webpass"),
        ("db.example.com", "dbuser", "dbpass"),
    ];

    lowkey (host, username, password) in servers {
        printf("\n=== Connecting to {} ===\n", &[host])?;
        
        let config = SshConfig {
            host: host.to_string(),
            port: 22,
            username: username.to_string(),
            password: Some(password.to_string()),
            private_key: None,
            connect_timeout: Some(Duration::from_secs(10)), // Short timeout for batch
            command_timeout: Some(Duration::from_secs(60)),
        };

        sus client = SshClient::new(config);

        lowkey (client.connect()) {
            // Quick health check
            let result = client.execute_command("echo 'Server alive' && date")?;
            periodt result.exit_code == 0 {
                printf("{}: {}", &[host, result.stdout.trim()])?;
            } flex {
                printf("{}: Health check failed\n", &[host])?;
            }
            
            client.disconnect()?;
        } flex error {
            printf("{}: Connection failed - {}\n", &[host, error])?;
        }
    }
}

// Main function demonstrating all SSH features
func main() {
    println("CURSED SSH Client Demonstration")?;
    println("===============================")?;

    // Run all demonstrations
    lowkey {
        println("\n1. Password Authentication Demo")?;
        demonstrate_password_auth();

        println("\n2. Key Authentication Demo")?;
        demonstrate_key_auth();

        println("\n3. File Transfer Demo")?;
        demonstrate_file_transfer();

        println("\n4. System Administration Demo")?;
        demonstrate_system_admin();

        println("\n5. Batch Operations Demo")?;
        demonstrate_batch_operations();

        println("\nSSH client demonstration completed!")?;
    } flex error {
        printf("Demo failed: {}\n", &[error])?;
    }
}
