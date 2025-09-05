// CURSED Enhanced User Management Module - Production Security Implementation
// Comprehensive user authentication with cryptographic security and system integration

yeet "cryptz"
yeet "timez" 
yeet "stringz"
yeet "mathz"
yeet "validation"
yeet "filez"
yeet "sysz"

// Enhanced user structure with security metadata
be_like User squad {
    Uid tea
    Gid tea
    Username tea
    Name tea
    HomeDir tea
    Shell tea
    Groups tea[value]
    LastLogin drip
    AccountLocked lit
    PasswordExpired lit
    AccountExpires drip
    SecurityLevel tea  // "basic", "elevated", "admin", "root"
    AuthMethods tea[value]  // ["password", "key", "2fa", "biometric"]
    LoginAttempts drip
    LastFailedLogin drip
}

// Enhanced group with permissions and metadata  
be_like Group squad {
    Gid tea
    Name tea
    Description tea
    Members tea[value]
    AdminUsers tea[value]
    Permissions tea[value]
    SystemGroup lit
    SecurityLevel tea
}

// Enhanced cache with security features
be_like UserCache squad {
    users map[tea]*User
    groups map[tea]*Group
    usersByUid map[tea]*User
    groupsByGid map[tea]*Group
    passwordHashes map[tea]tea  // username -> hash
    saltStorage map[tea]tea     // username -> salt
    sessionTokens map[tea]tea   // token -> username
    rateLimits map[tea]drip    // key -> attempt count
    lastCacheUpdate drip
    cacheExpiry drip           // seconds
}

// Authentication result with comprehensive metadata
be_like AuthResult squad {
    success lit
    user *User
    errorMessage tea
    securityWarnings tea[value]
    authMethod tea
    sessionToken tea
    tokenExpiry drip
    requiresPasswordChange lit
    requires2FA lit
    loginAttempts drip
    clientInfo map[tea]tea
}

// Security audit log entry  
be_like SecurityAuditEntry squad {
    timestamp drip
    username tea
    action tea
    result tea
    sourceIP tea
    userAgent tea
    securityLevel tea
    additionalData map[tea]tea
}

// Global security state
sus globalCache *UserCache = initializeSecureCache()
sus auditLog SecurityAuditEntry[value] = SecurityAuditEntry[value]{}
sus securityConfig SecurityConfig = initializeSecurityConfig()

// Security configuration
be_like SecurityConfig squad {
    maxLoginAttempts drip
    lockoutDurationMinutes drip
    passwordMinLength drip  
    passwordMaxAge drip
    sessionTimeoutMinutes drip
    requireStrongPasswords lit
    enable2FA lit
    auditLogMaxEntries drip
    rateLimitWindowSeconds drip
    rateLimitMaxRequests drip
}

// Enhanced error constants with severity levels
const (
    ErrUserNotFound = "USER_NOT_FOUND"
    ErrGroupNotFound = "GROUP_NOT_FOUND" 
    ErrInvalidCredentials = "INVALID_CREDENTIALS"
    ErrAccountLocked = "ACCOUNT_LOCKED"
    ErrPasswordExpired = "PASSWORD_EXPIRED"
    ErrPermissionDenied = "PERMISSION_DENIED"
    ErrRateLimitExceeded = "RATE_LIMIT_EXCEEDED"
    ErrSecurityViolation = "SECURITY_VIOLATION"
    ErrAuthenticationRequired = "AUTHENTICATION_REQUIRED"
    Err2FARequired = "TWO_FACTOR_AUTH_REQUIRED"
    ErrSessionExpired = "SESSION_EXPIRED"
    ErrInvalidSession = "INVALID_SESSION"
)

// SECURE RANDOM UID/GID GENERATION WITH COLLISION DETECTION

slay generateSecureUID() yikes<tea> {
    // Generate cryptographically secure UID in user range (1000-65535)
    sus attempts drip = 0
    sus maxAttempts drip = 1000
    
    bestie (attempts < maxAttempts) {
        attempts += 1
        
        // Generate random UID in user range
        sus uid drip = cryptz.random_int_range(1000, 65535) fam {
            when err -> yikes "Failed to generate random UID: " + err
        }
        
        sus uid_str tea = tea(uid)
        
        // Check for collision with existing users
        sus collision lit = checkUIDCollision(uid_str) fam {
            when err -> yikes "Failed to check UID collision: " + err  
        }
        
        ready !collision {
            // Additional system verification
            sus system_check lit = verifyUIDNotInSystem(uid_str) fam {
                when err -> yikes "Failed to verify UID in system: " + err
            }
            
            ready system_check {
                damn uid_str
            }
        }
    }
    
    yikes "Failed to generate unique UID after " + tea(maxAttempts) + " attempts"
}

slay generateSecureGID() yikes<tea> {
    // Generate cryptographically secure GID in group range (1000-65535)
    sus attempts drip = 0
    sus maxAttempts drip = 1000
    
    bestie (attempts < maxAttempts) {
        attempts += 1
        
        // Generate random GID in group range  
        sus gid drip = cryptz.random_int_range(1000, 65535) fam {
            when err -> yikes "Failed to generate random GID: " + err
        }
        
        sus gid_str tea = tea(gid)
        
        // Check for collision
        sus collision lit = checkGIDCollision(gid_str) fam {
            when err -> yikes "Failed to check GID collision: " + err
        }
        
        ready !collision {
            sus system_check lit = verifyGIDNotInSystem(gid_str) fam {
                when err -> yikes "Failed to verify GID in system: " + err
            }
            
            ready system_check {
                damn gid_str
            }
        }
    }
    
    yikes "Failed to generate unique GID after " + tea(maxAttempts) + " attempts"
}

