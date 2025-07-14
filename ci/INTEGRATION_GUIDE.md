# CURSED Self-Hosting CI Integration Guide

## Quick Start

1. **Setup the CI pipeline**:
   ```bash
   bash ci/setup_self_hosting_ci.sh
   ```

2. **Test locally**:
   ```bash
   bash ci/test_self_hosting_locally.sh
   ```

3. **Integrate with CI**: Add the configuration from `ci/self_hosting_ci_integration.yml` to your `.cirrus.yml`

## Integration Steps

### Step 1: Add CI Scripts to Your Pipeline

Add these steps to your existing `.cirrus.yml` after the `test_script` section:

```yaml
self_hosting_validation_script: |
  . /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh
  devenv shell bash ci/self_hosting_validation.sh
  
bootstrap_validation_script: |
  . /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh
  devenv shell bash ci/bootstrap_validation_tests.sh
  
performance_regression_script: |
  . /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh
  devenv shell bash ci/performance_regression_detection.sh
  
comprehensive_self_hosting_script: |
  . /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh
  devenv shell bash ci/comprehensive_self_hosting_test_suite.sh
```

### Step 2: Add Artifact Collection

Add these artifact paths to collect validation reports:

```yaml
artifacts:
  path: "cursed-linux-x86_64.tar.gz"
  path: "performance_report.json"
  path: "self_hosting_validation_report.json"
```

### Step 3: Configure Environment Variables

Add these environment variables for configuration:

```yaml
env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1
  TIMEOUT_SECONDS: 300
  REGRESSION_THRESHOLD: 1.5
```

## Testing Strategy

### Local Testing Commands

```bash
# Quick setup and validation
bash ci/setup_self_hosting_ci.sh
bash ci/test_self_hosting_locally.sh

# Individual component testing
bash ci/self_hosting_validation.sh
bash ci/bootstrap_validation_tests.sh
bash ci/performance_regression_detection.sh
bash ci/comprehensive_self_hosting_test_suite.sh

# Build compiler first if needed
cargo build --release --bin cursed
```

### CI Pipeline Flow

1. **Build Phase**: Compile CURSED compiler
2. **Unit Tests**: Run standard cargo tests
3. **Self-Hosting Validation**: Test compiler-compiles-compiler
4. **Bootstrap Validation**: Test all language features
5. **Performance Regression**: Monitor performance over time
6. **Comprehensive Testing**: Full self-hosting test suite
7. **Artifact Collection**: Collect reports and binaries

## Expected Outputs

### Success Indicators

- All CI scripts return exit code 0
- Performance regression within acceptable threshold
- Bootstrap tests pass in both interpretation and compilation modes
- Self-hosting validation produces identical outputs

### Generated Artifacts

- `performance_report.json`: Performance metrics and regression analysis
- `self_hosting_validation_report.json`: Validation status and results
- `cursed-linux-x86_64.tar.gz`: Compiled binaries

### Performance Baseline

The system maintains a performance baseline in `ci/performance_baseline.json` that tracks:
- Compilation time for various benchmark programs
- Execution time for compiled programs
- Memory usage during compilation and execution
- Regression detection against historical performance

## Troubleshooting

### Common Issues

1. **Compiler not found**: Ensure `cargo build --release` succeeds
2. **Timeout errors**: Increase `TIMEOUT_SECONDS` environment variable
3. **Performance regressions**: Check if actual degradation or adjust `REGRESSION_THRESHOLD`
4. **Missing dependencies**: Ensure `bc` and `jq` are available

### Debug Mode

Enable detailed logging:
```bash
export CURSED_DEBUG=1
bash ci/test_self_hosting_locally.sh
```

### Log Analysis

Check these files for debugging:
- `*_interp.out`: Interpretation mode outputs
- `*_compiled.out`: Compilation mode outputs
- `*_results.json`: Individual test results
- `performance_report.json`: Performance analysis

## Maintenance

### Updating Performance Baselines

Baselines are automatically updated when no regressions are detected. To manually update:

```bash
# After running performance tests successfully
cp performance_report.json ci/performance_baseline.json
git add ci/performance_baseline.json
git commit -m "Update performance baseline"
```

### Adding New Tests

To add new self-hosting tests:

1. Edit the appropriate test suite function in the CI scripts
2. Add test validation logic
3. Update test count tracking
4. Test locally before committing

### Script Modifications

When modifying CI scripts:

1. Test syntax: `bash -n script.sh`
2. Test functionality: `bash ci/test_self_hosting_locally.sh`
3. Update documentation as needed

## Security Considerations

- All test programs are created in temporary directories
- Test directories are automatically cleaned up
- No sensitive information is logged
- Performance data doesn't expose compiler internals

## Integration with Existing Workflows

The self-hosting CI pipeline is designed to:
- Run after existing unit tests
- Provide additional validation without breaking existing workflows
- Generate machine-readable reports for integration with other tools
- Maintain backward compatibility with existing CI configurations

## Next Steps

1. **Add to CI**: Integrate the configuration into your `.cirrus.yml`
2. **Test thoroughly**: Run local tests before committing
3. **Monitor performance**: Review performance reports regularly
4. **Update baselines**: Keep performance baselines current
5. **Document changes**: Update this guide when making modifications

---

This integration guide ensures robust self-hosting validation while maintaining production quality and catching regressions early in the development process.
