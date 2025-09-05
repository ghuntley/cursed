# Contributing to CURSED

Thank you for your interest in contributing to CURSED! This guide will help you get started with contributing to the language, tools, ecosystem, and community.

## 🚀 Quick Start

1. **Join the Community** - [Discord Server](./discord.md)
2. **Read the Code of Conduct** - [Code of Conduct](./code-of-conduct.md)
3. **Choose Your Contribution Type** - See sections below
4. **Set Up Development Environment** - Follow setup guide
5. **Make Your First Contribution** - Start with good first issues

## 🎯 Types of Contributions

### 🔧 Code Contributions

**Core Language Development**
- Compiler improvements and bug fixes
- Language feature implementations
- Performance optimizations
- Cross-platform compatibility

**Standard Library**
- New modules and functions
- Bug fixes and improvements
- Documentation and examples
- Performance enhancements

**Developer Tools**
- IDE plugins and extensions
- Build system improvements
- Debugging tools
- Package manager features

**Examples and Templates**
- Code examples for learning
- Project templates
- Migration guides
- Best practice demonstrations

### 📚 Documentation

**Technical Documentation**
- Language specification updates
- API documentation
- Tutorial improvements
- Architecture guides

**Community Resources**
- Blog posts and articles
- Video tutorials
- Conference talks
- Workshop materials

### 🐛 Bug Reports and Testing

**Quality Assurance**
- Bug discovery and reporting
- Test case development
- Performance testing
- Cross-platform validation

**Security**
- Security vulnerability reports
- Security best practices
- Audit assistance
- Penetration testing

## 🛠️ Development Setup

### Prerequisites

```bash
# Required tools
- Zig 0.11+ (for building compiler)
- LLVM 16+ (for code generation)
- Git (for version control)
- Python 3.8+ (for scripts and tooling)

# Optional but recommended
- VS Code with CURSED extension
- Docker (for containerized builds)
- Valgrind (for memory testing)
```

### Environment Setup

```bash
# 1. Fork the repository on GitHub
# 2. Clone your fork
git clone https://github.com/YOUR_USERNAME/cursed.git
cd cursed

# 3. Add upstream remote
git remote add upstream https://github.com/ghuntley/cursed.git

# 4. Install development dependencies
./scripts/setup-dev-env.sh

# 5. Build the project
zig build

# 6. Run tests
zig build test
./scripts/run-all-tests.sh

# 7. Verify installation
./zig-out/bin/cursed-zig --version
```

### Development Workflow

```bash
# 1. Create feature branch
git checkout -b feature/awesome-improvement

# 2. Make your changes
# Edit code, add tests, update docs

# 3. Test your changes
zig build test                    # Unit tests
./scripts/test-examples.sh        # Example tests
valgrind ./zig-out/bin/cursed-zig simple_test.💀  # Memory tests

# 4. Format and lint
./zig-out/bin/cursed-fmt src/**/*.💀
./zig-out/bin/cursed-lint src/**/*.💀

# 5. Commit changes
git add .
git commit -m "feat: add awesome improvement"

# 6. Push and create PR
git push origin feature/awesome-improvement
# Create pull request on GitHub
```

## 📋 Contribution Guidelines

### Code Style

**CURSED Code Style**
```cursed
# Use descriptive variable names
sus user_count drip = 0
sus connection_pool []Connection = []

# Clear function signatures with error handling
slay process_user_request(request Request, timeout drip) Response yikes<tea> {
    # Validate input
    ready (request.user_id == 0) {
        yikes "Invalid user ID"
    }
    
    # Process request
    sus response Response = handle_request(request) fam {
        when NetworkError -> yikes "Network failure"
        when TimeoutError -> yikes "Request timeout"
    }
    
    damn response
}

# Use modules for organization
yeet "vibez"    # I/O operations
yeet "networkz" # Network functionality
yeet "authz"    # Authentication
```

