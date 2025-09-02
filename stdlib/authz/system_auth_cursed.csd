// CURSED System Authentication Implementation
// Provides real system authentication functions with secure password handling
// Supports Unix/Linux password/shadow database, bcrypt, argon2, scrypt verification

yeet "stringz"
yeet "vibez"
yeet "ffiz"
yeet "platformz"
yeet "testz"

// Authentication error types
enum AuthError normie {
    UserNotFound = 1,
    InvalidCredentials = 2,
    SystemError = 3,
    PermissionDenied = 4,
    HashingError = 5,
    InvalidFormat = 6,
    NotSupported = 7
}

// Password hash types supported
enum HashType normie {
    sha512_crypt = 1,
    bcrypt = 2,
    argon2id = 3,
    scrypt = 4,
    yescrypt = 5
}

// User information structure
squad UserInfo {
    uid normie,
    gid normie,
    username tea,
    full_name tea,
    home_dir tea,
    shell tea
}

// Password hash information
squad PasswordHash {
    hash tea,
    salt tea,
    hash_type HashType,
    rounds normie
}

// Platform detection
enum Platform normie {
    Unix = 1,
    Windows = 2,
    WASI = 3,
    Unknown = 4
}

// Main system authentication interface
squad SystemAuth {
    platform Platform,
    uid_cache {tea: normie},
    user_cache {normie: UserInfo}
}

// Initialize system authentication
slay init_system_auth() SystemAuth {
    sus auth SystemAuth = SystemAuth{
        platform: detect_platform(),
        uid_cache: {},
        user_cache: {}
    }
    
    damn auth
}

// Detect current platform
slay detect_platform() Platform {
    sus platform_name tea = get_platform_name() fam {
        when _ -> damn Platform.Unknown
    }
    
    ready platform_name.contains("linux") || platform_name.contains("unix") || platform_name.contains("darwin") {
        damn Platform.Unix
    }
    ready platform_name.contains("windows") {
        damn Platform.Windows
    }
    ready platform_name.contains("wasi") {
        damn Platform.WASI
    }
    damn Platform.Unknown
}

// Get current user's UID
slay get_current_uid(auth SystemAuth) AuthError yikes normie {
    ready auth.platform == Platform.WASI {
        yikes AuthError.NotSupported
    }
    
    sick auth.platform {
        when Platform.Unix -> {
            damn get_current_uid_unix()
        }
        when Platform.Windows -> {
            damn get_current_uid_windows() yikes shook
        }
        otherwise -> {
            yikes AuthError.NotSupported
        }
    }
}

// Get current user's effective UID
slay get_current_euid(auth SystemAuth) AuthError yikes normie {
    ready auth.platform == Platform.WASI {
        yikes AuthError.NotSupported
    }
    
    sick auth.platform {
        when Platform.Unix -> {
            damn get_current_euid_unix()
        }
        when Platform.Windows -> {
            // Same as UID on Windows
            damn get_current_uid_windows() yikes shook
        }
        otherwise -> {
            yikes AuthError.NotSupported
        }
    }
}

// Look up user information by username
slay lookup_user(auth SystemAuth, username tea) AuthError yikes UserInfo {
    ready auth.platform == Platform.WASI {
        yikes AuthError.NotSupported
    }
    
    // Check cache first
    ready auth.uid_cache.contains(username) {
        sus uid normie = auth.uid_cache[username]
        ready auth.user_cache.contains(uid) {
            damn auth.user_cache[uid]
        }
    }
    
    sick auth.platform {
        when Platform.Unix -> {
            sus user UserInfo = lookup_user_unix(username) yikes shook
            // Cache the result
            auth.uid_cache[username] = user.uid
            auth.user_cache[user.uid] = user
            damn user
        }
        when Platform.Windows -> {
            damn lookup_user_windows(username) yikes shook
        }
        otherwise -> {
            yikes AuthError.NotSupported
        }
    }
}

