// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! # PSA Cryptography API Wrapper
//!
//! This crate provides abstraction over an implementation of the PSA Cryptography API.
//! You can find the API
//! [here](https://developer.arm.com/architectures/security-architectures/platform-security-architecture/documentation).

#![no_std]
#![deny(
    nonstandard_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    missing_debug_implementations,
    missing_docs,
    // The following ling is triggered when casting a reference to a raw pointer.
    //trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    missing_copy_implementations
)]
// This one is hard to avoid.
#![allow(clippy::multiple_crate_versions)]

pub mod operations;
pub mod types;

use core::sync::atomic::{AtomicBool, Ordering};
use types::status::{Error, Result, Status};

static INITIALISED: AtomicBool = AtomicBool::new(false);

/// Initialize the PSA Crypto library
///
/// Applications must call this function before calling any other function in crate.
/// Applications are permitted to call this function more than once. Once a call succeeds,
/// subsequent calls are guaranteed to succeed.
pub fn init() -> Result<()> {
    // It it not a problem to call psa_crypto_init more than once.
    Status::from(unsafe { psa_crypto_sys::psa_crypto_init() }).to_result()?;
    let _ = INITIALISED.compare_and_swap(false, true, Ordering::Relaxed);

    Ok(())
}

/// Check if the PSA Crypto library has been initialized
pub fn initialized() -> Result<()> {
    if INITIALISED.load(Ordering::Relaxed) {
        Ok(())
    } else {
        Err(Error::BadState)
    }
}
