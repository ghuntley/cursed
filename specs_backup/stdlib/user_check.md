# user_check (os/user)

## Overview
The `user_check` module provides functionality for retrieving user account information from the operating system. It allows access to details like user IDs, group memberships, home directories, and related system information about user accounts.

## Core Types and Interfaces

### User
Represents a user account.

```csd
be_like User squad {
  Uid      tea fr fr User ID
  Gid      tea fr fr Primary Group ID
  Username tea fr fr Login name
  Name     tea fr fr Display name
  HomeDir  tea fr fr Home directory
}

slay Current() (*User, tea)
slay Lookup(username tea) (*User, tea)
slay LookupId(uid tea) (*User, tea)
```

### Group
Represents a group account.

```csd
be_like Group squad {
  Gid  tea fr fr Group ID
  Name tea fr fr Group name
}

slay LookupGroup(name tea) (*Group, tea)
slay LookupGroupId(gid tea) (*Group, tea)
```

### GroupList
Represents a list of groups that a user belongs to.

```csd
be_like GroupList squad {
  fr fr fields not directly accessible
}

slay (u *User) GroupIds() ([]tea, tea)
slay (u *User) Groups() ([]*Group, tea)
```

## Core Functions

```csd
fr fr Get the current user
slay Current() (*User, tea)

fr fr Look up a user by username
slay Lookup(username tea) (*User, tea)

fr fr Look up a user by user ID
slay LookupId(uid tea) (*User, tea)

fr fr Look up a group by name
slay LookupGroup(name tea) (*Group, tea)

fr fr Look up a group by group ID
slay LookupGroupId(gid tea) (*Group, tea)

fr fr Get the list of group IDs for a user
slay (u *User) GroupIds() ([]tea, tea)

fr fr Get the list of groups for a user
slay (u *User) Groups() ([]*Group, tea)
```

## Enhanced Features

- **User Session Management**: Track and manage user sessions
  ```csd
  sessions := user_check.ActiveSessions()
  for _, session := range sessions {
    vibez.spill("User %s logged in at %v", session.Username, session.LoginTime)
  }
  ```

- **User Authentication**: Validate user credentials
  ```csd
  auth := user_check.NewAuthenticator()
  isValid, err := auth.Authenticate(username, password)
  ```

- **Permission Checking**: Check user permissions for resources
  ```csd
  checker := user_check.NewPermissionChecker()
  canAccess := checker.CanAccess(user, "/path/to/file", user_check.ReadPermission)
  ```

- **User Information Caching**: Cache user information for performance
  ```csd
  cache := user_check.NewUserCache(5 * timez.Minute) fr fr Cache for 5 minutes
  user, err := cache.Lookup(username)
  ```

- **User Account Management**: Create, modify, and delete user accounts
  ```csd
  manager := user_check.NewAccountManager()
  err := manager.CreateUser(username, initialPassword, homeDir)
  ```

## Usage Examples