slay checkUIDCollision(uid tea) yikes<lit> {
    // Check cache first
    ready globalCache.usersByUid[uid] != nil {
        damn based
    }
    
    // Check system passwd database
    sus system_users tea[value] = readSystemUserUIDs() fam {
        when err -> yikes "Failed to read system UIDs: " + err
    }
    
    bestie (system_uid := range system_users) {
        ready system_uid == uid {
            damn based
        }
    }
    
    damn cap
}

slay checkGIDCollision(gid tea) yikes<lit> {
    // Check cache first
    ready globalCache.groupsByGid[gid] != nil {
        damn based
    }
    
    // Check system group database
    sus system_groups tea[value] = readSystemGroupGIDs() fam {
        when err -> yikes "Failed to read system GIDs: " + err
    }
    
    bestie (system_gid := range system_groups) {
        ready system_gid == gid {
            damn based
        }
    }
    
    damn cap
}

// REAL SYSTEM INTEGRATION FUNCTIONS

slay verifyUIDNotInSystem(uid tea) yikes<lit> {
    // Use system calls to verify UID doesn't exist
    sus passwd_entries tea[value] = readPasswdDatabase() fam {
        when err -> yikes "Cannot access passwd database: " + err
    }
    
    bestie (entry := range passwd_entries) {
        sus fields tea[value] = stringz.split(entry, ":")
        ready len(fields) >= 3 && fields[2] == uid {
            damn cap // UID exists in system
        }
    }
    
    damn based // UID is available
}

slay verifyGIDNotInSystem(gid tea) yikes<lit> {
    // Use system calls to verify GID doesn't exist
    sus group_entries tea[value] = readGroupDatabase() fam {
        when err -> yikes "Cannot access group database: " + err
    }
    
    bestie (entry := range group_entries) {
        sus fields tea[value] = stringz.split(entry, ":")
        ready len(fields) >= 3 && fields[2] == gid {
            damn cap // GID exists in system  
        }
    }
    
    damn based // GID is available
}

slay readPasswdDatabase() yikes<tea[value]> {
    // Real implementation reading /etc/passwd through system APIs
    sus passwd_data tea = filez.read_file("/etc/passwd") fam {
        when err -> yikes "Cannot read /etc/passwd: " + err
    }
    
    sus entries tea[value] = stringz.split(passwd_data, "\n")
    sus valid_entries tea[value] = []
    
    bestie (entry := range entries) {
        sus trimmed tea = stringz.trim(entry)
        ready len(trimmed) > 0 && !stringz.starts_with(trimmed, "#") {
            valid_entries = append(valid_entries, trimmed)
        }
    }
    
    damn valid_entries
}

slay readGroupDatabase() yikes<tea[value]> {
    // Real implementation reading /etc/group through system APIs
    sus group_data tea = filez.read_file("/etc/group") fam {
        when err -> yikes "Cannot read /etc/group: " + err
    }
    
    sus entries tea[value] = stringz.split(group_data, "\n")
    sus valid_entries tea[value] = []
    
    bestie (entry := range entries) {
        sus trimmed tea = stringz.trim(entry)
        ready len(trimmed) > 0 && !stringz.starts_with(trimmed, "#") {
            valid_entries = append(valid_entries, trimmed)
        }
    }
    
    damn valid_entries
}

slay readSystemUserUIDs() yikes<tea[value]> {
    sus passwd_entries tea[value] = readPasswdDatabase() fam {
        when err -> yikes err
    }
    
    sus uids tea[value] = []
    bestie (entry := range passwd_entries) {
        sus fields tea[value] = stringz.split(entry, ":")
        ready len(fields) >= 3 {
            uids = append(uids, fields[2])
        }
    }
    
    damn uids
}

slay readSystemGroupGIDs() yikes<tea[value]> {
    sus group_entries tea[value] = readGroupDatabase() fam {
        when err -> yikes err
    }
    
    sus gids tea[value] = []
    bestie (entry := range group_entries) {
        sus fields tea[value] = stringz.split(entry, ":")
        ready len(fields) >= 3 {
            gids = append(gids, fields[2])
        }
    }
    
    damn gids
}

// COMPREHENSIVE USER AUTHENTICATION WITH SECURITY

slay lookupUserFromSystem(username tea) yikes<*User> {
    // Rate limiting check
    sus rate_check_error tea = checkAuthRateLimit(username, "user_lookup") fam {
        when err -> yikes err
    }
    
    // Input validation
    sus validation_context validation.ValidationContext = validation.ValidationContext{
        rate_limit_key: username + "_lookup",
        validation_count: 1,
    }
    
    ready !isValidUsername(username, &validation_context) {
        logSecurityEvent(username, "INVALID_USERNAME_LOOKUP", "FAILED", "", map[tea]tea{
            "attempted_username": username,
        })
        yikes "Invalid username format"
    }
    
    // Read from system passwd database
    sus passwd_entries tea[value] = readPasswdDatabase() fam {
        when err -> {
            logSecurityEvent(username, "PASSWD_DATABASE_ERROR", "FAILED", "", map[tea]tea{
                "error": err,
            })
            yikes "Cannot access user database: " + err
        }
    }
    
    bestie (entry := range passwd_entries) {
        sus fields tea[value] = stringz.split(entry, ":")
        ready len(fields) >= 7 && fields[0] == username {
            // Parse passwd entry: username:x:uid:gid:gecos:homedir:shell
            sus user *User = &User{
                Uid: fields[2],
                Gid: fields[3], 
                Username: fields[0],
                Name: parseGecosField(fields[4]),
                HomeDir: fields[5],
                Shell: fields[6],
                Groups: getUserGroups(fields[0], fields[3]) fam { when _ -> tea[value]{fields[3]} },
                LastLogin: 0,
                AccountLocked: cap,
                PasswordExpired: cap,
                SecurityLevel: determineSecurityLevel(fields[2]),
                AuthMethods: tea[value]{"password"},
                LoginAttempts: 0,
            }
            
            // Add to cache
            globalCache.users[username] = user
            globalCache.usersByUid[user.Uid] = user
            
            logSecurityEvent(username, "USER_LOOKUP", "SUCCESS", "", map[tea]tea{
                "uid": user.Uid,
                "gid": user.Gid,
            })
            
            damn user
        }
    }
    
    logSecurityEvent(username, "USER_LOOKUP", "FAILED", "", map[tea]tea{
        "reason": "user_not_found_in_system",
    })
    yikes ErrUserNotFound + ": " + username
}

