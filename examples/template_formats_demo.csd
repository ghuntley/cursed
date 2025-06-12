// Comprehensive Template Formats Demo for CURSED
import "stdlib::template";

sus main() {
    // Sample data for demonstration
    sus project_data = {
        "name": "CURSED Template Engine",
        "version": "1.0.0",
        "description": "Multi-format template system",
        "author": "CURSED Team",
        "license": "MIT",
        "dependencies": ["serde", "yaml", "json"],
        "scripts": {
            "build": "cargo build --release",
            "test": "cargo test",
            "docs": "cargo doc --open"
        },
        "repository": "https://github.com/cursed/templates"
    };

    // 1. README.md generation
    sus readme_format = TemplateFormat::Document(DocumentFormat::Readme);
    sus readme_renderer = TemplateFormatRenderer::new(readme_format);
    sus readme_content = readme_renderer.render(&project_data);
    print("=== README.md ===\n");
    print(readme_content);
    print("\n");

    // 2. Package.json generation
    sus package_format = TemplateFormat::Build(BuildFormat::PackageJson);
    sus package_renderer = TemplateFormatRenderer::new(package_format);
    sus package_content = package_renderer.render(&project_data);
    print("=== package.json ===\n");
    print(package_content);
    print("\n");

    // 3. Makefile generation
    sus makefile_data = {
        "build": {
            "dependencies": "clean",
            "commands": [
                "cargo build --release",
                "strip target/release/cursed"
            ]
        },
        "test": {
            "commands": ["cargo test", "cargo test --ignored"]
        },
        "clean": {
            "commands": ["rm -rf target/"]
        }
    };
    
    sus makefile_format = TemplateFormat::Build(BuildFormat::Makefile);
    sus makefile_renderer = TemplateFormatRenderer::new(makefile_format);
    sus makefile_content = makefile_renderer.render(&makefile_data);
    print("=== Makefile ===\n");
    print(makefile_content);
    print("\n");

    // 4. Docker Compose generation
    sus compose_data = {
        "services": {
            "web": {
                "image": "nginx:alpine",
                "ports": ["80:80"],
                "volumes": ["./html:/usr/share/nginx/html"]
            },
            "api": {
                "build": ".",
                "ports": ["3000:3000"],
                "environment": ["NODE_ENV=production"]
            }
        },
        "volumes": {
            "db_data": {}
        }
    };
    
    sus compose_format = TemplateFormat::Config(ConfigFormat::DockerCompose);
    sus compose_renderer = TemplateFormatRenderer::new(compose_format);
    sus compose_content = compose_renderer.render(&compose_data);
    print("=== docker-compose.yml ===\n");
    print(compose_content);
    print("\n");

    // 5. OpenAPI specification
    sus api_data = {
        "info": {
            "title": "CURSED API",
            "version": "1.0.0",
            "description": "RESTful API for CURSED"
        },
        "paths": {
            "/users": {
                "get": {
                    "summary": "List users",
                    "responses": {
                        "200": {
                            "description": "Successful response"
                        }
                    }
                }
            }
        }
    };
    
    sus openapi_format = TemplateFormat::Api(ApiFormat::OpenApi);
    sus openapi_renderer = TemplateFormatRenderer::new(openapi_format);
    sus openapi_content = openapi_renderer.render(&api_data);
    print("=== OpenAPI spec ===\n");
    print(openapi_content);
    print("\n");

    // 6. Kubernetes deployment
    sus k8s_data = {
        "kind": "Deployment",
        "metadata": {
            "name": "cursed-app",
            "namespace": "default"
        },
        "spec": {
            "replicas": 3,
            "selector": {
                "matchLabels": {
                    "app": "cursed"
                }
            },
            "template": {
                "metadata": {
                    "labels": {
                        "app": "cursed"
                    }
                },
                "spec": {
                    "containers": [{
                        "name": "cursed",
                        "image": "cursed:latest",
                        "ports": [{
                            "containerPort": 8080
                        }]
                    }]
                }
            }
        }
    };
    
    sus k8s_format = TemplateFormat::Config(ConfigFormat::Kubernetes);
    sus k8s_renderer = TemplateFormatRenderer::new(k8s_format);
    sus k8s_content = k8s_renderer.render(&k8s_data);
    print("=== Kubernetes Deployment ===\n");
    print(k8s_content);
    print("\n");

    // 7. Format detection demo
    print("=== Format Detection ===\n");
    
    // Detect from file extensions
    sus json_format = FormatDetector::from_extension("config.json");
    sus yaml_format = FormatDetector::from_extension("deploy.yaml");
    sus make_format = FormatDetector::from_extension("Makefile");
    
    print("JSON format detected: ");
    print(json_format.is_some());
    print("\n");
    print("YAML format detected: ");
    print(yaml_format.is_some());
    print("\n");
    print("Makefile format detected: ");
    print(make_format.is_some());
    print("\n");

    // Detect from content
    sus json_content = "{\"key\": \"value\"}";
    sus docker_content = "FROM ubuntu:20.04\nRUN apt-get update";
    
    sus detected_json = FormatDetector::from_content(json_content);
    sus detected_docker = FormatDetector::from_content(docker_content);
    
    print("JSON content detected: ");
    print(detected_json.is_some());
    print("\n");
    print("Docker content detected: ");
    print(detected_docker.is_some());
    print("\n");

    // 8. Content-Type headers
    print("=== Content-Type Headers ===\n");
    
    sus html_renderer = TemplateFormatRenderer::new(TemplateFormat::Html);
    sus xml_renderer = TemplateFormatRenderer::new(TemplateFormat::Xml);
    sus csv_renderer = TemplateFormatRenderer::new(TemplateFormat::Csv);
    
    print("HTML content type: ");
    print(html_renderer.content_type());
    print("\n");
    print("XML content type: ");
    print(xml_renderer.content_type());
    print("\n");
    print("CSV content type: ");
    print(csv_renderer.content_type());
    print("\n");

    // 9. Template composition
    print("=== Template Composition ===\n");
    
    sus header_data = {
        "title": "My Document",
        "subtitle": "Generated with CURSED"
    };
    
    sus body_data = {
        "content": "This is the main content",
        "sections": ["Introduction", "Features", "Conclusion"]
    };
    
    sus templates = [
        (TemplateFormat::Markdown, header_data),
        (TemplateFormat::Markdown, body_data)
    ];
    
    sus composed = FormatConverter::compose(templates, "\n---\n");
    print("Composed document:\n");
    print(composed);
    print("\n");

    print("Template formats demo completed! ✨\n");
}
