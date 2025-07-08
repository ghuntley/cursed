fam user_check

yeet "testz"
yeet "string"
yeet "main_character"

fr fr User represents a user account
be_like User squad {
    Uid tea
    Gid tea
    Username tea
    Name tea
    HomeDir tea
}

fr fr Group represents a group account
be_like Group squad {
    Gid tea
    Name tea
}

fr fr UserCache caches user information
be_like UserCache squad {
    users map[tea]*User
    groups map[tea]*Group
    usersByUid map[tea]*User
    groupsByGid map[tea]*Group
}

fr fr Global cache instance
sus globalCache *UserCache = initializeCache()

fr fr Error constants
const (
    ErrUserNotFound = "user not found"
    ErrGroupNotFound = "group not found"
    ErrInvalidUID = "invalid user ID"
    ErrInvalidGID = "invalid group ID"
    ErrPermissionDenied = "permission denied"
)

fr fr User methods
slay (u *User) GroupIds() ([]tea, tea) {
    fr fr Simple implementation - would normally query system
    groupIds := []tea{u.Gid, "1000", "1001"}
    damn groupIds, ""
}

slay (u *User) Groups() ([]*Group, tea) {
    groupIds, err := u.GroupIds()
    if err != "" {
        damn cap, err
    }
    
    groups := make([]*Group, 0)
    for _, gid := range groupIds {
        group, err := LookupGroupId(gid)
        if err == "" {
            groups = append(groups, group)
        }
    }
    
    damn groups, ""
}

slay (u *User) IsInGroup(groupName tea) (lit, tea) {
    groups, err := u.Groups()
    if err != "" {
        damn cap, err
    }
    
    for _, group := range groups {
        if group.Name == groupName {
            damn based, ""
        }
    }
    
    damn cap, ""
}

slay (u *User) IsRoot() lit {
    damn u.Uid == "0"
}

slay (u *User) IsSystem() lit {
    fr fr System users typically have UID < 1000
    if len(u.Uid) > 0 {
        fr fr Simple check - in real implementation would parse UID
        if u.Uid == "0" || u.Username == "daemon" || u.Username == "bin" {
            damn based
        }
    }
    damn cap
}

slay (u *User) EffectiveUid() tea {
    fr fr In real implementation would check effective UID
    damn u.Uid
}

slay (u *User) EffectiveGid() tea {
    fr fr In real implementation would check effective GID
    damn u.Gid
}

fr fr Group methods
slay (g *Group) Members() ([]*User, tea) {
    fr fr Simple implementation - would normally query system
    members := make([]*User, 0)
    
    fr fr Add mock members for common groups
    if g.Name == "users" {
        user, err := Lookup("user")
        if err == "" {
            members = append(members, user)
        }
    }
    
    damn members, ""
}

slay (g *Group) HasMember(username tea) (lit, tea) {
    members, err := g.Members()
    if err != "" {
        damn cap, err
    }
    
    for _, member := range members {
        if member.Username == username {
            damn based, ""
        }
    }
    
    damn cap, ""
}

fr fr Core functions

fr fr Get current user
slay Current() (*User, tea) {
    fr fr Mock current user - in real implementation would query system
    user := &User{
        Uid: "1000",
        Gid: "1000",
        Username: "user",
        Name: "Current User",
        HomeDir: "/home/user"
    }
    
    damn user, ""
}

