use once_cell::sync::Lazy;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Error as IoError;
use std::sync::Mutex;

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

// Initialize output configuration from environment variable
// Defaults to Off if not specified
static OUTPUT_CONFIG: Lazy<Mutex<Output>> = Lazy::new(|| {
    // Use explicit Off value as default
    Mutex::new(Output::Off)
});

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
        },
        Err(_) => Output::Off, // Default to Off if env var isn't set
    }
}

// Helper to initialize a CSV file with a header
fn init_csv_file(filename: &str) -> Result<(), IoError> {
    // Create or truncate the file
    let file = File::create(filename)?;
    
    // Write the header using csv crate
    let mut wtr = csv::WriterBuilder::new().from_writer(file);
    wtr.write_record(&["function", "duration_ms"])
        .map_err(|e| IoError::new(std::io::ErrorKind::Other, e.to_string()))?;
    
    wtr.flush()
        .map_err(|e| IoError::new(std::io::ErrorKind::Other, e.to_string()))?;
    
    Ok(())
}

/// Set the output method for timing data
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

    match OUTPUT_CONFIG.lock() {
        Ok(mut config) => {
            *config = output;
        },
        Err(poisoned) => {
            // Recover from poisoned mutex during testing
            *poisoned.into_inner() = output;
        }
    }
}

/// Get the current output configuration
pub fn get_output() -> Output {
    match OUTPUT_CONFIG.lock() {
        Ok(config) => config.clone(),
        Err(poisoned) => poisoned.into_inner().clone() // Recover from poisoned mutex
    }
}

/// Force re-read of the environment variable (useful for testing)
pub fn refresh_from_env() {
    let output = read_output_from_env();
    match OUTPUT_CONFIG.lock() {
        Ok(mut config) => {
            *config = output;
        },
        Err(poisoned) => {
            // Recover from poisoned mutex during testing
            *poisoned.into_inner() = output;
        }
    }
}

/// Record timing data
pub fn record_timing(function_name: &str, duration_ms: f64) {
    let config = match OUTPUT_CONFIG.lock() {
        Ok(guard) => guard.clone(),
        Err(poisoned) => poisoned.into_inner().clone()
    };

    match &config {
        Output::Off => {
            // Do nothing when timing is disabled
        },
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
            let file_result = OpenOptions::new()
                .write(true)
                .append(true)
                .open(filename);
            
            if let Ok(file) = file_result {
                let mut wtr = csv::WriterBuilder::new()
                    .has_headers(false)  // Don't write header again
                    .from_writer(file);
                
                // Try to write, but don't crash if it fails
                let _ = wtr.write_record(&[function_name, &format!("{:.3}", duration_ms)]);
                let _ = wtr.flush();
            }
        }
    }
}
