# Cloud Monitoring Enhanced Test Suite
# Tests real metrics calculations instead of placeholder values

yeet "vibez"
yeet "timez"
yeet "stringz"
yeet "mathz"
yeet "testz"
yeet "cloudz"
yeet "kubernetesz"

# Cloud monitoring test suite
test_start("Cloud Monitoring Enhanced Metrics")

# Test AWS cost analysis with real calculations
slay test_aws_real_cost_analysis() {
    vibez.spill("Testing AWS real cost analysis...")
    
    # Create test resource with realistic metadata
    sus test_resource cloudz.CloudResource = cloudz.CloudResource{
        id: "i-1234567890abcdef0",
        name: "web-server-prod",
        type: cloudz.ResourceType.Compute,
        provider: cloudz.CloudProvider.AWS,
        region: "us-east-1",
        tags: {
            "Environment": "production",
            "Team": "backend",
            "Cost-Center": "engineering"
        },
        metadata: {
            "instance_type": "t3.medium",
            "storage_gb": 100,
            "image": "ami-ubuntu-20.04"
        },
        created_at: timez.now() - 864000,  # 10 days ago
        updated_at: timez.now()
    }
    
    # Test real cost calculation
    sus actual_cost drip = cloudz.MultiCloud.get_resource_cost(test_resource)
    assert_greater_than_float(actual_cost, 0.0)
    assert_less_than_float(actual_cost, 200.0)  # Reasonable monthly cost
    vibez.spill("  ✓ Actual cost: ${:.2f}/month", actual_cost)
    
    # Test savings analysis
    sus savings drip = cloudz.MultiCloud.analyze_aws_costs(test_resource)
    assert_greater_than_float(savings, 0.0)
    assert_less_than_float(savings, actual_cost)  # Savings should be less than total cost
    vibez.spill("  ✓ Potential savings: ${:.2f}/month", savings)
    
    # Test different instance types
    test_resource.metadata["instance_type"] = "m5.large"
    sus large_cost drip = cloudz.MultiCloud.get_resource_cost(test_resource)
    assert_greater_than_float(large_cost, actual_cost)  # Larger instances cost more
    vibez.spill("  ✓ Large instance cost: ${:.2f}/month", large_cost)
    
    vibez.spill("AWS cost analysis test passed")
}

# Test Azure cost analysis with hybrid benefits
slay test_azure_hybrid_benefits() {
    vibez.spill("Testing Azure hybrid benefits calculation...")
    
    # Test Windows VM with hybrid benefit potential
    sus windows_vm cloudz.CloudResource = cloudz.CloudResource{
        id: "vm-windows-prod",
        name: "windows-server-prod",
        type: cloudz.ResourceType.Compute,
        provider: cloudz.CloudProvider.Azure,
        region: "eastus",
        tags: {
            "Environment": "production",
            "OS": "Windows"
        },
        metadata: {
            "instance_type": "Standard_D4s_v3",
            "storage_gb": 128,
            "image": "Windows-Server-2019"
        },
        created_at: timez.now() - 2592000,  # 30 days ago
        updated_at: timez.now()
    }
    
    sus windows_cost drip = cloudz.MultiCloud.get_resource_cost(windows_vm)
    sus windows_savings drip = cloudz.MultiCloud.analyze_azure_costs(windows_vm)
    
    # Windows VMs should have significant hybrid benefit potential
    assert_greater_than_float(windows_savings, windows_cost * 0.2)  # At least 20% savings
    vibez.spill("  ✓ Windows VM hybrid benefit savings: ${:.2f}/month", windows_savings)
    
    # Test Linux VM (less savings potential)
    windows_vm.metadata["image"] = "Ubuntu-20.04"
    sus linux_savings drip = cloudz.MultiCloud.analyze_azure_costs(windows_vm)
    assert_less_than_float(linux_savings, windows_savings)  # Linux has less savings potential
    vibez.spill("  ✓ Linux VM savings: ${:.2f}/month", linux_savings)
    
    vibez.spill("Azure hybrid benefits test passed")
}

