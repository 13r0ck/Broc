use libloading::Library;
use broc_build::link::{link, LinkType};
use broc_builtins::bitcode;
use broc_load::{EntryPoint, ExecutionMode, LoadConfig, Threading};
use broc_mono::ir::SingleEntryPoint;
use broc_packaging::cache::BrocCacheDir;
use broc_region::all::LineInfo;
use tempfile::tempdir;

#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
use broc_collections::all::MutMap;

#[allow(unused_imports)]
use broc_mono::ir::pretty_print_ir_symbols;

#[allow(dead_code)]
fn promote_expr_to_module(src: &str) -> String {
    let mut buffer = String::from("app \"test\" provides [main] to \"./platform\"\n\nmain =\n");

    for line in src.lines() {
        // indent the body!
        buffer.push_str("    ");
        buffer.push_str(line);
        buffer.push('\n');
    }

    buffer
}

#[allow(dead_code)]
pub fn helper(
    arena: &bumpalo::Bump,
    src: &str,
    _leak: bool,
    lazy_literals: bool,
) -> (String, Vec<broc_problem::can::Problem>, Library) {
    use std::path::PathBuf;

    let dir = tempdir().unwrap();
    let filename = PathBuf::from("Test.broc");
    let src_dir = PathBuf::from("fake/test/path");
    let app_o_file = dir.path().join("app.o");

    let module_src;
    let temp;
    if src.starts_with("app") {
        // this is already a module
        module_src = src;
    } else {
        // this is an expression, promote it to a module
        temp = promote_expr_to_module(src);
        module_src = &temp;
    }

    let load_config = LoadConfig {
        target_info: broc_target::TargetInfo::default_x86_64(),
        render: broc_reporting::report::RenderTarget::ColorTerminal,
        palette: broc_reporting::report::DEFAULT_PALETTE,
        threading: Threading::Single,
        exec_mode: ExecutionMode::Executable,
    };
    let loaded = broc_load::load_and_monomorphize_from_str(
        arena,
        filename,
        module_src,
        src_dir,
        BrocCacheDir::Disallowed,
        load_config,
    );

    let mut loaded = loaded.expect("failed to load module");

    use broc_load::MonomorphizedModule;
    let MonomorphizedModule {
        module_id,
        procedures,
        mut interns,
        exposed_to_host,
        mut layout_interner,
        ..
    } = loaded;

    // You can comment and uncomment this block out to get more useful information
    // while you're working on the dev backend!
    {
        // println!("=========== Pbrocedures ==========");
        // if pretty_print_ir_symbols() {
        //     println!("");
        //     for pbroc in procedures.values() {
        //         println!("{}", pbroc.to_pretty(200));
        //     }
        // } else {
        //     println!("{:?}", procedures.values());
        // }
        // println!("=================================\n");

        // println!("=========== Interns    ==========");
        // println!("{:?}", interns);
        // println!("=================================\n");

        // println!("=========== Exposed    ==========");
        // println!("{:?}", exposed_to_host);
        // println!("=================================\n");
    }

    debug_assert_eq!(exposed_to_host.top_level_values.len(), 1);
    let entry_point = match loaded.entry_point {
        EntryPoint::Executable {
            exposed_to_host,
            platform_path: _,
        } => {
            // TODO support multiple of these!
            debug_assert_eq!(exposed_to_host.len(), 1);
            let (symbol, layout) = exposed_to_host[0];

            SingleEntryPoint { symbol, layout }
        }
        EntryPoint::Test => {
            unreachable!()
        }
    };
    let main_fn_symbol = entry_point.symbol;
    let main_fn_layout = entry_point.layout;

    let mut layout_ids = broc_mono::layout::LayoutIds::default();
    let main_fn_name = layout_ids
        .get_toplevel(main_fn_symbol, &main_fn_layout)
        .to_exposed_symbol_string(main_fn_symbol, &interns);

    let mut lines = Vec::new();
    // errors whose reporting we delay (so we can see that code gen generates runtime errors)
    let mut delayed_errors = Vec::new();

    for (home, (module_path, src)) in loaded.sources {
        use broc_reporting::report::{can_problem, type_problem, BrocDocAllocator, DEFAULT_PALETTE};

        let can_problems = loaded.can_problems.remove(&home).unwrap_or_default();
        let type_problems = loaded.type_problems.remove(&home).unwrap_or_default();

        let error_count = can_problems.len() + type_problems.len();

        if error_count == 0 {
            continue;
        }

        let line_info = LineInfo::new(&src);
        let src_lines: Vec<&str> = src.split('\n').collect();
        let palette = DEFAULT_PALETTE;

        // Report parsing and canonicalization problems
        let alloc = BrocDocAllocator::new(&src_lines, home, &interns);

        use broc_problem::can::Problem::*;
        for problem in can_problems.into_iter() {
            // Ignore "unused" problems
            match problem {
                UnusedDef(_, _) | UnusedArgument(_, _, _, _) | UnusedModuleImport(_, _) => {
                    delayed_errors.push(problem);
                    continue;
                }
                _ => {
                    let report = can_problem(&alloc, &line_info, module_path.clone(), problem);
                    let mut buf = String::new();

                    report.render_color_terminal(&mut buf, &alloc, &palette);

                    lines.push(buf);
                }
            }
        }

        for problem in type_problems {
            if let Some(report) = type_problem(&alloc, &line_info, module_path.clone(), problem) {
                let mut buf = String::new();

                report.render_color_terminal(&mut buf, &alloc, &palette);

                lines.push(buf);
            }
        }
    }

    if !lines.is_empty() {
        println!("{}", lines.join("\n"));
        assert_eq!(0, 1, "Mistakes were made");
    }

    let env = broc_gen_dev::Env {
        arena,
        module_id,
        exposed_to_host: exposed_to_host.top_level_values.keys().copied().collect(),
        lazy_literals,
        generate_allocators: true, // Needed for testing, since we don't have a platform
    };

    let target = target_lexicon::Triple::host();
    let module_object = broc_gen_dev::build_module(
        &env,
        &mut interns,
        &mut layout_interner,
        &target,
        procedures,
    );

    let module_out = module_object
        .write()
        .expect("failed to build output object");
    std::fs::write(&app_o_file, module_out).expect("failed to write object to file");

    let builtins_host_tempfile =
        broc_bitcode::host_tempfile().expect("failed to write host builtins object to tempfile");

    if false {
        std::fs::copy(&app_o_file, "/tmp/app.o").unwrap();
    }

    let (mut child, dylib_path) = link(
        &target,
        app_o_file.clone(),
        // Long term we probably want a smarter way to link in zig builtins.
        // With the current method all methods are kept and it adds about 100k to all outputs.
        &[
            app_o_file.to_str().unwrap(),
            builtins_host_tempfile.path().to_str().unwrap(),
        ],
        LinkType::Dylib,
    )
    .expect("failed to link dynamic library");

    child.wait().unwrap();

    // Extend the lifetime of the tempfile so it doesn't get dropped
    // (and thus deleted) before the linking process is done using it!
    let _ = builtins_host_tempfile;

    // Load the dylib
    let path = dylib_path.as_path().to_str().unwrap();

    // std::fs::copy(&path, "/tmp/libapp.so").unwrap();

    let lib = unsafe { Library::new(path) }.expect("failed to load shared library");

    (main_fn_name, delayed_errors, lib)
}

