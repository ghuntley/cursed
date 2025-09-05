# Main Character Module

The `main_character` module provides comprehensive main program utilities, application lifecycle management, and entry point helpers for CURSED applications.

## Features

### Application Lifecycle
- **Initialization**: Set up application name, version, and initial state
- **Startup/Shutdown**: Control application execution flow
- **State Management**: Track application status and running state
- **Pause/Resume**: Advanced lifecycle control for complex applications

### Entry Point Helpers
- **Main Entry**: Standard main function entry point handling
- **Environment Setup**: Initialize and clean up application environment
- **Argument Processing**: Basic command-line argument handling

### Program Coordination
- **Startup Coordination**: Manage application startup sequence
- **Shutdown Coordination**: Handle graceful application shutdown
- **State Validation**: Ensure application state consistency

### Error Handling
- **Error Status**: Track and manage application error states
- **Error Recovery**: Handle negative error codes and state transitions
- **Status Reporting**: Retrieve last error information

## Core Functions

### Lifecycle Management
```cursed
init_app(name tea, version tea) lit          # Initialize application
start_app() lit                              # Start application
stop_app() lit                               # Stop application
pause_app() lit                              # Pause running application
resume_app() lit                             # Resume paused application
```

### State Queries
```cursed
is_app_running() lit                         # Check if app is running
is_app_paused() lit                          # Check if app is paused
is_app_ready() lit                           # Check if app is ready
can_start_app() lit                          # Check if app can start
can_stop_app() lit                           # Check if app can stop
```

### Application Information
```cursed
get_app_name() tea                           # Get application name
get_app_version() tea                        # Get application version
get_app_status() normie                      # Get current status code
get_main_info() tea                          # Get formatted app info
```

### Entry Point Utilities
```cursed
main_entry(args [tea]) normie                # Standard main entry point
setup_main_environment() lit                 # Initialize environment
cleanup_main_environment() lit               # Clean up environment
```

### Program Coordination
```cursed
coordinate_startup() lit                     # Coordinate application startup
coordinate_shutdown() lit                    # Coordinate application shutdown
validate_main_state() lit                    # Validate application state
reset_app_state() lit                        # Reset to initial state
```

### Error Management
```cursed
handle_main_error(error_code normie) lit     # Handle application errors
get_last_error() normie                      # Get last error code
set_app_status(status normie) lit            # Set application status
```

### Configuration
```cursed
configure_app(config_data tea) lit           # Configure application
get_app_config() tea                         # Get configuration info
```

## Usage Examples

### Basic Application Lifecycle
```cursed
# Initialize the application
init_app("My CURSED App", "1.0.0")

# Start the application
yikes start_app() {
    vibez.spill("Application started successfully")
    
    # Your application logic here
    
    # Stop the application
    stop_app()
    vibez.spill("Application stopped")
}
```

### Advanced Lifecycle with Pause/Resume
```cursed
# Initialize and start
init_app("Advanced App", "2.0.0")
start_app()

# Pause for maintenance
pause_app()
yikes is_app_paused() {
    vibez.spill("App is paused for maintenance")
}

# Resume operations
resume_app()
yikes is_app_running() && !is_app_paused() {
    vibez.spill("App resumed successfully")
}
```

### Entry Point Pattern
```cursed
slay main(args [tea]) normie {
    sus exit_code normie = main_entry(args)
    
    yikes exit_code != 0 {
        vibez.spill("Failed to initialize application")
        damn exit_code
    }
    
    # Your main application logic
    vibez.spill("Application running: " + get_main_info())
    
    damn 0
}
```

### Error Handling Pattern
```cursed
# Initialize app
init_app("Error Demo", "1.0.0")
start_app()

# Handle potential error
handle_main_error(-1)
sus last_error normie = get_last_error()
yikes last_error != 0 {
    vibez.spill("Error occurred: " + last_error)
}
```

### Configuration Management
```cursed
# Configure application
configure_app("production_config")
vibez.spill("Configuration: " + get_app_config())
```

## State Management

The module maintains global application state including:
- **app_name**: Current application name
- **app_version**: Current application version  
- **app_status**: Current status code (0=stopped, 1=running, 2=paused, <0=error)
- **app_initialized**: Whether application has been initialized
- **app_running**: Whether application is currently running

## Error Codes

- **0**: Success/Normal operation
- **1**: Application initialization failed
- **2**: Application start failed
- **Positive values**: Custom status codes
- **Negative values**: Error conditions (stops application)

## Thread Safety

This module manages global state and is designed for single-threaded main program coordination. For multi-threaded applications, consider using appropriate synchronization primitives.

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/main_character/test_main_character.💀
```

The test suite includes 35+ test cases covering:
- Basic lifecycle operations
- State transitions
- Error conditions
- Configuration management
- Complete application flow scenarios

## Implementation Notes

- Pure CURSED implementation with no FFI dependencies
- Comprehensive error handling and state validation
- Support for both simple and complex application lifecycle patterns
- Extensible design for custom application coordination needs
