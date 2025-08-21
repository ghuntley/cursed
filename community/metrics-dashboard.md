# CURSED Community Metrics Dashboard

## Key Performance Indicators (KPIs)

### Adoption Metrics
- **Monthly Active Users** - GitHub Insights + Package Downloads
- **New Users** - First-time GitHub clones/downloads
- **User Retention** - Return users within 30 days
- **Geographic Distribution** - Global adoption patterns

### Community Growth
- **Discord Members** - Total and active members
- **GitHub Stars/Watchers** - Repository engagement
- **Forum Posts** - Discussion volume and quality
- **Social Media Followers** - Twitter, LinkedIn, Reddit

### Contribution Health
- **Active Contributors** - Monthly commit authors
- **First-time Contributors** - New contributor onboarding
- **Pull Request Volume** - Contribution throughput
- **Issue Resolution Time** - Support efficiency

### Code Quality
- **Test Coverage** - Automated coverage reports
- **Bug Report Rate** - Issues per 1000 users
- **Performance Benchmarks** - Compilation and runtime metrics
- **Documentation Coverage** - API docs completeness

## Data Sources

### GitHub Analytics
```yaml
metrics:
  - repository_traffic
  - clone_statistics  
  - release_downloads
  - issue_metrics
  - pr_metrics
  - contributor_stats
```

### Discord Analytics
```yaml
metrics:
  - member_count
  - active_users
  - message_volume
  - channel_engagement
  - voice_activity
```

### Package Manager
```yaml
metrics:
  - download_count
  - unique_users
  - version_adoption
  - platform_distribution
```

## Dashboard Implementation

### Real-time Dashboard
```html
<!-- Community Dashboard HTML Template -->
<div class="dashboard">
  <div class="metrics-grid">
    <div class="metric-card">
      <h3>Monthly Active Users</h3>
      <div class="metric-value" id="mau">Loading...</div>
      <div class="metric-trend" id="mau-trend">+12% this month</div>
    </div>
    
    <div class="metric-card">
      <h3>Discord Members</h3>
      <div class="metric-value" id="discord">Loading...</div>
      <div class="metric-trend" id="discord-trend">+156 new members</div>
    </div>
    
    <div class="metric-card">
      <h3>Active Contributors</h3>
      <div class="metric-value" id="contributors">Loading...</div>
      <div class="metric-trend" id="contrib-trend">+8 this month</div>
    </div>
    
    <div class="metric-card">
      <h3>Average Response Time</h3>
      <div class="metric-value" id="response-time">Loading...</div>
      <div class="metric-trend" id="response-trend">-2h improvement</div>
    </div>
  </div>
  
  <div class="charts-section">
    <canvas id="adoption-chart"></canvas>
    <canvas id="contribution-chart"></canvas>
  </div>
</div>
```

### Data Collection Scripts
```python
#!/usr/bin/env python3
# community-metrics-collector.py

import requests
import json
from datetime import datetime, timedelta

class CommunityMetrics:
    def __init__(self, github_token, discord_token):
        self.github_token = github_token
        self.discord_token = discord_token
        
    def collect_github_metrics(self):
        headers = {'Authorization': f'token {self.github_token}'}
        
        # Repository stats
        repo_stats = requests.get(
            'https://api.github.com/repos/ghuntley/cursed',
            headers=headers
        ).json()
        
        # Traffic data
        traffic = requests.get(
            'https://api.github.com/repos/ghuntley/cursed/traffic/views',
            headers=headers
        ).json()
        
        # Contributors
        contributors = requests.get(
            'https://api.github.com/repos/ghuntley/cursed/contributors',
            headers=headers
        ).json()
        
        return {
            'stars': repo_stats['stargazers_count'],
            'forks': repo_stats['forks_count'],
            'watchers': repo_stats['watchers_count'],
            'traffic': traffic,
            'contributors': len(contributors)
        }
    
    def collect_discord_metrics(self):
        headers = {'Authorization': f'Bot {self.discord_token}'}
        
        # Guild stats
        guild_stats = requests.get(
            'https://discord.com/api/v10/guilds/GUILD_ID',
            headers=headers
        ).json()
        
        return {
            'member_count': guild_stats['member_count'],
            'online_count': guild_stats.get('approximate_presence_count', 0)
        }
    
    def generate_report(self):
        github_data = self.collect_github_metrics()
        discord_data = self.collect_discord_metrics()
        
        report = {
            'timestamp': datetime.now().isoformat(),
            'github': github_data,
            'discord': discord_data,
            'computed': {
                'growth_rate': self.calculate_growth_rate(),
                'engagement_score': self.calculate_engagement_score()
            }
        }
        
        return report
```

## Automated Reporting

### Weekly Community Report
- Growth metrics and trends
- Top contributors recognition
- Issue resolution summary
- Upcoming events and milestones

### Monthly Health Report
- Community satisfaction survey results
- Contributor retention analysis
- Platform performance benchmarks
- Strategic recommendations

### Quarterly Business Review
- Adoption milestone achievements
- Competitive analysis updates
- Community feedback synthesis
- Roadmap adjustments

## Alert System

### Growth Alerts
- Significant adoption spikes or drops
- New contributor milestone achievements
- Viral content or mentions detection
- Platform outages or issues

### Health Alerts
- Support SLA breaches
- Contributor churn warnings
- Community sentiment changes
- Security incident notifications

## Privacy and Compliance

### Data Collection
- Anonymized user analytics only
- GDPR compliance for EU users
- Clear opt-out mechanisms
- Transparent data usage policies

### Data Retention
- 2-year retention for trend analysis
- Immediate deletion upon user request
- Regular data audit and cleanup
- Secure storage and transmission
