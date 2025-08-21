#!/usr/bin/env python3
"""
CURSED Community Adoption Metrics Collection System
Collects and analyzes adoption metrics for CURSED v1.0.x
"""

import json
import requests
import sqlite3
import time
from datetime import datetime, timedelta
from typing import Dict, List, Optional
import os
import sys

class MetricsCollector:
    """Collects adoption and community metrics for CURSED"""
    
    def __init__(self, config_path: str = "metrics_config.json"):
        self.config = self.load_config(config_path)
        self.db_path = self.config.get("database", "metrics.db")
        self.init_database()
    
    def load_config(self, config_path: str) -> Dict:
        """Load configuration from JSON file"""
        default_config = {
            "github": {
                "repo": "ghuntley/cursed",
                "token": os.environ.get("GITHUB_TOKEN")
            },
            "npm": {
                "package": "@cursed/cli"
            },
            "docker": {
                "image": "cursedlang/cursed"
            },
            "website": {
                "docs_url": "https://docs.cursedlang.org",
                "download_url": "https://install.cursedlang.org"
            },
            "database": "metrics.db",
            "collection_interval": 3600  # 1 hour
        }
        
        try:
            with open(config_path, 'r') as f:
                config = json.load(f)
                return {**default_config, **config}
        except FileNotFoundError:
            print(f"Config file {config_path} not found, using defaults")
            return default_config
    
    def init_database(self):
        """Initialize SQLite database for metrics storage"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        # GitHub metrics table
        cursor.execute("""
            CREATE TABLE IF NOT EXISTS github_metrics (
                timestamp TEXT PRIMARY KEY,
                stars INTEGER,
                forks INTEGER,
                watchers INTEGER,
                open_issues INTEGER,
                closed_issues INTEGER,
                contributors INTEGER,
                releases INTEGER,
                downloads INTEGER
            )
        """)
        
        # Package metrics table
        cursor.execute("""
            CREATE TABLE IF NOT EXISTS package_metrics (
                timestamp TEXT PRIMARY KEY,
                npm_downloads INTEGER,
                docker_pulls INTEGER,
                homebrew_installs INTEGER
            )
        """)
        
        # Website metrics table
        cursor.execute("""
            CREATE TABLE IF NOT EXISTS website_metrics (
                timestamp TEXT PRIMARY KEY,
                docs_pageviews INTEGER,
                download_requests INTEGER,
                unique_visitors INTEGER,
                countries INTEGER
            )
        """)
        
        # Community metrics table
        cursor.execute("""
            CREATE TABLE IF NOT EXISTS community_metrics (
                timestamp TEXT PRIMARY KEY,
                discord_members INTEGER,
                stack_overflow_questions INTEGER,
                reddit_subscribers INTEGER,
                twitter_followers INTEGER,
                blog_posts INTEGER
            )
        """)
        
        conn.commit()
        conn.close()
    
    def collect_github_metrics(self) -> Dict:
        """Collect metrics from GitHub API"""
        repo = self.config["github"]["repo"]
        token = self.config["github"]["token"]
        
        headers = {}
        if token:
            headers["Authorization"] = f"Bearer {token}"
        
        try:
            # Repository stats
            repo_url = f"https://api.github.com/repos/{repo}"
            repo_response = requests.get(repo_url, headers=headers)
            repo_data = repo_response.json()
            
            # Contributors
            contributors_url = f"{repo_url}/contributors"
            contributors_response = requests.get(contributors_url, headers=headers)
            contributors_count = len(contributors_response.json()) if contributors_response.ok else 0
            
            # Issues
            issues_url = f"{repo_url}/issues?state=all&per_page=1"
            issues_response = requests.get(issues_url, headers=headers)
            
            # Releases and downloads
            releases_url = f"{repo_url}/releases"
            releases_response = requests.get(releases_url, headers=headers)
            releases_data = releases_response.json() if releases_response.ok else []
            
            total_downloads = 0
            for release in releases_data:
                for asset in release.get("assets", []):
                    total_downloads += asset.get("download_count", 0)
            
            metrics = {
                "stars": repo_data.get("stargazers_count", 0),
                "forks": repo_data.get("forks_count", 0),
                "watchers": repo_data.get("watchers_count", 0),
                "open_issues": repo_data.get("open_issues_count", 0),
                "contributors": contributors_count,
                "releases": len(releases_data),
                "downloads": total_downloads
            }
            
            return metrics
            
        except Exception as e:
            print(f"Error collecting GitHub metrics: {e}")
            return {}
    
    def collect_package_metrics(self) -> Dict:
        """Collect package download metrics"""
        metrics = {
            "npm_downloads": 0,
            "docker_pulls": 0,
            "homebrew_installs": 0
        }
        
        try:
            # NPM downloads (if package exists)
            npm_package = self.config["npm"]["package"]
            npm_url = f"https://api.npmjs.org/downloads/point/last-week/{npm_package}"
            npm_response = requests.get(npm_url, timeout=10)
            if npm_response.ok:
                metrics["npm_downloads"] = npm_response.json().get("downloads", 0)
        except Exception as e:
            print(f"Error collecting NPM metrics: {e}")
        
        try:
            # Docker Hub pulls (if image exists)
            docker_image = self.config["docker"]["image"]
            docker_url = f"https://hub.docker.com/v2/repositories/{docker_image}/"
            docker_response = requests.get(docker_url, timeout=10)
            if docker_response.ok:
                metrics["docker_pulls"] = docker_response.json().get("pull_count", 0)
        except Exception as e:
            print(f"Error collecting Docker metrics: {e}")
        
        return metrics
    
    def collect_website_metrics(self) -> Dict:
        """Collect website analytics (placeholder - would integrate with actual analytics)"""
        # This would integrate with Google Analytics, Plausible, or similar
        # For now, return mock data structure
        return {
            "docs_pageviews": 0,
            "download_requests": 0,
            "unique_visitors": 0,
            "countries": 0
        }
    
    def collect_community_metrics(self) -> Dict:
        """Collect community engagement metrics"""
        metrics = {
            "discord_members": 0,
            "stack_overflow_questions": 0,
            "reddit_subscribers": 0,
            "twitter_followers": 0,
            "blog_posts": 0
        }
        
        try:
            # Stack Overflow questions tagged with 'cursed-lang'
            so_url = "https://api.stackexchange.com/2.3/questions?tagged=cursed-lang&site=stackoverflow"
            so_response = requests.get(so_url, timeout=10)
            if so_response.ok:
                metrics["stack_overflow_questions"] = so_response.json().get("total", 0)
        except Exception as e:
            print(f"Error collecting Stack Overflow metrics: {e}")
        
        return metrics
    
    def store_metrics(self, timestamp: str, github: Dict, packages: Dict, 
                     website: Dict, community: Dict):
        """Store collected metrics in database"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        # Store GitHub metrics
        cursor.execute("""
            INSERT OR REPLACE INTO github_metrics 
            (timestamp, stars, forks, watchers, open_issues, closed_issues, 
             contributors, releases, downloads)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        """, (
            timestamp,
            github.get("stars", 0),
            github.get("forks", 0), 
            github.get("watchers", 0),
            github.get("open_issues", 0),
            github.get("closed_issues", 0),
            github.get("contributors", 0),
            github.get("releases", 0),
            github.get("downloads", 0)
        ))
        
        # Store package metrics
        cursor.execute("""
            INSERT OR REPLACE INTO package_metrics
            (timestamp, npm_downloads, docker_pulls, homebrew_installs)
            VALUES (?, ?, ?, ?)
        """, (
            timestamp,
            packages.get("npm_downloads", 0),
            packages.get("docker_pulls", 0),
            packages.get("homebrew_installs", 0)
        ))
        
        # Store website metrics
        cursor.execute("""
            INSERT OR REPLACE INTO website_metrics
            (timestamp, docs_pageviews, download_requests, unique_visitors, countries)
            VALUES (?, ?, ?, ?, ?)
        """, (
            timestamp,
            website.get("docs_pageviews", 0),
            website.get("download_requests", 0),
            website.get("unique_visitors", 0),
            website.get("countries", 0)
        ))
        
        # Store community metrics
        cursor.execute("""
            INSERT OR REPLACE INTO community_metrics
            (timestamp, discord_members, stack_overflow_questions, 
             reddit_subscribers, twitter_followers, blog_posts)
            VALUES (?, ?, ?, ?, ?, ?)
        """, (
            timestamp,
            community.get("discord_members", 0),
            community.get("stack_overflow_questions", 0),
            community.get("reddit_subscribers", 0),
            community.get("twitter_followers", 0),
            community.get("blog_posts", 0)
        ))
        
        conn.commit()
        conn.close()
    
    def generate_report(self, days: int = 30) -> Dict:
        """Generate adoption metrics report for the last N days"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        # Get date range
        end_date = datetime.now()
        start_date = end_date - timedelta(days=days)
        start_str = start_date.isoformat()
        end_str = end_date.isoformat()
        
        report = {
            "period": {"start": start_str, "end": end_str, "days": days},
            "github": {},
            "packages": {},
            "community": {},
            "trends": {}
        }
        
        # GitHub metrics trends
        cursor.execute("""
            SELECT * FROM github_metrics 
            WHERE timestamp BETWEEN ? AND ?
            ORDER BY timestamp DESC LIMIT 1
        """, (start_str, end_str))
        
        latest_github = cursor.fetchone()
        if latest_github:
            report["github"] = {
                "stars": latest_github[1],
                "forks": latest_github[2],
                "watchers": latest_github[3],
                "open_issues": latest_github[4],
                "contributors": latest_github[6],
                "total_downloads": latest_github[8]
            }
        
        # Calculate trends (growth over period)
        cursor.execute("""
            SELECT stars, forks, downloads FROM github_metrics 
            WHERE timestamp < ?
            ORDER BY timestamp DESC LIMIT 1
        """, (start_str,))
        
        previous_github = cursor.fetchone()
        if previous_github and latest_github:
            report["trends"] = {
                "stars_growth": latest_github[1] - previous_github[0],
                "forks_growth": latest_github[2] - previous_github[1],
                "downloads_growth": latest_github[8] - previous_github[2]
            }
        
        conn.close()
        return report
    
    def collect_all_metrics(self):
        """Collect all metrics and store in database"""
        timestamp = datetime.now().isoformat()
        
        print(f"Collecting metrics at {timestamp}")
        
        github_metrics = self.collect_github_metrics()
        print(f"GitHub metrics: {github_metrics}")
        
        package_metrics = self.collect_package_metrics()
        print(f"Package metrics: {package_metrics}")
        
        website_metrics = self.collect_website_metrics()
        print(f"Website metrics: {website_metrics}")
        
        community_metrics = self.collect_community_metrics()
        print(f"Community metrics: {community_metrics}")
        
        self.store_metrics(timestamp, github_metrics, package_metrics,
                          website_metrics, community_metrics)
        
        print("Metrics collection completed")
    
    def run_continuous(self):
        """Run continuous metrics collection"""
        interval = self.config["collection_interval"]
        print(f"Starting continuous metrics collection (interval: {interval}s)")
        
        while True:
            try:
                self.collect_all_metrics()
                time.sleep(interval)
            except KeyboardInterrupt:
                print("\nMetrics collection stopped")
                break
            except Exception as e:
                print(f"Error in metrics collection: {e}")
                time.sleep(60)  # Wait 1 minute before retrying

def main():
    """Main entry point"""
    if len(sys.argv) > 1:
        command = sys.argv[1]
        
        collector = MetricsCollector()
        
        if command == "collect":
            collector.collect_all_metrics()
        elif command == "run":
            collector.run_continuous()
        elif command == "report":
            days = int(sys.argv[2]) if len(sys.argv) > 2 else 30
            report = collector.generate_report(days)
            print(json.dumps(report, indent=2))
        else:
            print("Usage: python metrics-collection.py [collect|run|report [days]]")
    else:
        print("CURSED Metrics Collection System")
        print("Commands:")
        print("  collect - Collect metrics once")
        print("  run - Run continuous collection")
        print("  report [days] - Generate report for last N days")

if __name__ == "__main__":
    main()
