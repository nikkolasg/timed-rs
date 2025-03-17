use std::thread;
use std::time::Duration;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use timed_core::{Output, set_output};

// Apply the timing instrumentation to our test functions
#[timed::timed_instrument]
fn test_function_default_level() {
    // Simulate some work
    thread::sleep(Duration::from_millis(100));
}

#[timed::timed_instrument(level = "debug")]
fn test_function_debug_level() {
    // Simulate some work
    thread::sleep(Duration::from_millis(50));
}

fn main() {
    // Set up tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    info!("Testing with Tracing output");
    // Default is already Tracing, but setting it explicitly for demonstration
    set_output(Output::Tracing);
    
    // Run test functions with Tracing output
    test_function_default_level();
    test_function_debug_level();
    
    // Switch to CSV output
    info!("Testing with CSV output");
    set_output(Output::CSV("timing_results.csv".to_string()));
    
    // Run test functions with CSV output
    test_function_default_level();
    test_function_debug_level();
    
    info!("Test complete. Check timing_results.csv for results.");
} 