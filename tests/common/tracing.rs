use tracing_subscriber::prelude::*;
use tracing_subscriber::fmt::format::FmtSpan;

pub fn setup() {
    // Initialize tracing only once
    static ONCE: std::sync::Once = std::sync::Once::new();
    
    ONCE.call_once(|| {
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_line_number(true)
            .with_thread_names(true)
            .with_span_events(FmtSpan::CLOSE);
        
        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(tracing_subscriber::EnvFilter::from_default_env())
            .init();
    });
}

#[macro_export]
macro_rules! init_tracing {
    () => {
        #[path = "common/tracing.rs"]
        mod tracing_setup;

        #[ctor::ctor]
        fn init() {
            tracing_setup::setup();
        }
    };
}