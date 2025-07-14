#!/bin/bash

# CURSED Performance Monitoring Deployment Script
# Production deployment script for comprehensive performance monitoring

set -e

echo "🚀 CURSED Performance Monitoring Deployment Script"
echo "=================================================="

# Configuration
CURSED_HOME=${CURSED_HOME:-"/opt/cursed"}
METRICS_DIR="${CURSED_HOME}/metrics"
LOGS_DIR="${CURSED_HOME}/logs"
CONFIG_DIR="${CURSED_HOME}/config"
DATA_DIR="${CURSED_HOME}/data"

# Create directories
echo "📁 Creating directory structure..."
mkdir -p "${METRICS_DIR}"
mkdir -p "${LOGS_DIR}"
mkdir -p "${CONFIG_DIR}"
mkdir -p "${DATA_DIR}"

# Copy configuration files
echo "📋 Installing configuration files..."
cp performance_monitoring.toml "${CONFIG_DIR}/"
chmod 644 "${CONFIG_DIR}/performance_monitoring.toml"

# Set up logging
echo "📝 Setting up logging..."
touch "${LOGS_DIR}/performance.log"
touch "${LOGS_DIR}/gc_monitor.log"
touch "${LOGS_DIR}/metrics.log"
touch "${LOGS_DIR}/alerts.log"

# Create metrics output directory
echo "📊 Setting up metrics output..."
mkdir -p "${METRICS_DIR}/exports"
mkdir -p "${METRICS_DIR}/reports"
mkdir -p "${METRICS_DIR}/alerts"

