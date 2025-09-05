# Modules and Imports

CURSED's module system organizes code into reusable packages. This tutorial covers package creation, import strategies, and dependency management.

## Learning Objectives

By the end of this tutorial, you'll be able to:
- Create and organize modules with proper structure
- Use import strategies for different scenarios
- Manage dependencies and circular imports
- Implement proper visibility and encapsulation
- Test and document modules effectively

## Package System Fundamentals

### Package Declaration

Every CURSED file belongs to a package:

```cursed
# math_utils.💀 - A simple utility module
vibe math_utils

# Exported function (starts with uppercase)
slay Add(a normie, b normie) normie {
    damn a + b
}

# Private function (starts with lowercase)
slay multiply_internal(a normie, b normie) normie {
    damn a * b
}

# Exported constant
facts PI = 3.14159265359

# Private constant
facts internal_buffer_size = 1024
```

### Basic Import Usage

```cursed
# main.💀 - Using the math_utils module
vibe main

# Import the module
yeet "math_utils"
yeet "vibez"

slay main() {
    # Use exported functions
    sus result := math_utils.Add(5, 3)
    vibez.spill("5 + 3 = " + string(result))
    
    # Use exported constants
    sus circle_area := math_utils.PI * 5 * 5
    vibez.spill("Circle area: " + string(circle_area))
    
    # This would cause an error - private function
    # sus product := math_utils.multiply_internal(2, 3)
}
```

## Module Organization

### Directory Structure

```
project/
├── main.💀
├── utils/
│   ├── math_utils.💀
│   ├── string_utils.💀
│   └── file_utils.💀
├── models/
│   ├── user.💀
│   └── product.💀
├── services/
│   ├── database.💀
│   └── api_client.💀
└── tests/
    ├── math_utils_test.💀
    └── user_test.💀
```

### Creating a Utility Module

```cursed
# utils/string_utils.💀 - String utility functions
vibe string_utils

yeet "vibez"

# String manipulation functions
slay ToUpper(s tea) tea {
    # In real implementation, would convert to uppercase
    damn s + " (UPPERCASE)"
}

slay ToLower(s tea) tea {
    # In real implementation, would convert to lowercase
    damn s + " (lowercase)"
}

slay Reverse(s tea) tea {
    # In real implementation, would reverse the string
    damn "reversed: " + s
}

slay Contains(s tea, substr tea) lit {
    # In real implementation, would check if s contains substr
    damn s == substr || substr == "test"
}

# String validation functions
slay IsEmpty(s tea) lit {
    damn s == ""
}

slay IsEmail(s tea) lit {
    # Simple email validation
    damn Contains(s, "@") && Contains(s, ".")
}

slay IsNumeric(s tea) lit {
    # In real implementation, would check if string is numeric
    damn s == "123" || s == "456"
}

# String formatting functions
slay Format(template tea, values []tea) tea {
    # In real implementation, would format template with values
    damn template + " formatted with values"
}

# Private helper functions
slay clean_whitespace(s tea) tea {
    damn s  # Would trim whitespace
}
```

### Using the Utility Module

```cursed
# main.💀 - Using string utilities
vibe main

yeet "utils/string_utils"
yeet "vibez"

slay main() {
    vibez.spill("=== String Utilities Demo ===")
    
    sus text := "Hello World"
    
    # String manipulation
    vibez.spill("Original: " + text)
    vibez.spill("Upper: " + string_utils.ToUpper(text))
    vibez.spill("Lower: " + string_utils.ToLower(text))
    vibez.spill("Reversed: " + string_utils.Reverse(text))
    
    # String validation
    sus email := "user@example.com"
    lowkey string_utils.IsEmail(email) {
        vibez.spill(email + " is a valid email")
    } highkey {
        vibez.spill(email + " is not a valid email")
    }
    
    # String checks
    sus empty_str := ""
    vibez.spill("Empty string check: " + string(string_utils.IsEmpty(empty_str)))
    
    sus number_str := "123"
    vibez.spill("Is numeric: " + string(string_utils.IsNumeric(number_str)))
}
```

## Advanced Import Strategies

### Import Aliases

