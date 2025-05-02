use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, Meta, MetaNameValue, NestedMeta};

/// Timing instrumentation for functions
///
/// Usage:
/// - `#[timed::timed_instrument]` - uses INFO level and function name by default
/// - `#[timed::timed_instrument(level = "debug")]` - specify tracing level
/// - `#[timed::timed_instrument(name = "my_custom_name")]` - specify custom name for output
/// - `#[timed::timed_instrument(level = "trace", name = "detailed_op")]` - specify both
#[proc_macro_attribute]
pub fn timed_instrument(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse arguments
    let args = parse_macro_input!(attr as AttributeArgs);
    let (level, custom_name) = parse_args(&args);

    // Parse the function
    let input = parse_macro_input!(item as ItemFn);
    let fn_name_ident = &input.sig.ident;
    let vis = &input.vis;
    let sig = &input.sig;
    let body = &input.block;

    // Determine the name to use for timing/tracing
    let output_name = match custom_name {
        Some(name) => quote! { #name },
        None => quote! { stringify!(#fn_name_ident) },
    };

    // Generate the instrumented function
    let output = quote! {
        #vis #sig {
            let span = tracing::span!(#level, #output_name);
            let _enter = span.enter();

            let start_time = std::time::Instant::now();

            let result = #body;

            let duration = start_time.elapsed();
            // Record timing, will use current output configuration
            timed_core::record_timing(#output_name, duration.as_secs_f64() * 1000.0);

            result
        }
    };

    output.into()
}

/// Parse the level and optional name from attribute arguments
fn parse_args(args: &[NestedMeta]) -> (proc_macro2::TokenStream, Option<String>) {
    let mut level = quote!(tracing::Level::INFO); // Default level
    let mut custom_name: Option<String> = None;

    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. })) = arg {
            if path.is_ident("level") {
                if let Lit::Str(lit_str) = lit {
                    let level_str = lit_str.value();
                    level = match level_str.to_lowercase().as_str() {
                        "trace" => quote!(tracing::Level::TRACE),
                        "debug" => quote!(tracing::Level::DEBUG),
                        "warn" => quote!(tracing::Level::WARN),
                        "error" => quote!(tracing::Level::ERROR),
                        _ => quote!(tracing::Level::INFO),
                    };
                }
            } else if path.is_ident("name") {
                 if let Lit::Str(lit_str) = lit {
                    custom_name = Some(lit_str.value());
                 }
            }
        }
    }

    (level, custom_name)
}
