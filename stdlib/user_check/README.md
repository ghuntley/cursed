# user_check

Comprehensive user and group management system for CURSED applications. Provides user authentication, group membership checking, and system user information with caching and security features.

## Overview

The `user_check` module provides:
- User account lookup and management
- Group membership verification
- Current user identification
- System user detection
- User creation and deletion (mock implementations)
- Caching for performance optimization
- Comprehensive user/group search functionality

## Core Structures

### User Representation

#### `User`
Complete user account information.

```cursed
be_like User squad {
    Uid tea          // User ID
    Gid tea          // Primary group ID
    Username tea     // Username
    Name tea         // Full name
    HomeDir tea      // Home directory path
}
```

#### User Methods

#### `(u *User) GroupIds() -> ([]tea, tea)`
Returns list of group IDs the user belongs to.

#### `(u *User) Groups() -> ([]*Group, tea)`
Returns list of Group objects the user belongs to.

#### `(u *User) IsInGroup(groupName: tea) -> (lit, tea)`
Checks if user is member of specified group.

#### `(u *User) IsRoot() -> lit`
Checks if user is root (UID 0).

#### `(u *User) IsSystem() -> lit`
Checks if user is a system user (UID < 1000 or special names).

#### `(u *User) EffectiveUid() -> tea`
Returns effective user ID (same as UID in this implementation).

#### `(u *User) EffectiveGid() -> tea`
Returns effective group ID (same as GID in this implementation).

### Group Representation

#### `Group`
Group account information.

```cursed
be_like Group squad {
    Gid tea     // Group ID
    Name tea    // Group name
}
```

#### Group Methods

#### `(g *Group) Members() -> ([]*User, tea)`
Returns list of users who are members of this group.

#### `(g *Group) HasMember(username: tea) -> (lit, tea)`
Checks if specified user is a member of this group.

## Core Functions

### Current User Operations

#### `Current() -> (*User, tea)`
Gets current user information.

**Returns:** Tuple of (User pointer, error message)

**Example:**
```cursed
yeet "user_check"

(sus current_user *User, sus err tea) = Current()
lowkey err == "" {
    vibez.spill("Current user: " + current_user.Username)
    vibez.spill("Home directory: " + current_user.HomeDir)
    vibez.spill("Is root: " + string(current_user.IsRoot()))
} yikes {
    vibez.spill("Error getting current user: " + err)
}
```

### User Lookup

#### `Lookup(username: tea) -> (*User, tea)`
Looks up user by username.

**Parameters:**
- `username`: Username to find

**Returns:** Tuple of (User pointer, error message)

**Supported Users:**
- `"root"`: System root user (UID 0)
- `"user"`: Regular user (UID 1000)
- `"daemon"`: System daemon user (UID 1)
- `"bin"`: System bin user (UID 2)

#### `LookupId(uid: tea) -> (*User, tea)`
Looks up user by user ID.

**Parameters:**
- `uid`: User ID as string

**Returns:** Tuple of (User pointer, error message)

**Example:**
```cursed
// Look up user by name
(sus user *User, sus err tea) = Lookup("root")
lowkey err == "" {
    vibez.spill("Found user: " + user.Name)
}

// Look up user by UID
(sus user_by_id *User, sus err tea) = LookupId("1000")
lowkey err == "" {
    vibez.spill("User with UID 1000: " + user_by_id.Username)
}
```

### Group Lookup

#### `LookupGroup(name: tea) -> (*Group, tea)`
Looks up group by name.

**Parameters:**
- `name`: Group name to find

**Returns:** Tuple of (Group pointer, error message)

**Supported Groups:**
- `"root"`: Root group (GID 0)
- `"wheel"`: Administrative group (GID 10)
- `"users"`: Standard users group (GID 1000)
- `"admin"`: Admin group (GID 20)
- `"staff"`: Staff group (GID 50)

#### `LookupGroupId(gid: tea) -> (*Group, tea)`
Looks up group by group ID.