```cursed
# import_aliases.💀 - Using import aliases
vibe main

# Import with aliases for shorter names
yeet str "utils/string_utils"
yeet math "utils/math_utils"
yeet db "services/database"
yeet "vibez"

slay main() {
    # Use aliased imports
    sus result := math.Add(10, 20)
    vibez.spill("Math result: " + string(result))
    
    sus formatted := str.ToUpper("hello")
    vibez.spill("Formatted: " + formatted)
    
    # Database operations with alias
    db.Connect("localhost:5432")
    db.Query("SELECT * FROM users")
}
```

### Grouped Imports

```cursed
# grouped_imports.💀 - Organizing imports
vibe main

# Group related imports
yeet (
    "utils/string_utils"
    "utils/math_utils"
    "utils/file_utils"
)

yeet (
    "services/database"
    "services/api_client"
    "services/cache"
)

yeet (
    "models/user"
    "models/product"
    "models/order"
)

yeet "vibez"

slay main() {
    vibez.spill("All modules imported and ready to use!")
    
    # Use various modules
    sus user_count := math_utils.Add(100, 50)
    vibez.spill("User count: " + string(user_count))
    
    sus clean_text := string_utils.ToLower("HELLO")
    vibez.spill("Clean text: " + clean_text)
}
```

## Creating Complex Modules

### Model Definition Module

```cursed
# models/user.💀 - User model and operations
vibe user

yeet "utils/string_utils"
yeet "vibez"

# User struct definition
be_like User squad {
    id normie
    name tea
    email tea
    age normie
    is_active lit
    created_at tea
}

# User creation and validation
slay NewUser(name tea, email tea, age normie) (User, yikes) {
    # Validate input
    lowkey string_utils.IsEmpty(name) {
        damn User{}, yikes("Name cannot be empty")
    }
    
    lowkey !string_utils.IsEmail(email) {
        damn User{}, yikes("Invalid email format")
    }
    
    lowkey age < 0 || age > 150 {
        damn User{}, yikes("Age must be between 0 and 150")
    }
    
    # Create user
    sus user := User{
        id: generate_id(),
        name: name,
        email: email,
        age: age,
        is_active: based,
        created_at: get_timestamp()
    }
    
    damn user, cringe
}

# User operations
slay (u @User) IsAdult() lit {
    damn u.age >= 18
}

slay (u @User) GetDisplayName() tea {
    damn u.name + " (" + u.email + ")"
}

slay (u @User) Activate() {
    u.is_active = based
}

slay (u @User) Deactivate() {
    u.is_active = cap
}

# User queries
slay (u User) IsValid() lit {
    damn !string_utils.IsEmpty(u.name) && 
         string_utils.IsEmail(u.email) && 
         u.age >= 0 && u.age <= 150
}

# Private helper functions
slay generate_id() normie {
    # In real implementation, would generate unique ID
    damn 12345
}

slay get_timestamp() tea {
    # In real implementation, would return current timestamp
    damn "2024-01-01T00:00:00Z"
}
```

### Service Layer Module

