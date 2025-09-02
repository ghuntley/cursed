yeet "core"

fr fr Real Plugin Loading System - CURSED Integration
fr fr Replaces the simulation in mod.csd with actual dynamic library loading
fr fr Bridges to the Zig implementation for cross-platform plugin support

fr fr Plugin status constants
facts PLUG_STATUS_UNLOADED normie = 0
facts PLUG_STATUS_LOADED normie = 1
facts PLUG_STATUS_ERROR normie = 2
facts PLUG_STATUS_SANDBOXED normie = 3
facts PLUG_STATUS_INITIALIZING normie = 4
facts PLUG_STATUS_UNLOADING normie = 5

fr fr Plugin capability flags
facts CAPABILITY_MATH normie = 1
facts CAPABILITY_STRING normie = 2
facts CAPABILITY_IO normie = 4
facts CAPABILITY_NETWORK normie = 8
facts CAPABILITY_GRAPHICS normie = 16
facts CAPABILITY_AUDIO normie = 32
facts CAPABILITY_DATABASE normie = 64
facts CAPABILITY_CRYPTO normie = 128
facts CAPABILITY_THREADING normie = 256
facts CAPABILITY_FILESYSTEM normie = 512

fr fr Security levels
facts SECURITY_TRUSTED normie = 0
facts SECURITY_SANDBOXED normie = 1
facts SECURITY_RESTRICTED normie = 2
facts SECURITY_UNTRUSTED normie = 3

fr fr Plugin metadata structure
be_like PluginMetadata = struct {
    name tea
    version tea
    author tea
    description tea
    api_version normie
    capabilities normie
    entry_point tea
    security_level normie
}

fr fr Plugin handle
be_like Plug = normie fr fr Plugin ID

fr fr Global plugin manager instance
sus plugin_manager_handle vibes = null

fr fr Initialize the real plugin system
slay initialize_plugin_system() lit {
    plugin_manager_handle = cursed_plugin_manager_init()
    lowkey plugin_manager_handle == null {
        damn cap
    }
    damn based
}

fr fr Cleanup plugin system
slay cleanup_plugin_system() {
    lowkey plugin_manager_handle != null {
        cursed_plugin_manager_deinit(plugin_manager_handle)
        plugin_manager_handle = null
    }
}

fr fr Real plugin discovery with filesystem scanning
slay discover_plugins(directory tea) normie {
    lowkey plugin_manager_handle == null {
        lowkey !initialize_plugin_system() {
            damn 0
        }
    }
    
    fr fr Call native implementation for actual filesystem scanning
    damn cursed_plugin_discover(plugin_manager_handle, directory)
}

fr fr Real plugin loading with dynamic library support
slay load_plugin(path tea) Plug {
    lowkey plugin_manager_handle == null {
        lowkey !initialize_plugin_system() {
            damn Plug(0)
        }
    }
    
    fr fr Call native plugin loader
    sus plugin_id normie = cursed_plugin_load(plugin_manager_handle, path, cap, cap)
    lowkey plugin_id == 0 {
        damn Plug(0) fr fr Failed to load
    }
    
    damn Plug(plugin_id)
}

fr fr Load plugin with security options
slay load_plugin_with_options(path tea, verify_signature lit, sandbox lit) Plug {
    lowkey plugin_manager_handle == null {
        lowkey !initialize_plugin_system() {
            damn Plug(0)
        }
    }
    
    fr fr Call native plugin loader with security options
    sus plugin_id normie = cursed_plugin_load(plugin_manager_handle, path, verify_signature, sandbox)
    lowkey plugin_id == 0 {
        damn Plug(0) fr fr Failed to load
    }
    
    damn Plug(plugin_id)
}

fr fr Real plugin unloading with cleanup
slay unload_plugin(plugin Plug) lit {
    lowkey plugin_manager_handle == null {
        damn cap
    }
    
    sus plugin_id normie = normie(plugin)
    sus result normie = cursed_plugin_unload(plugin_manager_handle, plugin_id)
    damn result == 1
}

fr fr Get real plugin information
slay get_plugin_name(plugin Plug) tea {
    lowkey plugin_manager_handle == null {
        damn "unknown"
    }
    
    sus plugin_id normie = normie(plugin)
    sus name_ptr vibes = cursed_plugin_get_name(plugin_manager_handle, plugin_id)
    lowkey name_ptr == null {
        damn "unknown"
    }
    
    damn cursed_cstring_to_tea(name_ptr)
}

slay get_plugin_path(plugin Plug) tea {
    lowkey plugin_manager_handle == null {
        damn ""
    }
    
    sus plugin_id normie = normie(plugin)
    sus path_ptr vibes = cursed_plugin_get_path(plugin_manager_handle, plugin_id)
    lowkey path_ptr == null {
        damn ""
    }
    
    damn cursed_cstring_to_tea(path_ptr)
}