**Parameters:**
- `gid`: Group ID as string

**Example:**
```cursed
// Look up group by name
(sus group *Group, sus err tea) = LookupGroup("wheel")
lowkey err == "" {
    vibez.spill("Found group: " + group.Name + " (GID: " + group.Gid + ")")
}

// Check group membership
(sus members []*User, sus err tea) = group.Members()
lowkey err == "" {
    vibez.spill("Group has " + string(len(members)) + " members")
}
```

## Enhanced Functions

### Bulk Operations

#### `GetAllUsers() -> ([]*User, tea)`
Returns all known users in the system.

#### `GetAllGroups() -> ([]*Group, tea)`
Returns all known groups in the system.

**Example:**
```cursed
// List all users
(sus all_users []*User, sus err tea) = GetAllUsers()
lowkey err == "" {
    vibez.spill("System has " + string(len(all_users)) + " users:")
    bestie i := 0; i < len(all_users); i = i + 1 {
        sus user *User = all_users[i]
        vibez.spill("  " + user.Username + " (" + user.Uid + ")")
    }
}

// List all groups
(sus all_groups []*Group, sus err tea) = GetAllGroups()
lowkey err == "" {
    bestie i := 0; i < len(all_groups); i = i + 1 {
        sus group *Group = all_groups[i]
        vibez.spill("Group: " + group.Name + " (" + group.Gid + ")")
    }
}
```

### Existence Checks

#### `UserExists(username: tea) -> lit`
Checks if user exists without retrieving full information.

#### `GroupExists(groupName: tea) -> lit`
Checks if group exists without retrieving full information.

**Example:**
```cursed
// Quick existence checks
lowkey UserExists("admin") {
    vibez.spill("Admin user exists")
} yikes {
    vibez.spill("Admin user not found")
}

lowkey GroupExists("developers") {
    vibez.spill("Developers group exists")
}
```

### Current User Utilities

#### `CurrentUserGroups() -> ([]*Group, tea)`
Gets groups for current user.

#### `IsCurrentUserInGroup(groupName: tea) -> (lit, tea)`
Checks if current user is in specified group.

#### `GetEffectiveUid() -> tea`
Gets effective UID of current user.

#### `GetEffectiveGid() -> tea`
Gets effective GID of current user.

#### `IsCurrentUserRoot() -> lit`
Checks if current user is root.

#### `IsCurrentUserSystem() -> lit`
Checks if current user is a system user.

**Example:**
```cursed
// Current user analysis
lowkey IsCurrentUserRoot() {
    vibez.spill("⚠️  Running as root user")
} yikes if IsCurrentUserSystem() {
    vibez.spill("Running as system user")
} yikes {
    vibez.spill("Running as regular user")
}

// Check group membership
(sus is_admin lit, sus err tea) = IsCurrentUserInGroup("admin")
lowkey err == "" && is_admin {
    vibez.spill("Current user has admin privileges")
}
```

### Directory Operations

#### `GetUserHomeDir(username: tea) -> (tea, tea)`
Gets home directory for specified user.

#### `GetCurrentUserHomeDir() -> (tea, tea)`
Gets home directory for current user.

**Example:**
```cursed
// Get user home directories
(sus home_dir tea, sus err tea) = GetCurrentUserHomeDir()
lowkey err == "" {
    vibez.spill("Current user home: " + home_dir)
}

(sus root_home tea, sus err tea) = GetUserHomeDir("root")
lowkey err == "" {
    vibez.spill("Root home directory: " + root_home)
}
```

## Advanced Features

### User and Group Management

#### `CreateUser(username: tea, fullName: tea, homeDir: tea) -> (*User, tea)`
Creates a new user account (mock implementation).

#### `CreateGroup(groupName: tea) -> (*Group, tea)`
Creates a new group (mock implementation).

#### `DeleteUser(username: tea) -> tea`
Deletes user account (mock implementation).