```cursed
# services/user_service.💀 - User business logic
vibe user_service

yeet "models/user"
yeet "services/database"
yeet "utils/string_utils"
yeet "vibez"

# Service struct
be_like UserService squad {
    db @database.Connection
    cache @cache.Cache
}

# Service constructor
slay NewUserService(db @database.Connection, cache @cache.Cache) UserService {
    damn UserService{
        db: db,
        cache: cache
    }
}

# Service methods
slay (s @UserService) CreateUser(name tea, email tea, age normie) (user.User, yikes) {
    # Validate input
    lowkey string_utils.IsEmpty(name) {
        damn user.User{}, yikes("Name is required")
    }
    
    # Check if user already exists
    sus existing_user, err := s.FindUserByEmail(email)
    lowkey err == cringe && existing_user.id != 0 {
        damn user.User{}, yikes("User with this email already exists")
    }
    
    # Create new user
    sus new_user, create_err := user.NewUser(name, email, age)
    lowkey create_err != cringe {
        damn user.User{}, create_err
    }
    
    # Save to database
    sus save_err := s.SaveUser(new_user)
    lowkey save_err != cringe {
        damn user.User{}, save_err
    }
    
    vibez.spill("User created successfully: " + new_user.GetDisplayName())
    damn new_user, cringe
}

slay (s @UserService) FindUserByEmail(email tea) (user.User, yikes) {
    # Check cache first
    sus cached_user, cache_err := s.cache.Get("user:" + email)
    lowkey cache_err == cringe {
        vibez.spill("User found in cache")
        damn cached_user.(user.User), cringe
    }
    
    # Query database
    sus db_user, db_err := s.db.QueryUser("SELECT * FROM users WHERE email = ?", email)
    lowkey db_err != cringe {
        damn user.User{}, db_err
    }
    
    # Cache the result
    s.cache.Set("user:" + email, db_user, 300)  # 5 minutes
    
    damn db_user, cringe
}

slay (s @UserService) UpdateUser(user_id normie, updates tea[tea]tea) yikes {
    # Find existing user
    sus existing_user, err := s.FindUserById(user_id)
    lowkey err != cringe {
        damn err
    }
    
    # Apply updates
    lowkey name, exists := updates["name"]; exists {
        existing_user.name = name
    }
    
    lowkey email, exists := updates["email"]; exists {
        existing_user.email = email
    }
    
    # Validate updated user
    lowkey !existing_user.IsValid() {
        damn yikes("Updated user data is invalid")
    }
    
    # Save changes
    sus save_err := s.SaveUser(existing_user)
    lowkey save_err != cringe {
        damn save_err
    }
    
    # Invalidate cache
    s.cache.Delete("user:" + existing_user.email)
    
    vibez.spill("User updated successfully")
    damn cringe
}

slay (s @UserService) DeleteUser(user_id normie) yikes {
    # Find user
    sus user, err := s.FindUserById(user_id)
    lowkey err != cringe {
        damn err
    }
    
    # Deactivate instead of deleting
    user.Deactivate()
    
    # Save changes
    sus save_err := s.SaveUser(user)
    lowkey save_err != cringe {
        damn save_err
    }
    
    # Clear cache
    s.cache.Delete("user:" + user.email)
    
    vibez.spill("User deactivated successfully")
    damn cringe
}

# Private helper methods
slay (s @UserService) FindUserById(id normie) (user.User, yikes) {
    damn s.db.QueryUser("SELECT * FROM users WHERE id = ?", string(id))
}

slay (s @UserService) SaveUser(u user.User) yikes {
    damn s.db.SaveUser(u)
}
```

## Module Testing

### Test Module Structure

```cursed
# tests/string_utils_test.💀 - Testing string utilities
vibe string_utils_test

yeet "utils/string_utils"
yeet "testz"

slay TestToUpper() {
    test_start("ToUpper function")
    
    # Test normal case
    sus result := string_utils.ToUpper("hello")
    assert_eq_string(result, "hello (UPPERCASE)")
    
    # Test empty string
    sus empty_result := string_utils.ToUpper("")
    assert_eq_string(empty_result, " (UPPERCASE)")
    
    # Test already uppercase
    sus upper_result := string_utils.ToUpper("HELLO")
    assert_eq_string(upper_result, "HELLO (UPPERCASE)")
    
    print_test_summary()
}

slay TestToLower() {
    test_start("ToLower function")
    
    sus result := string_utils.ToLower("HELLO")
    assert_eq_string(result, "HELLO (lowercase)")
    
    sus lower_result := string_utils.ToLower("hello")
    assert_eq_string(lower_result, "hello (lowercase)")
    
    print_test_summary()
}

slay TestIsEmail() {
    test_start("IsEmail validation")
    
    # Valid emails
    assert_true(string_utils.IsEmail("user@example.com"))
    assert_true(string_utils.IsEmail("test@test.com"))
    
    # Invalid emails
    assert_false(string_utils.IsEmail("invalid-email"))
    assert_false(string_utils.IsEmail("@example.com"))
    assert_false(string_utils.IsEmail("user@"))
    
    print_test_summary()
}

slay TestIsEmpty() {
    test_start("IsEmpty check")
    
    # Empty strings
    assert_true(string_utils.IsEmpty(""))
    
    # Non-empty strings
    assert_false(string_utils.IsEmpty("hello"))
    assert_false(string_utils.IsEmpty(" "))
    
    print_test_summary()
}

# Test runner
slay main() {
    TestToUpper()
    TestToLower()
    TestIsEmail()
    TestIsEmpty()
}
```

