/// This file serves both as an example of how to use the timed crates and
/// as an automated test that can run in CI to verify the library works correctly.
///
/// It demonstrates two main features:
/// 1. Using the timed attribute macro to instrument functions
/// 2. Configuring different output methods (Tracing vs CSV file)
///
/// The test also verifies that:
/// - When using Tracing output, no CSV file is created
/// - When using CSV output, a properly formatted file is created with expected contents
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;
use timed_core::{set_output, Output};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

// Constants for testing
const CSV_FILENAME: &str = "timing_results.csv";

// Apply the timing instrumentation to our test functions
#[timed::timed_instrument]
fn test_function_default_level() {
    // Simulate some work with deterministic duration for testing
    thread::sleep(Duration::from_millis(100));
}

#[timed::timed_instrument(level = "debug")]
fn test_function_debug_level() {
    // Simulate some work with deterministic duration for testing
    thread::sleep(Duration::from_millis(50));
}

// Helper function to clean up any existing CSV file
fn cleanup_csv_file() {
    if Path::new(CSV_FILENAME).exists() {
        fs::remove_file(CSV_FILENAME).expect("Failed to remove existing CSV file");
    }
    // Verify file was removed or didn't exist
    assert!(!Path::new(CSV_FILENAME).exists(), "CSV file still exists after cleanup");
}

/// The main function runs a series of tests to verify the timed library works correctly.
/// Each test has assertions to verify the expected behavior.
fn main() {
    // Set up tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    // ===== Test 1: Tracing Output =====
    info!("==== TEST 1: Testing with Tracing output ====");
    
    // Ensure no CSV file exists before test
    cleanup_csv_file();
    
    // Configure for Tracing output
    set_output(Output::Tracing);
    
    // Run test functions with Tracing output
    test_function_default_level();
    test_function_debug_level();
    
    // Verify no CSV file was created (tracing shouldn't create files)
    assert!(!Path::new(CSV_FILENAME).exists(), 
            "CSV file was created when using Tracing output, but shouldn't have been");
    info!("✅ Tracing output test passed");

    // ===== Test 2: CSV Output =====
    info!("==== TEST 2: Testing with CSV output ====");
    
    // Switch to CSV output
    set_output(Output::CSV(CSV_FILENAME.to_string()));
    
    // Run test functions with CSV output
    test_function_default_level();
    test_function_debug_level();
    
    // Verify CSV file was created
    assert!(Path::new(CSV_FILENAME).exists(), 
            "CSV file wasn't created when using CSV output");
    
    // Read contents of the CSV file
    let csv_content = fs::read_to_string(CSV_FILENAME)
        .expect("Failed to read CSV file");
    
    // Verify CSV file has header and at least two data rows (one for each test function)
    let lines: Vec<&str> = csv_content.lines().collect();
    assert!(lines.len() >= 3, "CSV file should have at least 3 lines (header + 2 data rows)");
    assert!(lines[0].contains("function") && lines[0].contains("duration_ms"), 
            "CSV header doesn't contain expected columns");
    
    // Verify function names are in the CSV
    assert!(csv_content.contains("test_function_default_level"), 
            "CSV doesn't contain default level function name");
    assert!(csv_content.contains("test_function_debug_level"), 
            "CSV doesn't contain debug level function name");
    
    // Verify timing data is reasonable (format check)
    for i in 1..lines.len() {
        let fields: Vec<&str> = lines[i].split(',').collect();
        assert_eq!(fields.len(), 2, "CSV row should have exactly 2 fields");
        
        // Check that second field is a valid number (duration)
        let duration_str = fields[1].trim();
        assert!(duration_str.parse::<f64>().is_ok(), 
                "Duration '{}' is not a valid number", duration_str);
    }
    
    info!("✅ CSV output test passed");
    info!("✅ All tests passed!");
}
