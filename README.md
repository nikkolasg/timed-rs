# Timed

A Rust library for timing function execution with configurable output.

## Features

- Time function execution with a simple attribute macro: `#[timed::timed_instrument]`
- Configure log level: `#[timed::timed_instrument(level = "debug")]`
- Choose output method:
  - Disable timing: `timed_core::set_output(timed_core::Output::Off)`
  - Tracing logs: `timed_core::set_output(timed_core::Output::Tracing)`
  - CSV file: `timed_core::set_output(timed_core::Output::CSV("timing_results.csv".to_string()))`
- Configure via environment variable: `TIMED_OUTPUT=off|tracing|filename.csv`

## Environment Variable Configuration

The library can be configured using the `TIMED_OUTPUT` environment variable:

- `TIMED_OUTPUT=off` - Disable all timing (default if not set)
- `TIMED_OUTPUT=tracing` - Output timing using tracing logs
- `TIMED_OUTPUT=filename.csv` - Output timing to the specified CSV file

This allows enabling/disabling the instrumentation in production without code changes.

## Usage

1. Add the dependencies to your project:
   ```toml
   [dependencies]
   timed = "0.1.0"
   timed-core = "0.1.0"
   ```

2. Use the macro in your code:
   ```rust
   use timed_core::Output;
   
   // Configure output method (or use environment variable)
   timed_core::set_output(Output::CSV("timing_results.csv".to_string()));
   
   // Apply the macro to functions you want to time
   #[timed::timed_instrument]
   fn my_function() {
       // Your code here
   }
   
   #[timed::timed_instrument(level = "debug")]
   fn another_function() {
       // Your code here
   }
   ```

3. Run your program with environment configuration:
   ```bash
   # Disable timing
   TIMED_OUTPUT=off cargo run
   
   # Use tracing output
   TIMED_OUTPUT=tracing cargo run
   
   # Use CSV output
   TIMED_OUTPUT=timing_results.csv cargo run
   ```

## Testing

Run the automated test suite:

```
cargo test -p timed-test
```

The tests cover:
1. Default configuration (Off)
2. Explicit output configuration (Off, Tracing, CSV)
3. Environment variable based configuration
4. CSV file format and content validation

Each test function follows standard Rust unit testing patterns with proper setup and assertions.

## Workspace Structure

- `timed` - The proc macro component
- `timed-core` - The runtime component with output configuration
- `timed-test` - Test suite for all library functionality

## CSV Output Format

When using CSV output, the library creates a file with:
- Header row: `function,duration_ms`
- One row per function call with the function name and execution time in milliseconds
- The file is created fresh when `set_output()` is called and appended to on each function call
