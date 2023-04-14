//! Provides the WASM backend to generate Broc binaries.
mod backend;
mod code_builder;
mod layout;
mod low_level;
mod storage;

// Helpers for interfacing to a Wasm module from outside
pub mod wasm32_result;
pub mod wasm32_sized;

use bitvec::prelude::BitVec;
use bumpalo::collections::Vec;
use bumpalo::{self, Bump};

use broc_collections::all::{MutMap, MutSet};
use broc_module::symbol::{Interns, ModuleId, Symbol};
use broc_mono::code_gen_help::CodeGenHelp;
use broc_mono::ir::{Pbroc, PbrocLayout};
use broc_mono::layout::{LayoutIds, STLayoutInterner};
use broc_target::TargetInfo;
use broc_wasm_module::parse::ParseError;
use broc_wasm_module::{Align, LocalId, ValueType, WasmModule};

use crate::backend::{PbrocLookupData, PbrocSource, WasmBackend};
use crate::code_builder::CodeBuilder;

const TARGET_INFO: TargetInfo = TargetInfo::default_wasm32();
const PTR_SIZE: u32 = {
    let value = TARGET_INFO.ptr_width() as u32;

    // const assert that our pointer width is actually 4
    // the code relies on the pointer width being exactly 4
    assert!(value == 4);

    value
};
const PTR_TYPE: ValueType = ValueType::I32;

pub const MEMORY_NAME: &str = "memory";
pub const BUILTINS_IMPORT_MODULE_NAME: &str = "env";
pub const STACK_POINTER_NAME: &str = "__stack_pointer";

pub struct Env<'a> {
    pub arena: &'a Bump,
    pub module_id: ModuleId,
    pub exposed_to_host: MutSet<Symbol>,
    pub stack_bytes: u32,
}

impl Env<'_> {
    pub const DEFAULT_STACK_BYTES: u32 = 1024 * 1024;
}

/// Parse the preprocessed host binary
/// If successful, the module can be passed to build_app_binary
pub fn parse_host<'a>(arena: &'a Bump, host_bytes: &[u8]) -> Result<WasmModule<'a>, ParseError> {
    let require_relocatable = true;
    WasmModule::preload(arena, host_bytes, require_relocatable)
}

