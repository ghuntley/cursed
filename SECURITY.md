# Security Policy

## Supported Versions

We actively provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them responsibly:

### Email
Send details to: **security@cursedlang.org**

### Encrypted Communication
For sensitive vulnerabilities, use our PGP key:
- Key ID: `CURSED-SEC-2025`
- Fingerprint: `[Will be generated and published]`
- Download: `https://cursedlang.org/security/pgp-key.asc`

### What to Include
Please include the following information:
- Type of issue (buffer overflow, privilege escalation, etc.)
- Full paths of source file(s) related to the manifestation
- Location of affected source code (tag/branch/commit or direct URL)
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact assessment and affected versions

## Response Timeline

| Severity | Response Time | Fix Timeline |
|----------|---------------|--------------|
| Critical | 4 hours       | 24-48 hours  |
| High     | 24 hours      | 1-7 days     |
| Medium   | 72 hours      | 2-4 weeks    |
| Low      | 1 week        | Next release |

## Disclosure Process

1. **Report Received** - We acknowledge receipt within 24 hours
2. **Initial Assessment** - We assess severity and impact within 72 hours  
3. **Investigation** - We investigate and develop fixes
4. **Coordination** - We coordinate with reporter on disclosure timeline
5. **Fix Release** - We release patches and security advisories
6. **Public Disclosure** - Full details published after fixes are available

### Timeline
- **Day 0**: Vulnerability reported
- **Day 1-3**: Initial assessment and acknowledgment
- **Day 3-30**: Investigation and fix development
- **Day 30-90**: Coordinated disclosure (standard timeline)
- **Day 90+**: Public disclosure (responsible disclosure deadline)

## Security Best Practices for CURSED

### Memory Safety
CURSED provides memory safety through:
- Automatic bounds checking for arrays
- Garbage collection preventing use-after-free
- Type system preventing buffer overflows
- Stack overflow protection

### Secure Coding Guidelines
When writing CURSED applications:

1. **Input Validation**
```cursed
yeet "stringz"

slay validate_input(input tea) lit {
    ready (len(input) == 0) { damn unlit }
    ready (len(input) > 1000) { damn unlit }  # Prevent DoS
    damn based
}
```

2. **Error Handling**
```cursed
slay safe_operation() yikes<tea> {
    ready (unsafe_condition) {
        yikes "Security violation detected"
    }
    # Safe operation here
}
```

3. **Resource Management**
```cursed
yeet "filez"

slay read_config_safely() yikes<tea> {
    sus file tea = open_file("config.txt") fam {
        when _ -> yikes "Cannot open config file"
    }
    defer close_file(file)  # Automatic cleanup
    damn read_content(file)
}
```

### Network Security
For network applications:
- Use TLS for all external communications
- Validate all input from network sources
- Implement rate limiting and timeouts
- Use secure random number generation

### Concurrency Security
When using goroutines:
- Avoid shared mutable state without synchronization
- Use channels for safe communication
- Implement timeouts to prevent deadlocks
- Validate data passed between goroutines

## Known Security Considerations

### Current Limitations
- CURSED compiler is written in Zig and inherits its security properties
- Standard library crypto modules use well-tested algorithms but implementations are new
- Cross-compilation may introduce platform-specific security considerations

### Mitigations in Place
- Memory safety through garbage collection and bounds checking
- Type system prevents many classes of vulnerabilities
- Regular security audits of standard library implementations
- Fuzzing of parser and runtime components

## Security Advisories

Security advisories will be published at:
- GitHub Security Advisories: https://github.com/ghuntley/cursed/security/advisories
- Official website: https://cursedlang.org/security/advisories
- Mailing list: security-announce@cursedlang.org

## Bug Bounty Program

We currently do not have a formal bug bounty program, but we recognize and credit security researchers who responsibly disclose vulnerabilities:

### Hall of Fame
Security researchers who have helped improve CURSED security:
- [To be populated as reports are received]

### Recognition
- Public acknowledgment in release notes and security advisories
- CVE contributor credit where applicable
- Optional link to researcher's website or social media

## Contact Information

- **Security Team Email**: security@cursedlang.org
- **General Inquiries**: hello@cursedlang.org
- **Emergency Contact**: Available to established security researchers

## Legal

This security policy follows industry best practices for responsible disclosure. We commit to:
- Not pursuing legal action against security researchers acting in good faith
- Crediting researchers who follow responsible disclosure
- Working cooperatively to understand and fix security issues

Thank you for helping keep CURSED and the community safe!
