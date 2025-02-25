//! Provides the LLVM backend to generate Broc binaries. Used to generate a
//! binary with the fastest possible execution speed.
#![warn(clippy::dbg_macro)]
// See github.com/roc-lang/broc/issues/800 for discussion of the large_enum_variant check.
#![allow(clippy::large_enum_variant)]
// we actually want to compare against the literal float bits
#![allow(clippy::float_cmp)]
// Not a useful lint for us
#![allow(clippy::too_many_arguments)]

pub mod llvm;

pub mod run_broc;
