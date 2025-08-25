yeet "testz"
yeet "authz"

test_start("AUTHZ Comprehensive Authentication Tests")

// Test user authentication
sus auth_result tea = authenticate_user("testuser", "password123")
assert_eq_string(auth_result, "authenticated")

sus invalid_auth tea = authenticate_user("invalid", "wrong")
assert_eq_string(invalid_auth, "failed")

// Test password hashing
sus hashed tea = hash_password("securepass")
assert_not_eq_string(hashed, "securepass")
assert_true(verify_password("securepass", hashed))
assert_false(verify_password("wrongpass", hashed))

// Test token generation
sus token tea = generate_jwt_token("user123", 3600)
assert_not_eq_string(token, "")
assert_true(validate_jwt_token(token))

// Test role-based access control
set_user_role("admin", "administrator")
assert_true(check_permission("admin", "read"))
assert_true(check_permission("admin", "write"))
assert_false(check_permission("guest", "admin"))

// Test session management
sus session_id tea = create_session("user456")
assert_true(validate_session(session_id))
destroy_session(session_id)
assert_false(validate_session(session_id))

// Test OAuth integration
sus oauth_url tea = generate_oauth_url("github", "client123")
assert_contains_string(oauth_url, "github.com")
assert_contains_string(oauth_url, "client123")

// Test security headers
sus headers drip = get_security_headers()
assert_true(headers > 0)

print_test_summary()
