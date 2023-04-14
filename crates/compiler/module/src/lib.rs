//! Implements data structures used for efficiently representing unique modules
//! and identifiers in Broc programs.
#![warn(clippy::dbg_macro)]
// See github.com/roc-lang/broc/issues/800 for discussion of the large_enum_variant check.
#![allow(clippy::large_enum_variant, clippy::upper_case_acronyms)]

pub mod called_via;
pub mod ident;
pub mod low_level;
pub mod module_err;
pub mod symbol;
