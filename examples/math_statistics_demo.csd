// CURSED Statistics Module Demo
// Demonstrates comprehensive statistical analysis capabilities

import "stdlib::math::statistics";

slay main() {
    println("=== CURSED Statistics Module Demo ===\n")?;
    
    // Sample dataset for analysis
    sus sales_data = [120.5, 135.2, 98.7, 156.3, 142.8, 89.1, 167.4, 
                      133.9, 145.6, 112.3, 178.2, 124.7, 139.8, 155.1, 
                      91.4, 168.7, 129.3, 147.2, 136.5, 159.8];
    
    println("Dataset: Sales figures for 20 weeks")?;
    printf("Data: {:?}\n", sales_data)?;
    
    // =================== DESCRIPTIVE STATISTICS ===================
    println("\n=== Descriptive Statistics ===")?;
    
    facts mean_sales = mean(sales_data)?;
    facts median_sales = median(sales_data)?;
    facts mode_sales = mode(sales_data)?;
    facts variance_sales = variance(sales_data)?;
    facts std_dev_sales = standard_deviation(sales_data)?;
    
    printf("Mean (Average): ${:.2}\n", mean_sales)?;
    printf("Median (Middle value): ${:.2}\n", median_sales)?;
    printf("Mode (Most frequent): ${:.2}\n", mode_sales)?;
    printf("Variance: {:.2}\n", variance_sales)?;
    printf("Standard Deviation: {:.2}\n", std_dev_sales)?;
    
    // Advanced descriptive statistics
    lowkey (sales_data.len() >= 3) {
        facts skew = skewness(sales_data)?;
        printf("Skewness (asymmetry): {:.3}\n", skew)?;
        
        lowkey (skew > 0.5) {
            println("  → Right-skewed distribution (few high values)")?;
        } bestie lowkey (skew < -0.5) {
            println("  → Left-skewed distribution (few low values)")?;
        } bestie {
            println("  → Approximately symmetric distribution")?;
        }
    }
    
    lowkey (sales_data.len() >= 4) {
        facts kurt = kurtosis(sales_data)?;
        printf("Kurtosis (tail heaviness): {:.3}\n", kurt)?;
        
        lowkey (kurt > 1.0) {
            println("  → Heavy-tailed distribution (more outliers)")?;
        } bestie lowkey (kurt < -1.0) {
            println("  → Light-tailed distribution (fewer outliers)")?;
        } bestie {
            println("  → Normal-like tail behavior")?;
        }
    }
    
    // =================== STATISTICAL MEASURES ===================
    println("\n=== Statistical Measures ===")?;
    
    facts q1_sales = q1(sales_data)?;
    facts q3_sales = q3(sales_data)?;
    facts iqr_sales = interquartile_range(sales_data)?;
    facts range_sales = range(sales_data)?;
    
    printf("First Quartile (Q1): ${:.2}\n", q1_sales)?;
    printf("Third Quartile (Q3): ${:.2}\n", q3_sales)?;
    printf("Interquartile Range (IQR): ${:.2}\n", iqr_sales)?;
    printf("Range (Max - Min): ${:.2}\n", range_sales)?;
    
    // Five-number summary
    facts (min_val, q1_val, med_val, q3_val, max_val) = five_number_summary(sales_data)?;
    printf("\nFive-Number Summary:\n")?;
    printf("  Minimum: ${:.2}\n", min_val)?;
    printf("  Q1: ${:.2}\n", q1_val)?;
    printf("  Median: ${:.2}\n", med_val)?;
    printf("  Q3: ${:.2}\n", q3_val)?;
    printf("  Maximum: ${:.2}\n", max_val)?;
    
    // Percentiles
    println("\nKey Percentiles:")?;
    facts p10 = percentile(sales_data, 10.0)?;
    facts p90 = percentile(sales_data, 90.0)?;
    printf("  10th percentile: ${:.2}\n", p10)?;
    printf("  90th percentile: ${:.2}\n", p90)?;
    
    // =================== OUTLIER DETECTION ===================
    println("\n=== Outlier Detection ===")?;
    
    // IQR method
    facts iqr_outliers = outliers_iqr(sales_data, 1.5)?;
    printf("IQR Method outliers: {:?}\n", iqr_outliers)?;
    lowkey (iqr_outliers.is_empty()) {
        println("  → No outliers detected using IQR method")?;
    } bestie {
        printf("  → {} outlier(s) detected\n", iqr_outliers.len())?;
    }
    
    // Z-score method
    facts zscore_outliers = outliers_z_score(sales_data, 2.0)?;
    printf("Z-score Method outliers (threshold=2.0): {:?}\n", zscore_outliers)?;
    lowkey (zscore_outliers.is_empty()) {
        println("  → No outliers detected using Z-score method")?;
    } bestie {
        printf("  → {} outlier(s) detected\n", zscore_outliers.len())?;
    }
    
    // =================== PROBABILITY DISTRIBUTIONS ===================
    println("\n=== Probability Distributions ===")?;
    
    // Normal distribution analysis
    facts normal_pdf_at_mean = normal_pdf(mean_sales, mean_sales, std_dev_sales)?;
    facts normal_cdf_at_mean = normal_cdf(mean_sales, mean_sales, std_dev_sales)?;
    
    printf("Normal Distribution (μ={:.2}, σ={:.2}):\n", mean_sales, std_dev_sales)?;
    printf("  PDF at mean: {:.6}\n", normal_pdf_at_mean)?;
    printf("  CDF at mean: {:.3}\n", normal_cdf_at_mean)?;
    
    // Probability of sales being above/below certain thresholds
    facts high_threshold = mean_sales + std_dev_sales;
    facts low_threshold = mean_sales - std_dev_sales;
    
    facts prob_above_high = 1.0 - normal_cdf(high_threshold, mean_sales, std_dev_sales)?;
    facts prob_below_low = normal_cdf(low_threshold, mean_sales, std_dev_sales)?;
    
    printf("  P(Sales > ${:.2}): {:.3}\n", high_threshold, prob_above_high)?;
    printf("  P(Sales < ${:.2}): {:.3}\n", low_threshold, prob_below_low)?;
    
    // =================== CORRELATION ANALYSIS ===================
    println("\n=== Correlation Analysis ===")?;
    
    // Create related datasets for correlation demo
    sus weeks = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0,
                 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0];
    
    sus advertising_spend = [50.2, 48.7, 52.1, 55.8, 49.3, 46.9, 58.2, 51.4, 
                             53.7, 47.8, 61.3, 50.9, 54.2, 57.1, 45.8, 59.6, 
                             52.8, 56.3, 53.1, 58.9];
    
    facts sales_ad_correlation = correlation(sales_data, advertising_spend)?;
    facts sales_ad_covariance = covariance(sales_data, advertising_spend)?;
    
    printf("Sales vs Advertising Correlation: {:.3}\n", sales_ad_correlation)?;
    printf("Sales vs Advertising Covariance: {:.2}\n", sales_ad_covariance)?;
    
    lowkey (sales_ad_correlation > 0.7) {
        println("  → Strong positive correlation")?;
    } bestie lowkey (sales_ad_correlation > 0.3) {
        println("  → Moderate positive correlation")?;
    } bestie lowkey (sales_ad_correlation > -0.3) {
        println("  → Weak correlation")?;
    } bestie lowkey (sales_ad_correlation > -0.7) {
        println("  → Moderate negative correlation")?;
    } bestie {
        println("  → Strong negative correlation")?;
    }
    
    // =================== ADVANCED MEANS ===================
    println("\n=== Advanced Mean Calculations ===")?;
    
    // Only use positive values for geometric and harmonic means
    sus positive_sales = sales_data.filter(|&x| x > 0.0);
    
    facts arithmetic_mean = mean(positive_sales)?;
    facts geometric_mean_val = geometric_mean(positive_sales)?;
    facts harmonic_mean_val = harmonic_mean(positive_sales)?;
    facts rms_val = root_mean_square(positive_sales)?;
    
    printf("Arithmetic Mean: ${:.2}\n", arithmetic_mean)?;
    printf("Geometric Mean: ${:.2}\n", geometric_mean_val)?;
    printf("Harmonic Mean: ${:.2}\n", harmonic_mean_val)?;
    printf("Root Mean Square: ${:.2}\n", rms_val)?;
    
    println("\nMean Relationships (AM ≥ GM ≥ HM):")?;
    printf("  {:.2} ≥ {:.2} ≥ {:.2}: {}\n", 
           arithmetic_mean, geometric_mean_val, harmonic_mean_val,
           arithmetic_mean >= geometric_mean_val && geometric_mean_val >= harmonic_mean_val)?;
    
    // =================== COEFFICIENT OF VARIATION ===================
    facts cv = coefficient_of_variation(sales_data)?;
    printf("\nCoefficient of Variation: {:.3}\n", cv)?;
    lowkey (cv < 0.1) {
        println("  → Low variability (very consistent)")?;
    } bestie lowkey (cv < 0.3) {
        println("  → Moderate variability (reasonably consistent)")?;
    } bestie {
        println("  → High variability (inconsistent)")?;
    }
    
    // =================== DATA QUALITY ASSESSMENT ===================
    println("\n=== Data Quality Assessment ===")?;
    
    lowkey (has_invalid_values(sales_data)) {
        println("⚠️  Warning: Dataset contains invalid values (NaN/Infinity)")?;
        facts cleaned = clean_data(sales_data);
        printf("   Cleaned dataset size: {} → {}\n", sales_data.len(), cleaned.len())?;
    } bestie {
        println("✅ Dataset quality: All values are valid")?;
    }
    
    // Sample vs Population statistics comparison
    println("\n=== Sample vs Population Statistics ===")?;
    facts pop_variance = variance(sales_data)?;
    facts sample_var = sample_variance(sales_data)?;
    facts pop_std = standard_deviation(sales_data)?;
    facts sample_std = sample_standard_deviation(sales_data)?;
    
    printf("Population Variance: {:.2}\n", pop_variance)?;
    printf("Sample Variance: {:.2}\n", sample_var)?;
    printf("Population Std Dev: {:.2}\n", pop_std)?;
    printf("Sample Std Dev: {:.2}\n", sample_std)?;
    
    // =================== BUSINESS INSIGHTS ===================
    println("\n=== Business Insights ===")?;
    
    printf("📊 Sales Performance Summary:\n")?;
    printf("   Average weekly sales: ${:.2}\n", mean_sales)?;
    printf("   Typical range: ${:.2} - ${:.2}\n", q1_sales, q3_sales)?;
    printf("   Best week: ${:.2}\n", max_val)?;
    printf("   Worst week: ${:.2}\n", min_val)?;
    
    facts weeks_above_average = sales_data.iter()
        .filter(|&&x| x > mean_sales)
        .count();
    printf("   Weeks above average: {}/{} ({:.1}%)\n", 
           weeks_above_average, sales_data.len(), 
           weeks_above_average as f64 / sales_data.len() as f64 * 100.0)?;
    
    // Performance categories
    facts excellent_threshold = percentile(sales_data, 80.0)?;
    facts good_threshold = percentile(sales_data, 60.0)?;
    facts poor_threshold = percentile(sales_data, 20.0)?;
    
    printf("\n📈 Performance Categories:\n")?;
    printf("   Excellent (top 20%%): >${:.2}\n", excellent_threshold)?;
    printf("   Good (60-80%%): ${:.2} - ${:.2}\n", good_threshold, excellent_threshold)?;
    printf("   Average (20-60%%): ${:.2} - ${:.2}\n", poor_threshold, good_threshold)?;
    printf("   Needs Improvement (bottom 20%%): <${:.2}\n", poor_threshold)?;
    
    // Forecasting insights
    lowkey (sales_ad_correlation > 0.5) {
        printf("\n🎯 Recommendation: Advertising shows positive correlation with sales\n")?;
        printf("   Consider increasing ad spend to boost sales performance\n")?;
    }
    
    lowkey (cv > 0.2) {
        printf("\n⚠️  Note: High sales variability detected\n")?;
        printf("   Consider strategies to stabilize week-to-week performance\n")?;
    }
    
    println("\n=== Demo Complete ===")?;
}

// Error handling helper
slay handle_stat_error(error: MathError) {
    lowkey let MathError::DomainError { function, value, message } = error {
        printf("Domain error in {}: value {} - {}\n", function, value, message)?;
    } bestie lowkey let MathError::DivisionByZero { function } = error {
        printf("Division by zero in function: {}\n", function)?;
    } bestie {
        printf("Math error: {}\n", error)?;
    }
}
