yeet "testz"
yeet "user_check"

fr fr Comprehensive test suite for user_check module
fr fr User and group management functionality

test_start("test_current_user")
fr fr Test current user retrieval
sus user, err := Current()
assert_eq_string(err, "")
assert_eq_string(user.Uid, "1000")
assert_eq_string(user.Gid, "1000")
assert_eq_string(user.Username, "user")
assert_eq_string(user.Name, "Current User")
assert_eq_string(user.HomeDir, "/home/user")
print_test_summary()

test_start("test_user_lookup")
fr fr Test user lookup by username
sus rootUser, err := Lookup("root")
assert_eq_string(err, "")
assert_eq_string(rootUser.Uid, "0")
assert_eq_string(rootUser.Gid, "0")
assert_eq_string(rootUser.Username, "root")
assert_eq_string(rootUser.Name, "root")
assert_eq_string(rootUser.HomeDir, "/root")

sus user, err2 := Lookup("user")
assert_eq_string(err2, "")
assert_eq_string(user.Uid, "1000")
assert_eq_string(user.Username, "user")

sus daemon, err3 := Lookup("daemon")
assert_eq_string(err3, "")
assert_eq_string(daemon.Uid, "1")
assert_eq_string(daemon.Username, "daemon")

sus bin, err4 := Lookup("bin")
assert_eq_string(err4, "")
assert_eq_string(bin.Uid, "2")
assert_eq_string(bin.Username, "bin")

sus nonexistent, err5 := Lookup("nonexistent")
assert_eq_string(nonexistent, cap)
assert_eq_string(err5, ErrUserNotFound)
print_test_summary()

test_start("test_user_lookup_by_id")
fr fr Test user lookup by UID
sus rootUser, err := LookupId("0")
assert_eq_string(err, "")
assert_eq_string(rootUser.Username, "root")
assert_eq_string(rootUser.Uid, "0")

sus user, err2 := LookupId("1000")
assert_eq_string(err2, "")
assert_eq_string(user.Username, "user")
assert_eq_string(user.Uid, "1000")

sus daemon, err3 := LookupId("1")
assert_eq_string(err3, "")
assert_eq_string(daemon.Username, "daemon")

sus nonexistent, err4 := LookupId("9999")
assert_eq_string(nonexistent, cap)
assert_eq_string(err4, ErrUserNotFound)
print_test_summary()

test_start("test_group_lookup")
fr fr Test group lookup by name
sus rootGroup, err := LookupGroup("root")
assert_eq_string(err, "")
assert_eq_string(rootGroup.Gid, "0")
assert_eq_string(rootGroup.Name, "root")

sus wheelGroup, err2 := LookupGroup("wheel")
assert_eq_string(err2, "")
assert_eq_string(wheelGroup.Gid, "10")
assert_eq_string(wheelGroup.Name, "wheel")

sus usersGroup, err3 := LookupGroup("users")
assert_eq_string(err3, "")
assert_eq_string(usersGroup.Gid, "1000")
assert_eq_string(usersGroup.Name, "users")

sus adminGroup, err4 := LookupGroup("admin")
assert_eq_string(err4, "")
assert_eq_string(adminGroup.Gid, "20")
assert_eq_string(adminGroup.Name, "admin")

sus staffGroup, err5 := LookupGroup("staff")
assert_eq_string(err5, "")
assert_eq_string(staffGroup.Gid, "50")
assert_eq_string(staffGroup.Name, "staff")

sus nonexistent, err6 := LookupGroup("nonexistent")
assert_eq_string(nonexistent, cap)
assert_eq_string(err6, ErrGroupNotFound)
print_test_summary()

test_start("test_group_lookup_by_id")
fr fr Test group lookup by GID
sus rootGroup, err := LookupGroupId("0")
assert_eq_string(err, "")
assert_eq_string(rootGroup.Name, "root")
assert_eq_string(rootGroup.Gid, "0")