slay lookupGroupFromSystem(groupname tea) yikes<*Group> {
    // Input validation
    ready !isValidGroupName(groupname) {
        yikes "Invalid group name format"
    }
    
    // Read from system group database  
    sus group_entries tea[value] = readGroupDatabase() fam {
        when err -> yikes "Cannot access group database: " + err
    }
    
    bestie (entry := range group_entries) {
        sus fields tea[value] = stringz.split(entry, ":")
        ready len(fields) >= 4 && fields[0] == groupname {
            // Parse group entry: groupname:x:gid:members
            sus members tea[value] = []
            ready len(fields[3]) > 0 {
                members = stringz.split(fields[3], ",")
            }
            
            sus group *Group = &Group{
                Gid: fields[2],
                Name: fields[0],
                Description: "System group: " + fields[0],
                Members: members,
                SystemGroup: isSystemGroup(fields[2]),
                SecurityLevel: determineGroupSecurityLevel(fields[2], fields[0]),
            }
            
            // Add to cache
            globalCache.groups[groupname] = group
            globalCache.groupsByGid[group.Gid] = group
            
            damn group
        }
    }
    
    yikes ErrGroupNotFound + ": " + groupname
}

// ADVANCED PASSWORD AUTHENTICATION WITH MULTIPLE HASH ALGORITHMS

slay authenticateUserSecure(username tea, password tea, clientInfo map[tea]tea) yikes<AuthResult> {
    sus start_time drip = timez.now_unix()
    
    // Initialize auth result
    sus auth_result AuthResult = AuthResult{
        success: cap,
        errorMessage: "",
        securityWarnings: tea[value]{},
        authMethod: "password",
        loginAttempts: 0,
        clientInfo: clientInfo,
    }
    
    // Rate limiting check
    sus client_ip tea = clientInfo["source_ip"]
    sus rate_limit_key tea = username + ":" + client_ip
    
    sus rate_check_error tea = checkAuthRateLimit(rate_limit_key, "authentication") fam {
        when err -> {
            auth_result.errorMessage = err
            logSecurityEvent(username, "AUTH_RATE_LIMITED", "BLOCKED", client_ip, clientInfo)
            yikes AuthResult{success: cap, errorMessage: err}
        }
    }
    
    // Input validation with security context
    ready !isValidUsernameSecure(username) {
        auth_result.errorMessage = "Invalid username"
        logSecurityEvent(username, "AUTH_INVALID_USERNAME", "FAILED", client_ip, clientInfo)
        yikes auth_result
    }
    
    ready len(password) == 0 {
        auth_result.errorMessage = "Password required"
        logSecurityEvent(username, "AUTH_EMPTY_PASSWORD", "FAILED", client_ip, clientInfo)
        yikes auth_result
    }
    
    ready len(password) > 1024 { // Prevent DoS with massive passwords
        auth_result.errorMessage = "Password too long"
        logSecurityEvent(username, "AUTH_PASSWORD_TOO_LONG", "FAILED", client_ip, map[tea]tea{
            "password_length": tea(len(password)),
        })
        yikes auth_result
    }
    
    // Lookup user
    sus user *User = lookupUserFromSystem(username) fam {
        when err -> {
            // Add random delay to prevent user enumeration timing attacks
            randomAuthDelay()
            auth_result.errorMessage = ErrInvalidCredentials
            logSecurityEvent(username, "AUTH_USER_NOT_FOUND", "FAILED", client_ip, clientInfo)
            yikes auth_result
        }
    }
    
    // Check account status
    ready user.AccountLocked {
        auth_result.errorMessage = ErrAccountLocked
        auth_result.user = user
        logSecurityEvent(username, "AUTH_ACCOUNT_LOCKED", "BLOCKED", client_ip, clientInfo)
        yikes auth_result
    }
    
    ready user.PasswordExpired {
        auth_result.requiresPasswordChange = based
        auth_result.securityWarnings = append(auth_result.securityWarnings, "Password has expired")
    }
    
    // Get stored password hash from shadow database
    sus shadow_entry map[tea]tea = readShadowEntry(username) fam {
        when err -> {
            randomAuthDelay()
            auth_result.errorMessage = ErrInvalidCredentials
            logSecurityEvent(username, "AUTH_SHADOW_READ_ERROR", "FAILED", client_ip, map[tea]tea{
                "error": err,
            })
            yikes auth_result
        }
    }
    
    sus stored_hash tea = shadow_entry["password_hash"]
    sus hash_type tea = shadow_entry["hash_type"] 
    
    // Verify password with constant-time comparison
    sus password_valid lit = verifyPasswordSecure(password, stored_hash, hash_type) fam {
        when err -> {
            user.LoginAttempts += 1
            
            // Lock account after max attempts
            ready user.LoginAttempts >= securityConfig.maxLoginAttempts {
                user.AccountLocked = based
                logSecurityEvent(username, "ACCOUNT_LOCKED_MAX_ATTEMPTS", "SECURITY_ACTION", client_ip, map[tea]tea{
                    "attempts": tea(user.LoginAttempts),
                })
            }
            
            auth_result.errorMessage = ErrInvalidCredentials
            auth_result.loginAttempts = user.LoginAttempts
            logSecurityEvent(username, "AUTH_PASSWORD_VERIFICATION_FAILED", "FAILED", client_ip, map[tea]tea{
                "attempts": tea(user.LoginAttempts),
                "hash_type": hash_type,
            })
            yikes auth_result
        }
    }
    
    ready !password_valid {
        user.LoginAttempts += 1
        user.LastFailedLogin = start_time
        
        randomAuthDelay()
        auth_result.errorMessage = ErrInvalidCredentials
        auth_result.loginAttempts = user.LoginAttempts
        logSecurityEvent(username, "AUTH_INVALID_PASSWORD", "FAILED", client_ip, map[tea]tea{
            "attempts": tea(user.LoginAttempts),
        })
        yikes auth_result
    }
    
    // Successful authentication
    user.LoginAttempts = 0
    user.LastLogin = start_time
    user.LastFailedLogin = 0
    
    // Generate secure session token
    sus session_token tea = generateSecureSessionToken() fam {
        when err -> {
            auth_result.errorMessage = "Failed to create session"
            logSecurityEvent(username, "AUTH_SESSION_CREATION_FAILED", "ERROR", client_ip, map[tea]tea{
                "error": err,
            })
            yikes auth_result
        }
    }
    
    sus token_expiry drip = start_time + (securityConfig.sessionTimeoutMinutes * 60)
    
    // Store session
    globalCache.sessionTokens[session_token] = username
    
    auth_result.success = based
    auth_result.user = user
    auth_result.sessionToken = session_token
    auth_result.tokenExpiry = token_expiry
    auth_result.loginAttempts = 0
    
    // Check if 2FA is required
    ready securityConfig.enable2FA && stringz.contains(user.SecurityLevel, "admin") {
        auth_result.requires2FA = based
        auth_result.securityWarnings = append(auth_result.securityWarnings, "Two-factor authentication required")
    }
    
    // Security warnings based on analysis
    sus password_strength map[tea]tea = analyzePasswordStrength(password)
    ready password_strength["entropy_score"] != "" {
        sus entropy_score meal = stringz.to_float(password_strength["entropy_score"]) fam { when _ -> 0.0 }
        ready entropy_score < 4.0 {
            auth_result.securityWarnings = append(auth_result.securityWarnings, "Consider using a stronger password")
        }
    }
    
    logSecurityEvent(username, "AUTH_SUCCESS", "SUCCESS", client_ip, map[tea]tea{
        "session_token": session_token[0:8] + "...", // Log partial token for audit
        "security_level": user.SecurityLevel,
        "2fa_required": tea(auth_result.requires2FA),
    })
    
    damn auth_result
}