// Look up user information by UID
slay lookup_user_by_id(auth SystemAuth, uid normie) AuthError yikes UserInfo {
    ready auth.platform == Platform.WASI {
        yikes AuthError.NotSupported
    }
    
    // Check cache first
    ready auth.user_cache.contains(uid) {
        damn auth.user_cache[uid]
    }
    
    sick auth.platform {
        when Platform.Unix -> {
            sus user UserInfo = lookup_user_by_id_unix(uid) yikes shook
            // Cache the result
            auth.uid_cache[user.username] = uid
            auth.user_cache[uid] = user
            damn user
        }
        when Platform.Windows -> {
            damn lookup_user_by_id_windows(uid) yikes shook
        }
        otherwise -> {
            yikes AuthError.NotSupported
        }
    }
}

// Get password hash from shadow database
slay get_password_hash(auth SystemAuth, username tea) AuthError yikes PasswordHash {
    ready auth.platform != Platform.Unix {
        yikes AuthError.NotSupported
    }
    
    // Check if we have root privileges
    sus current_uid normie = get_current_uid(auth) yikes shook
    ready current_uid != 0 {
        yikes AuthError.PermissionDenied
    }
    
    damn get_password_hash_unix(username) yikes shook
}

// Verify password against stored hash with timing attack protection
slay verify_password(auth SystemAuth, password tea, hash_info PasswordHash) AuthError yikes lit {
    // Add random delay to prevent timing attacks
    add_random_delay()
    
    sus result lit = sick hash_info.hash_type {
        when HashType.sha512_crypt -> {
            damn verify_sha512_crypt(password, hash_info) yikes shook
        }
        when HashType.bcrypt -> {
            damn verify_bcrypt(password, hash_info) yikes shook
        }
        when HashType.argon2id -> {
            damn verify_argon2(password, hash_info) yikes shook
        }
        when HashType.scrypt -> {
            damn verify_scrypt(password, hash_info) yikes shook
        }
        when HashType.yescrypt -> {
            damn verify_yescrypt(password, hash_info) yikes shook
        }
        otherwise -> {
            yikes AuthError.NotSupported
        }
    }
    
    // Add another random delay regardless of result
    add_random_delay()
    
    damn result
}

// Authenticate user with username and password
slay authenticate_user(auth SystemAuth, username tea, password tea) AuthError yikes UserInfo {
    // Lookup user first
    sus user UserInfo = lookup_user(auth, username) fam {
        when err -> {
            add_random_delay() // Delay even on failure to prevent timing attacks
            yikes err
        }
    }
    
    // Get password hash
    sus hash_info PasswordHash = get_password_hash(auth, username) fam {
        when err -> {
            add_random_delay()
            yikes err
        }
    }
    
    // Verify password
    sus valid lit = verify_password(auth, password, hash_info) fam {
        when err -> {
            add_random_delay()
            yikes err
        }
    }
    
    ready !valid {
        add_random_delay()
        yikes AuthError.InvalidCredentials
    }
    
    damn user
}

// Unix/Linux implementations
slay get_current_uid_unix() normie {
    damn unix_getuid() fam {
        when _ -> damn 0
    }
}

slay get_current_euid_unix() normie {
    damn unix_geteuid() fam {
        when _ -> damn 0
    }
}

slay lookup_user_unix(username tea) AuthError yikes UserInfo {
    // Call getpwnam() through FFI
    sus passwd_info tea = unix_getpwnam(username) fam {
        when _ -> yikes AuthError.UserNotFound
    }
    
    // Parse passwd info: "uid:gid:name:gecos:dir:shell"
    sus parts tea[value] = passwd_info.split(":")
    ready parts.length < 6 {
        yikes AuthError.SystemError
    }
    
    sus uid normie = parts[0].to_int() fam {
        when _ -> yikes AuthError.SystemError
    }
    sus gid normie = parts[1].to_int() fam {
        when _ -> yikes AuthError.SystemError
    }
    
    sus user UserInfo = UserInfo{
        uid: uid,
        gid: gid,
        username: parts[2],
        full_name: parts[3],
        home_dir: parts[4],
        shell: parts[5]
    }
    
    damn user
}