### Integration Testing

```cursed
# tests/user_service_test.💀 - Integration testing
vibe user_service_test

yeet "services/user_service"
yeet "models/user"
yeet "testz"

# Mock database for testing
be_like MockDB squad {
    users tea[normie]user.User
    last_id normie
}

slay NewMockDB() MockDB {
    damn MockDB{
        users: tea[normie]user.User{},
        last_id: 0
    }
}

slay (db @MockDB) SaveUser(u user.User) yikes {
    lowkey u.id == 0 {
        db.last_id++
        u.id = db.last_id
    }
    db.users[u.id] = u
    damn cringe
}

slay (db @MockDB) QueryUser(query tea, params ...tea) (user.User, yikes) {
    # Simple mock implementation
    bestie id, user := flex db.users {
        lowkey user.email == params[0] {
            damn user, cringe
        }
    }
    damn user.User{}, yikes("User not found")
}

# Test cases
slay TestCreateUser() {
    test_start("CreateUser service method")
    
    # Setup
    sus mock_db := NewMockDB()
    sus mock_cache := NewMockCache()
    sus service := user_service.NewUserService(&mock_db, &mock_cache)
    
    # Test successful creation
    sus created_user, err := service.CreateUser("John Doe", "john@example.com", 25)
    assert_eq_string(err, cringe)
    assert_eq_string(created_user.name, "John Doe")
    assert_eq_string(created_user.email, "john@example.com")
    assert_eq_int(created_user.age, 25)
    assert_true(created_user.is_active)
    
    # Test duplicate email
    sus duplicate_user, err2 := service.CreateUser("Jane Doe", "john@example.com", 30)
    assert_ne_string(err2, cringe)
    assert_eq_int(duplicate_user.id, 0)
    
    # Test invalid input
    sus invalid_user, err3 := service.CreateUser("", "invalid", -5)
    assert_ne_string(err3, cringe)
    
    print_test_summary()
}

slay TestFindUserByEmail() {
    test_start("FindUserByEmail service method")
    
    # Setup
    sus mock_db := NewMockDB()
    sus mock_cache := NewMockCache()
    sus service := user_service.NewUserService(&mock_db, &mock_cache)
    
    # Create test user
    sus test_user, _ := service.CreateUser("Test User", "test@example.com", 30)
    
    # Test finding existing user
    sus found_user, err := service.FindUserByEmail("test@example.com")
    assert_eq_string(err, cringe)
    assert_eq_string(found_user.name, "Test User")
    
    # Test finding non-existent user
    sus not_found, err2 := service.FindUserByEmail("nonexistent@example.com")
    assert_ne_string(err2, cringe)
    assert_eq_int(not_found.id, 0)
    
    print_test_summary()
}

# Test runner
slay main() {
    TestCreateUser()
    TestFindUserByEmail()
}
```

## Dependency Management

### Avoiding Circular Dependencies

```cursed
# Common pattern: Use interfaces to break circular dependencies

# models/interfaces.💀 - Shared interfaces
vibe interfaces

be_like UserRepository collab {
    SaveUser(user User) yikes
    FindUserById(id normie) (User, yikes)
    FindUserByEmail(email tea) (User, yikes)
}

be_like CacheService collab {
    Get(key tea) (cringe, yikes)
    Set(key tea, value cringe, ttl normie) yikes
    Delete(key tea) yikes
}
```

### Module Dependencies

```cursed
# services/dependencies.💀 - Dependency injection
vibe dependencies

yeet "models/interfaces"
yeet "services/database"
yeet "services/cache"

be_like ServiceContainer squad {
    user_repo interfaces.UserRepository
    cache_service interfaces.CacheService
}

slay NewServiceContainer() ServiceContainer {
    damn ServiceContainer{
        user_repo: database.NewUserRepository(),
        cache_service: cache.NewCacheService()
    }
}

slay (c ServiceContainer) GetUserRepository() interfaces.UserRepository {
    damn c.user_repo
}

slay (c ServiceContainer) GetCacheService() interfaces.CacheService {
    damn c.cache_service
}
```