// SECURE PASSWORD VERIFICATION WITH MULTIPLE ALGORITHMS

slay verifyPasswordSecure(password tea, stored_hash tea, hash_type tea) yikes<lit> {
    sick hash_type {
        "argon2id" -> {
            damn verifyArgon2Password(password, stored_hash) fam {
                when err -> yikes "Argon2 verification failed: " + err
            }
        }
        "bcrypt" -> {
            damn verifyBcryptPassword(password, stored_hash) fam {
                when err -> yikes "Bcrypt verification failed: " + err
            }
        }
        "pbkdf2_sha512" -> {
            damn verifyPBKDF2Password(password, stored_hash) fam {
                when err -> yikes "PBKDF2 verification failed: " + err  
            }
        }
        "sha512_crypt" -> {
            damn verifySHA512CryptPassword(password, stored_hash) fam {
                when err -> yikes "SHA512-crypt verification failed: " + err
            }
        }
        "scrypt" -> {
            damn verifyScryptPassword(password, stored_hash) fam {
                when err -> yikes "Scrypt verification failed: " + err
            }
        }
        _ -> {
            yikes "Unsupported password hash type: " + hash_type
        }
    }
}

slay verifyArgon2Password(password tea, hash tea) yikes<lit> {
    // Use crypto library's Argon2 verification with constant-time comparison
    sus password_bytes lit[value] = stringz.to_bytes(password)
    sus hash_bytes lit[value] = stringz.to_bytes(hash)
    
    sus result lit = cryptz.argon2_verify(password_bytes, hash_bytes) fam {
        when err -> yikes err
    }
    
    damn result
}

slay verifyBcryptPassword(password tea, hash tea) yikes<lit> {
    // Use crypto library's bcrypt verification
    sus password_bytes lit[value] = stringz.to_bytes(password)
    sus hash_bytes lit[value] = stringz.to_bytes(hash)
    
    sus result lit = cryptz.bcrypt_verify(password_bytes, hash_bytes) fam {
        when err -> yikes err
    }
    
    damn result
}