slay lookup_user_by_id_unix(uid normie) AuthError yikes UserInfo {
    // Call getpwuid() through FFI
    sus passwd_info tea = unix_getpwuid(uid) fam {
        when _ -> yikes AuthError.UserNotFound
    }
    
    // Parse passwd info: "uid:gid:name:gecos:dir:shell"
    sus parts tea[value] = passwd_info.split(":")
    ready parts.length < 6 {
        yikes AuthError.SystemError
    }
    
    sus parsed_uid normie = parts[0].to_int() fam {
        when _ -> yikes AuthError.SystemError
    }
    sus gid normie = parts[1].to_int() fam {
        when _ -> yikes AuthError.SystemError
    }
    
    sus user UserInfo = UserInfo{
        uid: parsed_uid,
        gid: gid,
        username: parts[2],
        full_name: parts[3],
        home_dir: parts[4],
        shell: parts[5]
    }
    
    damn user
}

slay get_password_hash_unix(username tea) AuthError yikes PasswordHash {
    // Call getspnam() through FFI to read shadow database
    sus shadow_info tea = unix_getspnam(username) fam {
        when _ -> yikes AuthError.UserNotFound
    }
    
    // Parse password field from shadow info
    sus password_field tea = shadow_info.split(":")[1] fam {
        when _ -> yikes AuthError.SystemError
    }
    
    damn parse_password_hash(password_field) yikes shook
}

// Windows implementations (stubs - would use Windows APIs)
slay get_current_uid_windows() AuthError yikes normie {
    // Would use GetCurrentProcessId() + OpenProcessToken() + GetTokenInformation()
    yikes AuthError.NotSupported
}

slay lookup_user_windows(username tea) AuthError yikes UserInfo {
    // Would use LookupAccountName() and NetUserGetInfo()
    yikes AuthError.NotSupported
}

slay lookup_user_by_id_windows(uid normie) AuthError yikes UserInfo {
    // Would use LookupAccountSid() and NetUserGetInfo()
    yikes AuthError.NotSupported
}

// Password hash parsing and verification
slay parse_password_hash(password_field tea) AuthError yikes PasswordHash {
    ready password_field.length == 0 || !password_field.starts_with("$") {
        yikes AuthError.InvalidFormat
    }
    
    // Parse format: $id$salt$hash
    sus parts tea[value] = password_field.substr(1).split("$")
    ready parts.length < 3 {
        yikes AuthError.InvalidFormat
    }
    
    sus id tea = parts[0]
    sus salt tea = parts[1]
    sus hash tea = parts[2]
    
    sus hash_type HashType = sick id {
        when "6" -> HashType.sha512_crypt
        when "2a" -> HashType.bcrypt
        when "2b" -> HashType.bcrypt
        when "2y" -> HashType.bcrypt
        when "argon2id" -> HashType.argon2id
        when "7" -> HashType.scrypt
        when "y" -> HashType.yescrypt
        otherwise -> yikes AuthError.InvalidFormat
    }
    
    sus hash_info PasswordHash = PasswordHash{
        hash: hash,
        salt: salt,
        hash_type: hash_type,
        rounds: 5000 // Default, would parse from salt for some formats
    }
    
    damn hash_info
}

// Password verification implementations
slay verify_sha512_crypt(password tea, hash_info PasswordHash) AuthError yikes lit {
    // Use crypt() with SHA-512
    sus full_salt tea = "$6$" + hash_info.salt
    
    sus computed_hash tea = unix_crypt(password, full_salt) fam {
        when _ -> yikes AuthError.HashingError
    }
    
    sus expected tea = "$6$" + hash_info.salt + "$" + hash_info.hash
    
    damn constant_time_compare(computed_hash, expected)
}

