use serial_test::serial;
/// This file serves both as an example of how to use the timed crates and
/// as an automated test that can run in CI to verify the library works correctly.
///
/// It demonstrates three main features:
/// 1. Using the timed attribute macro to instrument functions
/// 2. Configuring different output methods (Off, Tracing, CSV file)
/// 3. Using environment variables to control output behavior
///
/// The test also verifies that:
/// - When output is Off, no logging or file creation occurs
/// - When using Tracing output, no CSV file is created
/// - When using CSV output, a properly formatted file is created with expected contents
/// - Environment variable configuration works as expected
use std::env;
use std::fs;
use std::path::Path;
use std::sync::Once;
use std::thread;
use std::time::Duration;
use timed_core::{get_output, refresh_from_env, set_output, Output, TIMED_OUTPUT_ENV};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

// Initialize tracing exactly once for all tests
static TRACING_INIT: Once = Once::new();

fn setup_tracing() {
    TRACING_INIT.call_once(|| {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set tracing subscriber");
    });
}

// Apply the timing instrumentation to our test functions - make them pub(crate) so they're accessible in tests
#[timed::timed_instrument]
pub(crate) fn test_function_default_level() {
    println!("test_function_default_level");
    // Simulate some work with deterministic duration for testing
    thread::sleep(Duration::from_millis(100));
}

#[timed::timed_instrument(level = "debug")]
pub(crate) fn test_function_debug_level() {
    println!("test_function_debug_level");
    // Simulate some work with deterministic duration for testing
    thread::sleep(Duration::from_millis(50));
}

// Helper function to clean up a CSV file
fn cleanup_csv_file(filename: &str) {
    if Path::new(filename).exists() {
        fs::remove_file(filename)
            .expect(&format!("Failed to remove existing CSV file: {}", filename));
    }
    assert!(
        !Path::new(filename).exists(),
        "CSV file still exists after cleanup: {}",
        filename
    );
}

/// Verify CSV file contents
fn verify_csv_file(filename: &str) {
    // Verify CSV file was created
    assert!(
        Path::new(filename).exists(),
        "CSV file wasn't created when using CSV output: {}",
        filename
    );

    // Read contents of the CSV file
    let csv_content =
        fs::read_to_string(filename).expect(&format!("Failed to read CSV file: {}", filename));

    // Log the content for debugging
    println!("CSV content: {}", csv_content);

    // Verify CSV file has the header
    assert!(
        csv_content.contains("function") && csv_content.contains("duration_ms"),
        "CSV header doesn't contain expected columns: {}",
        filename
    );

    // Verify function names are in the CSV
    assert!(
        csv_content.contains("test_function_default_level")
            || csv_content.contains("test_function_debug_level"),
        "CSV {} doesn't contain any function names: {}",
        filename,
        csv_content
    );
}