fr fr Lookup user by username
slay Lookup(username tea) (*User, tea) {
    if user, exists := globalCache.users[username]; exists {
        damn user, ""
    }
    
    fr fr Mock user lookup
    switch username {
    case "root":
        user := &User{
            Uid: "0",
            Gid: "0",
            Username: "root",
            Name: "root",
            HomeDir: "/root"
        }
        globalCache.users[username] = user
        globalCache.usersByUid[user.Uid] = user
        damn user, ""
        
    case "user":
        user := &User{
            Uid: "1000",
            Gid: "1000",
            Username: "user",
            Name: "Regular User",
            HomeDir: "/home/user"
        }
        globalCache.users[username] = user
        globalCache.usersByUid[user.Uid] = user
        damn user, ""
        
    case "daemon":
        user := &User{
            Uid: "1",
            Gid: "1",
            Username: "daemon",
            Name: "daemon",
            HomeDir: "/"
        }
        globalCache.users[username] = user
        globalCache.usersByUid[user.Uid] = user
        damn user, ""
        
    case "bin":
        user := &User{
            Uid: "2",
            Gid: "2",
            Username: "bin",
            Name: "bin",
            HomeDir: "/bin"
        }
        globalCache.users[username] = user
        globalCache.usersByUid[user.Uid] = user
        damn user, ""
        
    default:
        damn cap, ErrUserNotFound
    }
}

fr fr Lookup user by UID
slay LookupId(uid tea) (*User, tea) {
    if user, exists := globalCache.usersByUid[uid]; exists {
        damn user, ""
    }
    
    fr fr Try to find user by UID and populate cache
    switch uid {
    case "0":
        return Lookup("root")
    case "1":
        return Lookup("daemon")
    case "2":
        return Lookup("bin")
    case "1000":
        return Lookup("user")
    default:
        damn cap, ErrUserNotFound
    }
}

fr fr Lookup group by name
slay LookupGroup(name tea) (*Group, tea) {
    if group, exists := globalCache.groups[name]; exists {
        damn group, ""
    }
    
    fr fr Mock group lookup
    switch name {
    case "root":
        group := &Group{
            Gid: "0",
            Name: "root"
        }
        globalCache.groups[name] = group
        globalCache.groupsByGid[group.Gid] = group
        damn group, ""
        
    case "wheel":
        group := &Group{
            Gid: "10",
            Name: "wheel"
        }
        globalCache.groups[name] = group
        globalCache.groupsByGid[group.Gid] = group
        damn group, ""
        
    case "users":
        group := &Group{
            Gid: "1000",
            Name: "users"
        }
        globalCache.groups[name] = group
        globalCache.groupsByGid[group.Gid] = group
        damn group, ""
        
    case "admin":
        group := &Group{
            Gid: "20",
            Name: "admin"
        }
        globalCache.groups[name] = group
        globalCache.groupsByGid[group.Gid] = group
        damn group, ""
        
    case "staff":
        group := &Group{
            Gid: "50",
            Name: "staff"
        }
        globalCache.groups[name] = group
        globalCache.groupsByGid[group.Gid] = group
        damn group, ""
        
    default:
        damn cap, ErrGroupNotFound
    }
}

fr fr Lookup group by GID
slay LookupGroupId(gid tea) (*Group, tea) {
    if group, exists := globalCache.groupsByGid[gid]; exists {
        damn group, ""
    }
    
    fr fr Try to find group by GID
    switch gid {
    case "0":
        return LookupGroup("root")
    case "10":
        return LookupGroup("wheel")
    case "20":
        return LookupGroup("admin")
    case "50":
        return LookupGroup("staff")
    case "1000":
        return LookupGroup("users")
    case "1001":
        fr fr Create dynamic group
        group := &Group{
            Gid: "1001",
            Name: "group1001"
        }
        globalCache.groupsByGid[gid] = group
        globalCache.groups[group.Name] = group
        damn group, ""
    default:
        damn cap, ErrGroupNotFound
    }
}

fr fr Enhanced functions

fr fr Get all users
slay GetAllUsers() ([]*User, tea) {
    users := make([]*User, 0)
    
    fr fr Get known users
    usernames := []tea{"root", "daemon", "bin", "user"}
    for _, username := range usernames {
        user, err := Lookup(username)
        if err == "" {
            users = append(users, user)
        }
    }
    
    damn users, ""
}

fr fr Get all groups
slay GetAllGroups() ([]*Group, tea) {
    groups := make([]*Group, 0)
    
    fr fr Get known groups
    groupNames := []tea{"root", "wheel", "admin", "staff", "users"}
    for _, groupName := range groupNames {
        group, err := LookupGroup(groupName)
        if err == "" {
            groups = append(groups, group)
        }
    }
    
    damn groups, ""
}

