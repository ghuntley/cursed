# CURSED Bug Bash Issue Triage System

## Service Level Agreements (SLAs)

### Response Times
- **Critical (P0)**: <24 hours - Crashes, security, data corruption
- **High (P1)**: <48 hours - Major features broken, wrong results  
- **Medium (P2)**: <5 days - Minor bugs, usability issues
- **Low (P3)**: <2 weeks - Enhancements, documentation, cosmetic

### Resolution Targets
- **Critical**: 24-48 hours - Hotfix required
- **High**: 1-3 days - Include in next patch
- **Medium**: 1-2 weeks - Regular release cycle
- **Low**: Next major release - Backlog prioritization

## Issue Classification

### Severity Matrix
| Impact | Likelihood | Severity | SLA |
|--------|------------|----------|-----|
| High | High | Critical | <24h |
| High | Medium | High | <48h |
| Medium | High | High | <48h |
| Medium | Medium | Medium | <5d |
| Low | Any | Low | <2w |

### Component Labels
- `compiler`: Lexer, parser, codegen issues
- `runtime`: Interpreter, memory, concurrency
- `stdlib`: Standard library modules
- `tools`: LSP, formatter, linter, debugger
- `docs`: Documentation and examples
- `platform`: OS-specific or cross-compilation

### Workflow Labels  
- `needs-reproduction`: Requires minimal test case
- `needs-investigation`: Root cause analysis needed
- `ready-to-fix`: Well-understood, implementation ready
- `in-progress`: Currently being worked on
- `needs-testing`: Fix ready, needs validation
- `resolved`: Fixed and tested

## Automated Triage Rules

### Critical Auto-Assignment
```yaml
triggers:
  - keywords: ["crash", "segfault", "panic", "corruption"]
  - labels: ["security", "data-loss"]
actions:
  - assign: core-team
  - label: critical
  - notify: slack-critical
  - escalate: within 2 hours
```

### High Priority Routing
```yaml  
triggers:
  - keywords: ["wrong result", "broken feature", "regression"]
  - components: ["compiler", "runtime"]
actions:
  - assign: component-owner
  - label: high
  - add-to: current-milestone
```

## Issue Templates

### Bug Report Template
```markdown
---
name: Bug Report
about: Report a defect in CURSED v1.0.0-rc2
title: '[BUG] '
labels: 'bug, needs-triage'
---

## Environment
- CURSED Version: v1.0.0-rc2
- Platform: [Linux/macOS/Windows]  
- Architecture: [x86_64/aarch64]
- Zig Version: [output of `zig version`]

## Issue Summary
<!-- One-line description of the problem -->

## Expected Behavior
<!-- What should happen -->

## Actual Behavior  
<!-- What actually happens -->

## Reproduction Steps
1. 
2.
3.

## Minimal Code Sample
```cursed
# Provide minimal reproduction case
```

## Error Output
```
# Paste full error message and stack trace
```

## Additional Context
<!-- Environment details, related issues, workarounds -->

## Severity Assessment
- [ ] Critical - Crashes, data corruption, security issue
- [ ] High - Wrong results, major feature broken
- [ ] Medium - Minor incorrect behavior, usability issue  
- [ ] Low - Documentation, cosmetic, enhancement

## Testing Checklist
- [ ] Reproduced on clean installation
- [ ] Tested with multiple inputs
- [ ] Checked with latest rc2 build
- [ ] Searched for duplicate issues
```

### Feature Request Template
```markdown
---
name: Feature Request
about: Suggest enhancement for CURSED v1.0.0
title: '[FEATURE] '
labels: 'enhancement, needs-discussion'
---

## Feature Description
<!-- Clear description of proposed feature -->

## Use Case
<!-- Why is this needed? What problem does it solve? -->

## Proposed Implementation
<!-- How should this work? -->

## Code Examples
```cursed
# Show desired syntax/usage
```

## Alternatives Considered
<!-- What other approaches were evaluated? -->

## Impact Assessment
- Breaking Change: [Yes/No]
- Backward Compatibility: [Required/Nice-to-have]
- Performance Impact: [Positive/Neutral/Negative]
- Implementation Complexity: [Low/Medium/High]
```