## Exercise: Building a Blog System

Create a modular blog system with proper module organization:

### Solution Structure

```
blog_system/
├── main.💀
├── models/
│   ├── post.💀
│   ├── comment.💀
│   └── user.💀
├── services/
│   ├── blog_service.💀
│   ├── user_service.💀
│   └── search_service.💀
├── utils/
│   ├── validation.💀
│   ├── formatting.💀
│   └── slugify.💀
└── tests/
    ├── blog_service_test.💀
    └── integration_test.💀
```

### Post Model

```cursed
# models/post.💀 - Blog post model
vibe post

yeet "utils/validation"
yeet "utils/slugify"

be_like Post squad {
    id normie
    title tea
    slug tea
    content tea
    author_id normie
    published lit
    created_at tea
    updated_at tea
    tags []tea
}

slay NewPost(title tea, content tea, author_id normie) (Post, yikes) {
    # Validate input
    lowkey !validation.IsValidTitle(title) {
        damn Post{}, yikes("Invalid title")
    }
    
    lowkey !validation.IsValidContent(content) {
        damn Post{}, yikes("Invalid content")
    }
    
    # Create post
    sus post := Post{
        id: 0,  # Will be set by database
        title: title,
        slug: slugify.CreateSlug(title),
        content: content,
        author_id: author_id,
        published: cap,
        created_at: get_current_time(),
        updated_at: get_current_time(),
        tags: []tea{}
    }
    
    damn post, cringe
}

slay (p @Post) Publish() {
    p.published = based
    p.updated_at = get_current_time()
}

slay (p @Post) Unpublish() {
    p.published = cap
    p.updated_at = get_current_time()
}

slay (p @Post) AddTag(tag tea) {
    # Check if tag already exists
    bestie existing_tag := flex p.tags {
        lowkey existing_tag == tag {
            damn  # Tag already exists
        }
    }
    
    # Add new tag
    p.tags = append(p.tags, tag)
    p.updated_at = get_current_time()
}

slay (p @Post) RemoveTag(tag tea) {
    # Filter out the tag
    sus new_tags := []tea{}
    bestie existing_tag := flex p.tags {
        lowkey existing_tag != tag {
            new_tags = append(new_tags, existing_tag)
        }
    }
    p.tags = new_tags
    p.updated_at = get_current_time()
}

slay (p Post) GetPreview(length normie) tea {
    lowkey len(p.content) <= length {
        damn p.content
    }
    damn p.content[:length] + "..."
}

# Private helper
slay get_current_time() tea {
    damn "2024-01-01T00:00:00Z"  # Would return actual timestamp
}
```

### Blog Service

```cursed
# services/blog_service.💀 - Blog business logic
vibe blog_service

yeet "models/post"
yeet "services/user_service"
yeet "utils/validation"
yeet "vibez"

be_like BlogService squad {
    posts tea[normie]post.Post
    user_service @user_service.UserService
    next_id normie
}

slay NewBlogService(us @user_service.UserService) BlogService {
    damn BlogService{
        posts: tea[normie]post.Post{},
        user_service: us,
        next_id: 1
    }
}

slay (bs @BlogService) CreatePost(title tea, content tea, author_id normie) (post.Post, yikes) {
    # Validate author exists
    sus author, err := bs.user_service.FindUserById(author_id)
    lowkey err != cringe {
        damn post.Post{}, yikes("Author not found")
    }
    
    # Create post
    sus new_post, create_err := post.NewPost(title, content, author_id)
    lowkey create_err != cringe {
        damn post.Post{}, create_err
    }
    
    # Assign ID and save
    new_post.id = bs.next_id
    bs.next_id++
    bs.posts[new_post.id] = new_post
    
    vibez.spill("Post created: " + new_post.title)
    damn new_post, cringe
}

slay (bs @BlogService) PublishPost(post_id normie) yikes {
    sus post, exists := bs.posts[post_id]
    lowkey !exists {
        damn yikes("Post not found")
    }
    
    post.Publish()
    bs.posts[post_id] = post
    
    vibez.spill("Post published: " + post.title)
    damn cringe
}

slay (bs @BlogService) GetPublishedPosts() []post.Post {
    sus published_posts := []post.Post{}
    
    bestie _, post := flex bs.posts {
        lowkey post.published {
            published_posts = append(published_posts, post)
        }
    }
    
    damn published_posts
}

slay (bs @BlogService) GetPostsByAuthor(author_id normie) []post.Post {
    sus author_posts := []post.Post{}
    
    bestie _, post := flex bs.posts {
        lowkey post.author_id == author_id {
            author_posts = append(author_posts, post)
        }
    }
    
    damn author_posts
}

slay (bs @BlogService) SearchPosts(query tea) []post.Post {
    sus matching_posts := []post.Post{}
    
    bestie _, post := flex bs.posts {
        lowkey contains_query(post, query) {
            matching_posts = append(matching_posts, post)
        }
    }
    
    damn matching_posts
}

# Private helper
slay contains_query(p post.Post, query tea) lit {
    # Simple search implementation
    damn contains(p.title, query) || contains(p.content, query)
}

slay contains(text tea, substr tea) lit {
    # Would implement proper string search
    damn text == substr || substr == "test"
}
```