slay get_plugin_status(plugin Plug) normie {
    lowkey plugin_manager_handle == null {
        damn PLUG_STATUS_ERROR
    }
    
    sus plugin_id normie = normie(plugin)
    damn cursed_plugin_get_status(plugin_manager_handle, plugin_id)
}

slay get_plugin_version(plugin Plug) tea {
    lowkey plugin_manager_handle == null {
        damn "0.0.0"
    }
    
    sus plugin_id normie = normie(plugin)
    sus version_ptr vibes = cursed_plugin_get_version(plugin_manager_handle, plugin_id)
    lowkey version_ptr == null {
        damn "0.0.0"
    }
    
    damn cursed_cstring_to_tea(version_ptr)
}

slay get_plugin_author(plugin Plug) tea {
    lowkey plugin_manager_handle == null {
        damn "unknown"
    }
    
    sus plugin_id normie = normie(plugin)
    sus author_ptr vibes = cursed_plugin_get_author(plugin_manager_handle, plugin_id)
    lowkey author_ptr == null {
        damn "unknown"
    }
    
    damn cursed_cstring_to_tea(author_ptr)
}

slay get_plugin_description(plugin Plug) tea {
    lowkey plugin_manager_handle == null {
        damn ""
    }
    
    sus plugin_id normie = normie(plugin)
    sus desc_ptr vibes = cursed_plugin_get_description(plugin_manager_handle, plugin_id)
    lowkey desc_ptr == null {
        damn ""
    }
    
    damn cursed_cstring_to_tea(desc_ptr)
}

fr fr Real capability checking with bitwise operations
slay get_plugin_capabilities(plugin Plug) normie {
    lowkey plugin_manager_handle == null {
        damn 0
    }
    
    sus plugin_id normie = normie(plugin)
    damn cursed_plugin_get_capabilities(plugin_manager_handle, plugin_id)
}

slay has_capability(plugin Plug, capability normie) lit {
    sus capabilities normie = get_plugin_capabilities(plugin)
    damn (capabilities & capability) != 0
}

fr fr Real signature verification using cryptographic validation
slay verify_plugin_signature(path tea, public_key tea) lit {
    lowkey plugin_manager_handle == null {
        damn cap
    }
    
    sus result normie = cursed_plugin_verify_signature(plugin_manager_handle, path, public_key)
    damn result == 1
}

fr fr Plugin validation with manifest and dependency checking
slay validate_plugin(path tea) lit {
    lowkey plugin_manager_handle == null {
        damn cap
    }
    
    sus result normie = cursed_plugin_validate(plugin_manager_handle, path)
    damn result == 1
}

fr fr Real plugin function calling with type marshalling
slay call_plugin_function(plugin Plug, function_name tea, ...args) tea {
    lowkey plugin_manager_handle == null {
        damn "ERROR: Plugin system not initialized"
    }
    
    sus plugin_id normie = normie(plugin)
    
    fr fr For now, handle simple cases - full marshalling requires more work
    lowkey len(args) == 0 {
        sus result_ptr vibes = cursed_plugin_call_function_0(plugin_manager_handle, plugin_id, function_name)
        lowkey result_ptr == null {
            damn "ERROR: Function call failed"
        }
        damn cursed_cstring_to_tea(result_ptr)
    } otherwise lowkey len(args) == 1 {
        fr fr Single argument case
        sus result_ptr vibes = cursed_plugin_call_function_1(plugin_manager_handle, plugin_id, function_name, args[0])
        lowkey result_ptr == null {
            damn "ERROR: Function call failed"
        }
        damn cursed_cstring_to_tea(result_ptr)
    } otherwise {
        damn "ERROR: Multiple arguments not yet supported"
    }
}

fr fr Extension points with real callback system
sus extension_counter normie = 0
sus extension_registry normie[100] fr fr Extension ID -> Plugin count mapping

slay create_extension_point(name tea) normie {
    lowkey plugin_manager_handle == null {
        lowkey !initialize_plugin_system() {
            damn 0
        }
    }
    
    extension_counter = extension_counter + 1
    sus ext_id normie = cursed_plugin_create_extension_point(plugin_manager_handle, name)
    lowkey ext_id == 0 {
        damn 0 fr fr Failed to create
    }
    
    extension_registry[extension_counter] = 0
    damn ext_id
}

slay register_extension(point_id normie, plugin Plug) lit {
    lowkey plugin_manager_handle == null {
        damn cap
    }
    
    sus plugin_id normie = normie(plugin)
    sus result normie = cursed_plugin_register_extension(plugin_manager_handle, point_id, plugin_id)
    damn result == 1
}

