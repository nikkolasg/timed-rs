use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Error as IoError;

/// Output configuration for timing data
#[derive(Clone, Debug, PartialEq)]
pub enum Output {
    /// Disable timing output completely
    Off,
    /// Use tracing for output
    Tracing,
    /// Write to CSV file, with filename
    CSV(String),
}

// Environment variable name for controlling output
pub const TIMED_OUTPUT_ENV: &str = "TIMED_OUTPUT";

// Initialize default configuration from environment variable
// This is used when there is no thread-local override
static DEFAULT_CONFIG: Lazy<Output> = Lazy::new(|| {
    // Read from environment variable during initialization
    read_output_from_env()
});

// Thread-local storage for testing and configuration overrides
thread_local! {
    static THREAD_CONFIG: RefCell<Option<Output>> = const { RefCell::new(None) };
}

// Helper to read output configuration from environment
fn read_output_from_env() -> Output {
    match env::var(TIMED_OUTPUT_ENV) {
        Ok(value) => {
            let value = value.trim();
            if value.eq_ignore_ascii_case("tracing") {
                Output::Tracing
            } else if value.eq_ignore_ascii_case("off") || value.is_empty() {
                Output::Off
            } else {
                // Any other value is treated as a CSV filename
                let config = Output::CSV(value.to_string());
                // Initialize the CSV file if needed
                if let Output::CSV(ref filename) = config {
                    let _ = init_csv_file(filename);
                }
                config
            }
        }
        Err(_) => Output::Off, // Default to Off if env var isn't set
    }
}

// Helper to initialize a CSV file with a header
fn init_csv_file(filename: &str) -> Result<(), IoError> {
    // Create or truncate the file
    let file = File::create(filename)?;

    // Write the header using csv crate
    let mut wtr = csv::WriterBuilder::new().from_writer(file);
    wtr.write_record(["function", "duration_ms"])
        .map_err(|e| IoError::new(std::io::ErrorKind::Other, e.to_string()))?;

    wtr.flush()
        .map_err(|e| IoError::new(std::io::ErrorKind::Other, e.to_string()))?;

    Ok(())
}

/// Set the output method for timing data
///
/// This is primarily intended for testing. In normal application use,
/// it's recommended to use the environment variable for configuration.
///
/// # Examples
///
/// ```
/// // Disable timing output
/// timed_core::set_output(timed_core::Output::Off);
///
/// // Use tracing for output
/// timed_core::set_output(timed_core::Output::Tracing);
///
/// // Use CSV file for output
/// timed_core::set_output(timed_core::Output::CSV("timing_results.csv".to_string()));
/// ```
pub fn set_output(output: Output) {
    // If setting to CSV, create/truncate the file and write header
    if let Output::CSV(ref filename) = output {
        let _ = init_csv_file(filename);
    }

    // Store in thread-local storage for this thread only
    THREAD_CONFIG.with(|cell| {
        *cell.borrow_mut() = Some(output);
    });
}

/// Get the current output configuration
///
/// Returns the thread-local override configuration if set,
/// otherwise returns the default configuration from environment.
pub fn get_output() -> Output {
    // First check for a thread-local override
    let thread_override = THREAD_CONFIG.with(|cell| cell.borrow().clone());

    // If override exists, use it, otherwise use default
    thread_override.unwrap_or_else(|| DEFAULT_CONFIG.clone())
}

/// Force re-read of the environment variable
///
/// Useful for tests that need to change the environment variable
/// and have it take effect for the current thread.
pub fn refresh_from_env() {
    let output = read_output_from_env();
    THREAD_CONFIG.with(|cell| {
        *cell.borrow_mut() = Some(output);
    });
}

/// Record timing data
pub fn record_timing(function_name: &str, duration_ms: f64) {
    let config = get_output();

    match &config {
        Output::Off => {
            // Do nothing when timing is disabled
        }
        Output::Tracing => {
            // Use tracing for output
            tracing::event!(
                tracing::Level::INFO,
                "{} executed in {:.3} ms",
                function_name,
                duration_ms
            );
        }
        Output::CSV(filename) => {
            // Only try to create/append to CSV file if not Off
            let file_result = OpenOptions::new().append(true).open(filename);

            if let Ok(file) = file_result {
                let mut wtr = csv::WriterBuilder::new()
                    .has_headers(false) // Don't write header again
                    .from_writer(file);

                // Try to write, but don't crash if it fails
                let _ = wtr.write_record([function_name, &format!("{:.3}", duration_ms)]);
                let _ = wtr.flush();
            }
        }
    }
}