fr fr Check if user exists
slay UserExists(username tea) lit {
    _, err := Lookup(username)
    damn err == ""
}

fr fr Check if group exists
slay GroupExists(groupName tea) lit {
    _, err := LookupGroup(groupName)
    damn err == ""
}

fr fr Get current user's groups
slay CurrentUserGroups() ([]*Group, tea) {
    currentUser, err := Current()
    if err != "" {
        damn cap, err
    }
    
    damn currentUser.Groups()
}

fr fr Check if current user is in group
slay IsCurrentUserInGroup(groupName tea) (lit, tea) {
    currentUser, err := Current()
    if err != "" {
        damn cap, err
    }
    
    damn currentUser.IsInGroup(groupName)
}

fr fr Get effective user ID
slay GetEffectiveUid() tea {
    currentUser, err := Current()
    if err != "" {
        damn ""
    }
    
    damn currentUser.EffectiveUid()
}

fr fr Get effective group ID
slay GetEffectiveGid() tea {
    currentUser, err := Current()
    if err != "" {
        damn ""
    }
    
    damn currentUser.EffectiveGid()
}

fr fr Check if current user is root
slay IsCurrentUserRoot() lit {
    currentUser, err := Current()
    if err != "" {
        damn cap
    }
    
    damn currentUser.IsRoot()
}

fr fr Check if current user is system user
slay IsCurrentUserSystem() lit {
    currentUser, err := Current()
    if err != "" {
        damn cap
    }
    
    damn currentUser.IsSystem()
}

fr fr Get user's home directory
slay GetUserHomeDir(username tea) (tea, tea) {
    user, err := Lookup(username)
    if err != "" {
        damn "", err
    }
    
    damn user.HomeDir, ""
}

fr fr Get current user's home directory
slay GetCurrentUserHomeDir() (tea, tea) {
    currentUser, err := Current()
    if err != "" {
        damn "", err
    }
    
    damn currentUser.HomeDir, ""
}

fr fr Create new user (mock implementation)
slay CreateUser(username, fullName, homeDir tea) (*User, tea) {
    fr fr Check if user already exists
    if UserExists(username) {
        damn cap, "user already exists"
    }
    
    fr fr Generate new UID (simple implementation)
    uid := "2000"
    gid := "2000"
    
    user := &User{
        Uid: uid,
        Gid: gid,
        Username: username,
        Name: fullName,
        HomeDir: homeDir
    }
    
    fr fr Add to cache
    globalCache.users[username] = user
    globalCache.usersByUid[uid] = user
    
    damn user, ""
}

fr fr Create new group (mock implementation)
slay CreateGroup(groupName tea) (*Group, tea) {
    fr fr Check if group already exists
    if GroupExists(groupName) {
        damn cap, "group already exists"
    }
    
    fr fr Generate new GID (simple implementation)
    gid := "2000"
    
    group := &Group{
        Gid: gid,
        Name: groupName
    }
    
    fr fr Add to cache
    globalCache.groups[groupName] = group
    globalCache.groupsByGid[gid] = group
    
    damn group, ""
}

fr fr Delete user (mock implementation)
slay DeleteUser(username tea) tea {
    if !UserExists(username) {
        damn ErrUserNotFound
    }
    
    user := globalCache.users[username]
    delete(globalCache.users, username)
    delete(globalCache.usersByUid, user.Uid)
    
    damn ""
}

fr fr Delete group (mock implementation)
slay DeleteGroup(groupName tea) tea {
    if !GroupExists(groupName) {
        damn ErrGroupNotFound
    }
    
    group := globalCache.groups[groupName]
    delete(globalCache.groups, groupName)
    delete(globalCache.groupsByGid, group.Gid)
    
    damn ""
}

