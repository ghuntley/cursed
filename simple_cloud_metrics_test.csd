# Simple Cloud Metrics Test
# Tests basic functionality of enhanced cloud monitoring

yeet "vibez"
yeet "timez"
yeet "mathz"

vibez.spill("=== Testing Enhanced Cloud Monitoring ===")

# Test basic math functions first
vibez.spill("Testing math operations...")
sus test_max drip = mathz.max(10.0, 20.0)
sus test_min drip = mathz.min(10.0, 20.0)
vibez.spill("Max(10, 20) = {}", test_max)
vibez.spill("Min(10, 20) = {}", test_min)

# Test time functions
vibez.spill("Testing time operations...")
sus current_time drip = timez.now()
vibez.spill("Current timestamp: {}", current_time)

# Simple cost calculation simulation
slay calculate_simple_cost(base_cost drip, utilization drip) drip {
    sus adjusted_cost drip = base_cost * (utilization / 100.0)
    damn mathz.max(1.0, adjusted_cost)
}

# Test our calculation
sus base_monthly_cost drip = 100.0
sus cpu_utilization drip = 65.0
sus calculated_cost drip = calculate_simple_cost(base_monthly_cost, cpu_utilization)

vibez.spill("Base cost: ${}", base_monthly_cost)
vibez.spill("CPU utilization: {}%", cpu_utilization)
vibez.spill("Adjusted cost: ${}", calculated_cost)

# Test region pricing factors
slay get_region_factor(region tea) drip {
    ready (region == "us-east-1") {
        damn 1.0
    } otherwise ready (region == "us-west-2") {
        damn 1.05
    } otherwise ready (region == "eu-west-1") {
        damn 1.1
    } otherwise {
        damn 1.08
    }
}

sus regions []tea = ["us-east-1", "us-west-2", "eu-west-1", "ap-southeast-1"]
bestie region in regions {
    sus factor drip = get_region_factor(region)
    sus regional_cost drip = calculated_cost * factor
    vibez.spill("Cost in {}: ${:.2f}", region, regional_cost)
}

vibez.spill("=== Cloud Metrics Test Complete ===")