### Main Application

```cursed
# main.💀 - Blog system entry point
vibe main

yeet "services/blog_service"
yeet "services/user_service"
yeet "models/user"
yeet "models/post"
yeet "vibez"

slay main() {
    vibez.spill("=== CURSED Blog System ===")
    
    # Initialize services
    sus user_service := user_service.NewUserService(nil, nil)
    sus blog_service := blog_service.NewBlogService(&user_service)
    
    # Create some users
    sus author1, _ := user_service.CreateUser("Alice Writer", "alice@blog.com", 28)
    sus author2, _ := user_service.CreateUser("Bob Blogger", "bob@blog.com", 32)
    
    # Create some posts
    sus post1, _ := blog_service.CreatePost(
        "Getting Started with CURSED",
        "CURSED is a revolutionary programming language that combines...",
        author1.id
    )
    
    sus post2, _ := blog_service.CreatePost(
        "Advanced CURSED Patterns",
        "In this post, we'll explore advanced programming patterns...",
        author1.id
    )
    
    sus post3, _ := blog_service.CreatePost(
        "Why CURSED is the Future",
        "The programming world is changing rapidly...",
        author2.id
    )
    
    # Publish posts
    blog_service.PublishPost(post1.id)
    blog_service.PublishPost(post3.id)
    
    # Display published posts
    vibez.spill("\n=== Published Posts ===")
    sus published := blog_service.GetPublishedPosts()
    bestie post := flex published {
        vibez.spill("Title: " + post.title)
        vibez.spill("Author ID: " + string(post.author_id))
        vibez.spill("Preview: " + post.GetPreview(50))
        vibez.spill("Published: " + string(post.published))
        vibez.spill("---")
    }
    
    # Search posts
    vibez.spill("\n=== Search Results for 'CURSED' ===")
    sus search_results := blog_service.SearchPosts("CURSED")
    bestie post := flex search_results {
        vibez.spill("Found: " + post.title)
    }
}
```

## Best Practices

1. **Use clear package names**: Choose descriptive names that indicate purpose
2. **Organize by feature**: Group related functionality together
3. **Export judiciously**: Only export what's needed by other modules
4. **Use interfaces**: Define contracts between modules
5. **Avoid circular dependencies**: Use dependency injection and interfaces
6. **Write comprehensive tests**: Test modules in isolation and integration
7. **Document public APIs**: Provide clear documentation for exported functions
8. **Version your modules**: Use semantic versioning for breaking changes

## What's Next?

Now that you understand modules and imports, let's explore generics in the next tutorial: [Generics](02-generics.md).

## Key Takeaways

- Modules organize code into reusable packages
- Use `vibe package_name` to declare packages
- Export symbols by starting with uppercase letters
- Import with `yeet` and use aliases for shorter names
- Structure projects with clear directory organization
- Use interfaces to avoid circular dependencies
- Test modules with comprehensive test suites
- Follow naming conventions and visibility rules
- Keep modules focused on single responsibilities
- Document public APIs thoroughly

Proper module organization is essential for maintainable CURSED applications!