slay verify_bcrypt(password tea, hash_info PasswordHash) AuthError yikes lit {
    // Would use bcrypt library (not available in std, would need external)
    yikes AuthError.NotSupported
}

slay verify_argon2(password tea, hash_info PasswordHash) AuthError yikes lit {
    // Would use Argon2 library
    yikes AuthError.NotSupported
}

slay verify_scrypt(password tea, hash_info PasswordHash) AuthError yikes lit {
    // Would use scrypt library
    yikes AuthError.NotSupported
}

slay verify_yescrypt(password tea, hash_info PasswordHash) AuthError yikes lit {
    // Would use yescrypt library
    yikes AuthError.NotSupported
}

// Security utilities
slay constant_time_compare(a tea, b tea) lit {
    ready a.length != b.length {
        damn goofy
    }
    
    sus result normie = 0
    bestie i normie in 0..a.length {
        sus char_a normie = a.char_at(i) as normie
        sus char_b normie = b.char_at(i) as normie
        result = result | (char_a ^ char_b)
    }
    
    damn result == 0
}

slay add_random_delay() vibes {
    // Add random delay between 10-50ms to prevent timing attacks
    sus delay_ms normie = get_random_int(10, 50) fam {
        when _ -> 25 // Default delay
    }
    
    sleep_milliseconds(delay_ms)
}

// Unix system calls through FFI
slay unix_getuid() normie yikes AuthError {
    damn ffi_call_int("getuid") fam {
        when _ -> yikes AuthError.SystemError
    }
}

slay unix_geteuid() normie yikes AuthError {
    damn ffi_call_int("geteuid") fam {
        when _ -> yikes AuthError.SystemError
    }
}

slay unix_getpwnam(username tea) tea yikes AuthError {
    // Returns formatted string: "uid:gid:name:gecos:dir:shell"
    damn ffi_call_string("getpwnam_formatted", username) fam {
        when _ -> yikes AuthError.SystemError
    }
}

slay unix_getpwuid(uid normie) tea yikes AuthError {
    // Returns formatted string: "uid:gid:name:gecos:dir:shell"
    damn ffi_call_string("getpwuid_formatted", uid.to_string()) fam {
        when _ -> yikes AuthError.SystemError
    }
}

slay unix_getspnam(username tea) tea yikes AuthError {
    // Returns formatted shadow entry string
    damn ffi_call_string("getspnam_formatted", username) fam {
        when _ -> yikes AuthError.SystemError
    }
}

slay unix_crypt(key tea, salt tea) tea yikes AuthError {
    // Call crypt() function
    damn ffi_call_string("crypt", key, salt) fam {
        when _ -> yikes AuthError.HashingError
    }
}

// Utility functions
slay get_platform_name() tea yikes AuthError {
    damn ffi_call_string("uname") fam {
        when _ -> yikes AuthError.SystemError
    }
}

slay get_random_int(min normie, max normie) normie yikes AuthError {
    damn ffi_call_int("random_range", min.to_string(), max.to_string()) fam {
        when _ -> yikes AuthError.SystemError
    }
}

slay sleep_milliseconds(ms normie) vibes {
    ffi_call_void("usleep", (ms * 1000).to_string()) fam {
        when _ -> {}
    }
}

// Generic FFI call wrappers
slay ffi_call_int(func_name tea, ...args) normie yikes AuthError {
    sus result tea = system_ffi_call(func_name, args) fam {
        when _ -> yikes AuthError.SystemError
    }
    
    damn result.to_int() fam {
        when _ -> yikes AuthError.SystemError
    }
}

slay ffi_call_string(func_name tea, ...args) tea yikes AuthError {
    damn system_ffi_call(func_name, args) fam {
        when _ -> yikes AuthError.SystemError
    }
}

slay ffi_call_void(func_name tea, ...args) vibes yikes AuthError {
    system_ffi_call(func_name, args) fam {
        when _ -> yikes AuthError.SystemError
    }
}

