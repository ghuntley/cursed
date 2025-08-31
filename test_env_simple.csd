fr fr Simple Env Test - Direct Function Calls

slay get_env(key tea) tea {
    lowkey (key == "HOME") {
        damn "/home/cursed"
    }
    lowkey (key == "PATH") {
        damn "/usr/bin:/bin"
    }
    damn "default_value"
}

slay has_env(key tea) lit {
    lowkey (key == "HOME") {
        damn based
    }
    lowkey (key == "NONEXISTENT") {
        damn cap
    }
    damn based
}

sus home_val tea = get_env("HOME")
yap home_val

sus path_val tea = get_env("PATH")
yap path_val

sus unknown_val tea = get_env("UNKNOWN")
yap unknown_val

sus has_home lit = has_env("HOME")
yap has_home

sus has_nonexistent lit = has_env("NONEXISTENT")
yap has_nonexistent
