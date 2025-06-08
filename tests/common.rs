//! Common test utilities and setup

pub mod tracing {
    use std::sync::Once;
    
    static INIT: Once = Once::new();
    
    /// Initialize tracing for tests
    pub fn setup() {
        INIT.call_once(|| {
            let subscriber = tracing_subscriber::fmt()
                .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                .with_test_writer()
                .finish();
                
            tracing::subscriber::set_global_default(subscriber)
                .expect("Failed to set tracing subscriber");
        });
    }
}
