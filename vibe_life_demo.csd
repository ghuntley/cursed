yeet "vibe_life"

vibez.spill("=== CURSED vibe_life Module Demo ===")

# Environment Variables
vibez.spill("Environment Variables:")
sus home tea = getenv("HOME")
vibez.spill("  HOME:", home)
setenv("CURSED_VAR", "cursed_value")
sus cursed_val tea = getenv("CURSED_VAR")
vibez.spill("  CURSED_VAR:", cursed_val)

# Process Information
vibez.spill("Process Information:")
sus pid normie = getpid()
sus ppid normie = getppid()
vibez.spill("  PID:", pid)
vibez.spill("  PPID:", ppid)

# File Path Operations
vibez.spill("File Path Operations:")
sus paths [tea] = ["home", "user", "documents", "project"]
sus joined tea = path_join(paths)
vibez.spill("  Joined path:", joined)

sus (dir, file) = path_split("/home/user/project/file.txt")
vibez.spill("  Directory:", dir)
vibez.spill("  File:", file)

sus ext tea = path_ext("/home/user/project/file.txt")
vibez.spill("  Extension:", ext)

# Directory Operations
vibez.spill("Directory Operations:")
sus cwd tea = getcwd()
vibez.spill("  Current directory:", cwd)

sus temp tea = temp_dir()
vibez.spill("  Temp directory:", temp)

# System Information
vibez.spill("System Information:")
sus info tea = system_info()
sus host tea = hostname()
sus user tea = username()
vibez.spill("  System:", info)
vibez.spill("  Hostname:", host)
vibez.spill("  Username:", user)

# User/Group Information
vibez.spill("User/Group Information:")
sus uid normie = getuid()
sus gid normie = getgid()
vibez.spill("  UID:", uid)
vibez.spill("  GID:", gid)

# Constants
vibez.spill("Constants:")
vibez.spill("  EXIT_SUCCESS:", EXIT_SUCCESS)
vibez.spill("  EXIT_FAILURE:", EXIT_FAILURE)
vibez.spill("  SIGINT:", SIGINT)
vibez.spill("  SIGTERM:", SIGTERM)

vibez.spill("=== Demo Complete ===")