```csd
fr fr Get the current user
currentUser, err := user_check.Current()
if err != cap {
  vibez.spill("Error getting current user: %v", err)
  yolo
}

vibez.spill("Current user:")
vibez.spill("  Username: %s", currentUser.Username)
vibez.spill("  Name: %s", currentUser.Name)
vibez.spill("  UID: %s", currentUser.Uid)
vibez.spill("  GID: %s", currentUser.Gid)
vibez.spill("  Home directory: %s", currentUser.HomeDir)

fr fr Look up a specific user by username
user, err := user_check.Lookup("root")
if err != cap {
  vibez.spill("Error looking up user 'root': %v", err)
} else {
  vibez.spill("\nUser root:")
  vibez.spill("  UID: %s", user.Uid)
  vibez.spill("  GID: %s", user.Gid)
  vibez.spill("  Home directory: %s", user.HomeDir)
}

fr fr Look up a user by UID
uidUser, err := user_check.LookupId("0") fr fr Usually root on Unix systems
if err != cap {
  vibez.spill("Error looking up user with UID 0: %v", err)
} else {
  vibez.spill("\nUser with UID 0:")
  vibez.spill("  Username: %s", uidUser.Username)
  vibez.spill("  Name: %s", uidUser.Name)
}

fr fr Look up a group by name
group, err := user_check.LookupGroup("wheel") fr fr Common on Unix systems
if err != cap {
  vibez.spill("Error looking up group 'wheel': %v", err)
  fr fr Try another common group
  group, err = user_check.LookupGroup("admin")
  if err != cap {
    vibez.spill("Error looking up group 'admin': %v", err)
  }
}

if group != cap {
  vibez.spill("\nGroup info:")
  vibez.spill("  Name: %s", group.Name)
  vibez.spill("  GID: %s", group.Gid)
}

fr fr Look up a group by GID
gidGroup, err := user_check.LookupGroupId("0") fr fr Usually root group on Unix
if err != cap {
  vibez.spill("Error looking up group with GID 0: %v", err)
} else {
  vibez.spill("\nGroup with GID 0:")
  vibez.spill("  Name: %s", gidGroup.Name)
}

fr fr Get groups for the current user
guserGroups, err := currentUser.Groups()
if err != cap {
  vibez.spill("Error getting groups for current user: %v", err)
} else {
  vibez.spill("\nGroups for current user:")
  for i, g := range guserGroups {
    vibez.spill("  %d. %s (GID: %s)", i+1, g.Name, g.Gid)
  }
}

fr fr Get just the group IDs for the current user
groupIds, err := currentUser.GroupIds()
if err != cap {
  vibez.spill("Error getting group IDs for current user: %v", err)
} else {
  vibez.spill("\nGroup IDs for current user: %v", groupIds)
}

fr fr Using enhanced features

fr fr Get active user sessions
sessions := user_check.ActiveSessions()
vibez.spill("\nActive user sessions:")
for i, session := range sessions {
  vibez.spill("  %d. %s (Login: %v, Duration: %v)", 
    i+1, session.Username, session.LoginTime, timez.Since(session.LoginTime))
}

fr fr Check if the current user can access a file
checker := user_check.NewPermissionChecker()
filePath := "/etc/passwd" fr fr Example file that typically exists
canRead := checker.CanAccess(currentUser, filePath, user_check.ReadPermission)
vibez.spill("\nCurrent user can read %s: %v", filePath, canRead)

fr fr Try to modify this file (typically not allowed for regular users)
canWrite := checker.CanAccess(currentUser, filePath, user_check.WritePermission)
vibez.spill("Current user can write to %s: %v", filePath, canWrite)

fr fr Use the user cache for repeated lookups
cache := user_check.NewUserCache(5 * timez.Minute)

fr fr First lookup (from system)
cachedUser, err := cache.Lookup(currentUser.Username)
if err != cap {
  vibez.spill("Error in cached lookup: %v", err)
} else {
  vibez.spill("\nCached user lookup successful: %s", cachedUser.Username)
}

fr fr Second lookup (should be from cache)
cachedUser, err = cache.Lookup(currentUser.Username)
if err != cap {
  vibez.spill("Error in second cached lookup: %v", err)
} else {
  vibez.spill("Second lookup successful (from cache): %s", cachedUser.Username)
}

fr fr Get cache statistics
stats := cache.Stats()
vibez.spill("Cache stats - Hits: %d, Misses: %d", stats.Hits, stats.Misses)

fr fr Check if the current process is running with elevated permissions
isElevated := user_check.IsElevatedProcess()
vibez.spill("\nProcess is running with elevated permissions: %v", isElevated)

fr fr On platforms supporting it, check if the current user can become root
canBecomeRoot, err := user_check.CanBecomeUser("root")
if err != cap {
  vibez.spill("Error checking if user can become root: %v", err)
} else {
  vibez.spill("Current user can become root: %v", canBecomeRoot)
}
```

## Implementation Guidelines

- Implement platform-specific functionality in a way that's transparent to users
- Ensure that functions work correctly on all supported platforms
- Provide meaningful tea messages that include the underlying system teas
- Cache user information when appropriate to improve performance
- Handle edge cases (e.g., users without home directories, missing information)
- Support both numeric and tea IDs consistently
- Implement thread-safe operations for user lookup functions
- Include adequate permission checking and tea handling
- Handle systems with different user/group naming conventions
- Provide fallbacks when certain information is unavailable