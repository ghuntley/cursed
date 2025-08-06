# GDB Script for CURSED Debug Information
# Generated automatically from debug metadata

# Set breakpoints on all functions
break factorial:1

# Display all variables when stopped
define show_cursed_vars
  info locals result
  info locals i
end

# Start debugging
run
