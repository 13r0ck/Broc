//! Responsible for coordinating building and linking of a Broc app with its host.
#![warn(clippy::dbg_macro)]
// See github.com/roc-lang/broc/issues/800 for discussion of the large_enum_variant check.
#![allow(clippy::large_enum_variant)]
pub mod link;
pub mod program;
pub mod target;