#### `DeleteGroup(groupName: tea) -> tea`
Deletes group (mock implementation).

**Example:**
```cursed
// Create new user
(sus new_user *User, sus err tea) = CreateUser("alice", "Alice Johnson", "/home/alice")
lowkey err == "" {
    vibez.spill("Created user: " + new_user.Username)
    vibez.spill("UID: " + new_user.Uid)
} yikes {
    vibez.spill("Failed to create user: " + err)
}

// Create new group
(sus new_group *Group, sus err tea) = CreateGroup("developers")
lowkey err == "" {
    vibez.spill("Created group: " + new_group.Name)
}
```

### Group Membership Management

#### `AddUserToGroup(username: tea, groupName: tea) -> tea`
Adds user to group (mock implementation).

#### `RemoveUserFromGroup(username: tea, groupName: tea) -> tea`
Removes user from group (mock implementation).

**Example:**
```cursed
// Manage group membership
sus err tea = AddUserToGroup("alice", "developers")
lowkey err == "" {
    vibez.spill("Added alice to developers group")
}

sus err tea = RemoveUserFromGroup("bob", "admin")
lowkey err == "" {
    vibez.spill("Removed bob from admin group")
}
```

### Search Functionality

#### `SearchUsers(pattern: tea) -> ([]*User, tea)`
Searches users by username or full name pattern.

#### `SearchGroups(pattern: tea) -> ([]*Group, tea)`
Searches groups by name pattern.

**Example:**
```cursed
// Search for users
(sus matching_users []*User, sus err tea) = SearchUsers("admin")
lowkey err == "" {
    vibez.spill("Found " + string(len(matching_users)) + " users matching 'admin'")
    bestie i := 0; i < len(matching_users); i = i + 1 {
        vibez.spill("  " + matching_users[i].Username)
    }
}

// Search for groups
(sus matching_groups []*Group, sus err tea) = SearchGroups("user")
lowkey err == "" {
    bestie i := 0; i < len(matching_groups); i = i + 1 {
        vibez.spill("Group: " + matching_groups[i].Name)
    }
}
```

### Validation

#### `ValidateUsername(username: tea) -> tea`
Validates username format and characters.

#### `ValidateGroupName(groupName: tea) -> tea`
Validates group name format and characters.

**Rules:**
- Length: 1-32 characters
- Allowed characters: a-z, A-Z, 0-9, _, -
- No spaces or special characters

**Example:**
```cursed
// Validate names before creation
sus username tea = "new_user-123"
sus validation_err tea = ValidateUsername(username)
lowkey validation_err == "" {
    vibez.spill("Username is valid")
} yikes {
    vibez.spill("Invalid username: " + validation_err)
}

sus groupname tea = "dev-team"
sus group_validation_err tea = ValidateGroupName(groupname)
lowkey group_validation_err == "" {
    vibez.spill("Group name is valid")
}
```

## Caching System

### Cache Management

#### `ClearCache()`
Clears the internal user/group cache.

#### `GetCacheStats() -> (normie, normie)`
Returns cache statistics: (user_count, group_count).

**Example:**
```cursed
// Check cache performance
(sus user_count normie, sus group_count normie) = GetCacheStats()
vibez.spill("Cache contains " + string(user_count) + " users and " + string(group_count) + " groups")

// Clear cache if needed
ClearCache()
vibez.spill("Cache cleared")
```

### Cache Performance

The module uses internal caching to improve performance:
- **User cache**: Maps username → User and UID → User
- **Group cache**: Maps group name → Group and GID → Group
- **Automatic population**: Cache populated on first lookup
- **Manual control**: Cache can be cleared and repopulated

## System Information

### System Utilities

#### `IsValidUid(uid: tea) -> lit`
Validates UID format (numeric string).

#### `IsValidGid(gid: tea) -> lit`
Validates GID format (numeric string).

#### `GetSystemInfo() -> map[tea]tea`
Returns system information including user/group counts.