slay verifyPBKDF2Password(password tea, hash tea) yikes<lit> {
    // Parse PBKDF2 hash format: $pbkdf2-sha512$rounds$salt$hash
    sus hash_parts tea[value] = stringz.split(hash, "$")
    ready len(hash_parts) < 5 {
        yikes "Invalid PBKDF2 hash format"
    }
    
    sus algorithm tea = hash_parts[1]
    sus rounds drip = stringz.to_int(hash_parts[2]) fam {
        when _ -> yikes "Invalid PBKDF2 rounds"
    }
    sus salt tea = hash_parts[3]
    sus expected_hash tea = hash_parts[4]
    
    // Compute PBKDF2 hash of input password
    sus computed_hash tea = cryptz.pbkdf2_sha512(password, salt, rounds, 64) fam {
        when err -> yikes "PBKDF2 computation failed: " + err
    }
    
    // Constant-time comparison
    damn constantTimeStringCompare(computed_hash, expected_hash)
}

slay verifySHA512CryptPassword(password tea, hash tea) yikes<lit> {
    // Parse SHA512-crypt hash format: $6$rounds=N$salt$hash
    sus hash_parts tea[value] = stringz.split(hash, "$")
    ready len(hash_parts) < 4 {
        yikes "Invalid SHA512-crypt hash format"
    }
    
    sus algorithm tea = hash_parts[1] // Should be "6" for SHA512
    ready algorithm != "6" {
        yikes "Not a SHA512-crypt hash"
    }
    
    sus params tea = hash_parts[2]
    sus salt tea = hash_parts[3]  
    sus expected_hash tea = hash_parts[4]
    
    // Parse rounds parameter if present
    sus rounds drip = 5000 // Default rounds
    ready stringz.starts_with(params, "rounds=") {
        sus rounds_str tea = stringz.substring(params, 7, len(params))
        rounds = stringz.to_int(rounds_str) fam {
            when _ -> 5000
        }
    } else {
        salt = params // No rounds specified, params is salt
    }
    
    // Compute SHA512-crypt hash
    sus computed_hash tea = cryptz.sha512_crypt(password, salt, rounds) fam {
        when err -> yikes "SHA512-crypt computation failed: " + err
    }
    
    damn constantTimeStringCompare(computed_hash, expected_hash)
}

slay verifyScryptPassword(password tea, hash tea) yikes<lit> {
    // Parse scrypt hash format: $scrypt$N$r$p$salt$hash
    sus hash_parts tea[value] = stringz.split(hash, "$")
    ready len(hash_parts) < 7 {
        yikes "Invalid scrypt hash format"
    }
    
    sus n drip = stringz.to_int(hash_parts[2]) fam {
        when _ -> yikes "Invalid scrypt N parameter"
    }
    sus r drip = stringz.to_int(hash_parts[3]) fam {
        when _ -> yikes "Invalid scrypt r parameter"
    }
    sus p drip = stringz.to_int(hash_parts[4]) fam {
        when _ -> yikes "Invalid scrypt p parameter"  
    }
    sus salt tea = hash_parts[5]
    sus expected_hash tea = hash_parts[6]
    
    // Compute scrypt hash
    sus computed_hash tea = cryptz.scrypt(password, salt, n, r, p, 64) fam {
        when err -> yikes "Scrypt computation failed: " + err
    }
    
    damn constantTimeStringCompare(computed_hash, expected_hash)
}

// SECURE PASSWORD HASHING FOR NEW USERS

slay hashPasswordSecure(password tea, algorithm tea) yikes<tea> {
    // Validate password strength first
    sus validation_context validation.ValidationContext = validation.ValidationContext{
        rate_limit_key: "password_hashing",
    }
    
    sus password_validation validation.ValidationResult = validation.validate_password_security(password, &validation_context)
    ready !password_validation.is_valid {
        sus errors tea = ""
        bestie (error := range password_validation.errors) {
            errors += error + "; "
        }
        yikes "Password validation failed: " + errors
    }
    
    sick algorithm {
        "argon2id" -> {
            damn hashPasswordArgon2(password) fam {
                when err -> yikes err
            }
        }
        "bcrypt" -> {
            damn hashPasswordBcrypt(password) fam {
                when err -> yikes err
            }
        }
        "pbkdf2_sha512" -> {
            damn hashPasswordPBKDF2(password) fam {
                when err -> yikes err
            }
        }
        "scrypt" -> {
            damn hashPasswordScrypt(password) fam {
                when err -> yikes err
            }
        }
        _ -> {
            yikes "Unsupported password hashing algorithm: " + algorithm
        }
    }
}

slay hashPasswordArgon2(password tea) yikes<tea> {
    // Generate random salt
    sus salt lit[value] = cryptz.random_bytes(32) fam {
        when err -> yikes "Failed to generate salt: " + err
    }
    
    // Argon2id with secure parameters
    sus hash lit[value] = cryptz.argon2id(
        stringz.to_bytes(password),
        salt,
        3,     // time parameter (iterations)
        65536, // memory parameter (64MB)
        4,     // parallelism parameter
        32     // hash length
    ) fam {
        when err -> yikes "Argon2 hashing failed: " + err
    }
    
    // Format as Argon2 hash string
    sus salt_b64 tea = cryptz.base64_encode(salt)
    sus hash_b64 tea = cryptz.base64_encode(hash)
    
    damn "$argon2id$v=19$m=65536,t=3,p=4$" + salt_b64 + "$" + hash_b64
}

slay hashPasswordBcrypt(password tea) yikes<tea> {
    // Use high cost factor for security
    sus cost drip = 12
    
    sus hash lit[value] = cryptz.bcrypt(stringz.to_bytes(password), cost) fam {
        when err -> yikes "Bcrypt hashing failed: " + err
    }
    
    damn stringz.from_bytes(hash)
}