/// Generate a Wasm module in binary form, ready to write to a file. Entry point from broc_build.
///   env            environment data from previous compiler stages
///   interns        names of functions and variables (as memory-efficient interned strings)
///   host_module    parsed module from a Wasm object file containing all of the non-Broc code
///   procedures     Broc code in monomorphized intermediate representation
pub fn build_app_binary<'a, 'r>(
    env: &'r Env<'a>,
    layout_interner: &'r mut STLayoutInterner<'a>,
    interns: &'r mut Interns,
    host_module: WasmModule<'a>,
    procedures: MutMap<(Symbol, PbrocLayout<'a>), Pbroc<'a>>,
) -> std::vec::Vec<u8> {
    let (mut wasm_module, called_fns, _) =
        build_app_module(env, layout_interner, interns, host_module, procedures);

    wasm_module.eliminate_dead_code(env.arena, called_fns);

    let mut buffer = std::vec::Vec::with_capacity(wasm_module.size());
    wasm_module.serialize(&mut buffer);
    buffer
}

/// Generate an unserialized Wasm module
/// Shared by all consumers of gen_wasm: broc_build, broc_repl_wasm, and test_gen
/// (broc_repl_wasm and test_gen will add more generated code for a wrapper function
/// that defines a common interface to `main`, independent of return type.)
pub fn build_app_module<'a, 'r>(
    env: &'r Env<'a>,
    layout_interner: &'r mut STLayoutInterner<'a>,
    interns: &'r mut Interns,
    host_module: WasmModule<'a>,
    procedures: MutMap<(Symbol, PbrocLayout<'a>), Pbroc<'a>>,
) -> (WasmModule<'a>, BitVec<usize>, u32) {
    let mut layout_ids = LayoutIds::default();
    let mut pbrocs = Vec::with_capacity_in(procedures.len(), env.arena);
    let mut pbroc_lookup = Vec::with_capacity_in(procedures.len() * 2, env.arena);
    let mut host_to_app_map = Vec::with_capacity_in(env.exposed_to_host.len(), env.arena);
    let mut maybe_main_fn_index = None;

    // Adjust Wasm function indices to account for functions from the object file
    let fn_index_offset: u32 =
        host_module.import.function_count() as u32 + host_module.code.function_count;

    // Pre-pass over the procedure names & layouts
    // Create a lookup to tell us the final index of each pbroc in the output file
    for (i, ((sym, pbroc_layout), pbroc)) in procedures.into_iter().enumerate() {
        let fn_index = fn_index_offset + i as u32;
        pbrocs.push(pbroc);
        if env.exposed_to_host.contains(&sym) {
            maybe_main_fn_index = Some(fn_index);

            let exposed_name = layout_ids
                .get_toplevel(sym, &pbroc_layout)
                .to_exposed_symbol_string(sym, interns);

            let exposed_name_bump: &'a str = env.arena.alloc_str(&exposed_name);

            host_to_app_map.push((exposed_name_bump, fn_index));
        }

        pbroc_lookup.push(PbrocLookupData {
            name: sym,
            layout: pbroc_layout,
            source: PbrocSource::Broc,
        });
    }

    let mut backend = WasmBackend::new(
        env,
        layout_interner,
        interns,
        layout_ids,
        pbroc_lookup,
        host_to_app_map,
        host_module,
        fn_index_offset,
        CodeGenHelp::new(env.arena, TargetInfo::default_wasm32(), env.module_id),
    );

    if DEBUG_SETTINGS.user_pbrocs_ir {
        println!("## pbrocs");
        for pbroc in pbrocs.iter() {
            println!("{}", pbroc.to_pretty(backend.layout_interner, 200, true));
            // println!("{:?}", pbroc);
        }
    }

    // Generate pbrocs from user code
    for pbroc in pbrocs.iter() {
        backend.build_pbroc(pbroc);
    }

    // Generate specialized helpers for refcounting & equality
    let helper_pbrocs = backend.get_helpers();

    backend.register_symbol_debug_names();

    if DEBUG_SETTINGS.helper_pbrocs_ir {
        println!("## helper_pbrocs");
        for pbroc in helper_pbrocs.iter() {
            println!("{}", pbroc.to_pretty(backend.layout_interner, 200, true));
            // println!("{:#?}", pbroc);
        }
    }

    // Generate Wasm for helpers and Zig/Broc wrappers
    let sources = Vec::from_iter_in(
        backend
            .pbroc_lookup
            .iter()
            .map(|PbrocLookupData { source, .. }| *source),
        env.arena,
    );
    let mut helper_iter = helper_pbrocs.iter();
    for (idx, source) in sources.iter().enumerate() {
        use PbrocSource::*;
        match source {
            Broc => { /* already generated */ }
            Helper => backend.build_pbroc(helper_iter.next().unwrap()),
            HigherOrderMapper(inner_idx) => backend.build_higher_order_mapper(idx, *inner_idx),
            HigherOrderCompare(inner_idx) => backend.build_higher_order_compare(idx, *inner_idx),
        }
    }

    let (module, called_fns) = backend.finalize();
    let main_function_index =
        maybe_main_fn_index.expect("The app must expose at least one value to the host");

    (module, called_fns, main_function_index)
}

pub struct CopyMemoryConfig {
    from_ptr: LocalId,
    from_offset: u32,
    to_ptr: LocalId,
    to_offset: u32,
    size: u32,
    alignment_bytes: u32,
}

pub fn copy_memory(code_builder: &mut CodeBuilder, config: CopyMemoryConfig) {
    if config.from_ptr == config.to_ptr && config.from_offset == config.to_offset {
        return;
    }
    if config.size == 0 {
        return;
    }

    let alignment = Align::from(config.alignment_bytes);
    let mut i = 0;
    while config.size - i >= 8 {
        code_builder.get_local(config.to_ptr);
        code_builder.get_local(config.from_ptr);
        code_builder.i64_load(alignment, i + config.from_offset);
        code_builder.i64_store(alignment, i + config.to_offset);
        i += 8;
    }
    if config.size - i >= 4 {
        code_builder.get_local(config.to_ptr);
        code_builder.get_local(config.from_ptr);
        code_builder.i32_load(alignment, i + config.from_offset);
        code_builder.i32_store(alignment, i + config.to_offset);
        i += 4;
    }
    while config.size - i > 0 {
        code_builder.get_local(config.to_ptr);
        code_builder.get_local(config.from_ptr);
        code_builder.i32_load8_u(alignment, i + config.from_offset);
        code_builder.i32_store8(alignment, i + config.to_offset);
        i += 1;
    }
}

pub struct WasmDebugSettings {
    pbroc_start_end: bool,
    user_pbrocs_ir: bool,
    helper_pbrocs_ir: bool,
    let_stmt_ir: bool,
    instructions: bool,
    storage_map: bool,
    pub keep_test_binary: bool,
}

pub const DEBUG_SETTINGS: WasmDebugSettings = WasmDebugSettings {
    pbroc_start_end: false && cfg!(debug_assertions),
    user_pbrocs_ir: false && cfg!(debug_assertions), // Note: we also have `ROC_PRINT_IR_AFTER_REFCOUNT=1 cargo test-gen-wasm`
    helper_pbrocs_ir: false && cfg!(debug_assertions),
    let_stmt_ir: false && cfg!(debug_assertions),
    instructions: false && cfg!(debug_assertions),
    storage_map: false && cfg!(debug_assertions),
    keep_test_binary: false && cfg!(debug_assertions), // see also ROC_WRITE_FINAL_WASM
};

#[cfg(test)]
mod dummy_platform_functions {
    // `cargo test` produces an executable. At least on Windows, this means that extern symbols must be defined. This crate imports broc_std which
    // defines a bunch of externs, and uses the three below. We provide dummy implementations because these functions are not called.
    use core::ffi::c_void;

    /// # Safety
    /// This is only marked unsafe to typecheck without warnings in the rest of the code here.
    #[no_mangle]
    pub unsafe extern "C" fn broc_alloc(_size: usize, _alignment: u32) -> *mut c_void {
        unimplemented!("It is not valid to call broc alloc from within the compiler. Please use the \"platform\" feature if this is a platform.")
    }

    /// # Safety
    /// This is only marked unsafe to typecheck without warnings in the rest of the code here.
    #[no_mangle]
    pub unsafe extern "C" fn broc_realloc(
        _ptr: *mut c_void,
        _new_size: usize,
        _old_size: usize,
        _alignment: u32,
    ) -> *mut c_void {
        unimplemented!("It is not valid to call broc realloc from within the compiler. Please use the \"platform\" feature if this is a platform.")
    }

    /// # Safety
    /// This is only marked unsafe to typecheck without warnings in the rest of the code here.
    #[no_mangle]
    pub unsafe extern "C" fn broc_dealloc(_ptr: *mut c_void, _alignment: u32) {
        unimplemented!("It is not valid to call broc dealloc from within the compiler. Please use the \"platform\" feature if this is a platform.")
    }

    #[no_mangle]
    pub unsafe extern "C" fn broc_panic(_c_ptr: *mut c_void, _tag_id: u32) {
        unimplemented!("It is not valid to call broc panic from within the compiler. Please use the \"platform\" feature if this is a platform.")
    }

    #[no_mangle]
    pub fn broc_memcpy(_dst: *mut c_void, _src: *mut c_void, _n: usize) -> *mut c_void {
        unimplemented!("It is not valid to call broc memcpy from within the compiler. Please use the \"platform\" feature if this is a platform.")
    }

    #[no_mangle]
    pub fn broc_memset(_dst: *mut c_void, _c: i32, _n: usize) -> *mut c_void {
        unimplemented!("It is not valid to call broc memset from within the compiler. Please use the \"platform\" feature if this is a platform.")
    }
}
