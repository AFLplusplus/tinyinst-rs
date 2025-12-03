/*!
 * Rust bindings for [`TinyInst`](https://github.com/googleprojectzero/TinyInst)
 */
#![doc = include_str!("../README.md")]
/*! */
#![no_std]
#![warn(clippy::cargo)]
#![deny(clippy::cargo_common_metadata)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::missing_safety_doc, clippy::missing_panics_doc)]
#![cfg_attr(
    not(test),
    warn(
        missing_debug_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unused_extern_crates,
        unused_import_braces,
        unused_qualifications,
        unused_results
    )
)]
#![cfg_attr(
    test,
    deny(
        missing_debug_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unused_extern_crates,
        unused_import_braces,
        unused_qualifications,
        unused_results
    )
)]
#![cfg_attr(
    test,
    deny(
        bad_style,
        dead_code,
        improper_ctypes,
        non_shorthand_field_patterns,
        no_mangle_generic_items,
        overflowing_literals,
        path_statements,
        patterns_in_fns_without_body,
        unconditional_recursion,
        unused,
        unused_allocation,
        unused_comparisons,
        unused_parens,
        while_true
    )
)]

#[cfg(test)]
#[macro_use]
extern crate std;
pub extern crate alloc;

pub mod tinyinst;
