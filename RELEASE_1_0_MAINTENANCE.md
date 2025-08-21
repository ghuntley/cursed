# CURSED v1.0 Maintenance Branch - Oracle Post-Launch

## Release/1.0 Branch Policy

This branch maintains the stable CURSED v1.0 release for production users.

### Branch Purpose
- **Security patches** for production deployments
- **Critical bug fixes** that affect core functionality  
- **Documentation updates** for v1.0 features
- **Performance optimizations** without breaking changes

### What Goes in v1.0.x Patches
✅ **Security vulnerabilities**
✅ **Crash fixes and memory safety**  
✅ **Compiler correctness bugs**
✅ **Standard library bug fixes**
✅ **Documentation corrections**
✅ **Build system fixes for supported platforms**

❌ **New language features**
❌ **Breaking API changes**
❌ **Experimental standard library modules**
❌ **Non-critical performance improvements**

### Patch Release Cadence
- **Critical security fixes**: Immediate release (within 24 hours)
- **High-priority bugs**: Weekly patch releases (Tuesdays)
- **Regular maintenance**: Monthly patch releases (first Tuesday)
- **End-of-life**: 18 months from v1.0.0 release

### Current Release Status
- **Version**: v1.0.0
- **Release Date**: August 21, 2025
- **Next Patch**: v1.0.1 (scheduled for August 27, 2025)
- **Support Until**: February 21, 2027

### Patch Version History
| Version | Date | Type | Description |
|---------|------|------|-------------|
| v1.0.0  | 2025-08-21 | GA | Initial stable release |

### Merge Policy
1. **All patches must target v1.1+ first** (no direct commits to release/1.0)
2. **Cherry-pick approved fixes** from main development branch
3. **Require approval** from 2+ core maintainers
4. **Full CI validation** before merge
5. **Security review** for all changes

### Testing Requirements
- [ ] All existing tests pass
- [ ] Memory safety validation (valgrind)
- [ ] Cross-platform compilation
- [ ] Standard library regression tests
- [ ] Performance baseline maintenance

### Release Process
1. Cherry-pick commits from main branch
2. Update version number in build.zig  
3. Run comprehensive test suite
4. Tag release with v1.0.x
5. Generate release notes
6. Deploy to package repositories
7. Notify community via Discord and blog

### Community Support
- **Discord**: #v1-0-support channel for users on stable release
- **GitHub**: Issues labeled "v1.0-backport" for patch candidates
- **Documentation**: Maintained separately for v1.0 features
- **Migration Path**: Clear upgrade guidance for v1.1+ features