sus wheelGroup, err2 := LookupGroupId("10")
assert_eq_string(err2, "")
assert_eq_string(wheelGroup.Name, "wheel")

sus usersGroup, err3 := LookupGroupId("1000")
assert_eq_string(err3, "")
assert_eq_string(usersGroup.Name, "users")

sus dynamicGroup, err4 := LookupGroupId("1001")
assert_eq_string(err4, "")
assert_eq_string(dynamicGroup.Name, "group1001")
assert_eq_string(dynamicGroup.Gid, "1001")

sus nonexistent, err5 := LookupGroupId("9999")
assert_eq_string(nonexistent, cap)
assert_eq_string(err5, ErrGroupNotFound)
print_test_summary()

test_start("test_user_methods")
fr fr Test user methods
sus user, err := Lookup("user")
assert_eq_string(err, "")

fr fr Test GroupIds method
sus groupIds, err2 := user.GroupIds()
assert_eq_string(err2, "")
assert_eq_int(len(groupIds), 3)
assert_eq_string(groupIds[0], user.Gid)
assert_eq_string(groupIds[1], "1000")
assert_eq_string(groupIds[2], "1001")

fr fr Test Groups method
sus groups, err3 := user.Groups()
assert_eq_string(err3, "")
assert_true(len(groups) > 0)

fr fr Test IsInGroup method
sus inGroup, err4 := user.IsInGroup("users")
assert_eq_string(err4, "")
assert_eq_string(inGroup, based)

sus notInGroup, err5 := user.IsInGroup("nonexistent")
assert_eq_string(err5, "")
assert_eq_string(notInGroup, cap)

fr fr Test IsRoot method
assert_eq_string(user.IsRoot(), cap)

fr fr Test IsSystem method
assert_eq_string(user.IsSystem(), cap)

fr fr Test effective IDs
assert_eq_string(user.EffectiveUid(), user.Uid)
assert_eq_string(user.EffectiveGid(), user.Gid)
print_test_summary()

test_start("test_root_user_methods")
fr fr Test root user methods
sus rootUser, err := Lookup("root")
assert_eq_string(err, "")

assert_eq_string(rootUser.IsRoot(), based)
assert_eq_string(rootUser.IsSystem(), cap) fr fr Root is not considered system user
assert_eq_string(rootUser.EffectiveUid(), "0")
assert_eq_string(rootUser.EffectiveGid(), "0")
print_test_summary()

test_start("test_system_user_methods")
fr fr Test system user methods
sus daemonUser, err := Lookup("daemon")
assert_eq_string(err, "")

assert_eq_string(daemonUser.IsRoot(), cap)
assert_eq_string(daemonUser.IsSystem(), based)

sus binUser, err2 := Lookup("bin")
assert_eq_string(err2, "")

assert_eq_string(binUser.IsRoot(), cap)
assert_eq_string(binUser.IsSystem(), based)
print_test_summary()

test_start("test_group_methods")
fr fr Test group methods
sus usersGroup, err := LookupGroup("users")
assert_eq_string(err, "")

fr fr Test Members method
sus members, err2 := usersGroup.Members()
assert_eq_string(err2, "")
assert_true(len(members) >= 0)

fr fr Test HasMember method
sus hasMember, err3 := usersGroup.HasMember("user")
assert_eq_string(err3, "")
assert_eq_string(hasMember, based)

sus notMember, err4 := usersGroup.HasMember("nonexistent")
assert_eq_string(err4, "")
assert_eq_string(notMember, cap)
print_test_summary()

test_start("test_user_existence")
fr fr Test user existence checks
assert_eq_string(UserExists("root"), based)
assert_eq_string(UserExists("user"), based)
assert_eq_string(UserExists("daemon"), based)
assert_eq_string(UserExists("bin"), based)
assert_eq_string(UserExists("nonexistent"), cap)
print_test_summary()

