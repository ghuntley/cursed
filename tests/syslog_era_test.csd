vibe syslog_era_test

import "vibez"
import "syslog_era"

// Test the syslog_era package
slay main() {
    vibez.spill("Testing syslog_era package")
    
    // Create a new syslog writer with specified facility
    writer := syslog_era.new("syslog_test_app", syslog_era.LOG_USER)
    
    // Test sending messages with different priority levels
    syslog_era.debug(writer, "This is a debug message")
    syslog_era.info(writer, "This is an info message")
    syslog_era.warning(writer, "This is a warning message")
    syslog_era.error(writer, "This is an error message")
    syslog_era.crit(writer, "This is a critical message")
    syslog_era.alert(writer, "This is an alert message")
    syslog_era.emerg(writer, "This is an emergency message")
    
    // Test log function with custom priority
    syslog_era.log(writer, syslog_era.LOG_INFO, "Custom log message")
    
    // Test sending formatted message
    syslog_era.log_formatted(writer, syslog_era.LOG_WARNING, "Warning: %s at %d", "Disk space low", 85)
    
    // Test operations with local syslog
    local := syslog_era.new_local("local_test_app", syslog_era.LOG_LOCAL0)
    syslog_era.info(local, "Message to local syslog")
    
    // Test getting facility name and priority name
    facility_name := syslog_era.get_facility_name(syslog_era.LOG_LOCAL7)
    priority_name := syslog_era.get_priority_name(syslog_era.LOG_CRIT)
    vibez.spill("Facility LOCAL7 name:", facility_name)
    vibez.spill("Priority CRIT name:", priority_name)
    
    // Test closing the writer
    closed := syslog_era.close(writer)
    vibez.spill("Writer closed:", closed)
    
    vibez.spill("Test completed")
}