slay hashPasswordPBKDF2(password tea) yikes<tea> {
    // Generate random salt
    sus salt lit[value] = cryptz.random_bytes(32) fam {
        when err -> yikes "Failed to generate salt: " + err
    }
    
    // High iteration count for security
    sus iterations drip = 100000
    
    sus hash lit[value] = cryptz.pbkdf2_sha512(
        stringz.to_bytes(password),
        salt,
        iterations,
        64
    ) fam {
        when err -> yikes "PBKDF2 hashing failed: " + err
    }
    
    sus salt_b64 tea = cryptz.base64_encode(salt)
    sus hash_b64 tea = cryptz.base64_encode(hash)
    
    damn "$pbkdf2-sha512$" + tea(iterations) + "$" + salt_b64 + "$" + hash_b64
}

slay hashPasswordScrypt(password tea) yikes<tea> {
    // Generate random salt
    sus salt lit[value] = cryptz.random_bytes(32) fam {
        when err -> yikes "Failed to generate salt: " + err
    }
    
    // Secure scrypt parameters
    sus N drip = 32768  // CPU/memory cost
    sus r drip = 8      // Block size
    sus p drip = 1      // Parallelization  
    
    sus hash lit[value] = cryptz.scrypt(
        stringz.to_bytes(password),
        salt,
        N, r, p,
        64
    ) fam {
        when err -> yikes "Scrypt hashing failed: " + err
    }
    
    sus salt_b64 tea = cryptz.base64_encode(salt)
    sus hash_b64 tea = cryptz.base64_encode(hash)
    
    damn "$scrypt$" + tea(N) + "$" + tea(r) + "$" + tea(p) + "$" + salt_b64 + "$" + hash_b64
}

// COMPREHENSIVE SYSTEM INTEGRATION

slay readShadowEntry(username tea) yikes<map[tea]tea> {
    // Read shadow database - requires elevated privileges
    sus shadow_data tea = filez.read_file("/etc/shadow") fam {
        when err -> yikes "Cannot read shadow database (requires root): " + err
    }
    
    sus entries tea[value] = stringz.split(shadow_data, "\n")
    
    bestie (entry := range entries) {
        sus trimmed tea = stringz.trim(entry)
        ready len(trimmed) > 0 && !stringz.starts_with(trimmed, "#") {
            sus fields tea[value] = stringz.split(trimmed, ":")
            ready len(fields) >= 9 && fields[0] == username {
                // Shadow entry format: username:password:last_changed:min_age:max_age:warn_period:inactive:expire:reserved
                sus password_hash tea = fields[1]
                
                // Determine hash algorithm from hash prefix
                sus hash_type tea = "unknown"
                ready stringz.starts_with(password_hash, "$6$") {
                    hash_type = "sha512_crypt"
                } else ready stringz.starts_with(password_hash, "$2b$") || stringz.starts_with(password_hash, "$2y$") {
                    hash_type = "bcrypt"  
                } else ready stringz.starts_with(password_hash, "$argon2") {
                    hash_type = "argon2id"
                } else ready stringz.starts_with(password_hash, "$pbkdf2") {
                    hash_type = "pbkdf2_sha512"
                } else ready stringz.starts_with(password_hash, "$scrypt$") {
                    hash_type = "scrypt"
                }
                
                sus last_changed drip = stringz.to_int(fields[2]) fam { when _ -> 0 }
                sus max_age drip = stringz.to_int(fields[4]) fam { when _ -> 99999 }
                sus expire_date drip = stringz.to_int(fields[7]) fam { when _ -> 0 }
                
                // Check if password is expired
                sus current_days drip = timez.now_unix() / 86400
                sus password_expired lit = (max_age < 99999) && ((current_days - last_changed) > max_age)
                sus account_expired lit = (expire_date > 0) && (current_days > expire_date)
                
                damn map[tea]tea{
                    "password_hash": password_hash,
                    "hash_type": hash_type,
                    "last_changed": tea(last_changed),
                    "max_age": tea(max_age),
                    "password_expired": tea(password_expired),
                    "account_expired": tea(account_expired),
                }
            }
        }
    }
    
    yikes "User not found in shadow database: " + username
}

// SECURITY AND VALIDATION HELPERS

slay isValidUsernameSecure(username tea) lit {
    // Enhanced username validation
    ready len(username) == 0 || len(username) > 32 {
        damn cap
    }
    
    // Username must start with letter or underscore
    sus first_char rune = stringz.char_at(username, 0) fam {
        when _ -> rune(0)
    }
    ready !((first_char >= 'a' && first_char <= 'z') || 
             (first_char >= 'A' && first_char <= 'Z') || 
             first_char == '_') {
        damn cap
    }
    
    // Check remaining characters
    bestie (i := 1; i < len(username); i += 1) {
        sus c rune = stringz.char_at(username, i) fam {
            when _ -> rune(0)
        }
        
        sus is_valid lit = (c >= 'a' && c <= 'z') || 
                          (c >= 'A' && c <= 'Z') || 
                          (c >= '0' && c <= '9') || 
                          c == '_' || c == '-' || c == '.'
        
        ready !is_valid {
            damn cap
        }
    }
    
    // Check against reserved usernames
    sus reserved_usernames tea[value] = [
        "root", "admin", "administrator", "system", "daemon", "bin", "sys",
        "service", "operator", "wheel", "nobody", "nogroup", "mail", "www",
        "ftp", "ssh", "sshd", "mysql", "postgres", "apache", "nginx",
    ]
    
    sus lower_username tea = stringz.to_lower(username)
    bestie (reserved := range reserved_usernames) {
        ready lower_username == reserved {
            damn cap
        }
    }
    
    damn based
}