## Triage Process

### Daily Triage Workflow
1. **New Issues Review** (15 min)
   - Apply initial labels and severity
   - Route to appropriate component owner
   - Request additional information if needed

2. **Critical Issues Check** (30 min)
   - Verify all P0 issues have owners
   - Check progress on <24h SLA items
   - Escalate blocked critical issues

3. **Active Issues Status** (20 min)  
   - Update in-progress issues
   - Move completed items to testing
   - Close resolved issues

### Weekly Triage Meeting
- **Review Metrics**: SLA compliance, resolution rates
- **Prioritize Backlog**: Medium/Low priority items
- **Resource Planning**: Capacity vs. issue volume
- **Process Improvements**: Refine classification rules

## Metrics and Reporting

### SLA Compliance Dashboard
```
Critical Issues (P0):
  Open: 2 | Avg Response: 18h | SLA Met: 95%
  
High Priority (P1): 
  Open: 8 | Avg Response: 36h | SLA Met: 87%
  
Medium Priority (P2):
  Open: 23 | Avg Response: 3.2d | SLA Met: 92%
```

### Weekly Report Template
```markdown
# Week N Bug Bash Triage Report

## Summary
- New Issues: 47 (+12 from last week)
- Resolved: 31 (66% resolution rate)
- Critical: 3 open (all within SLA)
- High: 11 open (2 approaching SLA)

## Top Issues This Week
1. [#123] Compiler crash on complex generics (P0)
2. [#145] Channel deadlock in select (P1)  
3. [#167] Memory leak in string concatenation (P1)

## SLA Performance
- Critical: 97% (target: 95%)
- High: 89% (target: 90%) ⚠️
- Medium: 94% (target: 85%)

## Action Items
- [ ] Add compiler fuzzing for generic edge cases
- [ ] Review channel implementation for race conditions
- [ ] Investigate string memory management
```

## Escalation Procedures

### Critical Issue Escalation
1. **Immediate** (0-2h): Slack notification to core team
2. **Same Day** (2-8h): Email to technical leads
3. **Next Day** (8-24h): Status meeting with project manager
4. **Beyond SLA** (24h+): Executive escalation required

### Resource Constraints
- **Too Many Critical**: Stop feature work, focus on stability
- **High Volume**: Prioritize by user impact and data
- **Expertise Gaps**: Cross-train team or bring in specialists

## Community Engagement

### Bug Bash Leaderboard
Track community contributions:
- Most bugs reported (quality weighted)
- Best reproduction cases
- Most helpful comments
- Cross-platform testing coverage

### Recognition System
- **Bug Hunter Badge**: First to find P0/P1 issues  
- **Quality Tester Badge**: Excellent reproduction cases
- **Community Helper Badge**: Assists other testers
- **Platform Champion Badge**: Tests multiple platforms

### Feedback Loop
- **Weekly Community Update**: Progress on reported issues
- **Fix Verification**: Request original reporters test fixes  
- **Process Feedback**: Regular surveys on triage experience
- **Success Stories**: Highlight impactful community contributions

## Tools and Automation

### GitHub Integration
```yaml
# .github/workflows/triage.yml
on:
  issues:
    types: [opened]
    
jobs:
  auto-triage:
    steps:
      - name: Label Critical Issues
        if: contains(github.event.issue.body, 'crash')
        run: gh issue edit ${{ github.event.issue.number }} --add-label "critical"
      
      - name: Assign Component Owner
        uses: ./.github/actions/component-routing
```

### Monitoring Alerts  
```yaml
# SLA violation alerts
critical_sla_breach:
  condition: age > 24h AND severity = critical
  notify: ["core-team-slack", "on-call-pager"]
  
high_sla_warning:  
  condition: age > 36h AND severity = high
  notify: ["team-slack"]
```

This triage system ensures rapid response to critical issues while maintaining sustainable processing of the full issue volume during the Bug Bash.
