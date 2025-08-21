# CURSED v1.0.x Maintenance Lifecycle and Support Timeline

## Overview
This document defines the maintenance lifecycle, support timeline, and policies for CURSED v1.0.x releases following the v1.0.0-stable launch.

## Version Numbering Scheme

### Semantic Versioning
CURSED follows [Semantic Versioning 2.0.0](https://semver.org/):

- **MAJOR.MINOR.PATCH** (e.g., v1.0.1)
- **MAJOR**: Breaking changes, incompatible API changes
- **MINOR**: New features, backwards compatible
- **PATCH**: Bug fixes, security updates, backwards compatible

### v1.0.x Series
- **v1.0.0**: Initial stable release
- **v1.0.1-v1.0.99**: Patch releases for bug fixes and security updates
- **v1.1.0**: Next minor release with new features
- **v2.0.0**: Next major release with potential breaking changes

## Support Lifecycle Phases

### Phase 1: Active Support (18 months)
**Duration**: v1.0.0 release date + 18 months  
**End Date**: Approximately March 2026

**What's Included**:
- Regular patch releases (v1.0.x)
- Security vulnerability fixes
- Critical bug fixes
- Performance improvements
- Documentation updates
- Community support and issue triage
- New feature backports (when feasible)

**Release Frequency**:
- **Emergency patches**: Same day for P0 issues
- **Security patches**: 24-48 hours for critical vulnerabilities
- **Regular patches**: Weekly/bi-weekly as needed
- **Planned patches**: Monthly maintenance releases

### Phase 2: Extended Support (12 months)
**Duration**: 18 months after v1.0.0 release + 12 additional months  
**End Date**: Approximately March 2027

**What's Included**:
- Security vulnerability fixes only
- Critical stability fixes
- No new features or enhancements
- Limited community support
- Documentation maintenance

**Release Frequency**:
- **Security patches**: As needed for vulnerabilities
- **Critical patches**: For stability issues only
- **No regular releases**: Only emergency fixes

### Phase 3: End of Life (EOL)
**Date**: 30 months after v1.0.0 release

**What Happens**:
- No more security updates or bug fixes
- Community support transitions to newer versions
- Documentation archived but remains available
- Clear migration path to v1.1+ documented
- Final EOL notice published 6 months in advance

## Patch Release Criteria

### Automatic Patch Triggers
1. **Security Vulnerabilities** (CVE assigned or CVSS 7.0+)
2. **Data Corruption Bugs**
3. **System Crashes or Panics**
4. **Memory Safety Violations**
5. **Critical Performance Regressions** (>50% slowdown)

### Manual Patch Evaluation
- Compilation failures on supported platforms
- Standard library API breakages
- IDE/tooling integration issues
- High-impact community-reported bugs

### Patch Exclusions
- New language features
- API additions or changes
- Breaking changes (reserved for major versions)
- Performance optimizations (unless addressing regressions)
- Code style or cosmetic changes

## Release Process

### Patch Release Workflow
1. **Issue Identification and Triage** (4-24 hours)
   - Severity assessment (P0-P3)
   - Impact analysis and affected versions
   - Fix feasibility evaluation

2. **Fix Development** (24-72 hours)
   - Minimal, targeted fix implementation
   - Comprehensive regression testing
   - Security review for all changes
   - Cross-platform validation

3. **Release Preparation** (2-4 hours)
   - Version number assignment
   - Release notes preparation
   - Binary compilation for all targets
   - Package preparation and signing

4. **Release Deployment** (1-2 hours)
   - GitHub release publication
   - Package repository updates
   - Documentation updates
   - Community notifications

5. **Post-Release Validation** (24 hours)
   - Download verification
   - Installation testing across platforms
   - Community feedback monitoring
   - Success metrics tracking

### Quality Gates
- All existing tests must pass
- No new test failures introduced
- Memory safety validation with Valgrind
- Performance regression testing
- Security vulnerability scanning
- Cross-compilation verification

## Supported Platforms and Targets

### Tier 1 Platforms (Guaranteed Support)
- **Linux x86_64**: Ubuntu 20.04+, CentOS 8+, Debian 11+
- **macOS x86_64**: macOS 10.15+
- **macOS ARM64**: macOS 11.0+ (Apple Silicon)
- **Windows x86_64**: Windows 10+

### Tier 2 Platforms (Best Effort)
- **Linux ARM64**: Ubuntu 20.04+, Amazon Linux 2+
- **Linux ARM32**: Raspberry Pi OS
- **FreeBSD x86_64**: FreeBSD 13+

### Experimental Platforms
- **WebAssembly (WASI)**: Browser and server runtimes
- **Android ARM64**: Termux and native development
- **iOS ARM64**: Limited toolchain support

## Compatibility Guarantees

### Source Code Compatibility
- All valid CURSED v1.0.0 programs continue to compile and run
- Standard library API stability maintained
- No breaking changes to language syntax
- Deprecated features marked clearly with migration path

### Binary Compatibility
- Compiled binaries from v1.0.0 continue to work
- Standard library ABI compatibility maintained
- Plugin and FFI interfaces remain stable
- Cross-version library linking supported

### Migration Support
- Automated migration tools for major version upgrades
- Comprehensive migration documentation
- Side-by-side version installation support
- Legacy version containers and packages

## Community Communication

### Release Announcements
- **GitHub Releases**: Detailed changelog and download links
- **Discord**: Real-time community notifications
- **Blog**: Monthly summaries and feature highlights
- **Twitter**: Major release announcements
- **Mailing List**: Security and critical update notifications

### Documentation Updates
- Release notes with detailed change descriptions
- Migration guides for version upgrades
- Updated installation and setup instructions
- Community contribution recognition

### Feedback Channels
- GitHub Issues for bug reports
- GitHub Discussions for questions and feedback
- Discord for real-time community interaction
- Monthly community surveys for satisfaction tracking

## Metrics and Success Criteria

### Release Quality Metrics
- **Time to Fix**: P0 issues < 24 hours, P1 issues < 72 hours
- **Regression Rate**: <5% of releases introduce new issues
- **Test Coverage**: 90%+ code coverage maintained
- **Memory Safety**: Zero memory leaks in release builds

### Community Health Metrics
- **Issue Response Time**: <48 hours for initial response
- **Community Satisfaction**: >80% positive feedback
- **Adoption Rate**: Track downloads and usage statistics
- **Contribution Growth**: Active contributor count

### Security Metrics
- **Vulnerability Response**: <24 hours for critical issues
- **Disclosure Timeline**: 90-day responsible disclosure
- **Security Audit**: Annual third-party security reviews
- **Compliance**: Meet enterprise security requirements

## Enterprise Support

### Long-Term Support (LTS)
- Extended 5-year support available for enterprise customers
- Commercial support contracts with SLA guarantees
- Priority security patches and hotfixes
- Custom feature development and backporting

### Support Tiers
1. **Community**: Free, best-effort support via public channels
2. **Professional**: Paid support with 48-hour response SLA
3. **Enterprise**: Dedicated support team with 4-hour SLA
4. **Custom**: Tailored support agreements for large deployments

## Transition Planning

### v1.1 Migration Path
- Feature preview releases 3 months before v1.1.0
- Migration guide published with v1.1.0-rc1
- Side-by-side installation support
- Automated migration tooling

### v2.0 Future Planning
- Breaking changes RFC process 12 months in advance
- Community feedback collection and evaluation
- Experimental feature flags for testing new concepts
- Clear deprecation timeline and migration support

## Contact Information

### Maintenance Team
- **Lead Maintainer**: Geoffrey Huntley (@ghuntley)
- **Security Team**: security@cursedlang.org
- **Release Engineering**: releases@cursedlang.org
- **Community Management**: community@cursedlang.org

### Emergency Contacts
- **Critical Security Issues**: security@cursedlang.org
- **Infrastructure Outages**: ops@cursedlang.org
- **Enterprise Support**: enterprise@cursedlang.org

---

**Document Version**: 1.0  
**Last Updated**: August 21, 2025  
**Next Review**: February 21, 2026  

This document is a living document that will be updated as the CURSED project evolves. Changes will be announced through official channels and community feedback is welcomed.
