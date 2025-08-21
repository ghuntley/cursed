# CURSED Contribution Guidelines

## Getting Started

### Prerequisites
- Zig 0.13+ installed
- Git and GitHub account
- Discord for community discussions
- Basic familiarity with CURSED syntax

### First-Time Contributors
1. Join our [Discord server](https://discord.gg/cursed-lang)
2. Read the [Code of Conduct](CODE_OF_CONDUCT.md)
3. Browse [good-first-issues](https://github.com/ghuntley/cursed/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
4. Set up development environment with `direnv allow`

## Issue Labeling System

### Priority Labels
- **p0-critical** - Immediate attention, blocks releases
- **p1-high** - Important, next sprint priority
- **p2-medium** - Normal priority, planned work
- **p3-low** - Nice to have, community contributions welcome

### Type Labels
- **bug** - Something is broken
- **enhancement** - New feature or improvement
- **documentation** - Docs improvements
- **performance** - Performance optimization
- **refactor** - Code cleanup or restructuring

### Difficulty Labels
- **good-first-issue** - Perfect for newcomers (1-4 hours)
- **easy** - Requires basic CURSED knowledge (1-2 days)
- **intermediate** - Moderate complexity (3-5 days)
- **advanced** - Deep system knowledge required (1+ weeks)

### Component Labels
- **compiler** - Core compiler changes
- **stdlib** - Standard library modules
- **runtime** - Runtime system and GC
- **tooling** - LSP, formatter, linter
- **build-system** - Zig build configuration
- **cross-platform** - Platform-specific issues

### Special Labels
- **help-wanted** - Community contributions encouraged
- **mentor-available** - Maintainer will provide guidance
- **breaking-change** - Requires version bump
- **security** - Security-related fixes

## Contribution Process

### 1. Issue Discussion
- Comment on issue to express interest
- Discuss approach with maintainers
- Get approval before starting major work

### 2. Development
```bash
# Fork and clone repository
git clone https://github.com/YOUR_USERNAME/cursed.git
cd cursed

# Create feature branch
git checkout -b feature/your-feature-name

# Set up development environment
direnv allow
zig build

# Make your changes
# Run tests: zig build test
# Validate: ./zig-out/bin/cursed-zig comprehensive_stdlib_test.csd
```

### 3. Pull Request
- Use descriptive PR title and description
- Reference related issues with "Fixes #123"
- Include test coverage for new features
- Update documentation as needed
- Ensure CI passes

### 4. Review Process
- Code review by at least one maintainer
- Address feedback promptly
- Squash commits before merge
- Celebrate your contribution! 🎉

## Code Standards

### CURSED Code Style
- Use `cursed-fmt` for consistent formatting
- Follow existing patterns in similar code
- Add comprehensive test coverage
- Document public APIs with examples

### Zig Code Style
- Follow Zig community conventions
- Use meaningful variable names
- Keep functions focused and small
- Add error handling for failure cases

### Commit Guidelines
```
type(scope): brief description

Longer explanation if needed

Fixes #123
```

Types: feat, fix, docs, test, refactor, perf, build, ci

## Recognition System

### Contributor Levels
- **First-time Contributor** - Badge for first merged PR
- **Regular Contributor** - 5+ merged PRs
- **Community Champion** - Active in Discord + contributions
- **Core Contributor** - Trusted with significant features
- **Maintainer** - Repository write access

### Monthly Recognition
- Contributor of the Month highlight
- Featured in newsletter and social media
- Priority access to community events
- Early access to new features

## Getting Help

### Mentorship Program
- Request mentor assignment for complex issues
- 1:1 guidance for first-time contributors
- Regular check-ins during development
- Code review focus areas

### Support Channels
- **Discord #contributors** - Real-time help
- **GitHub Discussions** - Async technical discussions
- **Office Hours** - Weekly maintainer availability
- **Pair Programming** - Schedule with core team

## Community Events

### Monthly Contributor Sync
- First Friday of each month
- Review roadmap and priorities
- Recognize recent contributions
- Plan upcoming initiatives

### Quarterly Hackathon
- Focus on specific themes (performance, tooling, etc.)
- Prizes for best contributions
- Mentor support throughout event
- Demo day for showcasing work