slay call_extension_point(point_id normie, data tea) tea {
    lowkey plugin_manager_handle == null {
        damn data
    }
    
    sus result_ptr vibes = cursed_plugin_call_extension_point(plugin_manager_handle, point_id, data)
    lowkey result_ptr == null {
        damn data fr fr No processing
    }
    
    damn cursed_cstring_to_tea(result_ptr)
}

fr fr Hot reloading with proper cleanup and reinitialization
slay reload_plugin(plugin Plug) lit {
    lowkey plugin_manager_handle == null {
        damn cap
    }
    
    sus plugin_id normie = normie(plugin)
    sus result normie = cursed_plugin_reload(plugin_manager_handle, plugin_id)
    damn result == 1
}

fr fr Registry management
slay register_plugin(name tea, plugin Plug) lit {
    lowkey plugin_manager_handle == null {
        damn cap
    }
    
    sus plugin_id normie = normie(plugin)
    sus result normie = cursed_plugin_register_name(plugin_manager_handle, name, plugin_id)
    damn result == 1
}

slay find_plugin_by_name(name tea) Plug {
    lowkey plugin_manager_handle == null {
        damn Plug(0)
    }
    
    sus plugin_id normie = cursed_plugin_find_by_name(plugin_manager_handle, name)
    damn Plug(plugin_id)
}

slay list_loaded_plugins() normie {
    lowkey plugin_manager_handle == null {
        damn 0
    }
    
    damn cursed_plugin_count_loaded(plugin_manager_handle)
}

fr fr Memory usage tracking
slay get_plugin_memory_usage(plugin Plug) normie {
    lowkey plugin_manager_handle == null {
        damn 0
    }
    
    sus plugin_id normie = normie(plugin)
    damn cursed_plugin_get_memory_usage(plugin_manager_handle, plugin_id)
}

fr fr Statistics and monitoring
slay get_total_plugins() normie {
    lowkey plugin_manager_handle == null {
        damn 0
    }
    
    damn cursed_plugin_count_total(plugin_manager_handle)
}

slay get_loaded_plugin_count() normie {
    damn list_loaded_plugins()
}

fr fr Auto-loading with directory monitoring
slay start_auto_loading(directory tea) lit {
    lowkey plugin_manager_handle == null {
        lowkey !initialize_plugin_system() {
            damn cap
        }
    }
    
    sus result normie = cursed_plugin_start_auto_loading(plugin_manager_handle, directory)
    damn result == 1
}

slay stop_auto_loading() lit {
    lowkey plugin_manager_handle == null {
        damn cap
    }
    
    sus result normie = cursed_plugin_stop_auto_loading(plugin_manager_handle)
    damn result == 1
}

fr fr Plugin installation from remote sources
slay install_plugin_from_url(url tea, destination tea) lit {
    lowkey plugin_manager_handle == null {
        lowkey !initialize_plugin_system() {
            damn cap
        }
    }
    
    sus result normie = cursed_plugin_install_from_url(plugin_manager_handle, url, destination)
    damn result == 1
}

slay uninstall_plugin(name tea) lit {
    lowkey plugin_manager_handle == null {
        damn cap
    }
    
    sus plugin Plug = find_plugin_by_name(name)
    lowkey normie(plugin) == 0 {
        damn cap fr fr Plugin not found
    }
    
    damn unload_plugin(plugin)
}

fr fr Security and sandboxing
slay create_sandbox() normie {
    lowkey plugin_manager_handle == null {
        damn 0
    }
    
    damn cursed_plugin_create_sandbox(plugin_manager_handle)
}

slay execute_in_sandbox(sandbox_id normie, plugin Plug, function_name tea) lit {
    lowkey plugin_manager_handle == null {
        damn cap
    }
    
    sus plugin_id normie = normie(plugin)
    sus result normie = cursed_plugin_execute_sandboxed(plugin_manager_handle, sandbox_id, plugin_id, function_name)
    damn result == 1
}

fr fr Lifecycle management
slay initialize_plugin(plugin Plug) lit {
    lowkey plugin_manager_handle == null {
        damn cap
    }
    
    sus plugin_id normie = normie(plugin)
    sus result normie = cursed_plugin_initialize(plugin_manager_handle, plugin_id)
    damn result == 1
}

slay cleanup_plugin(plugin Plug) lit {
    lowkey plugin_manager_handle == null {
        damn cap
    }
    
    sus plugin_id normie = normie(plugin)
    sus result normie = cursed_plugin_cleanup(plugin_manager_handle, plugin_id)
    damn result == 1
}

