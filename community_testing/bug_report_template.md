# CURSED v1.0.0-rc2 Bug Report Template

**Please use this template for all bug reports during the Bug Bash. Copy and paste into a new GitHub Issue.**

---

## Bug Report

### Environment Information
- **CURSED Version**: v1.0.0-rc2 
- **Platform**: [Linux/macOS/Windows]
- **Architecture**: [x86_64/aarch64/other]
- **Zig Version**: [run `zig version` and paste output]
- **Installation Method**: [curl script/manual build/other]

### Issue Summary
**One-line description of the problem:**
<!-- Example: Compiler crashes when parsing nested generic types -->

### Expected Behavior
**What should happen:**
<!-- Example: Code should compile successfully and produce executable -->

### Actual Behavior  
**What actually happens:**
<!-- Example: Compiler panics with stack trace -->

### Reproduction Steps
**Step-by-step instructions to reproduce:**
1. 
2. 
3. 
4. 

### Minimal Code Sample
**Provide the smallest possible code that demonstrates the issue:**
```cursed
# Paste your minimal reproduction case here
# Remove any unnecessary code - keep only what's needed to trigger the bug
```

### Error Output
**Full error message, stack trace, or unexpected output:**
```
# Paste complete error output here
# Include compilation errors, runtime crashes, or incorrect results
```

### Additional Context
**Any additional information that might be relevant:**
- Related issues or similar problems
- Workarounds you've discovered
- When the issue first appeared
- System-specific details (environment variables, shell, etc.)

### Severity Assessment
**Select the severity level that best describes this issue:**

- [ ] **Critical (P0)** - Crashes, data corruption, security vulnerability, compiler hangs
- [ ] **High (P1)** - Wrong results, major feature broken, significant performance regression
- [ ] **Medium (P2)** - Minor incorrect behavior, usability issue, non-critical feature broken
- [ ] **Low (P3)** - Documentation issue, cosmetic problem, enhancement request

### Impact Assessment
**How does this affect your testing:**
- [ ] Blocks further testing completely
- [ ] Prevents testing specific features
- [ ] Minor inconvenience but testing can continue
- [ ] Cosmetic issue, no functional impact

### Testing Checklist
**Confirm you've done the following before reporting:**
- [ ] Reproduced the issue on a clean installation
- [ ] Tested with multiple inputs or scenarios
- [ ] Verified using the latest rc2 build
- [ ] Searched existing issues for duplicates
- [ ] Included minimal reproduction case
- [ ] Provided complete error output

### Component Classification
**Which component seems to be affected (check all that apply):**
- [ ] Compiler (lexer, parser, code generation)
- [ ] Runtime (interpreter, memory management, concurrency)
- [ ] Standard Library (vibez, mathz, stringz, etc.)
- [ ] Developer Tools (LSP, formatter, linter, debugger)
- [ ] Documentation (language reference, examples, guides)
- [ ] Cross-Platform (specific OS or architecture issues)

---

## For Critical Issues Only

### Crash Information
**If this is a crash, please provide:**
- Core dump location (if available): 
- Valgrind output (if applicable):
- System resource usage at time of crash:

### Security Implications  
**If this might be security-related:**
- [ ] This could expose sensitive information
- [ ] This could allow arbitrary code execution  
- [ ] This could cause denial of service
- [ ] This could bypass security restrictions

**Note: For security issues, consider reporting privately first via security@cursedlang.org**

---

## Reporter Information (Optional)

### Your Experience
- [ ] New to CURSED (first time using)
- [ ] Basic familiarity (used for small projects)
- [ ] Intermediate (used for medium projects)
- [ ] Advanced (extensive CURSED experience)

### Testing Focus
**What areas are you primarily testing:**
- [ ] Basic language features
- [ ] Standard library functionality  
- [ ] Developer tools integration
- [ ] Cross-platform compatibility
- [ ] Performance and memory usage
- [ ] Edge cases and error conditions

### Contribution Interest
- [ ] I'd be interested in helping fix this issue
- [ ] I can provide additional testing for the fix
- [ ] I can help with documentation updates
- [ ] I'm available for follow-up questions

---

## Thank You!

Your bug report helps make CURSED better for everyone. The development team will:

1. **Acknowledge** your report within SLA timeframes (24h for critical, 48h for high priority)
2. **Investigate** and may ask for additional information
3. **Fix** the issue and request your help testing the resolution
4. **Credit** your contribution in release notes

**Follow-up**: Watch this issue for updates and be prepared to test fixes when they're ready.

---

**Bug Bash Leaderboard**: Quality bug reports with good reproduction cases earn recognition in our community leaderboard!

**Need Help?** Join us in Discord #bug-bash channel for real-time assistance with bug reporting.