# Test GCP custom machine optimization
slay test_gcp_custom_machine_optimization() {
    vibez.spill("Testing GCP custom machine optimization...")
    
    # Create resource with underutilized standard machine
    sus gcp_vm cloudz.CloudResource = cloudz.CloudResource{
        id: "vm-underutilized",
        name: "analytics-dev",
        type: cloudz.ResourceType.Compute,
        provider: cloudz.CloudProvider.GCP,
        region: "us-central1",
        tags: {
            "Environment": "development",
            "Workload": "batch"
        },
        metadata: {
            "instance_type": "n1-standard-4",
            "storage_gb": 50
        },
        created_at: timez.now() - 1209600,  # 2 weeks ago
        updated_at: timez.now()
    }
    
    sus gcp_cost drip = cloudz.MultiCloud.get_resource_cost(gcp_vm)
    sus gcp_savings drip = cloudz.MultiCloud.analyze_gcp_costs(gcp_vm)
    
    # Should identify custom machine and preemptible savings
    assert_greater_than_float(gcp_savings, 0.0)
    vibez.spill("  ✓ GCP optimization savings: ${:.2f}/month", gcp_savings)
    
    # Test sustained use discount calculation
    gcp_vm.metadata["monthly_hours"] = 500.0  # Heavy usage
    sus heavy_usage_savings drip = cloudz.MultiCloud.analyze_gcp_costs(gcp_vm)
    assert_greater_than_float(heavy_usage_savings, gcp_savings * 0.8)  # Should get sustained use discounts
    vibez.spill("  ✓ Sustained use discount included", "")
    
    vibez.spill("GCP custom machine test passed")
}

# Test Kubernetes cluster metrics calculation
slay test_kubernetes_real_metrics() {
    vibez.spill("Testing Kubernetes real metrics calculation...")
    
    # Mock nodes data structure
    sus mock_nodes_data map<tea, any> = {
        "items": [
            {
                "metadata": {
                    "name": "node1",
                    "labels": {
                        "node-role.kubernetes.io/worker": "",
                        "node.kubernetes.io/instance-type": "m5.large"
                    }
                },
                "status": {
                    "capacity": {
                        "cpu": "2",
                        "memory": "8Gi",
                        "ephemeral-storage": "100Gi",
                        "pods": "110"
                    },
                    "allocatable": {
                        "cpu": "1900m",
                        "memory": "7Gi"
                    },
                    "conditions": [
                        {
                            "type": "Ready",
                            "status": "True"
                        }
                    ]
                },
                "spec": {
                    "taints": []
                }
            },
            {
                "metadata": {
                    "name": "node2",
                    "labels": {
                        "node-role.kubernetes.io/control-plane": "",
                        "node.kubernetes.io/instance-type": "m5.medium"
                    }
                },
                "status": {
                    "capacity": {
                        "cpu": "2",
                        "memory": "4Gi",
                        "ephemeral-storage": "50Gi",
                        "pods": "110"
                    },
                    "allocatable": {
                        "cpu": "1800m",
                        "memory": "3Gi"
                    },
                    "conditions": [
                        {
                            "type": "Ready",
                            "status": "True"
                        }
                    ]
                },
                "spec": {
                    "taints": []
                }
            }
        ]
    }
    
    # Test CPU usage calculation
    sus cpu_usage drip = kubernetesz.MonitoringManager.calculate_cpu_usage(mock_nodes_data)
    assert_greater_than_float(cpu_usage, 0.0)
    assert_less_than_float(cpu_usage, 100.0)
    vibez.spill("  ✓ Cluster CPU usage: {:.1f}%", cpu_usage)
    
    # Test memory usage calculation
    sus memory_usage drip = kubernetesz.MonitoringManager.calculate_memory_usage(mock_nodes_data)
    assert_greater_than_float(memory_usage, 0.0)
    assert_less_than_float(memory_usage, 100.0)
    vibez.spill("  ✓ Cluster memory usage: {:.1f}%", memory_usage)
    
    # Test disk usage calculation
    sus disk_usage drip = kubernetesz.MonitoringManager.calculate_disk_usage(mock_nodes_data)
    assert_greater_than_float(disk_usage, 0.0)
    assert_less_than_float(disk_usage, 100.0)
    vibez.spill("  ✓ Cluster disk usage: {:.1f}%", disk_usage)
    
    # Test network I/O calculation
    sus network_io drip = kubernetesz.MonitoringManager.calculate_network_io(mock_nodes_data)
    assert_greater_than_float(network_io, 0.0)
    assert_less_than_float(network_io, 50000.0)  # Reasonable max throughput
    vibez.spill("  ✓ Cluster network I/O: {:.1f} Mbps", network_io)
    
    vibez.spill("Kubernetes metrics test passed")
}

