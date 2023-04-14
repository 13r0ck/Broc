use crate::generic64::{aarch64, new_backend_64bit, x86_64};
use crate::{Backend, Env, Relocation};
use bumpalo::collections::Vec;
use object::write::{self, SectionId, SymbolId};
use object::write::{Object, StandardSection, StandardSegment, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, RelocationEncoding, RelocationKind, SectionKind,
    SymbolFlags, SymbolKind, SymbolScope,
};
use broc_collections::all::MutMap;
use broc_error_macros::internal_error;
use broc_module::symbol;
use broc_module::symbol::Interns;
use broc_mono::ir::{Pbroc, PbrocLayout};
use broc_mono::layout::{LayoutIds, STLayoutInterner};
use broc_target::TargetInfo;
use target_lexicon::{Architecture as TargetArch, BinaryFormat as TargetBF, Triple};

// This is used by some code below which is currently commented out.
// See that code for more details!
// const VERSION: &str = env!("CARGO_PKG_VERSION");

/// build_module is the high level builder/delegator.
/// It takes the request to build a module and output the object file for the module.
pub fn build_module<'a, 'r>(
    env: &'r Env<'a>,
    interns: &'r mut Interns,
    layout_interner: &'r mut STLayoutInterner<'a>,
    target: &Triple,
    procedures: MutMap<(symbol::Symbol, PbrocLayout<'a>), Pbroc<'a>>,
) -> Object<'a> {
    match target {
        Triple {
            architecture: TargetArch::X86_64,
            binary_format: TargetBF::Elf,
            ..
        } if cfg!(feature = "target-x86_64") => {
            let backend = new_backend_64bit::<
                x86_64::X86_64GeneralReg,
                x86_64::X86_64FloatReg,
                x86_64::X86_64Assembler,
                x86_64::X86_64SystemV,
            >(env, TargetInfo::default_x86_64(), interns, layout_interner);
            build_object(
                procedures,
                backend,
                Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little),
            )
        }
        Triple {
            architecture: TargetArch::X86_64,
            binary_format: TargetBF::Macho,
            ..
        } if cfg!(feature = "target-x86_64") => {
            let backend = new_backend_64bit::<
                x86_64::X86_64GeneralReg,
                x86_64::X86_64FloatReg,
                x86_64::X86_64Assembler,
                x86_64::X86_64SystemV,
            >(env, TargetInfo::default_x86_64(), interns, layout_interner);
            build_object(
                procedures,
                backend,
                Object::new(
                    BinaryFormat::MachO,
                    Architecture::X86_64,
                    Endianness::Little,
                ),
            )
        }
        Triple {
            architecture: TargetArch::Aarch64(_),
            binary_format: TargetBF::Elf,
            ..
        } if cfg!(feature = "target-aarch64") => {
            let backend =
                new_backend_64bit::<
                    aarch64::AArch64GeneralReg,
                    aarch64::AArch64FloatReg,
                    aarch64::AArch64Assembler,
                    aarch64::AArch64Call,
                >(env, TargetInfo::default_aarch64(), interns, layout_interner);
            build_object(
                procedures,
                backend,
                Object::new(BinaryFormat::Elf, Architecture::Aarch64, Endianness::Little),
            )
        }
        Triple {
            architecture: TargetArch::Aarch64(_),
            binary_format: TargetBF::Macho,
            ..
        } if cfg!(feature = "target-aarch64") => {
            let backend =
                new_backend_64bit::<
                    aarch64::AArch64GeneralReg,
                    aarch64::AArch64FloatReg,
                    aarch64::AArch64Assembler,
                    aarch64::AArch64Call,
                >(env, TargetInfo::default_aarch64(), interns, layout_interner);
            build_object(
                procedures,
                backend,
                Object::new(
                    BinaryFormat::MachO,
                    Architecture::Aarch64,
                    Endianness::Little,
                ),
            )
        }
        x => unimplemented!("the target, {:?}", x),
    }
}

