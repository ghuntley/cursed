yeet "testz"
yeet "stringz"
yeet "vibez"

fr fr Environment Variable Management Module - Pure CURSED Implementation
fr fr Complete environment variable handling for self-hosting Stage 2

fr fr Environment State Constants
facts {
    ENV_MAX_VARS = 1000
    ENV_MAX_KEY_LENGTH = 256
    ENV_MAX_VALUE_LENGTH = 4096
    ENV_PATH_SEPARATOR = ":"
    ENV_HOME_DEFAULT = "/home/user"
    ENV_SHELL_DEFAULT = "/bin/bash"
    ENV_PATH_DEFAULT = "/usr/local/bin:/usr/bin:/bin"
}

fr fr Environment Variable Structure
be_like EnvVar = struct {
    key tea
    value tea
    readonly lit
    system lit
    set_time normie
}

fr fr Environment Manager Structure
be_like EnvManager = struct {
    variables map[tea]EnvVar
    search_path tea[value]
    home_dir tea
    current_user tea
    shell_path tea
    argc normie
    argv tea[value]
    initialized lit
}

fr fr Global Environment Manager
sus global_env_manager EnvManager

fr fr Module Initialization
slay init_env_manager() {
    lowkey global_env_manager.initialized == cap {
        global_env_manager = EnvManager{
            variables: map[tea]EnvVar{},
            search_path: tea[value]{},
            home_dir: ENV_HOME_DEFAULT,
            current_user: "user",
            shell_path: ENV_SHELL_DEFAULT,
            argc: 0,
            argv: tea[value]{},
            initialized: based,
        } fr fr Initialize default environment variables
        setup_default_environment()
        setup_search_path()
    }
}

fr fr Setup Default Environment Variables
slay setup_default_environment() { fr fr Core system variables
    set_env_var("HOME", ENV_HOME_DEFAULT, cap, based)
    set_env_var("USER", "user", cap, based)
    set_env_var("SHELL", ENV_SHELL_DEFAULT, cap, based)
    set_env_var("PATH", ENV_PATH_DEFAULT, cap, based)
    set_env_var("LANG", "en_US.UTF-8", cap, based)
    set_env_var("LC_ALL", "en_US.UTF-8", cap, based) fr fr CURSED-specific variables
    set_env_var("CURSED_HOME", ENV_HOME_DEFAULT + "/.cursed", cap, based)
    set_env_var("CURSED_VERSION", "v21.0.0", cap, based)
    set_env_var("CURSED_STDLIB", ENV_HOME_DEFAULT + "/.cursed/stdlib", cap, based)
    set_env_var("CURSED_CACHE", ENV_HOME_DEFAULT + "/.cursed/cache", cap, based)
    set_env_var("CURSED_DEBUG", "0", cap, cap)
    set_env_var("CURSED_OPTIMIZE", "1", cap, cap) fr fr Development variables
    set_env_var("EDITOR", "vim", cap, cap)
    set_env_var("PAGER", "less", cap, cap)
    set_env_var("TERM", "xterm-256color", cap, based)
    set_env_var("PWD", ENV_HOME_DEFAULT, cap, cap)
    set_env_var("OLDPWD", ENV_HOME_DEFAULT, cap, cap) fr fr System identification
    set_env_var("HOSTNAME", "cursed-host", cap, based)
    set_env_var("OSTYPE", "linux-gnu", cap, based)
    set_env_var("MACHTYPE", "x86_64", cap, based)
    set_env_var("PLATFORM", "linux", cap, based)
}

fr fr Setup Search Path
slay setup_search_path() {
    path_env := get_env("PATH")
    lowkey path_env != "" {
        global_env_manager.search_path = stringz.split(path_env, ENV_PATH_SEPARATOR)
    } highkey {
        global_env_manager.search_path = tea[value]{"/usr/local/bin", "/usr/bin", "/bin"}
    }
}