# Test resource parsing functions
slay test_kubernetes_resource_parsing() {
    vibez.spill("Testing Kubernetes resource parsing...")
    
    # Test CPU resource parsing
    assert_eq_float(kubernetesz.MonitoringManager.parse_cpu_resource("2"), 2.0)
    assert_eq_float(kubernetesz.MonitoringManager.parse_cpu_resource("1500m"), 1.5)
    assert_eq_float(kubernetesz.MonitoringManager.parse_cpu_resource("500m"), 0.5)
    vibez.spill("  ✓ CPU parsing works correctly")
    
    # Test memory resource parsing
    assert_eq_float(kubernetesz.MonitoringManager.parse_memory_resource("4Gi"), 4294967296.0)
    assert_eq_float(kubernetesz.MonitoringManager.parse_memory_resource("1024Mi"), 1073741824.0)
    assert_eq_float(kubernetesz.MonitoringManager.parse_memory_resource("1048576Ki"), 1073741824.0)
    vibez.spill("  ✓ Memory parsing works correctly")
    
    # Test network capacity estimation
    assert_eq_float(kubernetesz.MonitoringManager.estimate_node_network_capacity("t3.micro"), 100.0)
    assert_eq_float(kubernetesz.MonitoringManager.estimate_node_network_capacity("m5.large"), 1000.0)
    assert_eq_float(kubernetesz.MonitoringManager.estimate_node_network_capacity("c5.2xlarge"), 5000.0)
    vibez.spill("  ✓ Network capacity estimation works correctly")
    
    vibez.spill("Kubernetes parsing test passed")
}

# Test environment detection
slay test_environment_detection() {
    vibez.spill("Testing environment tag detection...")
    
    # Test explicit environment tags
    sus prod_resource cloudz.CloudResource = cloudz.CloudResource{
        id: "test-1",
        name: "web-server",
        type: cloudz.ResourceType.Compute,
        provider: cloudz.CloudProvider.AWS,
        region: "us-east-1",
        tags: {"Environment": "production"},
        metadata: {},
        created_at: timez.now(),
        updated_at: timez.now()
    }
    
    assert_eq_string(cloudz.MultiCloud.get_environment_tag(prod_resource), "production")
    
    # Test name-based inference
    prod_resource.tags = {}
    prod_resource.name = "web-server-prod-1"
    assert_eq_string(cloudz.MultiCloud.get_environment_tag(prod_resource), "production")
    
    prod_resource.name = "dev-api-server"
    assert_eq_string(cloudz.MultiCloud.get_environment_tag(prod_resource), "development")
    
    prod_resource.name = "test-database"
    assert_eq_string(cloudz.MultiCloud.get_environment_tag(prod_resource), "test")
    
    vibez.spill("  ✓ Environment detection works correctly")
    vibez.spill("Environment detection test passed")
}