fn generate_wrapper<'a, B: Backend<'a>>(
    backend: &mut B,
    output: &mut Object,
    wrapper_name: String,
    wraps: String,
) {
    let text_section = output.section_id(StandardSection::Text);
    let pbroc_symbol = Symbol {
        name: wrapper_name.as_bytes().to_vec(),
        value: 0,
        size: 0,
        kind: SymbolKind::Text,
        scope: SymbolScope::Dynamic,
        weak: false,
        section: SymbolSection::Section(text_section),
        flags: SymbolFlags::None,
    };
    let pbroc_id = output.add_symbol(pbroc_symbol);
    let (pbroc_data, offset) = backend.build_wrapped_jmp();
    let pbroc_offset = output.add_symbol_data(pbroc_id, text_section, pbroc_data, 16);

    let name = wraps.as_str().as_bytes();
    // If the symbol is an undefined zig builtin, we need to add it here.
    let symbol = Symbol {
        name: name.to_vec(),
        value: 0,
        size: 0,
        kind: SymbolKind::Text,
        scope: SymbolScope::Dynamic,
        weak: true,
        section: SymbolSection::Undefined,
        flags: SymbolFlags::None,
    };
    output.add_symbol(symbol);
    if let Some(sym_id) = output.symbol_id(name) {
        let reloc = write::Relocation {
            offset: offset + pbroc_offset,
            size: 32,
            kind: RelocationKind::PltRelative,
            encoding: RelocationEncoding::X86Branch,
            symbol: sym_id,
            addend: -4,
        };

        match output.add_relocation(text_section, reloc) {
            Ok(obj) => obj,
            Err(e) => internal_error!("{:?}", e),
        }
    } else {
        internal_error!("failed to find fn symbol for {:?}", wraps);
    }
}

fn build_object<'a, B: Backend<'a>>(
    procedures: MutMap<(symbol::Symbol, PbrocLayout<'a>), Pbroc<'a>>,
    mut backend: B,
    mut output: Object<'a>,
) -> Object<'a> {
    let data_section = output.section_id(StandardSection::Data);

    let arena = backend.env().arena;

    /*
    // Commented out because we couldn't figure out how to get it to work on mac - see https://github.com/roc-lang/broc/pull/1323
    let comment = output.add_section(vec![], b".comment".to_vec(), SectionKind::OtherString);
    output.append_section_data(
        comment,
        format!("\0broc dev backend version {} \0", VERSION).as_bytes(),
        1,
    );
    */

    if backend.env().generate_allocators {
        generate_wrapper(
            &mut backend,
            &mut output,
            "broc_alloc".into(),
            "malloc".into(),
        );
        generate_wrapper(
            &mut backend,
            &mut output,
            "broc_realloc".into(),
            "realloc".into(),
        );
        generate_wrapper(
            &mut backend,
            &mut output,
            "broc_dealloc".into(),
            "free".into(),
        );
        generate_wrapper(
            &mut backend,
            &mut output,
            "broc_panic".into(),
            "broc_builtins.utils.test_panic".into(),
        );
    }

    // Setup layout_ids for procedure calls.
    let mut layout_ids = LayoutIds::default();
    let mut pbrocs = Vec::with_capacity_in(procedures.len(), arena);

    // Names and linker data for user procedures
    for ((sym, layout), pbroc) in procedures {
        build_pbroc_symbol(
            &mut output,
            &mut layout_ids,
            &mut pbrocs,
            &backend,
            sym,
            layout,
            pbroc,
        )
    }

    // Build procedures from user code
    let mut relocations = bumpalo::vec![in arena];
    for (fn_name, section_id, pbroc_id, pbroc) in pbrocs {
        build_pbroc(
            &mut output,
            &mut backend,
            &mut relocations,
            &mut layout_ids,
            data_section,
            fn_name,
            section_id,
            pbroc_id,
            pbroc,
        )
    }

    // Generate IR for specialized helper pbrocs (refcounting & equality)
    let helper_pbrocs = {
        let (module_id, _interner, interns, helper_pbroc_gen) = backend.module_interns_helpers_mut();

        let ident_ids = interns.all_ident_ids.get_mut(&module_id).unwrap();
        let helper_pbrocs = helper_pbroc_gen.take_pbrocs();
        module_id.register_debug_idents(ident_ids);

        helper_pbrocs
    };

    let empty = bumpalo::collections::Vec::new_in(arena);
    let helper_symbols_and_layouts = std::mem::replace(backend.helper_pbroc_symbols_mut(), empty);
    let mut helper_names_symbols_pbrocs = Vec::with_capacity_in(helper_pbrocs.len(), arena);

    // Names and linker data for helpers
    for ((sym, layout), pbroc) in helper_symbols_and_layouts.into_iter().zip(helper_pbrocs) {
        let fn_name = backend.function_symbol_to_string(
            sym,
            layout.arguments.iter().copied(),
            None,
            layout.result,
        );

        if let Some(pbroc_id) = output.symbol_id(fn_name.as_bytes()) {
            if let SymbolSection::Section(section_id) = output.symbol(pbroc_id).section {
                helper_names_symbols_pbrocs.push((fn_name, section_id, pbroc_id, pbroc));
                continue;
            }
        } else {
            // The symbol isn't defined yet and will just be used by other rc pbrocs.
            let section_id = output.add_section(
                output.segment_name(StandardSegment::Text).to_vec(),
                format!(".text.{:x}", sym.as_u64()).as_bytes().to_vec(),
                SectionKind::Text,
            );

            let rc_symbol = Symbol {
                name: fn_name.as_bytes().to_vec(),
                value: 0,
                size: 0,
                kind: SymbolKind::Text,
                scope: SymbolScope::Linkage,
                weak: false,
                section: SymbolSection::Section(section_id),
                flags: SymbolFlags::None,
            };
            let pbroc_id = output.add_symbol(rc_symbol);
            helper_names_symbols_pbrocs.push((fn_name, section_id, pbroc_id, pbroc));
            continue;
        }
        internal_error!("failed to create rc fn for symbol {:?}", sym);
    }

    // Build helpers
    for (fn_name, section_id, pbroc_id, pbroc) in helper_names_symbols_pbrocs {
        build_pbroc(
            &mut output,
            &mut backend,
            &mut relocations,
            &mut layout_ids,
            data_section,
            fn_name,
            section_id,
            pbroc_id,
            pbroc,
        )
    }

    // Relocations for all procedures (user code & helpers)
    for (section_id, reloc) in relocations {
        match output.add_relocation(section_id, reloc) {
            Ok(obj) => obj,
            Err(e) => internal_error!("{:?}", e),
        }
    }
    output
}