**Zig Code Style (for compiler/tools)**
```zig
// Follow Zig standard formatting
const std = @import("std");
const ArrayList = std.ArrayList;

const TokenType = enum {
    identifier,
    number,
    string,
    keyword,
};

fn parseExpression(allocator: std.mem.Allocator, tokens: []Token) !Expression {
    // Implementation here
}
```

### Documentation Standards

**Code Documentation**
```cursed
# Module: User Authentication System
# 
# This module provides secure user authentication with support for:
# - Password-based authentication
# - Token-based sessions
# - Multi-factor authentication
# - Password reset workflows
#
# Example usage:
#   yeet "authz"
#   sus auth = authz.new_authenticator(config)
#   sus user = auth.login("username", "password")

slay login(username tea, password tea) User yikes<tea> {
    # Authenticate user with username and password
    # 
    # Parameters:
    #   username - User's login identifier
    #   password - User's plaintext password
    # 
    # Returns:
    #   User - Authenticated user object
    # 
    # Errors:
    #   "invalid_credentials" - Username/password combination invalid
    #   "account_locked" - User account temporarily locked
    #   "network_error" - Database connection failed
    
    # Implementation here
}
```

### Commit Message Format

Use [Conventional Commits](https://www.conventionalcommits.org/) format:

```
type(scope): description

[optional body]

[optional footer(s)]
```

**Types:**
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation changes
- `style` - Code style changes (formatting, etc.)
- `refactor` - Code refactoring
- `test` - Adding or updating tests
- `chore` - Maintenance tasks

**Examples:**
```
feat(compiler): add support for const generics

Implements RFC-123 for compile-time constant parameters.
This enables generic functions with constant values.

Closes #456

fix(stdlib): resolve memory leak in channel operations

The channel cleanup was not properly releasing all allocated
memory when channels were closed. Added explicit cleanup in
the channel destructor.

Fixes #789

docs(tutorial): improve web development guide

- Add WebSocket example
- Update database integration section
- Fix code formatting issues

Co-authored-by: Jane Developer <jane@example.com>
```

### Testing Requirements

**All contributions must include appropriate tests:**

```cursed
# Unit test example
yeet "testz"

test "user authentication with valid credentials" {
    sus auth = create_test_authenticator()
    sus user = auth.login("testuser", "password123") fam {
        when err -> testz.fail("Login should succeed: " + err)
    }
    
    testz.assert_eq(user.username, "testuser")
    testz.assert(user.is_authenticated())
}

test "user authentication with invalid credentials" {
    sus auth = create_test_authenticator()
    sus result = auth.login("testuser", "wrong_password")
    
    testz.assert_error(result, "invalid_credentials")
}
```

**Integration test example:**
```bash
#!/bin/bash
# Test script: test_web_server_integration.sh

# Start test server
./zig-out/bin/cursed-zig examples/web-server.💀 &
SERVER_PID=$!
sleep 2

# Test endpoints
curl -f http://localhost:8080/ || exit 1
curl -f http://localhost:8080/api/status || exit 1

# Cleanup
kill $SERVER_PID
echo "Integration tests passed!"
```

## 📝 Pull Request Process

### Before Submitting

1. **Ensure tests pass** - All existing and new tests must pass
2. **Update documentation** - Include relevant documentation updates
3. **Add changelog entry** - Update CHANGELOG.md if needed
4. **Check performance** - Verify no significant performance regressions
5. **Test cross-platform** - Ensure changes work on all supported platforms

### PR Template

When creating a pull request, include:

```markdown
## Description
Brief description of changes and motivation.

## Type of Change
- [ ] Bug fix (non-breaking change that fixes an issue)
- [ ] New feature (non-breaking change that adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed
- [ ] Cross-platform testing done

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests added for new functionality
- [ ] All tests pass
- [ ] Changelog updated (if needed)

## Related Issues
Closes #123
```

### Review Process

1. **Automated Checks** - CI/CD pipeline runs automatically
2. **Community Review** - Community members can review and comment
3. **Maintainer Review** - Core team member reviews technical aspects
4. **Final Approval** - Maintainer approves and merges

**Review Timeline:**
- Simple fixes: 1-3 days
- New features: 1-2 weeks
- Major changes: 2-4 weeks

## 🎯 Good First Issues

Perfect for new contributors:

### Documentation
- Fix typos and grammar in documentation
- Add missing code examples
- Improve API documentation
- Translate documentation to other languages

### Examples
- Create simple example programs
- Add comments to existing examples
- Create migration guides from other languages
- Build tutorial projects

### Testing
- Add unit tests for existing functionality
- Create integration test scenarios
- Add edge case testing
- Performance benchmark creation

### Tooling
- Improve error messages
- Add command-line options
- Create development scripts
- IDE integration improvements

## 🏆 Recognition

### Contributor Levels

**New Contributor** 🌱
- First merged pull request
- Listed in contributors

**Regular Contributor** ⭐
- 5+ merged pull requests
- Discord contributor role
- Input on project direction

**Core Contributor** 🚀
- 20+ merged pull requests
- Trusted reviewer status
- Influence on roadmap decisions

**Maintainer** 👑
- Consistent long-term contributions
- Merge access to repository
- Leadership in community

### Hall of Fame

Top contributors are recognized in:
- README.md contributors section
- Annual contributor highlight blog posts
- Conference presentation acknowledgments
- CURSED swag and merchandise

## 🐛 Bug Reports

### Security Issues

**DO NOT** report security vulnerabilities publicly. Instead:

1. Email: security@cursedlang.org
2. Include detailed reproduction steps
3. Provide impact assessment
4. Wait for acknowledgment before public disclosure

### Regular Bug Reports

Use the GitHub issue template:

```markdown
**Bug Description**
Clear description of the bug.

**Reproduction Steps**
1. Step one
2. Step two
3. See error

**Expected Behavior**
What should happen.

**Actual Behavior**
What actually happens.

**Environment**
- CURSED version:
- Operating System:
- Architecture:
- Additional context:

**Code Example**
```cursed
# Minimal code that reproduces the issue
```

**Error Output**
```
Paste error messages here
```
```

## 💡 Feature Requests

### RFC Process

For significant features:

1. **Discuss in Discord** - Gauge community interest
2. **Create RFC document** - Detailed proposal in `rfcs/` directory
3. **Community feedback** - Open discussion period
4. **Implementation** - Approved RFCs can be implemented

### Feature Request Template

```markdown
**Feature Summary**
Brief one-line summary.

**Motivation**
Why is this feature needed? What problem does it solve?

**Detailed Design**
How should this feature work? Include syntax examples.

**Drawbacks**
What are the potential downsides?

**Alternatives**
What other approaches were considered?

**Implementation Notes**
Any technical considerations for implementation.
```

## 📚 Resources

### Development Resources
- [Language Specification](../reference/language-spec.md)
- [Compiler Architecture](../reference/compiler-architecture.md)
- [Standard Library Guide](../reference/stdlib/)
- [Performance Guidelines](../reference/performance.md)

### Community Resources
- [Discord Server](./discord.md)
- [Code of Conduct](./code-of-conduct.md)
- [FAQ](./faq.md)
- [Learning Resources](../README.md)

### External Links
- [GitHub Repository](https://github.com/ghuntley/cursed)
- [Issue Tracker](https://github.com/ghuntley/cursed/issues)
- [Discussions](https://github.com/ghuntley/cursed/discussions)
- [Wiki](https://github.com/ghuntley/cursed/wiki)

---

## 🤝 Getting Help

**Stuck or have questions?**

- 💬 Ask in [Discord #help-general](./discord.md)
- 📧 Email: contribute@cursedlang.org
- 📋 Open a [GitHub Discussion](https://github.com/ghuntley/cursed/discussions)
- 📖 Check the [FAQ](./faq.md)

**Thank you for contributing to CURSED!** 🔥

Your contributions help make CURSED better for everyone. Whether you're fixing a typo, adding a feature, or helping other developers, every contribution matters.

---

*This guide is a living document. If you find areas for improvement, please contribute updates!*