# Test multi-cloud cost comparison
slay test_multi_cloud_cost_comparison() {
    vibez.spill("Testing multi-cloud cost comparison...")
    
    # Create identical resources across providers
    sus base_resource cloudz.CloudResource = cloudz.CloudResource{
        id: "test-comparison",
        name: "web-server-prod",
        type: cloudz.ResourceType.Compute,
        region: "us-east-1",
        tags: {"Environment": "production"},
        metadata: {
            "instance_type": "medium",
            "storage_gb": 50
        },
        created_at: timez.now() - 2592000,
        updated_at: timez.now()
    }
    
    # Test AWS
    base_resource.provider = cloudz.CloudProvider.AWS
    sus aws_cost drip = cloudz.MultiCloud.get_resource_cost(base_resource)
    
    # Test Azure
    base_resource.provider = cloudz.CloudProvider.Azure
    sus azure_cost drip = cloudz.MultiCloud.get_resource_cost(base_resource)
    
    # Test GCP
    base_resource.provider = cloudz.CloudProvider.GCP
    sus gcp_cost drip = cloudz.MultiCloud.get_resource_cost(base_resource)
    
    # Verify cost relationships (GCP should be cheapest, AWS most expensive)
    assert_less_than_float(gcp_cost, aws_cost)
    assert_less_than_float(azure_cost, aws_cost)
    
    vibez.spill("  ✓ AWS cost: ${:.2f}/month", aws_cost)
    vibez.spill("  ✓ Azure cost: ${:.2f}/month", azure_cost)
    vibez.spill("  ✓ GCP cost: ${:.2f}/month", gcp_cost)
    vibez.spill("Multi-cloud comparison test passed")
}

# Test utilization factor calculations
slay test_utilization_factors() {
    vibez.spill("Testing utilization factor calculations...")
    
    # Test resource age impact
    sus new_resource cloudz.CloudResource = cloudz.CloudResource{
        id: "new-resource",
        name: "api-server",
        type: cloudz.ResourceType.Compute,
        provider: cloudz.CloudProvider.AWS,
        region: "us-east-1",
        tags: {"Environment": "production"},
        metadata: {"instance_type": "m5.large"},
        created_at: timez.now() - 86400,  # 1 day ago
        updated_at: timez.now()
    }
    
    sus old_resource cloudz.CloudResource = cloudz.CloudResource{
        id: "old-resource",
        name: "api-server",
        type: cloudz.ResourceType.Compute,
        provider: cloudz.CloudProvider.AWS,
        region: "us-east-1",
        tags: {"Environment": "production"},
        metadata: {"instance_type": "m5.large"},
        created_at: timez.now() - 2592000,  # 30 days ago
        updated_at: timez.now()
    }
    
    sus new_utilization drip = cloudz.MultiCloud.get_resource_cpu_utilization(new_resource)
    sus old_utilization drip = cloudz.MultiCloud.get_resource_cpu_utilization(old_resource)
    
    # Both should be reasonable values
    assert_greater_than_float(new_utilization, 0.0)
    assert_less_than_float(new_utilization, 100.0)
    assert_greater_than_float(old_utilization, 0.0)
    assert_less_than_float(old_utilization, 100.0)
    
    vibez.spill("  ✓ New resource CPU utilization: {:.1f}%", new_utilization)
    vibez.spill("  ✓ Old resource CPU utilization: {:.1f}%", old_utilization)
    
    vibez.spill("Utilization factors test passed")
}

# Run all tests
slay main() {
    vibez.spill("=== Cloud Monitoring Enhanced Test Suite ===")
    vibez.spill("")
    
    test_aws_real_cost_analysis()
    vibez.spill("")
    
    test_azure_hybrid_benefits()
    vibez.spill("")
    
    test_gcp_custom_machine_optimization()
    vibez.spill("")
    
    test_kubernetes_real_metrics()
    vibez.spill("")
    
    test_kubernetes_resource_parsing()
    vibez.spill("")
    
    test_environment_detection()
    vibez.spill("")
    
    test_multi_cloud_cost_comparison()
    vibez.spill("")
    
    test_utilization_factors()
    vibez.spill("")
    
    vibez.spill("=== All Cloud Monitoring Tests Completed ===")
    print_test_summary()
}

main()
