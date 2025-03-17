use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, Meta, MetaNameValue, NestedMeta};

/// Timing instrumentation for functions
///
/// Usage:
/// - `#[timed::timed_instrument]` - uses INFO level by default
/// - `#[timed::timed_instrument(level = "debug")]` - specify level (trace, debug, info, warn, error)
#[proc_macro_attribute]
pub fn timed_instrument(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse arguments to get log level
    let args = parse_macro_input!(attr as AttributeArgs);
    let level = parse_level_from_args(&args);

    // Parse the function
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let vis = &input.vis;
    let sig = &input.sig;
    let body = &input.block;

    // Generate the instrumented function
    let output = quote! {
        #vis #sig {
            let span = tracing::span!(#level, stringify!(#fn_name));
            let _enter = span.enter();

            let start_time = std::time::Instant::now();

            let result = #body;

            let duration = start_time.elapsed();
            // Record timing, will use current output configuration
            timed_core::record_timing(stringify!(#fn_name), duration.as_secs_f64() * 1000.0);

            result
        }
    };

    output.into()
}

/// Parse the tracing level from attribute arguments
fn parse_level_from_args(args: &[NestedMeta]) -> proc_macro2::TokenStream {
    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. })) = arg {
            if path.is_ident("level") {
                if let Lit::Str(lit_str) = lit {
                    let level_str = lit_str.value();
                    return match level_str.to_lowercase().as_str() {
                        "trace" => quote!(tracing::Level::TRACE),
                        "debug" => quote!(tracing::Level::DEBUG),
                        "warn" => quote!(tracing::Level::WARN),
                        "error" => quote!(tracing::Level::ERROR),
                        _ => quote!(tracing::Level::INFO),
                    };
                }
            }
        }
    }
    // Default to INFO
    quote!(tracing::Level::INFO)
}
