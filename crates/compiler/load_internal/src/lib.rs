//! The internal implementation of broc_load, separate from broc_load to support caching.
#![warn(clippy::dbg_macro)]
// See github.com/roc-lang/broc/issues/800 for discussion of the large_enum_variant check.
#![allow(clippy::large_enum_variant)]

use broc_module::symbol::ModuleId;
pub mod docs;
pub mod file;
mod work;

#[cfg(target_family = "wasm")]
mod wasm_instant;

pub const BUILTIN_MODULES: &[(ModuleId, &str)] = &[
    (ModuleId::BOOL, "Bool"),
    (ModuleId::RESULT, "Result"),
    (ModuleId::NUM, "Num"),
    (ModuleId::LIST, "List"),
    (ModuleId::STR, "Str"),
    (ModuleId::DICT, "Dict"),
    (ModuleId::SET, "Set"),
    (ModuleId::BOX, "Box"),
    (ModuleId::ENCODE, "Encode"),
    (ModuleId::DECODE, "Decode"),
    (ModuleId::HASH, "Hash"),
    (ModuleId::JSON, "Json"),
];
