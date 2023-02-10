#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    unused_variables,
    unused
)]

pub use libc::size_t;

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/circt_build/bindings.rs"
));

#[cfg(test)]
mod tests {}