# Set permissions
echo "🔒 Setting permissions..."
chmod 755 "${CURSED_HOME}"
chmod 755 "${METRICS_DIR}"
chmod 755 "${LOGS_DIR}"
chmod 755 "${CONFIG_DIR}"
chmod 755 "${DATA_DIR}"
chmod 644 "${LOGS_DIR}"/*.log

# Build CURSED compiler with performance monitoring
echo "🔨 Building CURSED compiler with performance monitoring..."
cargo build --release --features "performance-monitoring"

# Test the performance monitoring system
echo "🧪 Testing performance monitoring system..."
echo "Running performance monitoring integration test..."
cargo run --bin cursed test_performance_monitoring.csd

# Create systemd service file (optional)
create_systemd_service() {
    echo "🐧 Creating systemd service file..."
    cat > /etc/systemd/system/cursed-metrics.service << EOF
[Unit]
Description=CURSED Performance Monitoring Service
After=network.target

[Service]
Type=simple
User=cursed
Group=cursed
WorkingDirectory=${CURSED_HOME}
ExecStart=${CURSED_HOME}/target/release/cursed --metrics-daemon --config=${CONFIG_DIR}/performance_monitoring.toml
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

# Environment variables
Environment=CURSED_HOME=${CURSED_HOME}
Environment=CURSED_CONFIG=${CONFIG_DIR}/performance_monitoring.toml
Environment=CURSED_METRICS_DIR=${METRICS_DIR}
Environment=CURSED_LOGS_DIR=${LOGS_DIR}

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ReadWritePaths=${CURSED_HOME}

[Install]
WantedBy=multi-user.target
EOF

    systemctl daemon-reload
    systemctl enable cursed-metrics.service
    echo "✅ Systemd service created and enabled"
}

# Create monitoring dashboard script
create_dashboard() {
    echo "📊 Creating monitoring dashboard..."
    cat > "${CURSED_HOME}/dashboard.sh" << 'EOF'
#!/bin/bash

# CURSED Performance Monitoring Dashboard
# Real-time monitoring dashboard for CURSED compiler performance

CURSED_HOME=${CURSED_HOME:-"/opt/cursed"}
METRICS_DIR="${CURSED_HOME}/metrics"
LOGS_DIR="${CURSED_HOME}/logs"

echo "📊 CURSED Performance Monitoring Dashboard"
echo "=========================================="

# System Information
echo "🖥️  System Information:"
echo "Date: $(date)"
echo "Uptime: $(uptime)"
echo "Memory: $(free -h | grep Mem)"
echo "CPU: $(lscpu | grep 'Model name' | cut -d: -f2 | xargs)"
echo ""

# CURSED Compiler Status
echo "🔧 CURSED Compiler Status:"
if pgrep -f "cursed.*metrics-daemon" > /dev/null; then
    echo "✅ Metrics daemon: RUNNING"
else
    echo "❌ Metrics daemon: NOT RUNNING"
fi

# Metrics Summary
echo "📈 Metrics Summary:"
if [ -f "${METRICS_DIR}/cursed_metrics.txt" ]; then
    echo "Last metrics update: $(stat -c %y ${METRICS_DIR}/cursed_metrics.txt)"
    echo "Metrics file size: $(du -h ${METRICS_DIR}/cursed_metrics.txt | cut -f1)"
else
    echo "⚠️  No metrics file found"
fi

# Recent Alerts
echo "🔔 Recent Alerts:"
if [ -f "${LOGS_DIR}/alerts.log" ]; then
    tail -10 "${LOGS_DIR}/alerts.log" | while read line; do
        echo "  $line"
    done
else
    echo "  No alerts found"
fi

# Log Summary
echo "📝 Log Summary:"
for log in performance.log gc_monitor.log metrics.log alerts.log; do
    if [ -f "${LOGS_DIR}/${log}" ]; then
        size=$(du -h "${LOGS_DIR}/${log}" | cut -f1)
        lines=$(wc -l < "${LOGS_DIR}/${log}")
        echo "  ${log}: ${size} (${lines} lines)"
    fi
done

# Performance Summary
echo "⚡ Performance Summary:"
if [ -f "${LOGS_DIR}/performance.log" ]; then
    echo "  Average compilation time: $(grep 'compilation.*completed' ${LOGS_DIR}/performance.log | tail -10 | grep -o '[0-9]*ms' | sed 's/ms//' | awk '{sum+=$1} END {print sum/NR}')ms"
    echo "  Recent errors: $(grep 'ERROR' ${LOGS_DIR}/performance.log | tail -5 | wc -l)"
fi

echo ""
echo "🔄 Auto-refresh every 30 seconds (Ctrl+C to stop)"
EOF

    chmod +x "${CURSED_HOME}/dashboard.sh"
    echo "✅ Dashboard script created at ${CURSED_HOME}/dashboard.sh"
}

# Create metrics export script
create_export_script() {
    echo "📤 Creating metrics export script..."
    cat > "${CURSED_HOME}/export_metrics.sh" << 'EOF'
#!/bin/bash

# CURSED Metrics Export Script
# Export metrics to various formats for external monitoring systems

CURSED_HOME=${CURSED_HOME:-"/opt/cursed"}
METRICS_DIR="${CURSED_HOME}/metrics"
EXPORTS_DIR="${METRICS_DIR}/exports"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

echo "📤 CURSED Metrics Export"
echo "======================="

# Export to Prometheus format
echo "🔹 Exporting to Prometheus format..."
cargo run --bin cursed -- export-metrics --format prometheus --output "${EXPORTS_DIR}/prometheus_${TIMESTAMP}.txt"

# Export to JSON format
echo "🔹 Exporting to JSON format..."
cargo run --bin cursed -- export-metrics --format json --output "${EXPORTS_DIR}/metrics_${TIMESTAMP}.json"

# Export to CSV format
echo "🔹 Exporting to CSV format..."
cargo run --bin cursed -- export-metrics --format csv --output "${EXPORTS_DIR}/metrics_${TIMESTAMP}.csv"

# Generate HTML report
echo "🔹 Generating HTML report..."
cargo run --bin cursed -- generate-report --format html --output "${EXPORTS_DIR}/report_${TIMESTAMP}.html"

# Cleanup old exports (keep last 30 days)
echo "🧹 Cleaning up old exports..."
find "${EXPORTS_DIR}" -name "*.txt" -mtime +30 -delete
find "${EXPORTS_DIR}" -name "*.json" -mtime +30 -delete
find "${EXPORTS_DIR}" -name "*.csv" -mtime +30 -delete
find "${EXPORTS_DIR}" -name "*.html" -mtime +30 -delete

echo "✅ Metrics export completed"
echo "📁 Exports saved to: ${EXPORTS_DIR}"
EOF

    chmod +x "${CURSED_HOME}/export_metrics.sh"
    echo "✅ Export script created at ${CURSED_HOME}/export_metrics.sh"
}

# Create alert notification script
create_alert_script() {
    echo "🔔 Creating alert notification script..."
    cat > "${CURSED_HOME}/alert_handler.sh" << 'EOF'
#!/bin/bash

# CURSED Alert Handler Script
# Handle and notify about performance alerts

CURSED_HOME=${CURSED_HOME:-"/opt/cursed"}
LOGS_DIR="${CURSED_HOME}/logs"
ALERTS_DIR="${CURSED_HOME}/metrics/alerts"
ALERT_TYPE="$1"
ALERT_MESSAGE="$2"
ALERT_SEVERITY="$3"

echo "🔔 CURSED Alert Handler"
echo "====================="

# Log the alert
echo "$(date): [${ALERT_SEVERITY}] ${ALERT_TYPE}: ${ALERT_MESSAGE}" >> "${LOGS_DIR}/alerts.log"

# Send notification based on severity
case "${ALERT_SEVERITY}" in
    "CRITICAL")
        echo "🚨 CRITICAL ALERT: ${ALERT_MESSAGE}"
        # Send to PagerDuty, Slack, etc.
        ;;
    "WARNING")
        echo "⚠️  WARNING: ${ALERT_MESSAGE}"
        # Send to Slack, email, etc.
        ;;
    "INFO")
        echo "ℹ️  INFO: ${ALERT_MESSAGE}"
        # Log only
        ;;
esac

# Save alert details
ALERT_FILE="${ALERTS_DIR}/alert_$(date +%Y%m%d_%H%M%S).json"
cat > "${ALERT_FILE}" << JSON
{
    "timestamp": "$(date -Iseconds)",
    "type": "${ALERT_TYPE}",
    "message": "${ALERT_MESSAGE}",
    "severity": "${ALERT_SEVERITY}",
    "hostname": "$(hostname)",
    "cursed_version": "$(cargo run --bin cursed -- --version)"
}
JSON

echo "✅ Alert handled and logged"
EOF

    chmod +x "${CURSED_HOME}/alert_handler.sh"
    echo "✅ Alert handler script created at ${CURSED_HOME}/alert_handler.sh"
}

# Create health check script
create_health_check() {
    echo "🏥 Creating health check script..."
    cat > "${CURSED_HOME}/health_check.sh" << 'EOF'
#!/bin/bash

# CURSED Health Check Script
# Check the health of the CURSED compiler and monitoring system

CURSED_HOME=${CURSED_HOME:-"/opt/cursed"}
METRICS_DIR="${CURSED_HOME}/metrics"
LOGS_DIR="${CURSED_HOME}/logs"
EXIT_CODE=0

echo "🏥 CURSED Health Check"
echo "===================="

# Check if metrics daemon is running
if pgrep -f "cursed.*metrics-daemon" > /dev/null; then
    echo "✅ Metrics daemon: HEALTHY"
else
    echo "❌ Metrics daemon: UNHEALTHY"
    EXIT_CODE=1
fi

# Check recent metrics
if [ -f "${METRICS_DIR}/cursed_metrics.txt" ]; then
    LAST_MODIFIED=$(stat -c %Y "${METRICS_DIR}/cursed_metrics.txt")
    CURRENT_TIME=$(date +%s)
    TIME_DIFF=$((CURRENT_TIME - LAST_MODIFIED))
    
    if [ $TIME_DIFF -lt 300 ]; then  # 5 minutes
        echo "✅ Metrics file: HEALTHY (updated ${TIME_DIFF}s ago)"
    else
        echo "⚠️  Metrics file: STALE (updated ${TIME_DIFF}s ago)"
        EXIT_CODE=1
    fi
else
    echo "❌ Metrics file: MISSING"
    EXIT_CODE=1
fi

# Check log files
for log in performance.log gc_monitor.log metrics.log; do
    if [ -f "${LOGS_DIR}/${log}" ]; then
        SIZE=$(stat -c %s "${LOGS_DIR}/${log}")
        if [ $SIZE -gt 0 ]; then
            echo "✅ ${log}: HEALTHY"
        else
            echo "⚠️  ${log}: EMPTY"
        fi
    else
        echo "❌ ${log}: MISSING"
        EXIT_CODE=1
    fi
done

# Check disk space
DISK_USAGE=$(df "${CURSED_HOME}" | tail -1 | awk '{print $5}' | sed 's/%//')
if [ $DISK_USAGE -lt 80 ]; then
    echo "✅ Disk space: HEALTHY (${DISK_USAGE}% used)"
else
    echo "⚠️  Disk space: HIGH (${DISK_USAGE}% used)"
    EXIT_CODE=1
fi

# Check memory usage
MEMORY_USAGE=$(free | grep Mem | awk '{printf "%.0f", $3/$2 * 100.0}')
if [ $MEMORY_USAGE -lt 80 ]; then
    echo "✅ Memory usage: HEALTHY (${MEMORY_USAGE}% used)"
else
    echo "⚠️  Memory usage: HIGH (${MEMORY_USAGE}% used)"
    EXIT_CODE=1
fi

# Overall health status
if [ $EXIT_CODE -eq 0 ]; then
    echo "🟢 Overall health: HEALTHY"
else
    echo "🔴 Overall health: UNHEALTHY"
fi

exit $EXIT_CODE
EOF

    chmod +x "${CURSED_HOME}/health_check.sh"
    echo "✅ Health check script created at ${CURSED_HOME}/health_check.sh"
}

# Main deployment
echo "🚀 Starting performance monitoring deployment..."

# Create all scripts
create_dashboard
create_export_script
create_alert_script
create_health_check

# Ask about systemd service
if [ "$EUID" -eq 0 ]; then
    read -p "Create systemd service? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        create_systemd_service
    fi
fi

# Set up cron jobs
echo "⏰ Setting up cron jobs..."
cat > "${CURSED_HOME}/crontab" << 'EOF'
# CURSED Performance Monitoring Cron Jobs

# Export metrics every hour
0 * * * * /opt/cursed/export_metrics.sh

# Health check every 5 minutes
*/5 * * * * /opt/cursed/health_check.sh