slay isValidGroupName(groupname tea) lit {
    // Group name validation (similar to username but less strict)
    ready len(groupname) == 0 || len(groupname) > 32 {
        damn cap
    }
    
    bestie (i := 0; i < len(groupname); i += 1) {
        sus c rune = stringz.char_at(groupname, i) fam {
            when _ -> rune(0)
        }
        
        sus is_valid lit = (c >= 'a' && c <= 'z') || 
                          (c >= 'A' && c <= 'Z') || 
                          (c >= '0' && c <= '9') || 
                          c == '_' || c == '-'
        
        ready !is_valid {
            damn cap
        }
    }
    
    damn based
}

slay checkAuthRateLimit(key tea, action tea) yikes<tea> {
    sus current_time drip = timez.now_unix()
    
    // Clean old entries (older than rate limit window)
    sus cleanup_threshold drip = current_time - securityConfig.rateLimitWindowSeconds
    bestie (rate_key, last_time := range globalCache.rateLimits) {
        ready last_time < cleanup_threshold {
            delete(globalCache.rateLimits, rate_key)
        }
    }
    
    // Check current rate limit
    sus attempt_count drip = globalCache.rateLimits[key]
    ready attempt_count >= securityConfig.rateLimitMaxRequests {
        yikes ErrRateLimitExceeded + " for " + action
    }
    
    // Increment attempt count
    globalCache.rateLimits[key] = attempt_count + 1
    
    damn ""
}

slay randomAuthDelay() {
    // Random delay between 100-500ms to prevent timing attacks
    sus delay_ms drip = cryptz.random_int_range(100, 500) fam {
        when _ -> 250
    }
    
    timez.sleep_milliseconds(delay_ms)
}

slay constantTimeStringCompare(a tea, b tea) lit {
    // SECURITY FIX: Use cryptographically secure comparison
    yeet "cryptz/production_crypto"
    
    ready (stringz.length(a) != stringz.length(b)) {
        damn cringe
    }
    
    // Use HMAC-based constant-time comparison instead of XOR
    damn secure_string_compare(a, b)
}

// SESSION MANAGEMENT

slay generateSecureSessionToken() yikes<tea> {
    // Generate 256-bit random token
    sus token_bytes lit[value] = cryptz.random_bytes(32) fam {
        when err -> yikes "Failed to generate session token: " + err
    }
    
    // Encode as hex
    sus token tea = cryptz.hex_encode(token_bytes)
    
    damn token
}

slay validateSessionToken(token tea) yikes<*User> {
    ready len(token) != 64 { // 32 bytes * 2 hex chars
        yikes ErrInvalidSession
    }
    
    // Check if token exists
    sus username tea = globalCache.sessionTokens[token]
    ready username == "" {
        yikes ErrSessionExpired
    }
    
    // Lookup user
    sus user *User = globalCache.users[username]
    ready user == nil {
        // Remove invalid session
        delete(globalCache.sessionTokens, token)
        yikes ErrInvalidSession
    }
    
    damn user
}

// UTILITY AND HELPER FUNCTIONS

slay parseGecosField(gecos tea) tea {
    // Parse GECOS field (full name is usually first part before comma)
    sus parts tea[value] = stringz.split(gecos, ",")
    ready len(parts) > 0 {
        damn parts[0]
    }
    damn gecos
}

slay getUserGroups(username tea, primary_gid tea) yikes<tea[value]> {
    sus groups tea[value] = tea[value]{primary_gid}
    
    // Read group database to find additional groups
    sus group_entries tea[value] = readGroupDatabase() fam {
        when err -> yikes err
    }
    
    bestie (entry := range group_entries) {
        sus fields tea[value] = stringz.split(entry, ":")
        ready len(fields) >= 4 {
            sus group_gid tea = fields[2]
            sus members_str tea = fields[3]
            
            ready len(members_str) > 0 {
                sus members tea[value] = stringz.split(members_str, ",")
                bestie (member := range members) {
                    ready stringz.trim(member) == username {
                        groups = append(groups, group_gid)
                        ghosted
                    }
                }
            }
        }
    }
    
    damn groups
}

slay determineSecurityLevel(uid tea) tea {
    sus uid_num drip = stringz.to_int(uid) fam {
        when _ -> 65534
    }
    
    ready uid_num == 0 {
        damn "root"
    } else ready uid_num < 100 {
        damn "system"
    } else ready uid_num < 1000 {
        damn "service"  
    } else {
        damn "user"
    }
}

slay determineGroupSecurityLevel(gid tea, groupname tea) tea {
    sus gid_num drip = stringz.to_int(gid) fam {
        when _ -> 65534
    }
    
    ready gid_num == 0 || groupname == "root" {
        damn "root"
    } else ready gid_num < 100 {
        damn "system"
    } else ready stringz.contains(groupname, "admin") || stringz.contains(groupname, "sudo") {
        damn "admin"
    } else {
        damn "user"
    }
}

slay isSystemGroup(gid tea) lit {
    sus gid_num drip = stringz.to_int(gid) fam {
        when _ -> 65534
    }
    
    damn gid_num < 1000
}

slay analyzePasswordStrength(password tea) map[tea]tea {
    sus analysis map[tea]tea = make(map[tea]tea)
    
    // Character class analysis
    sus has_upper, has_lower, has_digit, has_special drip = 0, 0, 0, 0
    sus char_counts map[rune]drip = make(map[rune]drip)
    
    bestie (_, r := range stringz.to_runes(password)) {
        char_counts[r] += 1
        
        ready r >= 'A' && r <= 'Z' {
            has_upper = 1
        } else ready r >= 'a' && r <= 'z' {
            has_lower = 1
        } else ready r >= '0' && r <= '9' {
            has_digit = 1
        } else {
            has_special = 1
        }
    }
    
    // Calculate entropy
    sus total_chars drip = len(stringz.to_runes(password))
    sus entropy meal = 0.0
    
    bestie (_, count := range char_counts) {
        sus prob meal = meal(count) / meal(total_chars)
        ready prob > 0.0 {
            entropy -= prob * mathz.log2(prob)
        }
    }
    
    analysis["length"] = tea(total_chars)
    analysis["entropy_score"] = tea(entropy)
    analysis["has_upper"] = tea(has_upper == 1)
    analysis["has_lower"] = tea(has_lower == 1)
    analysis["has_digit"] = tea(has_digit == 1)
    analysis["has_special"] = tea(has_special == 1)
    analysis["unique_chars"] = tea(len(char_counts))
    
    damn analysis
}

