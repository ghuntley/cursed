# GDB debugging script for CURSED program
# Generated for: test_simple.💀

file debug/test_simple
set print pretty on
set print array on
set print array-indexes on
# CURSED-specific debugging setup
set language c

# Additional CURSED-specific debugging commands
define cursed-info
  info functions
  info variables
  info sources
end