test_start("test_group_existence")
fr fr Test group existence checks
assert_eq_string(GroupExists("root"), based)
assert_eq_string(GroupExists("wheel"), based)
assert_eq_string(GroupExists("users"), based)
assert_eq_string(GroupExists("admin"), based)
assert_eq_string(GroupExists("staff"), based)
assert_eq_string(GroupExists("nonexistent"), cap)
print_test_summary()

test_start("test_get_all_users")
fr fr Test getting all users
sus users, err := GetAllUsers()
assert_eq_string(err, "")
assert_eq_int(len(users), 4)

sus usernames := tea[value]{}
bestie i := 0; i < len(users); i++ {
    usernames = append(usernames, users[i].Username)
}

fr fr Check that all expected users are present
assert_true(contains(usernames, "root"))
assert_true(contains(usernames, "user"))
assert_true(contains(usernames, "daemon"))
assert_true(contains(usernames, "bin"))
print_test_summary()

test_start("test_get_all_groups")
fr fr Test getting all groups
sus groups, err := GetAllGroups()
assert_eq_string(err, "")
assert_eq_int(len(groups), 5)

sus groupNames := tea[value]{}
bestie i := 0; i < len(groups); i++ {
    groupNames = append(groupNames, groups[i].Name)
}

fr fr Check that all expected groups are present
assert_true(contains(groupNames, "root"))
assert_true(contains(groupNames, "wheel"))
assert_true(contains(groupNames, "users"))
assert_true(contains(groupNames, "admin"))
assert_true(contains(groupNames, "staff"))
print_test_summary()

test_start("test_current_user_functions")
fr fr Test current user functions
sus groups, err := CurrentUserGroups()
assert_eq_string(err, "")
assert_true(len(groups) > 0)

sus inGroup, err2 := IsCurrentUserInGroup("users")
assert_eq_string(err2, "")
assert_eq_string(inGroup, based)

sus uid := GetEffectiveUid()
assert_eq_string(uid, "1000")

sus gid := GetEffectiveGid()
assert_eq_string(gid, "1000")

assert_eq_string(IsCurrentUserRoot(), cap)
assert_eq_string(IsCurrentUserSystem(), cap)
print_test_summary()

test_start("test_user_home_directory")
fr fr Test user home directory functions
sus homeDir, err := GetUserHomeDir("root")
assert_eq_string(err, "")
assert_eq_string(homeDir, "/root")

sus userHome, err2 := GetUserHomeDir("user")
assert_eq_string(err2, "")
assert_eq_string(userHome, "/home/user")

sus currentHome, err3 := GetCurrentUserHomeDir()
assert_eq_string(err3, "")
assert_eq_string(currentHome, "/home/user")

sus nonexistentHome, err4 := GetUserHomeDir("nonexistent")
assert_eq_string(nonexistentHome, "")
assert_eq_string(err4, ErrUserNotFound)
print_test_summary()

test_start("test_user_creation")
fr fr Test user creation
sus newUser, err := CreateUser("testuser", "Test User", "/home/testuser")
assert_eq_string(err, "")
assert_eq_string(newUser.Username, "testuser")
assert_eq_string(newUser.Name, "Test User")
assert_eq_string(newUser.HomeDir, "/home/testuser")
assert_eq_string(newUser.Uid, "2000")
assert_eq_string(newUser.Gid, "2000")

assert_eq_string(UserExists("testuser"), based)

fr fr Test duplicate user creation
sus duplicate, err2 := CreateUser("testuser", "Duplicate", "/home/dup")
assert_eq_string(duplicate, cap)
assert_eq_string(err2, "user already exists")
print_test_summary()

test_start("test_group_creation")
fr fr Test group creation
sus newGroup, err := CreateGroup("testgroup")
assert_eq_string(err, "")
assert_eq_string(newGroup.Name, "testgroup")
assert_eq_string(newGroup.Gid, "2000")

assert_eq_string(GroupExists("testgroup"), based)

fr fr Test duplicate group creation
sus duplicate, err2 := CreateGroup("testgroup")
assert_eq_string(duplicate, cap)
assert_eq_string(err2, "group already exists")
print_test_summary()