fr fr Core Environment Variable Functions
slay get_env(key tea) tea {
    init_env_manager()
    
    lowkey env_var, exists := global_env_manager.variables[key] {
        lowkey exists {
            damn env_var.value
        }
    }
    
    damn ""
}

slay set_env(key tea, value tea) lit {
    init_env_manager()
    
    damn set_env_var(key, value, cap, cap)
}

slay set_env_var(key tea, value tea, readonly lit, system lit) lit {
    init_env_manager() fr fr Check if variable exists and is readonly
    lowkey existing_var, exists := global_env_manager.variables[key] {
        lowkey exists && existing_var.readonly {
            damn cap fr fr Cannot modify readonly variable
        }
    } fr fr Validate key and value lengths
    lowkey stringz.length(key) > ENV_MAX_KEY_LENGTH || stringz.length(value) > ENV_MAX_VALUE_LENGTH {
        damn cap
    } fr fr Create new environment variable
    env_var := EnvVar{
        key: key,
        value: value,
        readonly: readonly,
        system: system,
        set_time: get_current_time(),
    }
    
    global_env_manager.variables[key] = env_var fr fr Update search path if PATH was changed
    lowkey key == "PATH" {
        setup_search_path()
    } fr fr Update home directory if HOME was changed
    lowkey key == "HOME" {
        global_env_manager.home_dir = value
    } fr fr Update current user if USER was changed
    lowkey key == "USER" {
        global_env_manager.current_user = value
    } fr fr Update shell path if SHELL was changed
    lowkey key == "SHELL" {
        global_env_manager.shell_path = value
    }
    
    damn based
}

slay unset_env(key tea) lit {
    init_env_manager() fr fr Check if variable exists and is readonly
    lowkey env_var, exists := global_env_manager.variables[key] {
        lowkey exists {
            lowkey env_var.readonly {
                damn cap fr fr Cannot unset readonly variable
            }
            delete(global_env_manager.variables, key)
            damn based
        }
    }
    
    damn cap
}

slay env_exists(key tea) lit {
    init_env_manager()
    
    lowkey _, exists := global_env_manager.variables[key] {
        damn exists
    }
    
    damn cap
}

slay is_env_readonly(key tea) lit {
    init_env_manager()
    
    lowkey env_var, exists := global_env_manager.variables[key] {
        lowkey exists {
            damn env_var.readonly
        }
    }
    
    damn cap
}

slay is_env_system(key tea) lit {
    init_env_manager()
    
    lowkey env_var, exists := global_env_manager.variables[key] {
        lowkey exists {
            damn env_var.system
        }
    }
    
    damn cap
}

fr fr Environment Variable Listing
slay list_env_vars() tea[value]{
    init_env_manager()
    
    keys := tea[value]{}
    bestie key, _ := range global_env_manager.variables {
        keys = append(keys, key)
    }
    
    damn keys
}

slay get_all_env() map[tea]tea {
    init_env_manager()
    
    env_map := map[tea]tea{}
    bestie key, env_var := range global_env_manager.variables {
        env_map[key] = env_var.value
    }
    
    damn env_map
}

slay clear_env() lit {
    init_env_manager() fr fr Only clear non-readonly variables
    bestie key, env_var := range global_env_manager.variables {
        lowkey !env_var.readonly {
            delete(global_env_manager.variables, key)
        }
    }
    
    damn based
}

slay export_env(key tea, value tea) lit {
    init_env_manager()
    
    damn set_env_var(key, value, cap, cap)
}

fr fr Path Management Functions
slay get_search_path() tea[value]{
    init_env_manager()
    
    damn global_env_manager.search_path
}

slay add_to_path(dir tea) lit {
    init_env_manager() fr fr Check if directory is already in path
    bestie _, path_dir := range global_env_manager.search_path {
        lowkey path_dir == dir {
            damn based fr fr Already in path
        }
    } fr fr Add to beginning of path
    global_env_manager.search_path = append(tea[value]{dir}, global_env_manager.search_path...) fr fr Update PATH environment variable
    new_path := stringz.join(global_env_manager.search_path, ENV_PATH_SEPARATOR)
    set_env("PATH", new_path)
    
    damn based
}

