# Timed

A Rust library for timing function execution with configurable output.

## Features

- Time function execution with a simple attribute macro: `#[timed::timed_instrument]`
- Configure log level: `#[timed::timed_instrument(level = "debug")]`
- Choose output method:
  - Tracing logs: `timed_core::set_output(timed_core::Output::Tracing)`
  - CSV file: `timed_core::set_output(timed_core::Output::CSV("timing_results.csv".to_string()))`

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
   
   // Configure output method
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

## Workspace Structure

- `timed` - The proc macro component
- `timed-core` - The runtime component with output configuration
- `timed-test` - Example and testing code for the library