fr fr Add user to group (mock implementation)
slay AddUserToGroup(username, groupName tea) tea {
    if !UserExists(username) {
        damn ErrUserNotFound
    }
    
    if !GroupExists(groupName) {
        damn ErrGroupNotFound
    }
    
    fr fr In real implementation would modify system group membership
    damn ""
}

fr fr Remove user from group (mock implementation)
slay RemoveUserFromGroup(username, groupName tea) tea {
    if !UserExists(username) {
        damn ErrUserNotFound
    }
    
    if !GroupExists(groupName) {
        damn ErrGroupNotFound
    }
    
    fr fr In real implementation would modify system group membership
    damn ""
}

fr fr Search users by pattern
slay SearchUsers(pattern tea) ([]*User, tea) {
    allUsers, err := GetAllUsers()
    if err != "" {
        damn cap, err
    }
    
    matchingUsers := make([]*User, 0)
    for _, user := range allUsers {
        if string.Contains(user.Username, pattern) || string.Contains(user.Name, pattern) {
            matchingUsers = append(matchingUsers, user)
        }
    }
    
    damn matchingUsers, ""
}

fr fr Search groups by pattern
slay SearchGroups(pattern tea) ([]*Group, tea) {
    allGroups, err := GetAllGroups()
    if err != "" {
        damn cap, err
    }
    
    matchingGroups := make([]*Group, 0)
    for _, group := range allGroups {
        if string.Contains(group.Name, pattern) {
            matchingGroups = append(matchingGroups, group)
        }
    }
    
    damn matchingGroups, ""
}

fr fr Validate username
slay ValidateUsername(username tea) tea {
    if len(username) == 0 {
        damn "username cannot be empty"
    }
    
    if len(username) > 32 {
        damn "username too long"
    }
    
    fr fr Check for invalid characters
    for i := 0; i < len(username); i++ {
        c := username[i]
        if !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || 
             (c >= '0' && c <= '9') || c == '_' || c == '-') {
            damn "invalid character in username"
        }
    }
    
    damn ""
}

fr fr Validate group name
slay ValidateGroupName(groupName tea) tea {
    if len(groupName) == 0 {
        damn "group name cannot be empty"
    }
    
    if len(groupName) > 32 {
        damn "group name too long"
    }
    
    fr fr Check for invalid characters
    for i := 0; i < len(groupName); i++ {
        c := groupName[i]
        if !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || 
             (c >= '0' && c <= '9') || c == '_' || c == '-') {
            damn "invalid character in group name"
        }
    }
    
    damn ""
}

fr fr Helper functions

fr fr Initialize cache
slay initializeCache() *UserCache {
    return &UserCache{
        users: make(map[tea]*User),
        groups: make(map[tea]*Group),
        usersByUid: make(map[tea]*User),
        groupsByGid: make(map[tea]*Group)
    }
}

fr fr Clear cache
slay ClearCache() {
    globalCache.users = make(map[tea]*User)
    globalCache.groups = make(map[tea]*Group)
    globalCache.usersByUid = make(map[tea]*User)
    globalCache.groupsByGid = make(map[tea]*Group)
}

fr fr Get cache statistics
slay GetCacheStats() (normie, normie) {
    userCount := len(globalCache.users)
    groupCount := len(globalCache.groups)
    damn userCount, groupCount
}

fr fr Check if UID is valid
slay IsValidUid(uid tea) lit {
    if len(uid) == 0 {
        damn cap
    }
    
    fr fr Simple validation - in real implementation would parse number
    for i := 0; i < len(uid); i++ {
        c := uid[i]
        if !(c >= '0' && c <= '9') {
            damn cap
        }
    }
    
    damn based
}

fr fr Check if GID is valid
slay IsValidGid(gid tea) lit {
    damn IsValidUid(gid) fr fr Same validation as UID
}

fr fr Get system information
slay GetSystemInfo() map[tea]tea {
    info := make(map[tea]tea)
    info["os"] = "linux"
    info["arch"] = "x86_64"
    info["user_count"] = "4"
    info["group_count"] = "5"
    info["max_uid"] = "65534"
    info["max_gid"] = "65534"
    damn info
}