# Clean up old logs daily
0 2 * * * find /opt/cursed/logs -name "*.log" -mtime +30 -delete

# Generate daily report
0 6 * * * /opt/cursed/target/release/cursed --generate-report --output /opt/cursed/metrics/reports/daily_report_$(date +\%Y\%m\%d).html
EOF

echo "📋 Cron jobs created at ${CURSED_HOME}/crontab"
echo "To install: crontab ${CURSED_HOME}/crontab"

# Final verification
echo "✅ Running final verification..."
if [ -f "${CURSED_HOME}/target/release/cursed" ]; then
    echo "✅ CURSED compiler binary: OK"
else
    echo "❌ CURSED compiler binary: MISSING"
    exit 1
fi

if [ -f "${CONFIG_DIR}/performance_monitoring.toml" ]; then
    echo "✅ Configuration file: OK"
else
    echo "❌ Configuration file: MISSING"
    exit 1
fi

echo ""
echo "🎉 CURSED Performance Monitoring Deployment Complete!"
echo "===================================================="
echo ""
echo "📂 Installation directory: ${CURSED_HOME}"
echo "📊 Metrics directory: ${METRICS_DIR}"
echo "📝 Logs directory: ${LOGS_DIR}"
echo "⚙️  Configuration: ${CONFIG_DIR}/performance_monitoring.toml"
echo ""
echo "🚀 Next steps:"
echo "1. Review configuration in ${CONFIG_DIR}/performance_monitoring.toml"
echo "2. Start the monitoring system: ${CURSED_HOME}/dashboard.sh"
echo "3. Set up cron jobs: crontab ${CURSED_HOME}/crontab"
echo "4. Monitor health: ${CURSED_HOME}/health_check.sh"
echo "5. Export metrics: ${CURSED_HOME}/export_metrics.sh"
echo ""
echo "📚 Documentation: https://github.com/ghuntley/cursed/docs/performance-monitoring"
echo "🐛 Issues: https://github.com/ghuntley/cursed/issues"
echo ""
echo "✅ Ready for production deployment!"
