# CURSED Community Support Workflows

## Support Level Definitions

### Critical (Response <24h)
- Compiler crashes or segfaults
- Memory safety violations
- Production deployment blocking issues
- Security vulnerabilities

### High (Response <48h)
- Performance regressions
- Standard library bugs
- Build system failures
- Cross-platform compatibility issues

### Medium (Response <72h)
- Feature requests with clear use cases
- Documentation improvements
- Developer experience enhancements
- IDE integration issues

### Low (Best effort)
- General questions
- Enhancement suggestions
- Community discussions
- Educational content requests

## Support Triage Process

### 1. Initial Triage (Within 2 hours)
- Auto-label issues based on keywords
- Assign severity level
- Tag appropriate team members
- Request additional information if needed

### 2. Investigation Phase
- Reproduce issue locally
- Identify root cause
- Estimate fix complexity
- Assign to appropriate developer

### 3. Resolution Phase
- Implement fix or provide workaround
- Test across supported platforms
- Update documentation if needed
- Close with resolution summary

## Support Templates

### Bug Report Template
```markdown
**CURSED Version:** 
**Operating System:** 
**Architecture:** 

**Expected Behavior:**
What you expected to happen.

**Actual Behavior:**
What actually happened.

**Reproduction Steps:**
1. Step one
2. Step two
3. Step three

**Code Sample:**
```cursed
// Minimal code that reproduces the issue
```

**Additional Context:**
Error messages, screenshots, etc.
```

### Feature Request Template
```markdown
**Feature Description:**
Clear description of the proposed feature.

**Use Case:**
Why is this feature needed? What problem does it solve?

**Proposed Implementation:**
How might this be implemented?

**Alternatives Considered:**
What other approaches were considered?

**Impact:**
Who would benefit from this feature?
```

## Escalation Matrix

### Level 1: Community Support
- Discord helpers and volunteers
- Common issues and FAQs
- Getting started questions

### Level 2: Contributor Support
- Active contributors and maintainers
- Complex technical issues
- Architecture discussions

### Level 3: Core Team Support
- Core maintainers and project leads
- Critical bugs and security issues
- Strategic decisions and RFC reviews

## Success Metrics

### Response Time SLAs
- Critical: 95% within 24 hours
- High: 90% within 48 hours
- Medium: 85% within 72 hours

### Resolution Quality
- First-contact resolution rate: >60%
- Community satisfaction rating: >4.5/5
- Issue reopening rate: <10%

### Community Health
- Active Discord members: Growth target 20%/month
- GitHub issue response time: Average <36 hours
- Contributor retention: >75% active after 3 months