test_start("test_user_deletion")
fr fr Test user deletion
sus err := DeleteUser("testuser")
assert_eq_string(err, "")

assert_eq_string(UserExists("testuser"), cap)

fr fr Test deleting nonexistent user
sus err2 := DeleteUser("nonexistent")
assert_eq_string(err2, ErrUserNotFound)
print_test_summary()

test_start("test_group_deletion")
fr fr Test group deletion
sus err := DeleteGroup("testgroup")
assert_eq_string(err, "")

assert_eq_string(GroupExists("testgroup"), cap)

fr fr Test deleting nonexistent group
sus err2 := DeleteGroup("nonexistent")
assert_eq_string(err2, ErrGroupNotFound)
print_test_summary()

test_start("test_group_membership")
fr fr Test group membership operations
sus err := AddUserToGroup("user", "wheel")
assert_eq_string(err, "")

sus err2 := RemoveUserFromGroup("user", "wheel")
assert_eq_string(err2, "")

fr fr Test with nonexistent user
sus err3 := AddUserToGroup("nonexistent", "wheel")
assert_eq_string(err3, ErrUserNotFound)

fr fr Test with nonexistent group
sus err4 := AddUserToGroup("user", "nonexistent")
assert_eq_string(err4, ErrGroupNotFound)
print_test_summary()

test_start("test_search_functions")
fr fr Test search functions
sus users, err := SearchUsers("user")
assert_eq_string(err, "")
assert_true(len(users) > 0)

sus users2, err2 := SearchUsers("root")
assert_eq_string(err2, "")
assert_true(len(users2) > 0)

sus groups, err3 := SearchGroups("admin")
assert_eq_string(err3, "")
assert_true(len(groups) > 0)

sus groups2, err4 := SearchGroups("wheel")
assert_eq_string(err4, "")
assert_true(len(groups2) > 0)
print_test_summary()

test_start("test_validation_functions")
fr fr Test validation functions
sus err := ValidateUsername("validuser")
assert_eq_string(err, "")

sus err2 := ValidateUsername("valid_user")
assert_eq_string(err2, "")

sus err3 := ValidateUsername("valid-user")
assert_eq_string(err3, "")

sus err4 := ValidateUsername("")
assert_eq_string(err4, "username cannot be empty")

sus err5 := ValidateUsername("invalid@user")
assert_eq_string(err5, "invalid character in username")

sus err6 := ValidateGroupName("validgroup")
assert_eq_string(err6, "")

sus err7 := ValidateGroupName("")
assert_eq_string(err7, "group name cannot be empty")

sus err8 := ValidateGroupName("invalid@group")
assert_eq_string(err8, "invalid character in group name")
print_test_summary()

test_start("test_cache_functions")
fr fr Test cache functions
sus userCount, groupCount := GetCacheStats()
assert_true(userCount >= 0)
assert_true(groupCount >= 0)

ClearCache()

sus userCount2, groupCount2 := GetCacheStats()
assert_eq_int(userCount2, 0)
assert_eq_int(groupCount2, 0)

fr fr Repopulate cache
sus user, err := Lookup("user")
assert_eq_string(err, "")

sus userCount3, groupCount3 := GetCacheStats()
assert_eq_int(userCount3, 1)
assert_eq_int(groupCount3, 0)
print_test_summary()

test_start("test_id_validation")
fr fr Test ID validation functions
assert_eq_string(IsValidUid("1000"), based)
assert_eq_string(IsValidUid("0"), based)
assert_eq_string(IsValidUid("123"), based)
assert_eq_string(IsValidUid(""), cap)
assert_eq_string(IsValidUid("abc"), cap)
assert_eq_string(IsValidUid("12a"), cap)

assert_eq_string(IsValidGid("1000"), based)
assert_eq_string(IsValidGid("0"), based)
assert_eq_string(IsValidGid(""), cap)
assert_eq_string(IsValidGid("abc"), cap)
print_test_summary()

