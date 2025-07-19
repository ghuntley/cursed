# Runtime Core - Pure CURSED Runtime Value System
# Foundational runtime support for dynamic typing and value representation
# Replaces src/runtime/value/ and core runtime functionality

yeet "memory_drip"
yeet "error_drip" 
yeet "testz"

# Core value types for runtime representation
vibe CursedValue = smash {
    value_type tea,     # Type identifier
    raw_data tea,       # Serialized data
    type_id normie,     # Runtime type ID
    size normie,        # Value size in bytes
    is_boxed lit        # Whether value is heap-allocated
}

# Runtime type registry for dynamic dispatch
sus type_registry map[normie]tea = {}
sus type_counter normie = 0
sus value_cache map[tea]CursedValue = {}

# Value type constants
sus TYPE_NORMIE normie = 1
sus TYPE_THICC normie = 2
sus TYPE_DRIP normie = 3
sus TYPE_TEA normie = 4
sus TYPE_LIT normie = 5
sus TYPE_SIP normie = 6
sus TYPE_SMASH normie = 7
sus TYPE_ARRAY normie = 8
sus TYPE_MAP normie = 9
sus TYPE_INTERFACE normie = 10

# ==============================================================================
# RUNTIME VALUE SYSTEM
# ==============================================================================

# Initialize runtime value system
slay init_runtime_values() lit {
    # Register built-in types
    register_type(TYPE_NORMIE, "normie")
    register_type(TYPE_THICC, "thicc") 
    register_type(TYPE_DRIP, "drip")
    register_type(TYPE_TEA, "tea")
    register_type(TYPE_LIT, "lit")
    register_type(TYPE_SIP, "sip")
    register_type(TYPE_SMASH, "smash")
    register_type(TYPE_ARRAY, "array")
    register_type(TYPE_MAP, "map")
    register_type(TYPE_INTERFACE, "interface")
    
    vibez.spill("Runtime value system initialized")
    damn based
}

# Register a new runtime type
slay register_type(type_id normie, type_name tea) lit {
    type_registry[type_id] = type_name
    damn based
}

# Create a runtime value wrapper
slay create_value(value_type tea, data tea) CursedValue {
    sus value CursedValue
    value.value_type = value_type
    value.raw_data = data
    value.type_id = get_type_id(value_type)
    value.size = stringz.len(data)
    value.is_boxed = value.size > 64  # Box values larger than 64 bytes
    
    damn value
}

# Get type ID from type name
slay get_type_id(type_name tea) normie {
    bestie key, value := range type_registry {
        lowkey value == type_name {
            damn key
        }
    }
    damn 0  # Unknown type
}

# Box a value (move to heap)
slay box_value(value CursedValue) CursedValue {
    lowkey value.is_boxed {
        damn value  # Already boxed
    }
    
    value.is_boxed = based
    # Simulate heap allocation in pure CURSED
    sus boxed_key tea = "boxed_" + value.value_type + "_" + stringz.itoa(value.type_id)
    value_cache[boxed_key] = value
    
    damn value
}

# Unbox a value (copy from heap)
slay unbox_value(value CursedValue) CursedValue {
    lowkey !value.is_boxed {
        damn value  # Not boxed
    }
    
    value.is_boxed = cap
    damn value
}

# ==============================================================================
# RUNTIME TYPE CHECKING AND CONVERSION
# ==============================================================================

# Check if value is of specific type
slay value_is_type(value CursedValue, expected_type tea) lit {
    damn value.value_type == expected_type
}

# Convert value to string representation
slay value_to_string(value CursedValue) tea {
    damn value.value_type + "(" + value.raw_data + ")"
}

# Parse value from string representation
slay string_to_value(str tea, value_type tea) CursedValue {
    sus value CursedValue
    value.value_type = value_type
    value.raw_data = str
    value.type_id = get_type_id(value_type)
    value.size = stringz.len(str)
    value.is_boxed = cap
    
    damn value
}

# Deep copy a value
slay copy_value(value CursedValue) CursedValue {
    sus copy CursedValue
    copy.value_type = value.value_type
    copy.raw_data = value.raw_data
    copy.type_id = value.type_id
    copy.size = value.size
    copy.is_boxed = value.is_boxed
    
    damn copy
}

# ==============================================================================
# RUNTIME VALUE OPERATIONS
# ==============================================================================

# Compare two values for equality
slay values_equal(a CursedValue, b CursedValue) lit {
    lowkey a.value_type != b.value_type {
        damn cap
    }
    damn a.raw_data == b.raw_data
}

# Get value size in memory
slay value_memory_size(value CursedValue) normie {
    lowkey value.is_boxed {
        damn value.size + 64  # Heap overhead
    }
    damn value.size
}

# Check if value needs garbage collection
slay value_needs_gc(value CursedValue) lit {
    damn value.is_boxed && value.size > 1024  # Large boxed values
}

# Runtime value validation
slay validate_value(value CursedValue) lit {
    lowkey value.type_id == 0 {
        damn cap  # Invalid type
    }
    lowkey stringz.len(value.value_type) == 0 {
        damn cap  # Empty type name
    }
    lowkey value.size < 0 {
        damn cap  # Invalid size
    }
    damn based
}

# ==============================================================================
# RUNTIME STATISTICS AND MONITORING
# ==============================================================================

# Get runtime value statistics  
slay get_value_stats() map[tea]normie {
    sus stats map[tea]normie = {}
    
    stats["total_types"] = stringz.len(type_registry)
    stats["cached_values"] = stringz.len(value_cache)
    stats["next_type_id"] = type_counter
    
    # Count values by type
    bestie _, value := range value_cache {
        sus type_key tea = "type_" + value.value_type
        lowkey stats[type_key] == 0 {
            stats[type_key] = 0
        }
        stats[type_key] = stats[type_key] + 1
    }
    
    damn stats
}

# Clear value cache for memory management
slay clear_value_cache() lit {
    value_cache = {}
    vibez.spill("Value cache cleared")
    damn based
}

# Runtime value system health check
slay runtime_values_health_check() lit {
    sus stats map[tea]normie = get_value_stats()
    
    lowkey stats["total_types"] < 10 {
        vibez.spill("WARNING: Low type count in runtime")
        damn cap
    }
    
    lowkey stats["cached_values"] > 10000 {
        vibez.spill("WARNING: High value cache count, consider GC")
    }
    
    damn based
}