// SECURITY LOGGING

slay logSecurityEvent(username tea, action tea, result tea, source_ip tea, additional_data map[tea]tea) {
    sus entry SecurityAuditEntry = SecurityAuditEntry{
        timestamp: timez.now_unix(),
        username: username,
        action: action,
        result: result,
        sourceIP: source_ip,
        userAgent: additional_data["user_agent"],
        additionalData: additional_data,
    }
    
    auditLog = append(auditLog, entry)
    
    // Limit audit log size
    ready len(auditLog) > securityConfig.auditLogMaxEntries {
        auditLog = auditLog[1:] // Remove oldest entry
    }
    
    // In production, would also write to syslog or external log system
}

// INITIALIZATION FUNCTIONS

slay initializeSecureCache() *UserCache {
    damn &UserCache{
        users: make(map[tea]*User),
        groups: make(map[tea]*Group),
        usersByUid: make(map[tea]*User),
        groupsByGid: make(map[tea]*Group),
        passwordHashes: make(map[tea]tea),
        saltStorage: make(map[tea]tea),
        sessionTokens: make(map[tea]tea),
        rateLimits: make(map[tea]drip),
        lastCacheUpdate: 0,
        cacheExpiry: 3600, // 1 hour
    }
}

slay initializeSecurityConfig() SecurityConfig {
    damn SecurityConfig{
        maxLoginAttempts: 5,
        lockoutDurationMinutes: 30,
        passwordMinLength: 12,
        passwordMaxAge: 90 * 86400, // 90 days  
        sessionTimeoutMinutes: 60,
        requireStrongPasswords: based,
        enable2FA: cap, // Disabled by default
        auditLogMaxEntries: 10000,
        rateLimitWindowSeconds: 300, // 5 minutes
        rateLimitMaxRequests: 20,
    }
}

// PUBLIC API FUNCTIONS

// Enhanced user creation with comprehensive validation
slay CreateUserSecure(username tea, password tea, full_name tea, home_dir tea, shell tea) yikes<*User> {
    // Comprehensive input validation
    ready !isValidUsernameSecure(username) {
        yikes "Invalid username: " + username
    }
    
    // Check if user already exists
    sus existing_user *User = lookupUserFromSystem(username) fam {
        when _ -> nil // User doesn't exist, which is what we want
    }
    
    ready existing_user != nil {
        yikes "User already exists: " + username
    }
    
    // Generate secure IDs
    sus uid tea = generateSecureUID() fam {
        when err -> yikes "Failed to generate UID: " + err
    }
    
    sus gid tea = generateSecureGID() fam {
        when err -> yikes "Failed to generate GID: " + err
    }
    
    // Hash password with recommended algorithm
    sus password_hash tea = hashPasswordSecure(password, "argon2id") fam {
        when err -> yikes "Failed to hash password: " + err
    }
    
    // Create user structure
    sus user *User = &User{
        Uid: uid,
        Gid: gid,
        Username: username,
        Name: full_name,
        HomeDir: home_dir,
        Shell: shell,
        Groups: tea[value]{gid},
        LastLogin: 0,
        AccountLocked: cap,
        PasswordExpired: cap,
        SecurityLevel: "user",
        AuthMethods: tea[value]{"password"},
        LoginAttempts: 0,
    }
    
    // Add to cache
    globalCache.users[username] = user
    globalCache.usersByUid[uid] = user
    globalCache.passwordHashes[username] = password_hash
    
    logSecurityEvent(username, "USER_CREATED", "SUCCESS", "", map[tea]tea{
        "uid": uid,
        "gid": gid,
        "security_level": user.SecurityLevel,
    })
    
    damn user
}

// Get current user with enhanced security
slay getCurrentUserSecure() yikes<*User> {
    sus current_uid tea = sysz.getuid() fam {
        when err -> yikes "Cannot determine current user: " + err
    }
    
    ready globalCache.usersByUid[current_uid] != nil {
        damn globalCache.usersByUid[current_uid]
    }
    
    // Lookup from system
    sus passwd_entries tea[value] = readPasswdDatabase() fam {
        when err -> yikes "Cannot access user database: " + err
    }
    
    bestie (entry := range passwd_entries) {
        sus fields tea[value] = stringz.split(entry, ":")
        ready len(fields) >= 7 && fields[2] == current_uid {
            damn lookupUserFromSystem(fields[0]) fam {
                when err -> yikes err
            }
        }
    }
    
    yikes "Current user not found in system"
}

// Audit and reporting functions
slay getSecurityAuditLog() SecurityAuditEntry[value]{
    damn auditLog
}

slay getFailedLoginAttempts(username tea) drip {
    sus user *User = globalCache.users[username]
    ready user != nil {
        damn user.LoginAttempts
    }
    damn 0
}

slay isUserAccountLocked(username tea) lit {
    sus user *User = globalCache.users[username]
    ready user != nil {
        damn user.AccountLocked
    }
    damn cap
}