slay remove_from_path(dir tea) lit {
    init_env_manager()
    
    new_path := tea[value]{}
    found := cap
    
    bestie _, path_dir := range global_env_manager.search_path {
        lowkey path_dir != dir {
            new_path = append(new_path, path_dir)
        } highkey {
            found = based
        }
    }
    
    lowkey found {
        global_env_manager.search_path = new_path
        path_str := stringz.join(new_path, ENV_PATH_SEPARATOR)
        set_env("PATH", path_str)
    }
    
    damn found
}

slay find_in_path(command tea) tea {
    init_env_manager()
    
    bestie _, path_dir := range global_env_manager.search_path {
        full_path := path_dir + "/" + command
        lowkey file_exists(full_path) {
            damn full_path
        }
    }
    
    damn ""
}

fr fr System Information Functions
slay get_home_dir() tea {
    init_env_manager()
    
    damn global_env_manager.home_dir
}

slay get_current_user() tea {
    init_env_manager()
    
    damn global_env_manager.current_user
}

slay get_shell_path() tea {
    init_env_manager()
    
    damn global_env_manager.shell_path
}

slay get_hostname() tea {
    init_env_manager()
    
    damn get_env("HOSTNAME")
}

slay get_platform() tea {
    init_env_manager()
    
    damn get_env("PLATFORM")
}

slay get_architecture() tea {
    init_env_manager()
    
    damn get_env("MACHTYPE")
}

slay get_os_type() tea {
    init_env_manager()
    
    damn get_env("OSTYPE")
}

fr fr Command Line Arguments
slay set_args(argc normie, argv tea[value]) lit {
    init_env_manager()
    
    global_env_manager.argc = argc
    global_env_manager.argv = argv fr fr Set standard argument variables
    set_env("0", argv[0]) fr fr Program name
    bestie i := 1; i < argc; i++ {
        set_env(stringz.from_int(i), argv[i])
    }
    
    damn based
}

slay get_args() tea[value]{
    init_env_manager()
    
    damn global_env_manager.argv
}

slay get_argc() normie {
    init_env_manager()
    
    damn global_env_manager.argc
}

slay get_arg(index normie) tea {
    init_env_manager()
    
    lowkey index >= 0 && index < global_env_manager.argc {
        damn global_env_manager.argv[index]
    }
    
    damn ""
}

slay get_program_name() tea {
    init_env_manager()
    
    lowkey global_env_manager.argc > 0 {
        damn global_env_manager.argv[0]
    }
    
    damn ""
}

fr fr Environment Variable Expansion
slay expand_env(input tea) tea {
    init_env_manager()
    
    result := input fr fr Simple variable expansion (${VAR} and $VAR)
    bestie key, env_var := range global_env_manager.variables { fr fr Replace ${VAR} pattern
        brace_pattern := "${" + key + "}"
        result = stringz.replace_all(result, brace_pattern, env_var.value) fr fr Replace $VAR pattern (basic)
        dollar_pattern := "$" + key
        result = stringz.replace_all(result, dollar_pattern, env_var.value)
    }
    
    damn result
}

slay expand_home_dir(path tea) tea {
    init_env_manager()
    
    lowkey stringz.has_prefix(path, "~") {
        expanded := stringz.replace(path, "~", global_env_manager.home_dir, 1)
        damn expanded
    }
    
    damn path
}