**Example:**
```cursed
// System information
sus system_info map[tea]tea = GetSystemInfo()
vibez.spill("Operating System: " + system_info["os"])
vibez.spill("Architecture: " + system_info["arch"])
vibez.spill("User Count: " + system_info["user_count"])
vibez.spill("Group Count: " + system_info["group_count"])

// Validate IDs
lowkey IsValidUid("1000") {
    vibez.spill("Valid UID format")
}

lowkey IsValidGid("admin") {
    vibez.spill("Invalid GID format")
} yikes {
    vibez.spill("GID must be numeric")
}
```

## Usage Examples

### Complete User Management Workflow

```cursed
yeet "user_check"

slay user_management_demo() {
    vibez.spill("=== User Management Demo ===")
    
    // Get current user information
    (sus current_user *User, sus err tea) = Current()
    lowkey err == "" {
        vibez.spill("Current user: " + current_user.Username)
        vibez.spill("UID: " + current_user.Uid + ", GID: " + current_user.Gid)
        vibez.spill("Home: " + current_user.HomeDir)
        
        // Check if current user is privileged
        lowkey current_user.IsRoot() {
            vibez.spill("⚠️  Running with root privileges")
        } else if current_user.IsSystem() {
            vibez.spill("Running as system user")
        } else {
            vibez.spill("Running as regular user")
        }
        
        // Check group memberships
        (sus groups []*Group, sus err tea) = current_user.Groups()
        lowkey err == "" {
            vibez.spill("Group memberships:")
            bestie i := 0; i < len(groups); i = i + 1 {
                vibez.spill("  " + groups[i].Name + " (" + groups[i].Gid + ")")
            }
        }
    }
    
    // List all users in system
    vibez.spill("\n=== All Users ===")
    (sus all_users []*User, sus err tea) = GetAllUsers()
    lowkey err == "" {
        bestie i := 0; i < len(all_users); i = i + 1 {
            sus user *User = all_users[i]
            sus user_type tea = "regular"
            lowkey user.IsRoot() {
                user_type = "root"
            } else if user.IsSystem() {
                user_type = "system"
            }
            
            vibez.spill(user.Username + " (" + user.Uid + ") - " + user_type)
        }
    }
    
    // Demonstrate group management
    vibez.spill("\n=== Group Management ===")
    (sus all_groups []*Group, sus err tea) = GetAllGroups()
    lowkey err == "" {
        bestie i := 0; i < len(all_groups); i = i + 1 {
            sus group *Group = all_groups[i]
            vibez.spill("Group: " + group.Name + " (" + group.Gid + ")")
            
            // Check if current user is in this group
            (sus is_member lit, sus err tea) = IsCurrentUserInGroup(group.Name)
            lowkey err == "" && is_member {
                vibez.spill("  ✓ Current user is a member")
            }
        }
    }
}
```

### Security Audit Function

```cursed
slay security_audit() {
    vibez.spill("=== Security Audit ===")
    
    // Check for privileged users
    (sus all_users []*User, _) = GetAllUsers()
    sus privileged_users []tea = []tea{}
    sus system_users []tea = []tea{}
    
    bestie i := 0; i < len(all_users); i = i + 1 {
        sus user *User = all_users[i]
        
        lowkey user.IsRoot() {
            privileged_users = append(privileged_users, user.Username)
        } else if user.IsSystem() {
            system_users = append(system_users, user.Username)
        }
    }
    
    vibez.spill("Privileged users (" + string(len(privileged_users)) + "):")
    bestie i := 0; i < len(privileged_users); i = i + 1 {
        vibez.spill("  ⚠️  " + privileged_users[i])
    }
    
    vibez.spill("System users (" + string(len(system_users)) + "):")
    bestie i := 0; i < len(system_users); i = i + 1 {
        vibez.spill("  🔧 " + system_users[i])
    }
    
    // Check administrative groups
    sus admin_groups []tea = []tea{"root", "wheel", "admin"}
    bestie i := 0; i < len(admin_groups); i = i + 1 {
        sus group_name tea = admin_groups[i]
        lowkey GroupExists(group_name) {
            (sus group *Group, _) = LookupGroup(group_name)
            (sus members []*User, _) = group.Members()
            vibez.spill("Admin group '" + group_name + "' has " + string(len(members)) + " members")
        }
    }
}
```