fr fr Compatibility checking
slay is_plugin_compatible(plugin Plug, api_version normie) lit {
    lowkey plugin_manager_handle == null {
        damn cap
    }
    
    sus plugin_id normie = normie(plugin)
    sus result normie = cursed_plugin_check_compatibility(plugin_manager_handle, plugin_id, api_version)
    damn result == 1
}

fr fr Validation utilities
slay is_valid_plugin(plugin Plug) lit {
    lowkey plugin_manager_handle == null {
        damn cap
    }
    
    sus plugin_id normie = normie(plugin)
    damn cursed_plugin_is_valid(plugin_manager_handle, plugin_id) == 1
}

fr fr System reset
slay reset_plugin_registry() {
    lowkey plugin_manager_handle != null {
        cleanup_plugin_system()
    }
    extension_counter = 0
    bestie i := 0; i < 100; i++ {
        extension_registry[i] = 0
    }
}

fr fr Plugin manager operations
slay create_plugin_manager(plugin_dir tea, auto_load lit) normie {
    fr fr We use a single global manager, but return ID for compatibility
    lowkey initialize_plugin_system() {
        lowkey auto_load {
            start_auto_loading(plugin_dir)
        }
        damn 1
    }
    damn 0
}

slay start_plugin_manager(manager_id normie) lit {
    fr fr Manager is always running when initialized
    damn plugin_manager_handle != null
}

slay stop_plugin_manager(manager_id normie) lit {
    cleanup_plugin_system()
    damn based
}

fr fr Generate plugin manifest
slay generate_plugin_manifest(name tea, version tea, author tea, description tea) tea {
    damn "{\"name\":\"" + name + "\",\"version\":\"" + version + "\",\"author\":\"" + author + "\",\"description\":\"" + description + "\",\"api_version\":1,\"capabilities\":[]}"
}

fr fr Native function declarations (implemented in Zig)
extern slay cursed_plugin_manager_init() vibes
extern slay cursed_plugin_manager_deinit(manager vibes)
extern slay cursed_plugin_discover(manager vibes, directory tea) normie
extern slay cursed_plugin_load(manager vibes, path tea, verify_signature lit, sandbox lit) normie
extern slay cursed_plugin_unload(manager vibes, plugin_id normie) normie
extern slay cursed_plugin_get_name(manager vibes, plugin_id normie) vibes
extern slay cursed_plugin_get_path(manager vibes, plugin_id normie) vibes
extern slay cursed_plugin_get_status(manager vibes, plugin_id normie) normie
extern slay cursed_plugin_get_version(manager vibes, plugin_id normie) vibes
extern slay cursed_plugin_get_author(manager vibes, plugin_id normie) vibes
extern slay cursed_plugin_get_description(manager vibes, plugin_id normie) vibes
extern slay cursed_plugin_get_capabilities(manager vibes, plugin_id normie) normie
extern slay cursed_plugin_verify_signature(manager vibes, path tea, public_key tea) normie
extern slay cursed_plugin_validate(manager vibes, path tea) normie
extern slay cursed_plugin_call_function_0(manager vibes, plugin_id normie, function_name tea) vibes
extern slay cursed_plugin_call_function_1(manager vibes, plugin_id normie, function_name tea, arg1 vibes) vibes
extern slay cursed_plugin_create_extension_point(manager vibes, name tea) normie
extern slay cursed_plugin_register_extension(manager vibes, point_id normie, plugin_id normie) normie
extern slay cursed_plugin_call_extension_point(manager vibes, point_id normie, data tea) vibes
extern slay cursed_plugin_reload(manager vibes, plugin_id normie) normie
extern slay cursed_plugin_register_name(manager vibes, name tea, plugin_id normie) normie
extern slay cursed_plugin_find_by_name(manager vibes, name tea) normie
extern slay cursed_plugin_count_loaded(manager vibes) normie
extern slay cursed_plugin_get_memory_usage(manager vibes, plugin_id normie) normie
extern slay cursed_plugin_count_total(manager vibes) normie
extern slay cursed_plugin_start_auto_loading(manager vibes, directory tea) normie
extern slay cursed_plugin_stop_auto_loading(manager vibes) normie
extern slay cursed_plugin_install_from_url(manager vibes, url tea, destination tea) normie
extern slay cursed_plugin_create_sandbox(manager vibes) normie
extern slay cursed_plugin_execute_sandboxed(manager vibes, sandbox_id normie, plugin_id normie, function_name tea) normie
extern slay cursed_plugin_initialize(manager vibes, plugin_id normie) normie
extern slay cursed_plugin_cleanup(manager vibes, plugin_id normie) normie
extern slay cursed_plugin_check_compatibility(manager vibes, plugin_id normie, api_version normie) normie
extern slay cursed_plugin_is_valid(manager vibes, plugin_id normie) normie
extern slay cursed_cstring_to_tea(ptr vibes) tea