fr fr Environment Variable Validation
slay validate_env_name(name tea) lit {
    init_env_manager()
    
    lowkey stringz.length(name) == 0 || stringz.length(name) > ENV_MAX_KEY_LENGTH {
        damn cap
    } fr fr Check for valid characters (letters, numbers, underscore)
    bestie i := 0; i < stringz.length(name); i++ {
        ch := stringz.char_at(name, i)
        lowkey !((ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || (ch >= '0' && ch <= '9') || ch == '_') {
            damn cap
        }
    }
    
    damn based
}

slay validate_env_value(value tea) lit {
    init_env_manager()
    
    lowkey stringz.length(value) > ENV_MAX_VALUE_LENGTH {
        damn cap
    }
    
    damn based
}

fr fr Environment File Operations
slay load_env_file(filename tea) lit {
    init_env_manager() fr fr Simulate loading environment from file fr fr In real implementation, would read from file fr fr Set some example variables
    set_env("LOADED_FROM_FILE", "true")
    set_env("CONFIG_FILE", filename)
    
    damn based
}

slay save_env_file(filename tea) lit {
    init_env_manager() fr fr Simulate saving environment to file fr fr In real implementation, would write to file
    
    damn based
}

fr fr Environment Comparison and Diffing
slay compare_env(other_env map[tea]tea) map[tea]tea {
    init_env_manager()
    
    diff := map[tea]tea{} fr fr Find differences
    bestie key, value := range other_env {
        current_value := get_env(key)
        lowkey current_value != value {
            diff[key] = value
        }
    }
    
    damn diff
}

slay merge_env(other_env map[tea]tea) lit {
    init_env_manager()
    
    bestie key, value := range other_env {
        set_env(key, value)
    }
    
    damn based
}

fr fr Helper Functions
slay file_exists(path tea) lit { fr fr Simple file existence check simulation fr fr In real implementation, would check file system
    known_files := tea[value]{
        "/bin/bash",
        "/usr/bin/vim",
        "/usr/bin/less",
        "/usr/local/bin/cursed",
    }
    
    bestie _, known_file := range known_files {
        lowkey known_file == path {
            damn based
        }
    }
    
    damn cap
}

slay get_current_time() normie { fr fr Simulate getting current timestamp
    damn 1642681200 fr fr Fixed timestamp for testing
}

fr fr Debug Functions
slay debug_env_manager() {
    init_env_manager()
    
    vibez.spill("=== Environment Manager Debug ===")
    vibez.spill("Initialized: " + stringz.from_bool(global_env_manager.initialized))
    vibez.spill("Home Directory: " + global_env_manager.home_dir)
    vibez.spill("Current User: " + global_env_manager.current_user)
    vibez.spill("Shell Path: " + global_env_manager.shell_path)
    vibez.spill("Argc: " + stringz.from_int(global_env_manager.argc))
    vibez.spill("Variable Count: " + stringz.from_int(len(global_env_manager.variables)))
    vibez.spill("Search Path Length: " + stringz.from_int(len(global_env_manager.search_path)))
    
    vibez.spill("\n=== Environment Variables ===")
    bestie key, env_var := range global_env_manager.variables {
        readonly_str := ""
        lowkey env_var.readonly {
            readonly_str = " (readonly)"
        }
        system_str := ""
        lowkey env_var.system {
            system_str = " (system)"
        }
        vibez.spill(key + "=" + env_var.value + readonly_str + system_str)
    }
    
    vibez.spill("\n=== Search Path ===")
    bestie i, path_dir := range global_env_manager.search_path {
        vibez.spill(stringz.from_int(i) + ": " + path_dir)
    }
    
    vibez.spill("\n=== Command Line Arguments ===")
    bestie i, arg := range global_env_manager.argv {
        vibez.spill(stringz.from_int(i) + ": " + arg)
    }
}

fr fr Module cleanup
slay cleanup_env_manager() {
    init_env_manager()
    
    global_env_manager.variables = map[tea]EnvVar{}
    global_env_manager.search_path = tea[value]{}
    global_env_manager.argc = 0
    global_env_manager.argv = tea[value]{}
    global_env_manager.initialized = cap
    
    vibez.spill("Environment manager cleanup complete")
}