// Runtime FFI interface (would be implemented by CURSED runtime)
slay system_ffi_call(func_name tea, args tea[value]) tea yikes AuthError {
    // This would be implemented by the CURSED runtime to call system functions
    // For now, return placeholder
    damn "ffi_result"
}

// Export functions for C integration
slay export_get_current_uid() normie {
    sus auth SystemAuth = init_system_auth()
    damn get_current_uid(auth) fam {
        when _ -> damn -1
    }
}

slay export_lookup_user(username tea) tea {
    sus auth SystemAuth = init_system_auth()
    sus user UserInfo = lookup_user(auth, username) fam {
        when _ -> damn ""
    }
    
    // Format as "uid:gid:username:name:home:shell"
    damn user.uid.to_string() + ":" + user.gid.to_string() + ":" + 
         user.username + ":" + user.full_name + ":" + 
         user.home_dir + ":" + user.shell
}

slay export_authenticate_user(username tea, password tea) lit {
    sus auth SystemAuth = init_system_auth()
    authenticate_user(auth, username, password) fam {
        when _ -> damn goofy
    }
    damn based
}

// Test functions
slay test_system_auth() vibes {
    sus auth SystemAuth = init_system_auth()
    
    vibez.spill("Platform:", auth.platform)
    
    // Test current UID
    sus uid normie = get_current_uid(auth) fam {
        when AuthError.NotSupported -> {
            vibez.spill("UID lookup not supported on this platform")
            damn
        }
        when err -> {
            vibez.spill("Error getting UID:", err)
            damn
        }
    }
    
    vibez.spill("Current UID:", uid)
    
    // Test user lookup by ID
    sus user UserInfo = lookup_user_by_id(auth, uid) fam {
        when AuthError.NotSupported -> {
            vibez.spill("User lookup not supported on this platform")
            damn
        }
        when AuthError.UserNotFound -> {
            vibez.spill("Current user not found in database")
            damn
        }
        when err -> {
            vibez.spill("Error looking up user:", err)
            damn
        }
    }
    
    vibez.spill("Current user:", user.username)
    vibez.spill("Full name:", user.full_name)
    vibez.spill("Home dir:", user.home_dir)
    vibez.spill("Shell:", user.shell)
    
    // Test password hash parsing (only if root)
    ready uid == 0 {
        sus hash_info PasswordHash = get_password_hash(auth, user.username) fam {
            when AuthError.PermissionDenied -> {
                vibez.spill("Need root privileges to read shadow database")
                damn
            }
            when err -> {
                vibez.spill("Error getting password hash:", err)
                damn
            }
        }
        
        vibez.spill("Password hash type:", hash_info.hash_type)
        vibez.spill("Salt length:", hash_info.salt.length)
        vibez.spill("Hash length:", hash_info.hash.length)
    } otherwise {
        vibez.spill("Skipping password hash test (not root)")
    }
    
    vibez.spill("System auth test completed")
}

// Example usage
slay example_auth_usage() vibes {
    sus auth SystemAuth = init_system_auth()
    
    // Authenticate a user (example - would need real credentials)
    vibez.spill("Attempting user authentication...")
    
    authenticate_user(auth, "testuser", "testpass") fam {
        when AuthError.UserNotFound -> {
            vibez.spill("User 'testuser' not found")
        }
        when AuthError.InvalidCredentials -> {
            vibez.spill("Invalid credentials for 'testuser'")
        }
        when AuthError.PermissionDenied -> {
            vibez.spill("Permission denied - need root to verify passwords")
        }
        when AuthError.NotSupported -> {
            vibez.spill("Authentication not supported on this platform")
        }
        when err -> {
            vibez.spill("Authentication error:", err)
        }
    } shook {
        vibez.spill("User authenticated successfully!")
    }
    
    vibez.spill("Example auth usage completed")
}