### User Creation Workflow

```cursed
slay create_user_workflow(username tea, full_name tea) {
    vibez.spill("Creating user: " + username)
    
    // Validate username
    sus validation_err tea = ValidateUsername(username)
    lowkey validation_err != "" {
        vibez.spill("❌ Invalid username: " + validation_err)
        damn
    }
    
    // Check if user already exists
    lowkey UserExists(username) {
        vibez.spill("❌ User already exists: " + username)
        damn
    }
    
    // Create user
    sus home_dir tea = "/home/" + username
    (sus new_user *User, sus err tea) = CreateUser(username, full_name, home_dir)
    lowkey err == "" {
        vibez.spill("✅ User created successfully:")
        vibez.spill("  Username: " + new_user.Username)
        vibez.spill("  UID: " + new_user.Uid)
        vibez.spill("  Home: " + new_user.HomeDir)
        
        // Add to default group
        sus add_err tea = AddUserToGroup(username, "users")
        lowkey add_err == "" {
            vibez.spill("  Added to 'users' group")
        }
    } yikes {
        vibez.spill("❌ Failed to create user: " + err)
    }
}
```

## Error Handling

### Error Constants

```cursed
const (
    ErrUserNotFound = "user not found"
    ErrGroupNotFound = "group not found"
    ErrInvalidUID = "invalid user ID"
    ErrInvalidGID = "invalid group ID"
    ErrPermissionDenied = "permission denied"
)
```

### Robust Error Handling

```cursed
slay safe_user_lookup(identifier tea, by_name lit) (*User, tea) {
    lowkey by_name {
        // Validate username format
        sus validation_err tea = ValidateUsername(identifier)
        lowkey validation_err != "" {
            damn null, "invalid username format: " + validation_err
        }
        
        damn Lookup(identifier)
    } yikes {
        // Validate UID format
        lowkey !IsValidUid(identifier) {
            damn null, "invalid UID format: " + identifier
        }
        
        damn LookupId(identifier)
    }
}
```

## Testing

### Unit Tests

```bash
# Run user check tests
zig build test
./zig-out/bin/cursed-zig stdlib/user_check/test_user_check.💀
```

### Test Examples

```cursed
// Test user operations
slay test_user_operations() {
    test_start("User Operations")
    
    // Test user lookup
    (sus user *User, sus err tea) = Lookup("root")
    assert_eq_string(err, "")
    assert_eq_string(user.Username, "root")
    assert_eq_string(user.Uid, "0")
    assert_true(user.IsRoot())
    
    // Test user existence
    assert_true(UserExists("root"))
    assert_false(UserExists("nonexistent"))
    
    // Test validation
    assert_eq_string(ValidateUsername("valid_user"), "")
    assert_ne_string(ValidateUsername(""), "")  // Should fail
    
    print_test_summary()
}
```

## Dependencies

```cursed
yeet "testz"         // Testing framework
yeet "string"        // String operations
yeet "main_character" // Core functionality
```

## Architecture

### Layered Design

1. **Cache Layer**: Performance optimization through caching
2. **Lookup Layer**: User and group resolution
3. **Management Layer**: CRUD operations for users/groups
4. **Validation Layer**: Input validation and error checking
5. **Utility Layer**: Helper functions and system information

### Extension Points

- **Authentication backends**: LDAP, Active Directory integration
- **Custom validation**: Extended validation rules
- **Audit logging**: Track user/group operations
- **Permission systems**: Role-based access control

The `user_check` module provides comprehensive user and group management capabilities essential for CURSED applications requiring user authentication and authorization functionality.
