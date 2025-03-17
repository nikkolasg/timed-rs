# Timed

A Rust library for timing function execution with configurable output. Measure and report function execution time with minimal code changes.

## Features

- **Simple API**: Time function execution with a single attribute macro: `#[timed::timed_instrument]`
- **Configurable Log Levels**: Control verbosity with `#[timed::timed_instrument(level = "debug")]`
- **Multiple Output Methods**:
  - Disable timing: `set_output(Output::Off)`
  - Tracing logs: `set_output(Output::Tracing)`
  - CSV file export: `set_output(Output::CSV("timing_results.csv".to_string()))`
- **Runtime Configuration**: Change output method dynamically without recompiling
- **Environment Variable Control**: Configure via `TIMED_OUTPUT=off|tracing|filename.csv`
- **Thread-Safe**: Safe to use in multi-threaded applications

## Installation

Add the dependencies to your `Cargo.toml` file:

```toml
[dependencies]
timed = { git = "https://github.com/username/timed.git" }
timed-core = { git = "https://github.com/username/timed.git" }
```

For using tracing output, also add:

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

## Quick Start

```rust
use timed_core::{set_output, Output};

// Configure output method (once, at startup)
set_output(Output::Tracing);

// Add timing to any function
#[timed::timed_instrument]
fn my_function() {
    // Function code - timing is now automatic
}

// Run your function
my_function();
```

## Detailed Usage

### Setting Output Methods

```rust
use timed_core::{set_output, Output};

// Disable all timing output
set_output(Output::Off);

// Output to tracing logs (requires tracing setup)
set_output(Output::Tracing);

// Output to CSV file
set_output(Output::CSV("function_timing.csv".to_string()));
```

### Using the Macro

```rust
// Default timing (INFO level)
#[timed::timed_instrument]
fn regular_function() {
    // Function code
}

// Custom log level
#[timed::timed_instrument(level = "debug")]
fn debug_level_function() {
    // Function code
}

// Works with any function type
#[timed::timed_instrument]
async fn async_function() -> Result<(), Box<dyn std::error::Error>> {
    // Async function code
    Ok(())
}

// Works with methods too
impl MyStruct {
    #[timed::timed_instrument]
    fn my_method(&self) {
        // Method code
    }
}
```

### Setting Up Tracing

When using `Output::Tracing`, you need to set up the tracing subscriber:

```rust
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

// Setup tracing once at application startup
fn setup_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
}

fn main() {
    setup_tracing();
    timed_core::set_output(timed_core::Output::Tracing);
    
    // Now timing output will appear in logs
    my_function();
}
```

### Environment Variable Configuration

The library can be configured using the `TIMED_OUTPUT` environment variable:

```bash
# Disable all timing (default if not set)
TIMED_OUTPUT=off cargo run

# Output timing using tracing logs
TIMED_OUTPUT=tracing cargo run

# Output timing to a CSV file
TIMED_OUTPUT=timing_results.csv cargo run
```

In your code, you can force a re-read of the environment variable:

```rust
// Update configuration from environment variable
timed_core::refresh_from_env();
```

### Reading Current Configuration

```rust
use timed_core::{get_output, Output};

// Get current output configuration
let current_output = get_output();
match current_output {
    Output::Off => println!("Timing is disabled"),
    Output::Tracing => println!("Using tracing output"),
    Output::CSV(filename) => println!("Writing to CSV: {}", filename),
}
```

## CSV Output Format

When using CSV output, the library creates a file with:
- Header row: `function,duration_ms`
- One row per function call with the function name and execution time in milliseconds
- The file is created fresh when `set_output()` is called and appended to on each function call

Example CSV output:
```csv
function,duration_ms
my_function,12.345
another_function,3.890
```

## Testing

The library includes a comprehensive test suite:

```bash
# Run all tests
cargo test

# Run just the timed-test integration tests
cargo test -p timed-test
```

## Workspace Structure

The library is organized as a Rust workspace with three crates:

- `timed-core` - Core runtime with output configuration and timing functionality
- `timed` - Procedural macro for the `timed_instrument` attribute
- `timed-test` - Test suite and examples

## Thread Safety

The library uses thread-safe constructs (Mutex, Lazy static) for configuration, making it safe to use in multi-threaded applications.

## Common Issues

1. **Timing output not appearing**: Make sure you've configured an output method with `set_output()` or the environment variable.

2. **CSV file not created**: Check write permissions in the directory.

3. **Tracing output not visible**: Ensure you've set up the tracing subscriber correctly.

## License

[MIT License](LICENSE)