test_start("test_system_info")
fr fr Test system info function
sus info := GetSystemInfo()
assert_eq_string(info["os"], "linux")
assert_eq_string(info["arch"], "x86_64")
assert_eq_string(info["user_count"], "4")
assert_eq_string(info["group_count"], "5")
assert_eq_string(info["max_uid"], "65534")
assert_eq_string(info["max_gid"], "65534")
print_test_summary()

fr fr Integration tests
test_start("integration_tests")
fr fr Test complete user management workflow
sus user, err := CreateUser("integration_user", "Integration Test User", "/home/integration")
assert_eq_string(err, "")

sus group, err2 := CreateGroup("integration_group")
assert_eq_string(err2, "")

sus err3 := AddUserToGroup("integration_user", "integration_group")
assert_eq_string(err3, "")

sus foundUser, err4 := Lookup("integration_user")
assert_eq_string(err4, "")
assert_eq_string(foundUser.Username, "integration_user")

sus foundGroup, err5 := LookupGroup("integration_group")
assert_eq_string(err5, "")
assert_eq_string(foundGroup.Name, "integration_group")

sus err6 := RemoveUserFromGroup("integration_user", "integration_group")
assert_eq_string(err6, "")

sus err7 := DeleteUser("integration_user")
assert_eq_string(err7, "")

sus err8 := DeleteGroup("integration_group")
assert_eq_string(err8, "")

assert_eq_string(UserExists("integration_user"), cap)
assert_eq_string(GroupExists("integration_group"), cap)
print_test_summary()

fr fr Performance benchmarks
test_start("performance_benchmarks")
fr fr Test performance of user operations
bestie i := 0; i < 100; i++ {
    sus user, err := Lookup("user")
    assert_eq_string(err, "")
}

bestie i := 0; i < 100; i++ {
    sus group, err := LookupGroup("users")
    assert_eq_string(err, "")
}

bestie i := 0; i < 100; i++ {
    assert_eq_string(UserExists("user"), based)
    assert_eq_string(GroupExists("users"), based)
}

bestie i := 0; i < 50; i++ {
    sus users, err := GetAllUsers()
    assert_eq_string(err, "")
    assert_true(len(users) > 0)
}
print_test_summary()

fr fr Edge case testing
test_start("edge_cases")
fr fr Test edge cases and error conditions
fr fr Test very long usernames
sus longUsername := ""
bestie i := 0; i < 50; i++ {
    longUsername = longUsername + "a"
}
sus err := ValidateUsername(longUsername)
assert_eq_string(err, "username too long")

fr fr Test empty cache operations
ClearCache()
sus userCount, groupCount := GetCacheStats()
assert_eq_int(userCount, 0)
assert_eq_int(groupCount, 0)

fr fr Test user lookup after cache clear
sus user, err2 := Lookup("user")
assert_eq_string(err2, "")
assert_eq_string(user.Username, "user")

fr fr Test group member operations on empty group
sus emptyGroup, err3 := LookupGroup("wheel")
assert_eq_string(err3, "")
sus members, err4 := emptyGroup.Members()
assert_eq_string(err4, "")
assert_eq_int(len(members), 0)

fr fr Test user group operations
sus testUser, err5 := Lookup("user")
assert_eq_string(err5, "")
sus groups, err6 := testUser.Groups()
assert_eq_string(err6, "")
assert_true(len(groups) > 0)

fr fr Test invalid ID lookups
sus invalidUser, err7 := LookupId("invalid")
assert_eq_string(invalidUser, cap)
assert_eq_string(err7, ErrUserNotFound)

sus invalidGroup, err8 := LookupGroupId("invalid")
assert_eq_string(invalidGroup, cap)
assert_eq_string(err8, ErrGroupNotFound)
print_test_summary()

fr fr Helper function for testing array contains
slay contains(arr tea[value], item tea) lit {
    bestie i := 0; i < len(arr); i++ {
        if arr[i] == item {
            damn based
        }
    }
    damn cap
}
