#!/usr/bin/env python3

"""
CURSED Production Monitoring Setup
Configures monitoring, alerting, and observability for production deployment
"""

import os
import json
import yaml
import subprocess
from pathlib import Path
from typing import Dict, List, Optional
from dataclasses import dataclass
from datetime import datetime

@dataclass
class MonitoringConfig:
    """Configuration for monitoring setup"""
    project_name: str = "cursed"
    environment: str = "production"
    alert_email: str = "alerts@cursed-lang.org"
    slack_webhook: Optional[str] = None
    metrics_retention_days: int = 90

class MonitoringSetup:
    """Sets up comprehensive monitoring for CURSED production deployment"""
    
    def __init__(self, project_root: str, config: MonitoringConfig):
        self.project_root = Path(project_root)
        self.config = config
        self.monitoring_dir = self.project_root / "monitoring"
        self.monitoring_dir.mkdir(exist_ok=True)
    
    def create_prometheus_config(self):
        """Create Prometheus monitoring configuration"""
        print("📊 Creating Prometheus configuration...")
        
        prometheus_config = {
            'global': {
                'scrape_interval': '30s',
                'evaluation_interval': '30s'
            },
            'rule_files': [
                'cursed_alerts.yml'
            ],
            'alerting': {
                'alertmanagers': [
                    {
                        'static_configs': [
                            {'targets': ['alertmanager:9093']}
                        ]
                    }
                ]
            },
            'scrape_configs': [
                {
                    'job_name': 'cursed-compiler',
                    'static_configs': [
                        {'targets': ['localhost:8080']}
                    ],
                    'metrics_path': '/metrics',
                    'scrape_interval': '15s'
                },
                {
                    'job_name': 'cursed-registry',
                    'static_configs': [
                        {'targets': ['registry:8081']}
                    ],
                    'metrics_path': '/metrics'
                },
                {
                    'job_name': 'cursed-documentation',
                    'static_configs': [
                        {'targets': ['docs:8082']}
                    ],
                    'metrics_path': '/metrics'
                },
                {
                    'job_name': 'node-exporter',
                    'static_configs': [
                        {'targets': ['node-exporter:9100']}
                    ]
                }
            ]
        }
        
        config_file = self.monitoring_dir / "prometheus.yml"
        with open(config_file, 'w') as f:
            yaml.dump(prometheus_config, f, default_flow_style=False)
        
        print(f"✅ Prometheus config created: {config_file}")
    
    def create_alert_rules(self):
        """Create Prometheus alert rules"""
        print("🚨 Creating alert rules...")
        
        alert_rules = {
            'groups': [
                {
                    'name': 'cursed_alerts',
                    'rules': [
                        {
                            'alert': 'CompilerDown',
                            'expr': 'up{job="cursed-compiler"} == 0',
                            'for': '1m',
                            'labels': {
                                'severity': 'critical'
                            },
                            'annotations': {
                                'summary': 'CURSED compiler is down',
                                'description': 'The CURSED compiler service has been down for more than 1 minute.'
                            }
                        },
                        {
                            'alert': 'HighCompilationLatency',
                            'expr': 'histogram_quantile(0.95, cursed_compilation_duration_seconds) > 10',
                            'for': '5m',
                            'labels': {
                                'severity': 'warning'
                            },
                            'annotations': {
                                'summary': 'High compilation latency detected',
                                'description': '95th percentile compilation time is above 10 seconds for 5 minutes.'
                            }
                        },
                        {
                            'alert': 'HighErrorRate',
                            'expr': 'rate(cursed_compilation_errors_total[5m]) > 0.1',
                            'for': '2m',
                            'labels': {
                                'severity': 'warning'
                            },
                            'annotations': {
                                'summary': 'High compilation error rate',
                                'description': 'Compilation error rate is above 10% for 2 minutes.'
                            }
                        },
                        {
                            'alert': 'MemoryUsageHigh',
                            'expr': 'cursed_memory_usage_bytes / cursed_memory_limit_bytes > 0.8',
                            'for': '5m',
                            'labels': {
                                'severity': 'warning'
                            },
                            'annotations': {
                                'summary': 'High memory usage',
                                'description': 'Memory usage is above 80% for 5 minutes.'
                            }
                        },
                        {
                            'alert': 'DiskSpaceLow',
                            'expr': 'node_filesystem_avail_bytes / node_filesystem_size_bytes < 0.1',
                            'for': '5m',
                            'labels': {
                                'severity': 'critical'
                            },
                            'annotations': {
                                'summary': 'Low disk space',
                                'description': 'Available disk space is below 10%.'
                            }
                        },
                        {
                            'alert': 'RegistryDown',
                            'expr': 'up{job="cursed-registry"} == 0',
                            'for': '2m',
                            'labels': {
                                'severity': 'critical'
                            },
                            'annotations': {
                                'summary': 'Package registry is down',
                                'description': 'The CURSED package registry has been down for more than 2 minutes.'
                            }
                        },
                        {
                            'alert': 'DocumentationDown',
                            'expr': 'up{job="cursed-documentation"} == 0',
                            'for': '5m',
                            'labels': {
                                'severity': 'warning'
                            },
                            'annotations': {
                                'summary': 'Documentation site is down',
                                'description': 'The CURSED documentation site has been down for more than 5 minutes.'
                            }
                        }
                    ]
                }
            ]
        }
        
        rules_file = self.monitoring_dir / "cursed_alerts.yml"
        with open(rules_file, 'w') as f:
            yaml.dump(alert_rules, f, default_flow_style=False)
        
        print(f"✅ Alert rules created: {rules_file}")
    
    def create_alertmanager_config(self):
        """Create Alertmanager configuration"""
        print("📧 Creating Alertmanager configuration...")
        
        alertmanager_config = {
            'global': {
                'smtp_smarthost': 'localhost:587',
                'smtp_from': f'alerts@{self.config.project_name}-lang.org'
            },
            'route': {
                'group_by': ['alertname'],
                'group_wait': '10s',
                'group_interval': '10s',
                'repeat_interval': '1h',
                'receiver': 'web.hook'
            },
            'receivers': [
                {
                    'name': 'web.hook',
                    'email_configs': [
                        {
                            'to': self.config.alert_email,
                            'subject': f'[{self.config.project_name.upper()}] Alert: {{{{ .GroupLabels.alertname }}}}',
                            'body': '''
Alert: {{ .GroupLabels.alertname }}
Summary: {{ range .Alerts }}{{ .Annotations.summary }}{{ end }}
Description: {{ range .Alerts }}{{ .Annotations.description }}{{ end }}

Environment: ''' + self.config.environment + '''
Time: {{ .CommonAnnotations.time }}

View in Prometheus: http://prometheus:9090
View in Grafana: http://grafana:3000
'''
                        }
                    ]
                }
            ]
        }
        
        # Add Slack webhook if configured
        if self.config.slack_webhook:
            slack_config = {
                'slack_configs': [
                    {
                        'api_url': self.config.slack_webhook,
                        'channel': '#alerts',
                        'title': f'[{self.config.project_name.upper()}] Alert',
                        'text': '{{ range .Alerts }}{{ .Annotations.description }}{{ end }}'
                    }
                ]
            }
            alertmanager_config['receivers'][0].update(slack_config)
        
        config_file = self.monitoring_dir / "alertmanager.yml"
        with open(config_file, 'w') as f:
            yaml.dump(alertmanager_config, f, default_flow_style=False)
        
        print(f"✅ Alertmanager config created: {config_file}")
    
    def create_grafana_dashboards(self):
        """Create Grafana dashboard configurations"""
        print("📈 Creating Grafana dashboards...")
        
        # Main CURSED dashboard
        dashboard = {
            "dashboard": {
                "id": None,
                "title": "CURSED Compiler Metrics",
                "tags": ["cursed", "compiler"],
                "timezone": "browser",
                "refresh": "30s",
                "time": {
                    "from": "now-1h",
                    "to": "now"
                },
                "panels": [
                    {
                        "id": 1,
                        "title": "Compilation Rate",
                        "type": "stat",
                        "targets": [
                            {
                                "expr": "rate(cursed_compilations_total[5m])",
                                "legendFormat": "Compilations/sec"
                            }
                        ],
                        "fieldConfig": {
                            "defaults": {
                                "unit": "ops"
                            }
                        },
                        "gridPos": {"h": 4, "w": 6, "x": 0, "y": 0}
                    },
                    {
                        "id": 2,
                        "title": "Success Rate",
                        "type": "stat",
                        "targets": [
                            {
                                "expr": "rate(cursed_compilations_successful_total[5m]) / rate(cursed_compilations_total[5m]) * 100",
                                "legendFormat": "Success %"
                            }
                        ],
                        "fieldConfig": {
                            "defaults": {
                                "unit": "percent",
                                "min": 0,
                                "max": 100
                            }
                        },
                        "gridPos": {"h": 4, "w": 6, "x": 6, "y": 0}
                    },
                    {
                        "id": 3,
                        "title": "Memory Usage",
                        "type": "timeseries",
                        "targets": [
                            {
                                "expr": "cursed_memory_usage_bytes",
                                "legendFormat": "Memory Used"
                            }
                        ],
                        "fieldConfig": {
                            "defaults": {
                                "unit": "bytes"
                            }
                        },
                        "gridPos": {"h": 6, "w": 12, "x": 0, "y": 4}
                    },
                    {
                        "id": 4,
                        "title": "Compilation Duration",
                        "type": "timeseries",
                        "targets": [
                            {
                                "expr": "histogram_quantile(0.50, cursed_compilation_duration_seconds)",
                                "legendFormat": "50th percentile"
                            },
                            {
                                "expr": "histogram_quantile(0.95, cursed_compilation_duration_seconds)",
                                "legendFormat": "95th percentile"
                            }
                        ],
                        "fieldConfig": {
                            "defaults": {
                                "unit": "s"
                            }
                        },
                        "gridPos": {"h": 6, "w": 12, "x": 0, "y": 10}
                    },
                    {
                        "id": 5,
                        "title": "Error Rate by Type",
                        "type": "timeseries",
                        "targets": [
                            {
                                "expr": "rate(cursed_compilation_errors_total[5m])",
                                "legendFormat": "{{error_type}}"
                            }
                        ],
                        "gridPos": {"h": 6, "w": 12, "x": 0, "y": 16}
                    }
                ]
            },
            "overwrite": True
        }
        
        dashboards_dir = self.monitoring_dir / "grafana" / "dashboards"
        dashboards_dir.mkdir(parents=True, exist_ok=True)
        
        dashboard_file = dashboards_dir / "cursed-compiler.json"
        with open(dashboard_file, 'w') as f:
            json.dump(dashboard, f, indent=2)
        
        # System metrics dashboard
        system_dashboard = {
            "dashboard": {
                "id": None,
                "title": "CURSED System Metrics",
                "tags": ["cursed", "system"],
                "timezone": "browser",
                "refresh": "30s",
                "panels": [
                    {
                        "id": 1,
                        "title": "CPU Usage",
                        "type": "timeseries",
                        "targets": [
                            {
                                "expr": "100 - (avg by (instance) (rate(node_cpu_seconds_total{mode=\"idle\"}[5m])) * 100)",
                                "legendFormat": "CPU Usage %"
                            }
                        ],
                        "fieldConfig": {
                            "defaults": {
                                "unit": "percent",
                                "min": 0,
                                "max": 100
                            }
                        },
                        "gridPos": {"h": 6, "w": 12, "x": 0, "y": 0}
                    },
                    {
                        "id": 2,
                        "title": "Disk Space",
                        "type": "timeseries",
                        "targets": [
                            {
                                "expr": "(node_filesystem_size_bytes - node_filesystem_avail_bytes) / node_filesystem_size_bytes * 100",
                                "legendFormat": "Disk Usage % ({{mountpoint}})"
                            }
                        ],
                        "fieldConfig": {
                            "defaults": {
                                "unit": "percent",
                                "min": 0,
                                "max": 100
                            }
                        },
                        "gridPos": {"h": 6, "w": 12, "x": 0, "y": 6}
                    }
                ]
            },
            "overwrite": True
        }
        
        system_dashboard_file = dashboards_dir / "cursed-system.json"
        with open(system_dashboard_file, 'w') as f:
            json.dump(system_dashboard, f, indent=2)
        
        print(f"✅ Grafana dashboards created in {dashboards_dir}")
    
    def create_docker_compose(self):
        """Create Docker Compose for monitoring stack"""
        print("🐳 Creating Docker Compose monitoring stack...")
        
        docker_compose = {
            'version': '3.8',
            'services': {
                'prometheus': {
                    'image': 'prom/prometheus:latest',
                    'container_name': 'cursed-prometheus',
                    'ports': ['9090:9090'],
                    'volumes': [
                        './prometheus.yml:/etc/prometheus/prometheus.yml',
                        './cursed_alerts.yml:/etc/prometheus/cursed_alerts.yml',
                        'prometheus_data:/prometheus'
                    ],
                    'command': [
                        '--config.file=/etc/prometheus/prometheus.yml',
                        '--storage.tsdb.path=/prometheus',
                        '--web.console.libraries=/etc/prometheus/console_libraries',
                        '--web.console.templates=/etc/prometheus/consoles',
                        '--storage.tsdb.retention.time=90d',
                        '--web.enable-lifecycle'
                    ],
                    'restart': 'unless-stopped'
                },
                'alertmanager': {
                    'image': 'prom/alertmanager:latest',
                    'container_name': 'cursed-alertmanager',
                    'ports': ['9093:9093'],
                    'volumes': [
                        './alertmanager.yml:/etc/alertmanager/alertmanager.yml',
                        'alertmanager_data:/alertmanager'
                    ],
                    'restart': 'unless-stopped'
                },
                'grafana': {
                    'image': 'grafana/grafana:latest',
                    'container_name': 'cursed-grafana',
                    'ports': ['3000:3000'],
                    'environment': {
                        'GF_SECURITY_ADMIN_PASSWORD': 'cursed_admin_2024'
                    },
                    'volumes': [
                        'grafana_data:/var/lib/grafana',
                        './grafana/dashboards:/etc/grafana/provisioning/dashboards'
                    ],
                    'restart': 'unless-stopped'
                },
                'node-exporter': {
                    'image': 'prom/node-exporter:latest',
                    'container_name': 'cursed-node-exporter',
                    'ports': ['9100:9100'],
                    'volumes': [
                        '/proc:/host/proc:ro',
                        '/sys:/host/sys:ro',
                        '/:/rootfs:ro'
                    ],
                    'command': [
                        '--path.procfs=/host/proc',
                        '--path.sysfs=/host/sys',
                        '--collector.filesystem.mount-points-exclude=^/(sys|proc|dev|host|etc)($$|/)'
                    ],
                    'restart': 'unless-stopped'
                }
            },
            'volumes': {
                'prometheus_data': {},
                'alertmanager_data': {},
                'grafana_data': {}
            }
        }
        
        compose_file = self.monitoring_dir / "docker-compose.yml"
        with open(compose_file, 'w') as f:
            yaml.dump(docker_compose, f, default_flow_style=False)
        
        print(f"✅ Docker Compose created: {compose_file}")
    
    def create_monitoring_scripts(self):
        """Create monitoring management scripts"""
        print("📜 Creating monitoring scripts...")
        
        # Start script
        start_script = self.monitoring_dir / "start_monitoring.sh"
        start_content = f'''#!/bin/bash

# Start CURSED monitoring stack

set -e

echo "🚀 Starting CURSED monitoring stack..."

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker first."
    exit 1
fi

# Start the monitoring stack
docker-compose up -d

echo "✅ Monitoring stack started!"
echo ""
echo "📊 Services:"
echo "  - Prometheus: http://localhost:9090"
echo "  - Grafana: http://localhost:3000 (admin/cursed_admin_2024)"
echo "  - Alertmanager: http://localhost:9093"
echo ""
echo "⏳ Waiting for services to be ready..."
sleep 10

# Check service health
services=("prometheus:9090" "grafana:3000" "alertmanager:9093")
for service in "${{services[@]}}"; do
    if curl -f -s "http://$service/api/health" > /dev/null 2>&1 || curl -f -s "http://$service" > /dev/null 2>&1; then
        echo "✅ $service is healthy"
    else
        echo "⚠️  $service may not be ready yet"
    fi
done

echo ""
echo "🎉 Monitoring setup complete!"
'''
        start_script.write_text(start_content)
        start_script.chmod(0o755)
        
        # Stop script
        stop_script = self.monitoring_dir / "stop_monitoring.sh"
        stop_content = '''#!/bin/bash

# Stop CURSED monitoring stack

echo "🛑 Stopping CURSED monitoring stack..."

docker-compose down

echo "✅ Monitoring stack stopped"
'''
        stop_script.write_text(stop_content)
        stop_script.chmod(0o755)
        
        # Status script
        status_script = self.monitoring_dir / "status_monitoring.sh"
        status_content = '''#!/bin/bash

# Check CURSED monitoring stack status

echo "📊 CURSED Monitoring Stack Status"
echo "================================="

docker-compose ps

echo ""
echo "🔗 Service URLs:"
echo "  - Prometheus: http://localhost:9090"
echo "  - Grafana: http://localhost:3000"
echo "  - Alertmanager: http://localhost:9093"
echo "  - Node Exporter: http://localhost:9100"
'''
        status_script.write_text(status_content)
        status_script.chmod(0o755)
        
        print(f"✅ Monitoring scripts created in {self.monitoring_dir}")
    
    def create_health_check_script(self):
        """Create health check script for CURSED services"""
        print("🏥 Creating health check script...")
        
        health_check = self.monitoring_dir / "health_check.py"
        health_check_content = '''#!/usr/bin/env python3

"""
CURSED Health Check Script
Monitors the health of all CURSED services
"""

import requests
import json
import sys
from datetime import datetime

def check_service(name, url, timeout=10):
    """Check if a service is healthy"""
    try:
        response = requests.get(url, timeout=timeout)
        if response.status_code == 200:
            return True, "OK"
        else:
            return False, f"HTTP {response.status_code}"
    except requests.exceptions.RequestException as e:
        return False, str(e)

def main():
    services = {
        "Prometheus": "http://localhost:9090/-/healthy",
        "Grafana": "http://localhost:3000/api/health", 
        "Alertmanager": "http://localhost:9093/-/healthy",
        "Node Exporter": "http://localhost:9100/metrics"
    }
    
    print(f"🏥 CURSED Health Check - {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print("=" * 60)
    
    all_healthy = True
    
    for service_name, service_url in services.items():
        healthy, message = check_service(service_name, service_url)
        status = "✅" if healthy else "❌"
        print(f"{status} {service_name}: {message}")
        
        if not healthy:
            all_healthy = False
    
    print("=" * 60)
    
    if all_healthy:
        print("🎉 All services are healthy!")
        sys.exit(0)
    else:
        print("⚠️  Some services are unhealthy!")
        sys.exit(1)

if __name__ == "__main__":
    main()
'''
        health_check.write_text(health_check_content)
        health_check.chmod(0o755)
        
        print(f"✅ Health check script created: {health_check}")
    
    def create_deployment_documentation(self):
        """Create monitoring deployment documentation"""
        print("📖 Creating monitoring documentation...")
        
        readme = self.monitoring_dir / "README.md"
        readme_content = f'''# CURSED Production Monitoring

This directory contains the complete monitoring setup for CURSED production deployment.

## 🚀 Quick Start

1. **Start the monitoring stack:**
   ```bash
   ./start_monitoring.sh
   ```

2. **Check status:**
   ```bash
   ./status_monitoring.sh
   ```

3. **Run health check:**
   ```bash
   ./health_check.py
   ```

## 📊 Services

### Prometheus (Port 9090)
- Metrics collection and alerting
- URL: http://localhost:9090
- Config: `prometheus.yml`

### Grafana (Port 3000)
- Metrics visualization and dashboards
- URL: http://localhost:3000
- Credentials: admin / cursed_admin_2024
- Dashboards: `grafana/dashboards/`

### Alertmanager (Port 9093)
- Alert routing and notifications
- URL: http://localhost:9093
- Config: `alertmanager.yml`

### Node Exporter (Port 9100)
- System metrics collection
- URL: http://localhost:9100

## 🚨 Alerts

The following alerts are configured:

- **CompilerDown**: CURSED compiler service is down
- **HighCompilationLatency**: 95th percentile > 10 seconds
- **HighErrorRate**: Error rate > 10%
- **MemoryUsageHigh**: Memory usage > 80%
- **DiskSpaceLow**: Available disk space < 10%
- **RegistryDown**: Package registry is down
- **DocumentationDown**: Documentation site is down

## 📧 Notifications

Alerts are sent to:
- Email: {self.config.alert_email}
- Slack: {"Configured" if self.config.slack_webhook else "Not configured"}

## 🔧 Configuration

### Environment Variables
- `CURSED_METRICS_PORT`: Port for CURSED metrics endpoint (default: 8080)
- `CURSED_ALERT_EMAIL`: Email for alerts
- `CURSED_SLACK_WEBHOOK`: Slack webhook URL (optional)

### Customization
- Edit `prometheus.yml` to add new scrape targets
- Edit `cursed_alerts.yml` to modify alert rules
- Edit `alertmanager.yml` to change notification settings
- Add new dashboards in `grafana/dashboards/`

## 📈 Metrics

CURSED exposes the following metrics:

- `cursed_compilations_total`: Total number of compilations
- `cursed_compilations_successful_total`: Successful compilations
- `cursed_compilation_duration_seconds`: Compilation duration histogram
- `cursed_compilation_errors_total`: Compilation errors by type
- `cursed_memory_usage_bytes`: Current memory usage
- `cursed_memory_limit_bytes`: Memory limit

## 🔍 Troubleshooting

### Services not starting
1. Check Docker is running: `docker info`
2. Check port availability: `netstat -tlnp | grep :9090`
3. Check logs: `docker-compose logs <service>`

### Metrics not appearing
1. Verify CURSED is exposing metrics on the configured port
2. Check Prometheus targets: http://localhost:9090/targets
3. Verify firewall/network connectivity

### Alerts not firing
1. Check alert rules syntax: http://localhost:9090/rules
2. Verify Alertmanager configuration: http://localhost:9093/#/status
3. Check email/Slack configuration

## 🛠️ Maintenance

### Backup
Important data to backup:
- Grafana dashboards and datasources: `grafana_data` volume
- Prometheus data: `prometheus_data` volume
- Alert history: `alertmanager_data` volume

### Updates
Update the monitoring stack:
```bash
docker-compose pull
docker-compose up -d
```

### Cleanup
Remove all monitoring data:
```bash
./stop_monitoring.sh
docker-compose down -v
```

## 🏗️ Production Deployment

For production deployment:

1. **Use external storage** for persistent volumes
2. **Configure TLS/SSL** for all services
3. **Set strong passwords** for Grafana
4. **Configure proper backup** strategy
5. **Set up log aggregation** (ELK stack, etc.)
6. **Configure reverse proxy** (nginx, etc.)

## 📚 Resources

- [Prometheus Documentation](https://prometheus.io/docs/)
- [Grafana Documentation](https://grafana.com/docs/)
- [Alertmanager Documentation](https://prometheus.io/docs/alerting/latest/alertmanager/)
'''
        readme.write_text(readme_content)
        
        print(f"✅ Documentation created: {readme}")
    
    def setup_monitoring(self):
        """Set up complete monitoring infrastructure"""
        print(f"🚀 Setting up monitoring for {self.config.project_name}...")
        
        self.create_prometheus_config()
        self.create_alert_rules()
        self.create_alertmanager_config()
        self.create_grafana_dashboards()
        self.create_docker_compose()
        self.create_monitoring_scripts()
        self.create_health_check_script()
        self.create_deployment_documentation()
        
        print(f"\\n🎉 Monitoring setup completed!")
        print(f"📁 All files created in: {self.monitoring_dir}")
        print(f"\\n🏃‍♂️ Next steps:")
        print(f"  1. cd {self.monitoring_dir}")
        print(f"  2. ./start_monitoring.sh")
        print(f"  3. Open http://localhost:3000 (Grafana)")
        print(f"  4. Import dashboards and configure data sources")

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="CURSED Monitoring Setup")
    parser.add_argument("--project-root", default=".", help="Project root directory")
    parser.add_argument("--environment", default="production", help="Environment name")
    parser.add_argument("--alert-email", default="alerts@cursed-lang.org", help="Alert email address")
    parser.add_argument("--slack-webhook", help="Slack webhook URL for alerts")
    
    args = parser.parse_args()
    
    config = MonitoringConfig(
        environment=args.environment,
        alert_email=args.alert_email,
        slack_webhook=args.slack_webhook
    )
    
    setup = MonitoringSetup(args.project_root, config)
    setup.setup_monitoring()

if __name__ == "__main__":
    main()
