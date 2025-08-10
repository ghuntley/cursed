yeet "testz"
yeet "stat_flexin"

test_start("Secure Metrics Export Test")

fr fr Test basic metrics collection
sus metrics_data dict = GetAllMetrics()
assert_true(len(metrics_data) > 0)

fr fr Test Prometheus export
sus prometheus_output tea = ExportMetricsAsPrometheus()
assert_true(len(prometheus_output) > 0)

fr fr Verify no injection vulnerabilities in output
assert_false(contains(prometheus_output, "<script>"))
assert_false(contains(prometheus_output, "DROP TABLE"))
assert_false(contains(prometheus_output, "\n# MALICIOUS"))

vibez.spill("🔒 Metrics security test completed")
print_test_summary()