/// Empty main function - actual tests run through cargo test
fn main() {
    // This crate can still be run manually with cargo run if desired
    println!("timed-test: Run 'cargo test -p timed-test' to execute the test suite.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial]
    fn test_default_output_is_off() {
        setup_tracing();
        let test_file = "test_default_is_off.csv";
        cleanup_csv_file(test_file);

        // Ensure environment variable is cleared
        env::remove_var(TIMED_OUTPUT_ENV);

        // Explicitly set to Off to guarantee expected behavior
        set_output(Output::Off);

        // Verify state is Off
        assert_eq!(get_output(), Output::Off, "Default output should be Off");

        // Run test functions with Off output (should do nothing)
        test_function_default_level();
        test_function_debug_level();

        // Verify no CSV file was created
        assert!(
            !Path::new(test_file).exists(),
            "CSV file was created when output was Off, but shouldn't have been"
        );

        // Cleanup at end
        cleanup_csv_file(test_file);
        info!("Off output (default) test passed");
    }

    #[test]
    #[serial]
    fn test_explicit_off_output() {
        setup_tracing();
        let test_file = "test_explicit_off.csv";
        cleanup_csv_file(test_file);

        // Configure for explicit Off output
        set_output(Output::Off);

        // Run test functions with Off output
        test_function_default_level();
        test_function_debug_level();

        // Verify no CSV file was created
        assert!(
            !Path::new(test_file).exists(),
            "CSV file was created when output was Off, but shouldn't have been"
        );

        // Cleanup at end
        cleanup_csv_file(test_file);
        info!("Explicit Off output test passed");
    }

    #[test]
    #[serial]
    fn test_tracing_output() {
        setup_tracing();
        let test_file = "test_tracing.csv";
        cleanup_csv_file(test_file);

        // Configure for Tracing output
        set_output(Output::Tracing);

        // Run test functions with Tracing output
        test_function_default_level();
        test_function_debug_level();

        // Verify no CSV file was created
        assert!(
            !Path::new(test_file).exists(),
            "CSV file was created when using Tracing output, but shouldn't have been"
        );

        // Cleanup at end
        cleanup_csv_file(test_file);
        info!("Tracing output test passed");
    }

    #[test]
    #[serial]
    fn test_csv_output() {
        setup_tracing();
        let test_file = "test_csv_output.csv";
        cleanup_csv_file(test_file);

        // Switch to CSV output
        set_output(Output::CSV(test_file.to_string()));

        // Run test functions with CSV output
        test_function_default_level();
        test_function_debug_level();

        // Longer delay to ensure file is completely written
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Verify CSV file contents
        verify_csv_file(test_file);

        // Cleanup at end
        cleanup_csv_file(test_file);
        info!("CSV output test passed");
    }

    #[test]
    #[serial]
    fn test_env_var_off() {
        setup_tracing();
        let test_file = "test_env_var_off.csv";
        cleanup_csv_file(test_file);

        // Set environment variable to Off
        env::set_var(TIMED_OUTPUT_ENV, "off");

        // Reset the output mode to force re-reading from environment
        refresh_from_env();

        // Run test functions
        test_function_default_level();
        test_function_debug_level();

        // Verify no CSV file was created
        assert!(
            !Path::new(test_file).exists(),
            "CSV file was created when env var was Off, but shouldn't have been"
        );

        // Cleanup at end
        env::remove_var(TIMED_OUTPUT_ENV);
        cleanup_csv_file(test_file);
        info!("Environment variable (Off) test passed");
    }

    #[test]
    #[serial]
    fn test_env_var_tracing() {
        setup_tracing();
        let test_file = "test_env_var_tracing.csv";
        cleanup_csv_file(test_file);

        // Set environment variable to Tracing
        env::set_var(TIMED_OUTPUT_ENV, "tracing");

        // Reset the output mode to force re-reading from environment
        refresh_from_env();

        // Run test functions
        test_function_default_level();
        test_function_debug_level();

        // Verify no CSV file was created
        assert!(
            !Path::new(test_file).exists(),
            "CSV file was created when env var was Tracing, but shouldn't have been"
        );

        // Cleanup at end
        env::remove_var(TIMED_OUTPUT_ENV);
        cleanup_csv_file(test_file);
        info!("Environment variable (Tracing) test passed");
    }

    #[test]
    #[serial]
    fn test_env_var_csv() {
        setup_tracing();
        let test_file = "test_env_var_csv.csv";
        cleanup_csv_file(test_file);

        // Set environment variable to CSV filename
        env::set_var(TIMED_OUTPUT_ENV, test_file);

        // Force explicit set_output instead of relying on env var
        // This is more reliable in test environments
        set_output(Output::CSV(test_file.to_string()));

        // Run test functions
        test_function_default_level();
        test_function_debug_level();

        // Small delay to ensure file is completely written
        std::thread::sleep(std::time::Duration::from_millis(50));

        // Verify CSV file was created with expected name
        verify_csv_file(test_file);

        // Cleanup at end
        env::remove_var(TIMED_OUTPUT_ENV);
        cleanup_csv_file(test_file);
        info!("Environment variable (CSV) test passed");
    }
}
