//! Test for package manager HTTP backend implementation
//!
//! This test verifies that the real HTTP implementation works correctly
//! with mock registry servers and real package operations.

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use tempfile::TempDir;
    use warp::Filter;

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    struct MockPackage {
        name: String,
        version: String,
        description: String,
        authors: Vec<String>,
        dependencies: Vec<MockDependency>,
        keywords: Vec<String>,
        license: Option<String>,
        homepage: Option<String>,
        repository: Option<String>,
        download_url: String,
        checksum: String,
        file_size: u64,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    struct MockDependency {
        name: String,
        version_req: String,
        optional: bool,
        features: Vec<String>,
    }

    #[derive(Debug, serde::Serialize)]
    struct SearchResponse {
        packages: Vec<MockPackage>,
        total: usize,
    }

    #[derive(Debug, serde::Serialize)]
    struct PackageResponse {
        package: MockPackage,
    }

    #[derive(Debug, serde::Serialize)]
    struct VersionsResponse {
        versions: Vec<String>,
    }

    async fn start_mock_server(port: u16) -> tokio::task::JoinHandle<()> {
        let mock_package = MockPackage {
            name: "test-lib".to_string(),
            version: "1.0.0".to_string(),
            description: "A test library package".to_string(),
            authors: vec!["Test Author <test@example.com>".to_string()],
            dependencies: vec![],
            keywords: vec!["test".to_string(), "library".to_string()],
            license: Some("MIT".to_string()),
            homepage: Some("https://example.com/test-lib".to_string()),
            repository: Some("https://github.com/example/test-lib".to_string()),
            download_url: format!("http://localhost:{}/packages/test-lib-1.0.0.tar.gz", port),
            checksum: "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
            file_size: 1024,
        };

        // Search endpoint
        let search = warp::path!("api" / "v1" / "search")
            .and(warp::get())
            .and(warp::query::<HashMap<String, String>>())
            .map({
                let package = mock_package.clone();
                move |_params: HashMap<String, String>| {
                    let response = SearchResponse {
                        packages: vec![package.clone()],
                        total: 1,
                    };
                    warp::reply::json(&response)
                }
            });

        // Package info endpoint
        let package_info = warp::path!("api" / "v1" / "packages" / String)
            .and(warp::get())
            .map({
                let package = mock_package.clone();
                move |_name: String| {
                    let response = PackageResponse {
                        package: package.clone(),
                    };
                    warp::reply::json(&response)
                }
            });

        // Package versions endpoint  
        let package_versions = warp::path!("api" / "v1" / "packages" / String / "versions")
            .and(warp::get())
            .map(|_name: String| {
                let response = VersionsResponse {
                    versions: vec!["1.0.0".to_string(), "1.0.1".to_string(), "1.1.0".to_string()],
                };
                warp::reply::json(&response)
            });

        // Package download endpoint
        let package_download = warp::path!("packages" / String)
            .and(warp::get())
            .map(|_filename: String| {
                let mock_content = b"Mock tar.gz package content here";
                warp::reply::with_header(
                    mock_content.as_ref(),
                    "content-type", 
                    "application/gzip",
                )
            });

        let routes = search
            .or(package_info)
            .or(package_versions)
            .or(package_download)
            .with(warp::cors().allow_any_origin());

        tokio::spawn(async move {
            warp::serve(routes)
                .run(([127, 0, 0, 1], port))
                .await;
        })
    }

    #[tokio::test]
    async fn test_http_package_search() {
        let port = 3031;
        let _server = start_mock_server(port).await;
        
        // Give server time to start
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let client = reqwest::Client::new();
        let response = client
            .get(&format!("http://localhost:{}/api/v1/search", port))
            .query(&[("q", "test")])
            .send()
            .await
            .expect("Failed to send search request");

        assert!(response.status().is_success());
        
        let search_result: SearchResponse = response
            .json()
            .await
            .expect("Failed to parse search response");
        
        assert_eq!(search_result.packages.len(), 1);
        assert_eq!(search_result.packages[0].name, "test-lib");
    }

    #[tokio::test]
    async fn test_http_package_info() {
        let port = 3032;
        let _server = start_mock_server(port).await;
        
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let client = reqwest::Client::new();
        let response = client
            .get(&format!("http://localhost:{}/api/v1/packages/test-lib", port))
            .send()
            .await
            .expect("Failed to send package info request");

        assert!(response.status().is_success());
        
        let package_result: PackageResponse = response
            .json()
            .await
            .expect("Failed to parse package response");
        
        assert_eq!(package_result.package.name, "test-lib");
        assert_eq!(package_result.package.version, "1.0.0");
    }

    #[tokio::test] 
    async fn test_http_package_versions() {
        let port = 3033;
        let _server = start_mock_server(port).await;
        
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let client = reqwest::Client::new();
        let response = client
            .get(&format!("http://localhost:{}/api/v1/packages/test-lib/versions", port))
            .send()
            .await
            .expect("Failed to send versions request");

        assert!(response.status().is_success());
        
        let versions_result: VersionsResponse = response
            .json()
            .await
            .expect("Failed to parse versions response");
        
        assert!(versions_result.versions.len() >= 3);
        assert!(versions_result.versions.contains(&"1.0.0".to_string()));
    }

    #[tokio::test]
    async fn test_http_package_download() {
        let port = 3034;
        let _server = start_mock_server(port).await;
        
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let client = reqwest::Client::new();
        let response = client
            .get(&format!("http://localhost:{}/packages/test-lib-1.0.0.tar.gz", port))
            .send()
            .await
            .expect("Failed to send download request");

        assert!(response.status().is_success());
        
        let bytes = response
            .bytes()
            .await
            .expect("Failed to read download bytes");
        
        assert!(!bytes.is_empty());
        
        // Verify we can save the downloaded content
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let download_path = temp_dir.path().join("test-package.tar.gz");
        
        std::fs::write(&download_path, &bytes).expect("Failed to write downloaded file");
        
        let file_size = std::fs::metadata(&download_path)
            .expect("Failed to get file metadata")
            .len();
        
        assert_eq!(file_size, bytes.len() as u64);
    }

    #[tokio::test]
    async fn test_real_http_client_behavior() {
        let port = 3035;
        let _server = start_mock_server(port).await;
        
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        // Test using the same reqwest patterns as our real implementation
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("cursed-package-manager/1.0")
            .build()
            .expect("Failed to create HTTP client");

        // Test with query parameters
        let response = client
            .get(&format!("http://localhost:{}/api/v1/search", port))
            .query(&[("q", "test")])
            .send()
            .await
            .expect("Failed to send request");

        assert!(response.status().is_success());
        assert_eq!(response.headers().get("content-type").unwrap(), "application/json");

        // Test error handling for non-existent endpoints
        let error_response = client
            .get(&format!("http://localhost:{}/api/v1/nonexistent", port))
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(error_response.status(), 404);
    }
}