#[allow(unused_macros)]
macro_rules! assert_evals_to {
    ($src:expr, $expected:expr, $ty:ty) => {{
        assert_evals_to!($src, $expected, $ty, (|val| val));
    }};
    ($src:expr, $expected:expr, $ty:ty, $transform:expr) => {
        // Same as above, except with an additional transformation argument.
        {
            assert_evals_to!($src, $expected, $ty, $transform, true);
        }
    };
    ($src:expr, $expected:expr, $ty:ty, $transform:expr, $leak:expr) => {
        // Run both with and without lazy literal optimization.
        {
            assert_evals_to!($src, $expected, $ty, $transform, $leak, false);
        }
        {
            assert_evals_to!($src, $expected, $ty, $transform, $leak, true);
        }
    };
    ($src:expr, $expected:expr, $ty:ty, $transform:expr, $leak:expr, $lazy_literals:expr) => {
        use bumpalo::Bump;
        use broc_gen_dev::run_jit_function_raw;

        let arena = Bump::new();
        let (main_fn_name, errors, lib) =
            $crate::helpers::dev::helper(&arena, $src, $leak, $lazy_literals);

        let transform = |success| {
            let expected = $expected;
            #[allow(clippy::redundant_closure_call)]
            let given = $transform(success);
            assert_eq!(&given, &expected);
        };
        run_jit_function_raw!(lib, main_fn_name, $ty, transform, errors)
    };
}

#[allow(unused_imports)]
pub(crate) use assert_evals_to;
