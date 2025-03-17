use once_cell::sync::Lazy;
use std::fs::{File, OpenOptions};
use std::sync::Mutex;

/// Output configuration for timing data
#[derive(Clone, Debug)]
pub enum Output {
    /// Use tracing for output
    Tracing,
    /// Write to CSV file, with filename
    CSV(String),
}

// Default to Tracing output
static OUTPUT_CONFIG: Lazy<Mutex<Output>> = Lazy::new(|| Mutex::new(Output::Tracing));

/// Set the output method for timing data
///
/// # Examples
///
/// ```
/// // Use tracing for output
/// timed_core::set_output(timed_core::Output::Tracing);
///
/// // Use CSV file for output
/// timed_core::set_output(timed_core::Output::CSV("timing_results.csv".to_string()));
/// ```
pub fn set_output(output: Output) {
    // If setting to CSV, create/truncate the file and write header
    if let Output::CSV(ref filename) = output {
        // Create or truncate the file
        let _file = File::create(filename).expect("Failed to create output file");
        
        // Write the header
        let mut writer = csv::Writer::from_path(filename).expect("Failed to create CSV writer");
        writer
            .write_record(&["function", "duration_ms"])
            .expect("Failed to write CSV header");
        writer.flush().expect("Failed to flush CSV writer");
    }

    let mut config = OUTPUT_CONFIG.lock().unwrap();
    *config = output;
}

/// Record timing data
pub fn record_timing(function_name: &str, duration_ms: f64) {
    let config = OUTPUT_CONFIG.lock().unwrap();

    match &*config {
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
            // Open the file in append mode
            let file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(filename)
                .expect("Failed to open CSV file for appending");
                
            // Create a writer that appends to the file
            let mut writer = csv::WriterBuilder::new()
                .has_headers(false)  // Don't write header again
                .from_writer(file);
                
            writer
                .write_record(&[function_name, &format!("{:.3}", duration_ms)])
                .expect("Failed to write CSV record");
            writer.flush().expect("Failed to flush CSV writer");
        }
    }
}
