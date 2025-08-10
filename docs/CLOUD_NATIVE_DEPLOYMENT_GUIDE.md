# CURSED Cloud-Native Deployment Guide

This comprehensive guide demonstrates how to build, deploy, and manage enterprise-grade cloud-native applications using CURSED's cloud integration modules.

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Cloud Integration Modules](#cloud-integration-modules)
4. [Getting Started](#getting-started)
5. [Container Development](#container-development)
6. [Kubernetes Deployment](#kubernetes-deployment)
7. [CI/CD Pipelines](#cicd-pipelines)
8. [Infrastructure as Code](#infrastructure-as-code)
9. [Monitoring & Observability](#monitoring--observability)
10. [Service Mesh Integration](#service-mesh-integration)
11. [Production Best Practices](#production-best-practices)
12. [Troubleshooting](#troubleshooting)

## Overview

CURSED provides enterprise-grade cloud integration through three main modules:

- **cloudz**: Multi-cloud platform integration (AWS, Azure, GCP)
- **kubernetesz**: Kubernetes orchestration and management
- **deploymentz**: CI/CD, container building, and deployment automation

These modules enable you to build cloud-native applications that are:
- **Scalable**: Horizontal auto-scaling with load balancing
- **Resilient**: Health checks, circuit breakers, and self-healing
- **Observable**: Comprehensive metrics, logging, and tracing
- **Secure**: Security scanning, secrets management, and compliance
- **Cost-Optimized**: Resource optimization and cost monitoring

## Prerequisites

### Development Environment
```bash
# Install required tools
curl -sSf https://install.cursedlang.org | sh  # CURSED compiler
curl -LO https://storage.googleapis.com/kubernetes-release/release/$(curl -s https://storage.googleapis.com/kubernetes-release/release/stable.txt)/bin/linux/amd64/kubectl
sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl

# Container runtime
sudo apt install docker.io
sudo usermod -aG docker $USER

# Cloud CLI tools
curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip awscliv2.zip && sudo ./aws/install

curl -sL https://aka.ms/InstallAzureCLIDeb | sudo bash

curl https://sdk.cloud.google.com | bash
```

### Cloud Credentials
```bash
# AWS
export AWS_ACCESS_KEY_ID="your-access-key"
export AWS_SECRET_ACCESS_KEY="your-secret-key"
export AWS_DEFAULT_REGION="us-west-2"

# Azure
export AZURE_SUBSCRIPTION_ID="your-subscription-id"
export AZURE_TENANT_ID="your-tenant-id"
export AZURE_CLIENT_ID="your-client-id"
export AZURE_CLIENT_SECRET="your-client-secret"

# GCP
export GOOGLE_APPLICATION_CREDENTIALS="/path/to/service-account.json"
export GCP_PROJECT_ID="your-project-id"
```

## Cloud Integration Modules

### cloudz Module Structure
```
stdlib/cloudz/
├── cloudz.csd              # Main cloud integration module
├── aws/                    # AWS-specific implementations
├── azure/                  # Azure-specific implementations
├── gcp/                    # GCP-specific implementations
├── multi_cloud/            # Multi-cloud management
└── README.md               # Documentation
```

### kubernetesz Module Structure
```
stdlib/kubernetesz/
├── kubernetesz.csd         # Main Kubernetes module
├── pod_manager/            # Pod lifecycle management
├── deployment_manager/     # Deployment operations
├── service_manager/        # Service management
├── config_manager/         # ConfigMaps and Secrets
├── autoscaling_manager/    # HPA and VPA
├── monitoring_manager/     # Metrics and monitoring
└── helm_manager/           # Helm chart integration
```

### deploymentz Module Structure
```
stdlib/deploymentz/
├── deploymentz.csd         # Main deployment module
├── container_builder/      # Container build and security
├── ci_integration/         # CI/CD platform integration
├── kubernetes_deployment/  # K8s deployment strategies
├── infrastructure_as_code/ # Terraform and Ansible
├── observability_integration/ # Monitoring setup
└── service_mesh_integration/  # Istio and Linkerd
```

## Getting Started

### 1. Basic Cloud Application

Create a simple cloud-native application:

```cursed
# app.csd
yeet "vibez"
yeet "networkz"
yeet "jsonz"

slay main() {
    vibez.spill("🚀 Starting CURSED Cloud Application")
    
    # Health check endpoint
    networkz.serve("0.0.0.0:8080", handle_request)
}

slay handle_request(request networkz.Request) networkz.Response {
    sick (request.path) {
        when "/health" -> {
            damn networkz.Response{
                status_code: 200,
                headers: {"Content-Type": "application/json"},
                body: jsonz.marshal({"status": "healthy", "timestamp": timez.now()})
            }
        }
        when "/metrics" -> {
            damn networkz.Response{
                status_code: 200,
                headers: {"Content-Type": "text/plain"},
                body: generate_prometheus_metrics()
            }
        }
        otherwise -> {
            damn networkz.Response{
                status_code: 200,
                headers: {"Content-Type": "application/json"},
                body: jsonz.marshal({"message": "Hello from CURSED!", "version": "1.0.0"})
            }
        }
    }
}

slay generate_prometheus_metrics() tea {
    damn `# HELP http_requests_total Total HTTP requests
# TYPE http_requests_total counter
http_requests_total{method="GET",status="200"} 42

# HELP http_request_duration_seconds HTTP request duration
# TYPE http_request_duration_seconds histogram
http_request_duration_seconds_bucket{le="0.1"} 10
http_request_duration_seconds_bucket{le="0.5"} 20
http_request_duration_seconds_bucket{le="1.0"} 25
http_request_duration_seconds_bucket{le="+Inf"} 30
http_request_duration_seconds_sum 15.5
http_request_duration_seconds_count 30`
}
```

### 2. Dockerfile

```dockerfile
# Dockerfile
FROM cursed/runtime:latest AS builder
WORKDIR /app
COPY app.csd .
RUN cursed-zig --compile --optimize=ReleaseFast app.csd

FROM ubuntu:22.04
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/app .
EXPOSE 8080
USER 1001
CMD ["./app"]
```

## Container Development

### Building Containers with deploymentz

```cursed
# build_deploy.csd
yeet "deploymentz"
yeet "vibez"

slay main() {
    # Configure container build
    sus build_config deploymentz.BuildConfig = deploymentz.BuildConfig{
        dockerfile_path: "./Dockerfile",
        context_path: ".",
        build_args: {
            "BUILD_VERSION": "v1.0.0",
            "ENVIRONMENT": "production"
        },
        target_stage: "production",
        cache_from: ["myapp:cache"],
        push_registry: "ghcr.io",
        tags: [
            "ghcr.io/mycompany/myapp:v1.0.0",
            "ghcr.io/mycompany/myapp:latest"
        ],
        labels: {
            "org.opencontainers.image.source": "https://github.com/mycompany/myapp",
            "org.opencontainers.image.version": "v1.0.0",
            "org.opencontainers.image.created": timez.now_iso8601()
        },
        multi_arch: based,
        platforms: ["linux/amd64", "linux/arm64"]
    }

    # Build container
    vibez.spill("🐳 Building container...")
    sus build_result deploymentz.DeploymentResult = deploymentz.ContainerBuilder.build_image(build_config)
    
    ready (!build_result.success) {
        vibez.spill("❌ Build failed: {}", build_result.message)
        damn
    }
    
    vibez.spill("✅ Container built: {}", build_result.artifacts[0])

    # Security scanning
    vibez.spill("🔒 Running security scan...")
    sus scan_results map<tea, any> = deploymentz.ContainerBuilder.security_scan(build_result.artifacts[0])
    
    sus risk_level tea = scan_results["overall_risk"]?(tea)
    vibez.spill("🛡️ Security scan complete - Risk: {}", risk_level)
    
    ready (risk_level == "HIGH" || risk_level == "CRITICAL") {
        vibez.spill("❌ Security vulnerabilities found - blocking deployment")
        damn
    }
    
    vibez.spill("✅ Security scan passed")
}
```

### Multi-Stage Build Example

```dockerfile
# Multi-stage Dockerfile for CURSED applications
FROM cursed/build:latest AS dependencies
WORKDIR /app
COPY CursedPackage.toml CursedPackage.lock ./
RUN cursed-pkg install

FROM dependencies AS builder
COPY . .
RUN cursed-zig --compile --optimize=ReleaseFast \
    --strip --static --target=x86_64-linux \
    main.csd

FROM dependencies AS test
COPY . .
RUN cursed-zig test test_suite/

FROM scratch AS production
COPY --from=builder /app/main /app/main
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
USER 1001:1001
EXPOSE 8080
ENTRYPOINT ["/app/main"]
```

## Kubernetes Deployment

### Basic Deployment with kubernetesz

```cursed
# k8s_deploy.csd
yeet "kubernetesz"
yeet "deploymentz"
yeet "vibez"

slay main() {
    # Load Kubernetes configuration
    sus kube_config kubernetesz.KubeConfig = kubernetesz.load_kubeconfig("~/.kube/config") fam {
        when _ -> {
            vibez.spill("❌ Failed to load kubeconfig")
            damn
        }
    }

    # Create namespace
    sus namespace_result kubernetesz.KubeResult<tea> = kubernetesz.create_namespace(kube_config, "myapp")
    vibez.spill("✅ Namespace created: {}", namespace_result.data)

    # Deploy application
    deploy_application(kube_config)
    
    # Setup monitoring
    setup_monitoring(kube_config)
    
    # Configure auto-scaling
    setup_autoscaling(kube_config)
}

slay deploy_application(kube_config kubernetesz.KubeConfig) {
    # Deployment specification
    sus deployment_spec kubernetesz.DeploymentManager.DeploymentSpec = kubernetesz.DeploymentManager.DeploymentSpec{
        name: "myapp",
        namespace: "myapp",
        replicas: 3,
        image: "ghcr.io/mycompany/myapp:v1.0.0",
        labels: {
            "app": "myapp",
            "version": "v1.0.0",
            "tier": "backend"
        },
        selector: {
            "app": "myapp"
        },
        ports: [8080],
        env: {
            "NODE_ENV": "production",
            "LOG_LEVEL": "info",
            "METRICS_PORT": "9090"
        },
        resources: {
            "requests": {
                "cpu": "100m",
                "memory": "128Mi"
            },
            "limits": {
                "cpu": "500m",
                "memory": "512Mi"
            }
        },
        strategy: {
            "type": "RollingUpdate",
            "rollingUpdate": {
                "maxSurge": 1,
                "maxUnavailable": 1
            }
        }
    }

    # Create deployment
    sus deployment_result kubernetesz.KubeResult<tea> = kubernetesz.DeploymentManager.create_deployment(
        kube_config, deployment_spec
    )
    
    ready (!deployment_result.success) {
        vibez.spill("❌ Deployment failed: {}", deployment_result.error)
        damn
    }
    
    vibez.spill("✅ Deployment created: {}", deployment_result.data)

    # Create service
    sus service_spec kubernetesz.ServiceManager.ServiceSpec = kubernetesz.ServiceManager.ServiceSpec{
        name: "myapp-service",
        namespace: "myapp",
        type: "ClusterIP",
        selector: {"app": "myapp"},
        ports: [{
            "name": "http",
            "port": 80,
            "targetPort": 8080,
            "protocol": "TCP"
        }],
        labels: {"app": "myapp"}
    }

    sus service_result kubernetesz.KubeResult<tea> = kubernetesz.ServiceManager.create_service(
        kube_config, service_spec
    )
    
    vibez.spill("✅ Service created: {}", service_result.data)
}

slay setup_monitoring(kube_config kubernetesz.KubeConfig) {
    # Create ConfigMap for monitoring configuration
    sus monitoring_config map<tea, tea> = {
        "prometheus.yml": `global:
  scrape_interval: 15s
scrape_configs:
  - job_name: 'myapp'
    static_configs:
      - targets: ['myapp-service:9090']`,
        "alerts.yml": `groups:
  - name: myapp.rules
    rules:
    - alert: HighErrorRate
      expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.1
      for: 5m
      labels:
        severity: warning
      annotations:
        summary: High error rate detected`
    }

    sus config_result kubernetesz.KubeResult<tea> = kubernetesz.ConfigManager.create_configmap(
        kube_config, "myapp", "monitoring-config", monitoring_config
    )
    
    vibez.spill("✅ Monitoring ConfigMap created: {}", config_result.data)
}

slay setup_autoscaling(kube_config kubernetesz.KubeConfig) {
    # Horizontal Pod Autoscaler
    sus hpa_spec kubernetesz.AutoscalingManager.HPASpec = kubernetesz.AutoscalingManager.HPASpec{
        name: "myapp-hpa",
        namespace: "myapp",
        target_ref: {
            "apiVersion": "apps/v1",
            "kind": "Deployment",
            "name": "myapp"
        },
        min_replicas: 2,
        max_replicas: 20,
        target_cpu_utilization: 70,
        target_memory_utilization: 80
    }

    sus hpa_result kubernetesz.KubeResult<tea> = kubernetesz.AutoscalingManager.create_hpa(
        kube_config, hpa_spec
    )
    
    vibez.spill("✅ HPA created: {}", hpa_result.data)
}
```

### Advanced Deployment Strategies

#### Blue-Green Deployment
```cursed
slay blue_green_deployment(kube_config kubernetesz.KubeConfig) {
    # Deploy blue version (current)
    sus blue_spec kubernetesz.DeploymentManager.DeploymentSpec = create_deployment_spec("myapp-blue", "v1.0.0")
    sus blue_result kubernetesz.KubeResult<tea> = kubernetesz.DeploymentManager.create_deployment(kube_config, blue_spec)
    
    # Deploy green version (new)
    sus green_spec kubernetesz.DeploymentManager.DeploymentSpec = create_deployment_spec("myapp-green", "v1.1.0")
    sus green_result kubernetesz.KubeResult<tea> = kubernetesz.DeploymentManager.create_deployment(kube_config, green_spec)
    
    # Wait for green to be ready
    sus ready lit = deploymentz.KubernetesDeployment.wait_for_deployment_ready(
        kube_config, "myapp", "myapp-green", 300
    )
    
    ready (!ready) {
        vibez.spill("❌ Green deployment not ready, aborting")
        damn
    }
    
    # Switch traffic to green
    switch_service_to_green(kube_config)
    
    # Clean up blue deployment
    cleanup_blue_deployment(kube_config)
    
    vibez.spill("✅ Blue-green deployment completed")
}
```

#### Canary Deployment
```cursed
slay canary_deployment(kube_config kubernetesz.KubeConfig) {
    # Deploy canary with 10% traffic
    sus canary_spec kubernetesz.DeploymentManager.DeploymentSpec = create_deployment_spec("myapp-canary", "v1.1.0")
    canary_spec.replicas = 1  # 10% of total replicas
    
    sus canary_result kubernetesz.KubeResult<tea> = kubernetesz.DeploymentManager.create_deployment(kube_config, canary_spec)
    
    # Monitor canary metrics for 10 minutes
    sus monitoring_result lit = monitor_canary_deployment(kube_config, "myapp-canary", 600)
    
    ready (!monitoring_result) {
        vibez.spill("❌ Canary metrics failed, rolling back")
        rollback_canary(kube_config)
        damn
    }
    
    # Gradually increase canary traffic
    gradually_promote_canary(kube_config)
    
    vibez.spill("✅ Canary deployment completed")
}
```

## CI/CD Pipelines

### GitHub Actions Integration

```cursed
# ci_cd_setup.csd
yeet "deploymentz"
yeet "vibez"

slay setup_github_actions() {
    # Create deployment pipeline
    sus pipeline deploymentz.Pipeline = deploymentz.create_deployment_pipeline(
        "myapp-cicd",
        [
            deploymentz.DeploymentStage.Build,
            deploymentz.DeploymentStage.Test,
            deploymentz.DeploymentStage.SecurityScan,
            deploymentz.DeploymentStage.Package,
            deploymentz.DeploymentStage.Deploy,
            deploymentz.DeploymentStage.Validate
        ],
        "kubernetes"
    )

    # Create GitHub Actions workflow
    sus workflow_result deploymentz.DeploymentResult = deploymentz.CIIntegration.GitHub.create_workflow(
        "mycompany",  # Repository owner
        "myapp",      # Repository name
        "deploy",     # Workflow name
        pipeline
    )
    
    ready (workflow_result.success) {
        vibez.spill("✅ GitHub Actions workflow created")
    } otherwise {
        vibez.spill("❌ Failed to create workflow: {}", workflow_result.message)
    }
}
```

Generated GitHub Actions workflow:
```yaml
name: myapp-cicd

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Set up CURSED
      run: |
        curl -sSf https://install.cursedlang.org | sh
        echo "$HOME/.cursed/bin" >> $GITHUB_PATH
    
    - name: Build application
      run: |
        cursed-zig --compile --optimize=ReleaseFast main.csd
    
    - name: Run tests
      run: |
        cursed-zig test test_suite/
    
    - name: Set up Docker Buildx
      uses: docker/setup-buildx-action@v2
    
    - name: Login to Container Registry
      uses: docker/login-action@v2
      with:
        registry: ghcr.io
        username: ${{ github.actor }}
        password: ${{ secrets.GITHUB_TOKEN }}
    
    - name: Build and push container
      uses: docker/build-push-action@v4
      with:
        context: .
        push: true
        tags: |
          ghcr.io/${{ github.repository }}:${{ github.sha }}
          ghcr.io/${{ github.repository }}:latest
        platforms: linux/amd64,linux/arm64
    
    - name: Security scan
      run: |
        docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
          aquasec/trivy:latest image --exit-code 1 --severity HIGH,CRITICAL \
          ghcr.io/${{ github.repository }}:${{ github.sha }}
    
  deploy:
    needs: build
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
    - uses: actions/checkout@v3
    
    - name: Configure kubectl
      run: |
        mkdir -p ~/.kube
        echo "${{ secrets.KUBECONFIG }}" | base64 -d > ~/.kube/config
    
    - name: Deploy to Kubernetes
      run: |
        kubectl set image deployment/myapp \
          myapp=ghcr.io/${{ github.repository }}:${{ github.sha }} \
          -n production
        kubectl rollout status deployment/myapp -n production --timeout=300s
    
    - name: Run smoke tests
      run: |
        kubectl run smoke-test --image=curlimages/curl --rm -i --restart=Never \
          -- curl -f http://myapp-service.production.svc.cluster.local/health
```

### GitLab CI Integration

```cursed
slay setup_gitlab_ci() {
    sus pipeline deploymentz.Pipeline = create_basic_pipeline()
    
    sus gitlab_result deploymentz.DeploymentResult = deploymentz.CIIntegration.GitLab.create_pipeline(
        "12345",  # Project ID
        pipeline
    )
    
    vibez.spill("✅ GitLab CI pipeline created")
}
```

Generated GitLab CI:
```yaml
stages:
  - build
  - test
  - security
  - package
  - deploy

variables:
  DOCKER_DRIVER: overlay2
  DOCKER_TLS_CERTDIR: "/certs"

build:
  stage: build
  image: cursed/build:latest
  script:
    - cursed-zig --compile --optimize=ReleaseFast main.csd
  artifacts:
    paths:
      - main
    expire_in: 1 hour

test:
  stage: test
  image: cursed/build:latest
  script:
    - cursed-zig test test_suite/
  coverage: '/Coverage: \d+\.\d+%/'

security_scan:
  stage: security
  image: aquasec/trivy:latest
  script:
    - trivy fs --exit-code 1 --severity HIGH,CRITICAL .

package:
  stage: package
  image: docker:latest
  services:
    - docker:dind
  script:
    - docker build -t $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA .
    - docker push $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA

deploy_staging:
  stage: deploy
  image: bitnami/kubectl:latest
  script:
    - kubectl set image deployment/myapp myapp=$CI_REGISTRY_IMAGE:$CI_COMMIT_SHA -n staging
    - kubectl rollout status deployment/myapp -n staging --timeout=300s
  only:
    - develop

deploy_production:
  stage: deploy
  image: bitnami/kubectl:latest
  script:
    - kubectl set image deployment/myapp myapp=$CI_REGISTRY_IMAGE:$CI_COMMIT_SHA -n production
    - kubectl rollout status deployment/myapp -n production --timeout=300s
  when: manual
  only:
    - main
```

## Infrastructure as Code

### Terraform Integration

```cursed
# terraform_deployment.csd
yeet "deploymentz"
yeet "vibez"

slay main() {
    setup_aws_infrastructure()
    setup_kubernetes_cluster()
}

slay setup_aws_infrastructure() {
    sus terraform_config deploymentz.InfrastructureAsCode.Terraform.TerraformConfig = deploymentz.InfrastructureAsCode.Terraform.TerraformConfig{
        working_directory: "./terraform",
        terraform_file: "main.tf",
        variables: {
            "cluster_name": "production-eks",
            "region": "us-west-2",
            "node_instance_type": "t3.medium",
            "min_nodes": 2,
            "max_nodes": 10,
            "desired_nodes": 3
        },
        backend_config: {
            "bucket": "mycompany-terraform-state",
            "key": "eks/terraform.tfstate",
            "region": "us-west-2",
            "dynamodb_table": "terraform-state-lock"
        },
        provider_versions: {
            "aws": "~> 5.0",
            "kubernetes": "~> 2.0",
            "helm": "~> 2.0"
        }
    }

    # Plan infrastructure
    vibez.spill("📋 Planning infrastructure...")
    sus plan_result deploymentz.DeploymentResult = deploymentz.InfrastructureAsCode.Terraform.plan(terraform_config)
    
    ready (!plan_result.success) {
        vibez.spill("❌ Terraform plan failed: {}", plan_result.message)
        damn
    }
    
    vibez.spill("✅ Terraform plan completed")

    # Apply infrastructure
    vibez.spill("🏗️ Applying infrastructure...")
    sus apply_result deploymentz.DeploymentResult = deploymentz.InfrastructureAsCode.Terraform.apply(terraform_config)
    
    ready (!apply_result.success) {
        vibez.spill("❌ Terraform apply failed: {}", apply_result.message)
        damn
    }
    
    vibez.spill("✅ Infrastructure provisioned successfully")
}
```

Terraform configuration example:
```hcl
# terraform/main.tf
terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 2.0"
    }
    helm = {
      source  = "hashicorp/helm"
      version = "~> 2.0"
    }
  }
  
  backend "s3" {
    # Configuration provided via terraform_config
  }
}

# VPC
module "vpc" {
  source = "terraform-aws-modules/vpc/aws"
  
  name = "${var.cluster_name}-vpc"
  cidr = "10.0.0.0/16"
  
  azs             = ["${var.region}a", "${var.region}b", "${var.region}c"]
  private_subnets = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
  public_subnets  = ["10.0.101.0/24", "10.0.102.0/24", "10.0.103.0/24"]
  
  enable_nat_gateway = true
  enable_vpn_gateway = false
  
  tags = {
    "kubernetes.io/cluster/${var.cluster_name}" = "shared"
  }
}

# EKS Cluster
module "eks" {
  source = "terraform-aws-modules/eks/aws"
  
  cluster_name    = var.cluster_name
  cluster_version = "1.28"
  
  vpc_id     = module.vpc.vpc_id
  subnet_ids = module.vpc.private_subnets
  
  node_groups = {
    main = {
      desired_capacity = var.desired_nodes
      max_capacity     = var.max_nodes
      min_capacity     = var.min_nodes
      
      instance_types = [var.node_instance_type]
      
      k8s_labels = {
        Environment = "production"
        Application = "myapp"
      }
    }
  }
  
  # OIDC Provider for service accounts
  enable_irsa = true
  
  tags = {
    Environment = "production"
  }
}

# Install essential add-ons
resource "helm_release" "aws_load_balancer_controller" {
  name       = "aws-load-balancer-controller"
  repository = "https://aws.github.io/eks-charts"
  chart      = "aws-load-balancer-controller"
  namespace  = "kube-system"
  
  set {
    name  = "clusterName"
    value = module.eks.cluster_id
  }
  
  set {
    name  = "serviceAccount.create"
    value = "false"
  }
  
  set {
    name  = "serviceAccount.name"
    value = "aws-load-balancer-controller"
  }
  
  depends_on = [module.eks]
}

resource "helm_release" "cluster_autoscaler" {
  name       = "cluster-autoscaler"
  repository = "https://kubernetes.github.io/autoscaler"
  chart      = "cluster-autoscaler"
  namespace  = "kube-system"
  
  set {
    name  = "autoDiscovery.clusterName"
    value = module.eks.cluster_id
  }
  
  set {
    name  = "awsRegion"
    value = var.region
  }
  
  depends_on = [module.eks]
}

# Outputs
output "cluster_endpoint" {
  value = module.eks.cluster_endpoint
}

output "cluster_security_group_id" {
  value = module.eks.cluster_security_group_id
}

output "kubectl_config" {
  value = module.eks.kubeconfig
}
```

### Ansible Integration

```cursed
slay setup_ansible_configuration() {
    sus ansible_config deploymentz.InfrastructureAsCode.Ansible.AnsibleConfig = deploymentz.InfrastructureAsCode.Ansible.AnsibleConfig{
        playbook_path: "./ansible/site.yml",
        inventory_file: "./ansible/inventory.yml",
        variables: {
            "app_name": "myapp",
            "environment": "production",
            "replicas": 3
        },
        tags: ["deploy", "configure"],
        limit: "production",
        vault_password_file: "./ansible/.vault_pass"
    }

    sus ansible_result deploymentz.DeploymentResult = deploymentz.InfrastructureAsCode.Ansible.run_playbook(ansible_config)
    
    ready (ansible_result.success) {
        vibez.spill("✅ Ansible playbook executed successfully")
    } otherwise {
        vibez.spill("❌ Ansible execution failed: {}", ansible_result.message)
    }
}
```

Ansible playbook example:
```yaml
# ansible/site.yml
---
- name: Deploy CURSED application
  hosts: kubernetes_masters
  vars:
    app_name: "{{ app_name | default('myapp') }}"
    environment: "{{ environment | default('production') }}"
  tasks:
    - name: Create namespace
      kubernetes.core.k8s:
        name: "{{ app_name }}-{{ environment }}"
        api_version: v1
        kind: Namespace
        state: present
    
    - name: Deploy application
      kubernetes.core.k8s:
        state: present
        definition:
          apiVersion: apps/v1
          kind: Deployment
          metadata:
            name: "{{ app_name }}"
            namespace: "{{ app_name }}-{{ environment }}"
          spec:
            replicas: "{{ replicas | default(3) }}"
            selector:
              matchLabels:
                app: "{{ app_name }}"
            template:
              metadata:
                labels:
                  app: "{{ app_name }}"
              spec:
                containers:
                - name: "{{ app_name }}"
                  image: "ghcr.io/mycompany/{{ app_name }}:latest"
                  ports:
                  - containerPort: 8080
    
    - name: Create service
      kubernetes.core.k8s:
        state: present
        definition:
          apiVersion: v1
          kind: Service
          metadata:
            name: "{{ app_name }}-service"
            namespace: "{{ app_name }}-{{ environment }}"
          spec:
            selector:
              app: "{{ app_name }}"
            ports:
            - port: 80
              targetPort: 8080
            type: ClusterIP
```

## Monitoring & Observability

### Prometheus and Grafana Setup

```cursed
# monitoring_setup.csd
yeet "deploymentz"
yeet "kubernetesz"
yeet "vibez"

slay setup_comprehensive_monitoring() {
    sus kube_config kubernetesz.KubeConfig = load_kube_config()
    
    # Setup Prometheus monitoring
    setup_prometheus_monitoring(kube_config)
    
    # Setup Grafana dashboards
    setup_grafana_dashboards(kube_config)
    
    # Setup alerting
    setup_alerting_rules(kube_config)
    
    # Setup distributed tracing
    setup_jaeger_tracing(kube_config)
    
    # Setup log aggregation
    setup_log_aggregation(kube_config)
}

slay setup_prometheus_monitoring(kube_config kubernetesz.KubeConfig) {
    vibez.spill("📊 Setting up Prometheus monitoring...")
    
    # Configure Prometheus for the application
    sus monitoring_result deploymentz.DeploymentResult = deploymentz.ObservabilityIntegration.Prometheus.configure_monitoring(
        "myapp",      # Application name
        "production", # Namespace
        9090         # Metrics port
    )
    
    ready (!monitoring_result.success) {
        vibez.spill("❌ Prometheus setup failed: {}", monitoring_result.message)
        damn
    }
    
    # Create ServiceMonitor for scraping
    sus service_monitor tea = `apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: myapp-monitor
  namespace: production
  labels:
    app: myapp
spec:
  selector:
    matchLabels:
      app: myapp
  endpoints:
  - port: metrics
    interval: 30s
    path: /metrics
    honorLabels: true
  namespaceSelector:
    matchNames:
    - production`
    
    # Apply ServiceMonitor
    apply_yaml_manifest(kube_config, service_monitor)
    
    vibez.spill("✅ Prometheus monitoring configured")
}

slay setup_grafana_dashboards(kube_config kubernetesz.KubeConfig) {
    vibez.spill("📈 Setting up Grafana dashboards...")
    
    # Application performance dashboard
    sus app_dashboard tea = create_application_dashboard("myapp")
    
    # Infrastructure dashboard
    sus infra_dashboard tea = create_infrastructure_dashboard()
    
    # Business metrics dashboard
    sus business_dashboard tea = create_business_metrics_dashboard()
    
    # Create ConfigMaps for dashboards
    sus dashboard_config map<tea, tea> = {
        "app-dashboard.json": app_dashboard,
        "infra-dashboard.json": infra_dashboard,
        "business-dashboard.json": business_dashboard
    }
    
    sus config_result kubernetesz.KubeResult<tea> = kubernetesz.ConfigManager.create_configmap(
        kube_config, "monitoring", "grafana-dashboards", dashboard_config
    )
    
    vibez.spill("✅ Grafana dashboards configured")
}

slay setup_alerting_rules(kube_config kubernetesz.KubeConfig) {
    vibez.spill("🚨 Setting up alerting rules...")
    
    sus alerting_rules tea = `groups:
- name: application.rules
  rules:
  # Application health alerts
  - alert: ApplicationDown
    expr: up{job="myapp"} == 0
    for: 1m
    labels:
      severity: critical
    annotations:
      summary: "Application {{ $labels.instance }} is down"
      description: "Application has been down for more than 1 minute"
  
  # High error rate
  - alert: HighErrorRate
    expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.1
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "High error rate detected"
      description: "Error rate is {{ $value | humanizePercentage }} over the last 5 minutes"
  
  # High latency
  - alert: HighLatency
    expr: histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m])) > 0.5
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "High latency detected"
      description: "95th percentile latency is {{ $value }}s over the last 5 minutes"
  
  # Resource utilization
  - alert: HighCPUUsage
    expr: rate(container_cpu_usage_seconds_total{container="myapp"}[5m]) > 0.8
    for: 10m
    labels:
      severity: warning
    annotations:
      summary: "High CPU usage"
      description: "CPU usage is {{ $value | humanizePercentage }} for container {{ $labels.container }}"
  
  - alert: HighMemoryUsage
    expr: container_memory_usage_bytes{container="myapp"} / container_spec_memory_limit_bytes > 0.9
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "High memory usage"
      description: "Memory usage is {{ $value | humanizePercentage }} for container {{ $labels.container }}"

- name: infrastructure.rules
  rules:
  # Node resource alerts
  - alert: NodeHighCPU
    expr: 100 - (avg by(instance) (rate(node_cpu_seconds_total{mode="idle"}[5m])) * 100) > 80
    for: 10m
    labels:
      severity: warning
    annotations:
      summary: "Node high CPU usage"
      description: "Node {{ $labels.instance }} has high CPU usage: {{ $value }}%"
  
  - alert: NodeHighMemory
    expr: (node_memory_MemTotal_bytes - node_memory_MemAvailable_bytes) / node_memory_MemTotal_bytes * 100 > 85
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "Node high memory usage"
      description: "Node {{ $labels.instance }} has high memory usage: {{ $value }}%"
  
  # Kubernetes cluster alerts
  - alert: PodCrashLooping
    expr: rate(kube_pod_container_status_restarts_total[15m]) > 0
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "Pod is crash looping"
      description: "Pod {{ $labels.namespace }}/{{ $labels.pod }} is restarting frequently"`
    
    sus alert_config map<tea, tea> = {
        "alerting-rules.yml": alerting_rules
    }
    
    sus alert_result kubernetesz.KubeResult<tea> = kubernetesz.ConfigManager.create_configmap(
        kube_config, "monitoring", "prometheus-rules", alert_config
    )
    
    vibez.spill("✅ Alerting rules configured")
}

slay setup_jaeger_tracing(kube_config kubernetesz.KubeConfig) {
    vibez.spill("🔍 Setting up distributed tracing...")
    
    sus tracing_result deploymentz.DeploymentResult = deploymentz.ObservabilityIntegration.Jaeger.configure_tracing(
        "myapp", "production"
    )
    
    ready (!tracing_result.success) {
        vibez.spill("⚠️ Jaeger setup failed: {}", tracing_result.message)
        damn
    }
    
    vibez.spill("✅ Jaeger tracing configured")
}

slay create_application_dashboard(app_name tea) tea {
    damn stringz.format(`{
  "dashboard": {
    "id": null,
    "title": "{} Application Dashboard",
    "tags": ["application", "{}"],
    "timezone": "browser",
    "panels": [
      {
        "id": 1,
        "title": "Request Rate",
        "type": "graph",
        "gridPos": {"h": 8, "w": 12, "x": 0, "y": 0},
        "targets": [
          {
            "expr": "rate(http_requests_total{{app=\"{}\"}}[5m])",
            "legendFormat": "{{{{method}}}} {{{{status}}}}"
          }
        ],
        "yAxes": [
          {"label": "Requests/sec", "min": 0}
        ]
      },
      {
        "id": 2,
        "title": "Response Time",
        "type": "graph",
        "gridPos": {"h": 8, "w": 12, "x": 12, "y": 0},
        "targets": [
          {
            "expr": "histogram_quantile(0.50, rate(http_request_duration_seconds_bucket{{app=\"{}\"}}[5m]))",
            "legendFormat": "50th percentile"
          },
          {
            "expr": "histogram_quantile(0.95, rate(http_request_duration_seconds_bucket{{app=\"{}\"}}[5m]))",
            "legendFormat": "95th percentile"
          },
          {
            "expr": "histogram_quantile(0.99, rate(http_request_duration_seconds_bucket{{app=\"{}\"}}[5m]))",
            "legendFormat": "99th percentile"
          }
        ],
        "yAxes": [
          {"label": "Seconds", "min": 0}
        ]
      },
      {
        "id": 3,
        "title": "Error Rate",
        "type": "singlestat",
        "gridPos": {"h": 4, "w": 6, "x": 0, "y": 8},
        "targets": [
          {
            "expr": "rate(http_requests_total{{app=\"{}\",status=~\"5..\"}}[5m]) / rate(http_requests_total{{app=\"{}\"}}[5m]) * 100",
            "legendFormat": ""
          }
        ],
        "thresholds": "1,5",
        "colors": ["green", "yellow", "red"],
        "valueName": "current",
        "format": "percent",
        "postfix": "%"
      },
      {
        "id": 4,
        "title": "Active Pods",
        "type": "singlestat",
        "gridPos": {"h": 4, "w": 6, "x": 6, "y": 8},
        "targets": [
          {
            "expr": "kube_deployment_status_replicas_ready{{deployment=\"{}\"}}",
            "legendFormat": ""
          }
        ],
        "valueName": "current",
        "format": "short"
      },
      {
        "id": 5,
        "title": "CPU Usage",
        "type": "graph",
        "gridPos": {"h": 8, "w": 12, "x": 0, "y": 12},
        "targets": [
          {
            "expr": "rate(container_cpu_usage_seconds_total{{container=\"{}\"}}[5m]) * 100",
            "legendFormat": "{{{{pod}}}}"
          }
        ],
        "yAxes": [
          {"label": "CPU %", "min": 0, "max": 100}
        ]
      },
      {
        "id": 6,
        "title": "Memory Usage",
        "type": "graph",
        "gridPos": {"h": 8, "w": 12, "x": 12, "y": 12},
        "targets": [
          {
            "expr": "container_memory_usage_bytes{{container=\"{}\"}} / 1024 / 1024",
            "legendFormat": "{{{{pod}}}}"
          }
        ],
        "yAxes": [
          {"label": "Memory (MB)", "min": 0}
        ]
      }
    ],
    "time": {
      "from": "now-1h",
      "to": "now"
    },
    "refresh": "5s"
  }
}`, app_name, app_name, app_name, app_name, app_name, app_name, app_name, app_name, app_name, app_name, app_name)
}
```

## Service Mesh Integration

### Istio Service Mesh

```cursed
# service_mesh_setup.csd
yeet "deploymentz"
yeet "kubernetesz"
yeet "vibez"

slay setup_istio_service_mesh() {
    sus kube_config kubernetesz.KubeConfig = load_kube_config()
    
    # Enable Istio for the namespace
    enable_istio_injection(kube_config)
    
    # Configure traffic management
    setup_traffic_management(kube_config)
    
    # Setup security policies
    setup_security_policies(kube_config)
    
    # Configure observability
    setup_istio_observability(kube_config)
}

slay enable_istio_injection(kube_config kubernetesz.KubeConfig) {
    vibez.spill("🕸️ Enabling Istio service mesh...")
    
    sus istio_result deploymentz.DeploymentResult = deploymentz.ServiceMeshIntegration.Istio.enable_istio("production")
    
    ready (!istio_result.success) {
        vibez.spill("❌ Istio enablement failed: {}", istio_result.message)
        damn
    }
    
    vibez.spill("✅ Istio service mesh enabled")
}

slay setup_traffic_management(kube_config kubernetesz.KubeConfig) {
    vibez.spill("🚦 Setting up traffic management...")
    
    # Virtual Service for traffic routing
    sus virtual_service_result deploymentz.DeploymentResult = deploymentz.ServiceMeshIntegration.Istio.create_virtual_service(
        "myapp",
        "production",
        ["myapp.production.svc.cluster.local"],
        [
            {
                "match": [{"headers": {"version": {"exact": "v2"}}}],
                "route": [{"destination": {"host": "myapp", "subset": "v2"}}],
                "weight": 10
            },
            {
                "route": [{"destination": {"host": "myapp", "subset": "v1"}}],
                "weight": 90
            }
        ]
    )
    
    # Destination Rule for load balancing
    sus destination_rule tea = `apiVersion: networking.istio.io/v1beta1
kind: DestinationRule
metadata:
  name: myapp-destination
  namespace: production
spec:
  host: myapp
  trafficPolicy:
    loadBalancer:
      simple: LEAST_CONN
    connectionPool:
      tcp:
        maxConnections: 100
      http:
        http1MaxPendingRequests: 50
        maxRequestsPerConnection: 10
    circuitBreaker:
      consecutiveErrors: 3
      interval: 30s
      baseEjectionTime: 30s
      maxEjectionPercent: 50
  subsets:
  - name: v1
    labels:
      version: v1
  - name: v2
    labels:
      version: v2`
    
    apply_yaml_manifest(kube_config, destination_rule)
    
    # Gateway for external traffic
    sus gateway tea = `apiVersion: networking.istio.io/v1beta1
kind: Gateway
metadata:
  name: myapp-gateway
  namespace: production
spec:
  selector:
    istio: ingressgateway
  servers:
  - port:
      number: 80
      name: http
      protocol: HTTP
    hosts:
    - myapp.example.com
  - port:
      number: 443
      name: https
      protocol: HTTPS
    tls:
      mode: SIMPLE
      credentialName: myapp-tls
    hosts:
    - myapp.example.com`
    
    apply_yaml_manifest(kube_config, gateway)
    
    vibez.spill("✅ Traffic management configured")
}

slay setup_security_policies(kube_config kubernetesz.KubeConfig) {
    vibez.spill("🔒 Setting up security policies...")
    
    # mTLS Policy
    sus peer_authentication tea = `apiVersion: security.istio.io/v1beta1
kind: PeerAuthentication
metadata:
  name: default
  namespace: production
spec:
  mtls:
    mode: STRICT`
    
    apply_yaml_manifest(kube_config, peer_authentication)
    
    # Authorization Policy
    sus authorization_policy tea = `apiVersion: security.istio.io/v1beta1
kind: AuthorizationPolicy
metadata:
  name: myapp-authz
  namespace: production
spec:
  selector:
    matchLabels:
      app: myapp
  rules:
  - from:
    - source:
        principals: ["cluster.local/ns/frontend/sa/frontend-service-account"]
  - to:
    - operation:
        methods: ["GET", "POST"]
        paths: ["/api/*"]
  - when:
    - key: source.ip
      values: ["10.0.0.0/8"]`
    
    apply_yaml_manifest(kube_config, authorization_policy)
    
    # Request Authentication with JWT
    sus request_authentication tea = `apiVersion: security.istio.io/v1beta1
kind: RequestAuthentication
metadata:
  name: myapp-jwt
  namespace: production
spec:
  selector:
    matchLabels:
      app: myapp
  jwtRules:
  - issuer: "https://auth.example.com"
    jwksUri: "https://auth.example.com/.well-known/jwks.json"
    audiences:
    - "myapp-api"`
    
    apply_yaml_manifest(kube_config, request_authentication)
    
    vibez.spill("✅ Security policies configured")
}

slay setup_istio_observability(kube_config kubernetesz.KubeConfig) {
    vibez.spill("📊 Setting up Istio observability...")
    
    # Telemetry configuration
    sus telemetry tea = `apiVersion: telemetry.istio.io/v1alpha1
kind: Telemetry
metadata:
  name: default
  namespace: production
spec:
  metrics:
  - providers:
    - name: prometheus
  - overrides:
    - match:
        metric: ALL_METRICS
      tagOverrides:
        destination_app:
          value: "{{.destination_app | default \"unknown\"}}"
        source_app:
          value: "{{.source_app | default \"unknown\"}}"
  tracing:
  - providers:
    - name: jaeger
  accessLogging:
  - providers:
    - name: otel`
    
    apply_yaml_manifest(kube_config, telemetry)
    
    # Service Monitor for Istio metrics
    sus istio_service_monitor tea = `apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: istio-proxy
  namespace: istio-system
spec:
  selector:
    matchLabels:
      app: istiod
  endpoints:
  - port: http-monitoring
    interval: 15s
    path: /stats/prometheus`
    
    apply_yaml_manifest(kube_config, istio_service_monitor)
    
    vibez.spill("✅ Istio observability configured")
}
```

### Linkerd Service Mesh

```cursed
slay setup_linkerd_service_mesh() {
    vibez.spill("🔗 Setting up Linkerd service mesh...")
    
    # Inject Linkerd proxy
    sus linkerd_result deploymentz.DeploymentResult = deploymentz.ServiceMeshIntegration.Linkerd.inject_linkerd("k8s/deployment.yaml")
    
    ready (!linkerd_result.success) {
        vibez.spill("❌ Linkerd injection failed: {}", linkerd_result.message)
        damn
    }
    
    # Traffic Split for canary deployments
    sus traffic_split tea = `apiVersion: split.smi-spec.io/v1alpha1
kind: TrafficSplit
metadata:
  name: myapp-split
  namespace: production
spec:
  service: myapp
  backends:
  - service: myapp-stable
    weight: 90
  - service: myapp-canary
    weight: 10`
    
    apply_yaml_manifest(load_kube_config(), traffic_split)
    
    vibez.spill("✅ Linkerd service mesh configured")
}
```

## Production Best Practices

### Security Hardening

```cursed
# security_hardening.csd
yeet "kubernetesz"
yeet "deploymentz"
yeet "vibez"

slay implement_security_hardening() {
    sus kube_config kubernetesz.KubeConfig = load_kube_config()
    
    # Network policies
    setup_network_policies(kube_config)
    
    # Pod Security Standards
    setup_pod_security_standards(kube_config)
    
    # Secrets management
    setup_secrets_management(kube_config)
    
    # RBAC configuration
    setup_rbac(kube_config)
    
    # Security scanning
    setup_continuous_security_scanning(kube_config)
}

slay setup_network_policies(kube_config kubernetesz.KubeConfig) {
    vibez.spill("🌐 Setting up network policies...")
    
    # Default deny all policy
    sus default_deny tea = `apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: default-deny-all
  namespace: production
spec:
  podSelector: {}
  policyTypes:
  - Ingress
  - Egress`
    
    # Allow ingress from ingress controller
    sus allow_ingress tea = `apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: allow-ingress
  namespace: production
spec:
  podSelector:
    matchLabels:
      app: myapp
  policyTypes:
  - Ingress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: ingress-nginx
    ports:
    - protocol: TCP
      port: 8080`
    
    # Allow egress to external services
    sus allow_egress tea = `apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: allow-egress
  namespace: production
spec:
  podSelector:
    matchLabels:
      app: myapp
  policyTypes:
  - Egress
  egress:
  - to: []
    ports:
    - protocol: TCP
      port: 443  # HTTPS
    - protocol: TCP
      port: 53   # DNS
    - protocol: UDP
      port: 53   # DNS`
    
    apply_yaml_manifest(kube_config, default_deny)
    apply_yaml_manifest(kube_config, allow_ingress)
    apply_yaml_manifest(kube_config, allow_egress)
    
    vibez.spill("✅ Network policies configured")
}

slay setup_pod_security_standards(kube_config kubernetesz.KubeConfig) {
    vibez.spill("🛡️ Setting up Pod Security Standards...")
    
    # Pod Security Policy
    sus pod_security_policy tea = `apiVersion: policy/v1beta1
kind: PodSecurityPolicy
metadata:
  name: restricted
spec:
  privileged: false
  allowPrivilegeEscalation: false
  requiredDropCapabilities:
    - ALL
  volumes:
    - 'configMap'
    - 'emptyDir'
    - 'projected'
    - 'secret'
    - 'downwardAPI'
    - 'persistentVolumeClaim'
  runAsUser:
    rule: 'MustRunAsNonRoot'
  seLinux:
    rule: 'RunAsAny'
  fsGroup:
    rule: 'RunAsAny'`
    
    # Security Context in deployment
    sus secure_deployment_patch tea = `{
  "spec": {
    "template": {
      "spec": {
        "securityContext": {
          "runAsNonRoot": true,
          "runAsUser": 1001,
          "fsGroup": 2000,
          "seccompProfile": {
            "type": "RuntimeDefault"
          }
        },
        "containers": [{
          "name": "myapp",
          "securityContext": {
            "allowPrivilegeEscalation": false,
            "readOnlyRootFilesystem": true,
            "runAsNonRoot": true,
            "runAsUser": 1001,
            "capabilities": {
              "drop": ["ALL"]
            }
          }
        }]
      }
    }
  }
}`
    
    apply_yaml_manifest(kube_config, pod_security_policy)
    
    # Patch existing deployment
    patch_deployment_security(kube_config, "myapp", "production", secure_deployment_patch)
    
    vibez.spill("✅ Pod Security Standards configured")
}

slay setup_secrets_management(kube_config kubernetesz.KubeConfig) {
    vibez.spill("🔐 Setting up secrets management...")
    
    # External Secrets Operator configuration
    sus external_secret tea = `apiVersion: external-secrets.io/v1beta1
kind: ExternalSecret
metadata:
  name: myapp-secrets
  namespace: production
spec:
  refreshInterval: 5m
  secretStoreRef:
    name: aws-secrets-manager
    kind: SecretStore
  target:
    name: myapp-secrets
    creationPolicy: Owner
  data:
  - secretKey: database-password
    remoteRef:
      key: production/myapp/database
      property: password
  - secretKey: api-key
    remoteRef:
      key: production/myapp/api
      property: key`
    
    # Seal secrets for GitOps
    sus sealed_secret tea = `apiVersion: bitnami.com/v1alpha1
kind: SealedSecret
metadata:
  name: myapp-sealed-secrets
  namespace: production
spec:
  encryptedData:
    database-url: AgBy3i4OJSWK+PiTySYZZA9rO43cGDEQAx...
    redis-password: AgBy3i4OJSWK+PiTySYZZA9rO43cGDEQAx...
  template:
    metadata:
      name: myapp-secrets
      namespace: production`
    
    apply_yaml_manifest(kube_config, external_secret)
    apply_yaml_manifest(kube_config, sealed_secret)
    
    vibez.spill("✅ Secrets management configured")
}

slay setup_rbac(kube_config kubernetesz.KubeConfig) {
    vibez.spill("👤 Setting up RBAC...")
    
    # Service Account
    sus service_account tea = `apiVersion: v1
kind: ServiceAccount
metadata:
  name: myapp-service-account
  namespace: production
automountServiceAccountToken: false`
    
    # Role with minimal permissions
    sus role tea = `apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: myapp-role
  namespace: production
rules:
- apiGroups: [""]
  resources: ["configmaps"]
  verbs: ["get", "list"]
- apiGroups: [""]
  resources: ["secrets"]
  verbs: ["get"]`
    
    # Role Binding
    sus role_binding tea = `apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: myapp-role-binding
  namespace: production
subjects:
- kind: ServiceAccount
  name: myapp-service-account
  namespace: production
roleRef:
  kind: Role
  name: myapp-role
  apiGroup: rbac.authorization.k8s.io`
    
    apply_yaml_manifest(kube_config, service_account)
    apply_yaml_manifest(kube_config, role)
    apply_yaml_manifest(kube_config, role_binding)
    
    vibez.spill("✅ RBAC configured")
}
```

### Performance Optimization

```cursed
# performance_optimization.csd
yeet "kubernetesz"
yeet "vibez"

slay optimize_application_performance() {
    sus kube_config kubernetesz.KubeConfig = load_kube_config()
    
    # Resource optimization
    optimize_resource_allocation(kube_config)
    
    # Caching strategies
    implement_caching_strategies(kube_config)
    
    # Auto-scaling configuration
    configure_advanced_autoscaling(kube_config)
    
    # Performance monitoring
    setup_performance_monitoring(kube_config)
}

slay optimize_resource_allocation(kube_config kubernetesz.KubeConfig) {
    vibez.spill("⚡ Optimizing resource allocation...")
    
    # Vertical Pod Autoscaler
    sus vpa tea = `apiVersion: autoscaling.k8s.io/v1
kind: VerticalPodAutoscaler
metadata:
  name: myapp-vpa
  namespace: production
spec:
  targetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: myapp
  updatePolicy:
    updateMode: "Auto"
  resourcePolicy:
    containerPolicies:
    - containerName: myapp
      minAllowed:
        cpu: 100m
        memory: 128Mi
      maxAllowed:
        cpu: 2
        memory: 2Gi
      controlledResources: ["cpu", "memory"]`
    
    # Pod Disruption Budget
    sus pdb tea = `apiVersion: policy/v1
kind: PodDisruptionBudget
metadata:
  name: myapp-pdb
  namespace: production
spec:
  minAvailable: 2
  selector:
    matchLabels:
      app: myapp`
    
    # Resource Quotas
    sus resource_quota tea = `apiVersion: v1
kind: ResourceQuota
metadata:
  name: production-quota
  namespace: production
spec:
  hard:
    requests.cpu: "10"
    requests.memory: 20Gi
    limits.cpu: "20"
    limits.memory: 40Gi
    persistentvolumeclaims: "10"`
    
    apply_yaml_manifest(kube_config, vpa)
    apply_yaml_manifest(kube_config, pdb)
    apply_yaml_manifest(kube_config, resource_quota)
    
    vibez.spill("✅ Resource allocation optimized")
}

slay implement_caching_strategies(kube_config kubernetesz.KubeConfig) {
    vibez.spill("🚀 Implementing caching strategies...")
    
    # Redis cache deployment
    sus redis_deployment tea = `apiVersion: apps/v1
kind: Deployment
metadata:
  name: redis-cache
  namespace: production
spec:
  replicas: 3
  selector:
    matchLabels:
      app: redis-cache
  template:
    metadata:
      labels:
        app: redis-cache
    spec:
      containers:
      - name: redis
        image: redis:7-alpine
        ports:
        - containerPort: 6379
        resources:
          requests:
            cpu: 100m
            memory: 128Mi
          limits:
            cpu: 500m
            memory: 512Mi
        livenessProbe:
          tcpSocket:
            port: 6379
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          exec:
            command:
            - redis-cli
            - ping
          initialDelaySeconds: 5
          periodSeconds: 5`
    
    # CDN configuration
    sus cdn_config tea = `apiVersion: v1
kind: ConfigMap
metadata:
  name: cdn-config
  namespace: production
data:
  nginx.conf: |
    upstream backend {
        server myapp-service:80;
    }
    
    server {
        listen 80;
        
        # Static content caching
        location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg)$ {
            expires 1y;
            add_header Cache-Control "public, immutable";
            try_files $uri @backend;
        }
        
        # API responses with short cache
        location /api/ {
            expires 5m;
            add_header Cache-Control "public";
            proxy_pass http://backend;
        }
        
        location @backend {
            proxy_pass http://backend;
        }
    }`
    
    apply_yaml_manifest(kube_config, redis_deployment)
    apply_yaml_manifest(kube_config, cdn_config)
    
    vibez.spill("✅ Caching strategies implemented")
}
```

### Disaster Recovery

```cursed
# disaster_recovery.csd
yeet "cloudz"
yeet "kubernetesz"
yeet "vibez"

slay setup_disaster_recovery() {
    # Multi-region backup strategy
    setup_multi_region_backup()
    
    # Database backup and restore
    setup_database_backup()
    
    # Application state backup
    setup_application_backup()
    
    # Automated failover
    setup_automated_failover()
}

slay setup_multi_region_backup() {
    vibez.spill("🌍 Setting up multi-region backup...")
    
    # Primary region (us-west-2)
    sus primary_resources []cloudz.CloudResource = cloudz.list_resources(
        cloudz.CloudProvider.AWS, "us-west-2"
    ).data
    
    # Backup to secondary region (us-east-1)
    bestie resource in primary_resources {
        ready (resource.type == cloudz.ResourceType.Database) {
            create_cross_region_backup(resource, "us-east-1")
        }
        ready (resource.type == cloudz.ResourceType.Storage) {
            replicate_storage_cross_region(resource, "us-east-1")
        }
    }
    
    vibez.spill("✅ Multi-region backup configured")
}

slay setup_database_backup() {
    vibez.spill("💾 Setting up database backup...")
    
    # Automated database backups
    sus backup_job tea = `apiVersion: batch/v1
kind: CronJob
metadata:
  name: database-backup
  namespace: production
spec:
  schedule: "0 2 * * *"  # Daily at 2 AM
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: backup
            image: postgres:14
            env:
            - name: PGPASSWORD
              valueFrom:
                secretKeyRef:
                  name: database-credentials
                  key: password
            command:
            - /bin/bash
            - -c
            - |
              pg_dump -h database-host -U postgres myapp_db | \
              aws s3 cp - s3://myapp-backups/database/$(date +%Y%m%d_%H%M%S).sql
          restartPolicy: OnFailure`
    
    apply_yaml_manifest(load_kube_config(), backup_job)
    
    vibez.spill("✅ Database backup configured")
}
```

## Troubleshooting

### Common Issues and Solutions

#### Build Issues
```bash
# Container build fails
docker build --no-cache -t myapp:debug .
docker run -it myapp:debug /bin/bash

# Multi-architecture build issues
docker buildx create --use
docker buildx build --platform linux/amd64,linux/arm64 -t myapp:latest .
```

#### Kubernetes Deployment Issues
```cursed
# Debug deployment issues
slay debug_deployment_issues(kube_config kubernetesz.KubeConfig) {
    # Check deployment status
    sus deployment_status kubernetesz.KubeResult<map<tea, any>> = kubernetesz.DeploymentManager.get_deployment_status(
        kube_config, "production", "myapp"
    )
    
    ready (deployment_status.success) {
        sus status map<tea, any> = deployment_status.data["status"]?(map<tea, any>)
        vibez.spill("Deployment status: {}", jsonz.marshal(status))
    }
    
    # Check pod logs
    sus pods kubernetesz.KubeResult<[]map<tea, any>> = kubernetesz.PodManager.list_pods(
        kube_config, "production", "app=myapp"
    )
    
    bestie pod in pods.data {
        sus pod_name tea = pod["metadata"]?("name")?(tea)
        sus logs kubernetesz.KubeResult<tea> = kubernetesz.PodManager.get_pod_logs(
            kube_config, "production", pod_name, nah
        )
        vibez.spill("Pod {} logs: {}", pod_name, logs.data)
    }
}
```

#### Performance Issues
```cursed
# Monitor resource usage
slay monitor_resource_usage(kube_config kubernetesz.KubeConfig) {
    sus metrics kubernetesz.KubeResult<kubernetesz.MonitoringManager.ClusterMetrics> = kubernetesz.MonitoringManager.get_cluster_metrics(kube_config)
    
    ready (metrics.success) {
        sus cluster_metrics kubernetesz.MonitoringManager.ClusterMetrics = metrics.data
        
        vibez.spill("Cluster Metrics:")
        vibez.spill("- CPU Usage: {:.1f}%", cluster_metrics.cpu_usage)
        vibez.spill("- Memory Usage: {:.1f}%", cluster_metrics.memory_usage)
        vibez.spill("- Disk Usage: {:.1f}%", cluster_metrics.disk_usage)
        
        ready (cluster_metrics.cpu_usage > 80.0) {
            vibez.spill("⚠️ High CPU usage detected")
        }
        ready (cluster_metrics.memory_usage > 80.0) {
            vibez.spill("⚠️ High memory usage detected")
        }
    }
}
```

#### Network Issues
```bash
# Debug network connectivity
kubectl run debug-pod --image=nicolaka/netshoot -it --rm -- /bin/bash

# Inside the debug pod:
nslookup myapp-service.production.svc.cluster.local
curl -v http://myapp-service.production.svc.cluster.local
telnet myapp-service.production.svc.cluster.local 80
```

### Logging and Debugging

```cursed
# comprehensive_debugging.csd
yeet "kubernetesz"
yeet "vibez"

slay comprehensive_health_check() {
    sus kube_config kubernetesz.KubeConfig = load_kube_config()
    
    vibez.spill("🔍 Running comprehensive health check...")
    
    # Check cluster health
    check_cluster_health(kube_config)
    
    # Check application health
    check_application_health(kube_config)
    
    # Check resource utilization
    check_resource_utilization(kube_config)
    
    # Check security compliance
    check_security_compliance(kube_config)
}

slay check_cluster_health(kube_config kubernetesz.KubeConfig) {
    vibez.spill("🏥 Checking cluster health...")
    
    # Check node status
    sus node_health lit = check_node_health(kube_config)
    ready (!node_health) {
        vibez.spill("❌ Node health issues detected")
    } otherwise {
        vibez.spill("✅ All nodes healthy")
    }
    
    # Check system pods
    sus system_health lit = check_system_pods_health(kube_config)
    ready (!system_health) {
        vibez.spill("❌ System pod issues detected")
    } otherwise {
        vibez.spill("✅ All system pods healthy")
    }
}

slay check_application_health(kube_config kubernetesz.KubeConfig) {
    vibez.spill("🔧 Checking application health...")
    
    # Check deployment readiness
    sus deployment_ready lit = deploymentz.KubernetesDeployment.wait_for_deployment_ready(
        kube_config, "production", "myapp", 30
    )
    
    ready (!deployment_ready) {
        vibez.spill("❌ Application deployment not ready")
        # Get detailed status
        debug_deployment_issues(kube_config)
    } otherwise {
        vibez.spill("✅ Application deployment ready")
    }
    
    # Health check endpoints
    sus health_check_passed lit = deploymentz.KubernetesDeployment.validate_deployment_health(
        kube_config, "production", "myapp", "/health"
    )
    
    ready (!health_check_passed) {
        vibez.spill("❌ Application health checks failing")
    } otherwise {
        vibez.spill("✅ Application health checks passing")
    }
}
```

This comprehensive guide provides everything needed to build, deploy, and manage enterprise-grade cloud-native applications with CURSED. The modular approach allows teams to adopt individual components as needed while providing a complete solution for full cloud-native deployment scenarios.

For additional help and examples, see the `/examples` directory and the individual module documentation in `/stdlib`.
