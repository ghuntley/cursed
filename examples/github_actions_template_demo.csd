sus "stdlib::template" as templates;
sus "stdlib::object" as object;

// Example demonstrating the comprehensive GitHub Actions template renderer

facts main() {
    // Create a comprehensive CI/CD workflow configuration
    facts workflow_data = object::create_map();
    
    // Basic workflow information
    object::set_property(workflow_data, "name", "CI/CD Pipeline");
    
    // Workflow triggers
    facts triggers = object::create_map();
    
    // Push trigger configuration
    facts push_config = object::create_map();
    facts push_branches = object::create_array();
    object::push_array(push_branches, "main");
    object::push_array(push_branches, "develop");
    object::set_property(push_config, "branches", push_branches);
    object::set_property(triggers, "push", push_config);
    
    // Pull request trigger
    facts pr_config = object::create_map();
    facts pr_branches = object::create_array();
    object::push_array(pr_branches, "main");
    object::set_property(pr_config, "branches", pr_branches);
    object::set_property(triggers, "pull_request", pr_config);
    
    // Schedule trigger (cron)
    facts schedule_config = object::create_map();
    facts cron_array = object::create_array();
    object::push_array(cron_array, "0 2 * * 1"); // Every Monday at 2 AM
    object::set_property(schedule_config, "cron", cron_array);
    object::set_property(triggers, "schedule", schedule_config);
    
    object::set_property(workflow_data, "on", triggers);
    
    // Global environment variables
    facts global_env = object::create_map();
    object::set_property(global_env, "NODE_VERSION", "18");
    object::set_property(global_env, "CI", "true");
    object::set_property(global_env, "FORCE_COLOR", "1");
    object::set_property(workflow_data, "env", global_env);
    
    // Permissions
    facts permissions = object::create_map();
    object::set_property(permissions, "contents", "read");
    object::set_property(permissions, "actions", "read");
    object::set_property(permissions, "checks", "write");
    object::set_property(workflow_data, "permissions", permissions);
    
    // Concurrency settings
    facts concurrency = object::create_map();
    object::set_property(concurrency, "group", "${{ github.workflow }}-${{ github.ref }}");
    object::set_property(concurrency, "cancel-in-progress", true);
    object::set_property(workflow_data, "concurrency", concurrency);
    
    // Jobs configuration
    facts jobs = object::create_map();
    
    // Test job with matrix strategy
    facts test_job = object::create_map();
    object::set_property(test_job, "name", "Test Suite");
    object::set_property(test_job, "runs-on", "ubuntu-latest");
    
    // Strategy with matrix
    facts strategy = object::create_map();
    facts matrix = object::create_map();
    
    facts node_versions = object::create_array();
    object::push_array(node_versions, "16");
    object::push_array(node_versions, "18");
    object::push_array(node_versions, "20");
    object::set_property(matrix, "node-version", node_versions);
    
    facts os_matrix = object::create_array();
    object::push_array(os_matrix, "ubuntu-latest");
    object::push_array(os_matrix, "windows-latest");
    object::push_array(os_matrix, "macos-latest");
    object::set_property(matrix, "os", os_matrix);
    
    object::set_property(strategy, "matrix", matrix);
    object::set_property(strategy, "fail-fast", false);
    object::set_property(strategy, "max-parallel", 6);
    object::set_property(test_job, "strategy", strategy);
    
    // Test job steps
    facts test_steps = object::create_array();
    
    // Step 1: Checkout
    facts checkout_step = object::create_map();
    object::set_property(checkout_step, "name", "Checkout repository");
    object::set_property(checkout_step, "uses", "actions/checkout@v4");
    facts checkout_with = object::create_map();
    object::set_property(checkout_with, "fetch-depth", "0");
    object::set_property(checkout_step, "with", checkout_with);
    object::push_array(test_steps, checkout_step);
    
    // Step 2: Setup Node.js
    facts setup_node_step = object::create_map();
    object::set_property(setup_node_step, "name", "Setup Node.js ${{ matrix.node-version }}");
    object::set_property(setup_node_step, "uses", "actions/setup-node@v4");
    facts node_with = object::create_map();
    object::set_property(node_with, "node-version", "${{ matrix.node-version }}");
    object::set_property(node_with, "cache", "npm");
    object::set_property(setup_node_step, "with", node_with);
    object::push_array(test_steps, setup_node_step);
    
    // Step 3: Install dependencies
    facts install_step = object::create_map();
    object::set_property(install_step, "name", "Install dependencies");
    object::set_property(install_step, "run", "npm ci");
    object::push_array(test_steps, install_step);
    
    // Step 4: Run linting
    facts lint_step = object::create_map();
    object::set_property(lint_step, "name", "Run ESLint");
    object::set_property(lint_step, "run", "npm run lint");
    object::set_property(lint_step, "if", "matrix.node-version == '18'");
    object::push_array(test_steps, lint_step);
    
    // Step 5: Run tests with coverage
    facts test_step = object::create_map();
    object::set_property(test_step, "name", "Run tests");
    facts test_run_command = "npm test -- --coverage --watchAll=false\nnpm run test:e2e";
    object::set_property(test_step, "run", test_run_command);
    facts test_env = object::create_map();
    object::set_property(test_env, "NODE_ENV", "test");
    object::set_property(test_env, "CI", "true");
    object::set_property(test_step, "env", test_env);
    object::set_property(test_step, "timeout-minutes", "10");
    object::push_array(test_steps, test_step);
    
    // Step 6: Upload coverage
    facts coverage_step = object::create_map();
    object::set_property(coverage_step, "name", "Upload coverage reports");
    object::set_property(coverage_step, "uses", "codecov/codecov-action@v3");
    facts coverage_with = object::create_map();
    object::set_property(coverage_with, "file", "./coverage/lcov.info");
    object::set_property(coverage_with, "flags", "unittests");
    object::set_property(coverage_step, "with", coverage_with);
    object::set_property(coverage_step, "if", "matrix.node-version == '18' && matrix.os == 'ubuntu-latest'");
    object::push_array(test_steps, coverage_step);
    
    object::set_property(test_job, "steps", test_steps);
    object::set_property(jobs, "test", test_job);
    
    // Build job
    facts build_job = object::create_map();
    object::set_property(build_job, "name", "Build Application");
    object::set_property(build_job, "runs-on", "ubuntu-latest");
    
    facts build_needs = object::create_array();
    object::push_array(build_needs, "test");
    object::set_property(build_job, "needs", build_needs);
    
    facts build_steps = object::create_array();
    
    // Checkout for build
    object::push_array(build_steps, checkout_step);
    
    // Setup Node for build
    facts build_node_step = object::create_map();
    object::set_property(build_node_step, "name", "Setup Node.js");
    object::set_property(build_node_step, "uses", "actions/setup-node@v4");
    facts build_node_with = object::create_map();
    object::set_property(build_node_with, "node-version", "18");
    object::set_property(build_node_with, "cache", "npm");
    object::set_property(build_node_step, "with", build_node_with);
    object::push_array(build_steps, build_node_step);
    
    // Install and build
    object::push_array(build_steps, install_step);
    
    facts build_step = object::create_map();
    object::set_property(build_step, "name", "Build application");
    object::set_property(build_step, "run", "npm run build");
    facts build_env = object::create_map();
    object::set_property(build_env, "NODE_ENV", "production");
    object::set_property(build_step, "env", build_env);
    object::push_array(build_steps, build_step);
    
    // Upload build artifacts
    facts artifact_step = object::create_map();
    object::set_property(artifact_step, "name", "Upload build artifacts");
    object::set_property(artifact_step, "uses", "actions/upload-artifact@v3");
    facts artifact_with = object::create_map();
    object::set_property(artifact_with, "name", "build-files");
    object::set_property(artifact_with, "path", "dist/");
    object::set_property(artifact_with, "retention-days", "7");
    object::set_property(artifact_step, "with", artifact_with);
    object::push_array(build_steps, artifact_step);
    
    object::set_property(build_job, "steps", build_steps);
    object::set_property(jobs, "build", build_job);
    
    // Deploy job (conditional)
    facts deploy_job = object::create_map();
    object::set_property(deploy_job, "name", "Deploy to Production");
    object::set_property(deploy_job, "runs-on", "ubuntu-latest");
    object::set_property(deploy_job, "if", "github.ref == 'refs/heads/main' && github.event_name == 'push'");
    
    facts deploy_needs = object::create_array();
    object::push_array(deploy_needs, "build");
    object::set_property(deploy_job, "needs", deploy_needs);
    
    // Deploy environment
    facts deploy_environment = object::create_map();
    object::set_property(deploy_environment, "name", "production");
    object::set_property(deploy_environment, "url", "https://myapp.com");
    object::set_property(deploy_job, "environment", deploy_environment);
    
    facts deploy_steps = object::create_array();
    
    // Checkout for deploy
    object::push_array(deploy_steps, checkout_step);
    
    // Download artifacts
    facts download_step = object::create_map();
    object::set_property(download_step, "name", "Download build artifacts");
    object::set_property(download_step, "uses", "actions/download-artifact@v3");
    facts download_with = object::create_map();
    object::set_property(download_with, "name", "build-files");
    object::set_property(download_with, "path", "dist/");
    object::set_property(download_step, "with", download_with);
    object::push_array(deploy_steps, download_step);
    
    // Deploy step
    facts deploy_step = object::create_map();
    object::set_property(deploy_step, "name", "Deploy to production");
    object::set_property(deploy_step, "run", "echo 'Deploying to production...'");
    facts deploy_env = object::create_map();
    object::set_property(deploy_env, "DEPLOY_KEY", "${{ secrets.DEPLOY_KEY }}");
    object::set_property(deploy_env, "APP_ENV", "production");
    object::set_property(deploy_step, "env", deploy_env);
    object::push_array(deploy_steps, deploy_step);
    
    object::set_property(deploy_job, "steps", deploy_steps);
    object::set_property(jobs, "deploy", deploy_job);
    
    object::set_property(workflow_data, "jobs", jobs);
    
    // Render the GitHub Actions workflow
    facts renderer = templates::create_format_renderer("github-actions");
    facts workflow_yaml = templates::render(renderer, workflow_data);
    
    println("Generated GitHub Actions Workflow:");
    println("==================================");
    println(workflow_yaml);
    
    println("\n\nWorkflow Features Demonstrated:");
    println("- Multi-trigger support (push, PR, schedule)");
    println("- Matrix strategy for cross-platform testing");
    println("- Job dependencies and conditional execution");
    println("- Environment variables at global and step level");
    println("- Artifact upload/download between jobs");
    println("- Security with permissions and secrets");
    println("- Production deployment with environment protection");
    println("- Multi-line commands and proper YAML formatting");
    println("- Comprehensive step types (uses, run, with, env)");
}

main();