fn build_pbroc_symbol<'a, B: Backend<'a>>(
    output: &mut Object<'a>,
    layout_ids: &mut LayoutIds<'a>,
    pbrocs: &mut Vec<'a, (String, SectionId, SymbolId, Pbroc<'a>)>,
    backend: &B,
    sym: broc_module::symbol::Symbol,
    layout: PbrocLayout<'a>,
    pbroc: Pbroc<'a>,
) {
    let base_name = backend.function_symbol_to_string(
        sym,
        layout.arguments.iter().copied(),
        None,
        layout.result,
    );

    let fn_name = if backend.env().exposed_to_host.contains(&sym) {
        layout_ids
            .get_toplevel(sym, &layout)
            .to_exposed_symbol_string(sym, backend.interns())
    } else {
        base_name
    };

    let section_id = output.add_section(
        output.segment_name(StandardSegment::Text).to_vec(),
        format!(".text.{:x}", sym.as_u64()).as_bytes().to_vec(),
        SectionKind::Text,
    );

    let pbroc_symbol = Symbol {
        name: fn_name.as_bytes().to_vec(),
        value: 0,
        size: 0,
        kind: SymbolKind::Text,
        // TODO: Depending on whether we are building a static or dynamic lib, this should change.
        // We should use Dynamic -> anyone, Linkage -> static link, Compilation -> this module only.
        scope: if backend.env().exposed_to_host.contains(&sym) {
            SymbolScope::Dynamic
        } else {
            SymbolScope::Linkage
        },
        weak: false,
        section: SymbolSection::Section(section_id),
        flags: SymbolFlags::None,
    };
    let pbroc_id = output.add_symbol(pbroc_symbol);
    pbrocs.push((fn_name, section_id, pbroc_id, pbroc));
}

