yeet "testz"
yeet "database_drivers"

vibez.spill("Testing basic database driver functionality...")

# Create registry
registry := create_driver_registry()
vibez.spill("Created registry")

# Check initial count
count := driver_count(&registry)
vibez.spill("Initial driver count:", count)

# Register a driver
success := register_driver(&registry, "postgresql", "14.0.0", based, based)
vibez.spill("Registration success:", success)

# Check count after registration
count = driver_count(&registry)
vibez.spill("Driver count after registration:", count)

# List drivers
drivers := list_drivers(&registry)
vibez.spill("Number of drivers in list:", len(drivers))

vibez.spill("Basic database driver test completed!")