#[allow(clippy::too_many_arguments)]
fn build_pbroc<'a, B: Backend<'a>>(
    output: &mut Object,
    backend: &mut B,
    relocations: &mut Vec<'a, (SectionId, object::write::Relocation)>,
    layout_ids: &mut LayoutIds<'a>,
    data_section: SectionId,
    fn_name: String,
    section_id: SectionId,
    pbroc_id: SymbolId,
    pbroc: Pbroc<'a>,
) {
    let mut local_data_index = 0;
    let (pbroc_data, relocs, rc_pbroc_names) = backend.build_pbroc(pbroc, layout_ids);
    let pbroc_offset = output.add_symbol_data(pbroc_id, section_id, &pbroc_data, 16);
    for reloc in relocs.iter() {
        let elfreloc = match reloc {
            Relocation::LocalData { offset, data } => {
                let data_symbol = write::Symbol {
                    name: format!("{}.data{}", fn_name, local_data_index)
                        .as_bytes()
                        .to_vec(),
                    value: 0,
                    size: 0,
                    kind: SymbolKind::Data,
                    scope: SymbolScope::Compilation,
                    weak: false,
                    section: SymbolSection::Section(data_section),
                    flags: SymbolFlags::None,
                };
                local_data_index += 1;
                let data_id = output.add_symbol(data_symbol);
                output.add_symbol_data(data_id, data_section, data, 4);
                write::Relocation {
                    offset: offset + pbroc_offset,
                    size: 32,
                    kind: RelocationKind::Relative,
                    encoding: RelocationEncoding::Generic,
                    symbol: data_id,
                    addend: -4,
                }
            }
            Relocation::LinkedData { offset, name } => {
                if let Some(sym_id) = output.symbol_id(name.as_bytes()) {
                    write::Relocation {
                        offset: offset + pbroc_offset,
                        size: 32,
                        kind: RelocationKind::GotRelative,
                        encoding: RelocationEncoding::Generic,
                        symbol: sym_id,
                        addend: -4,
                    }
                } else {
                    internal_error!("failed to find data symbol for {:?}", name);
                }
            }
            Relocation::LinkedFunction { offset, name } => {
                // If the symbol is an undefined broc function, we need to add it here.
                if output.symbol_id(name.as_bytes()).is_none() && name.starts_with("broc_") {
                    let builtin_symbol = Symbol {
                        name: name.as_bytes().to_vec(),
                        value: 0,
                        size: 0,
                        kind: SymbolKind::Text,
                        scope: SymbolScope::Linkage,
                        weak: false,
                        section: SymbolSection::Undefined,
                        flags: SymbolFlags::None,
                    };
                    output.add_symbol(builtin_symbol);
                }
                // If the symbol is an undefined reference counting procedure, we need to add it here.
                if output.symbol_id(name.as_bytes()).is_none() {
                    for (sym, rc_name) in rc_pbroc_names.iter() {
                        if name == rc_name {
                            let section_id = output.add_section(
                                output.segment_name(StandardSegment::Text).to_vec(),
                                format!(".text.{:x}", sym.as_u64()).as_bytes().to_vec(),
                                SectionKind::Text,
                            );

                            let rc_symbol = Symbol {
                                name: name.as_bytes().to_vec(),
                                value: 0,
                                size: 0,
                                kind: SymbolKind::Text,
                                scope: SymbolScope::Linkage,
                                weak: false,
                                section: SymbolSection::Section(section_id),
                                flags: SymbolFlags::None,
                            };
                            output.add_symbol(rc_symbol);
                        }
                    }
                }

                if let Some(sym_id) = output.symbol_id(name.as_bytes()) {
                    write::Relocation {
                        offset: offset + pbroc_offset,
                        size: 32,
                        kind: RelocationKind::PltRelative,
                        encoding: RelocationEncoding::X86Branch,
                        symbol: sym_id,
                        addend: -4,
                    }
                } else {
                    internal_error!("failed to find fn symbol for {:?}", name);
                }
            }
            Relocation::JmpToReturn { .. } => unreachable!(),
        };
        relocations.push((section_id, elfreloc));
    }
}
