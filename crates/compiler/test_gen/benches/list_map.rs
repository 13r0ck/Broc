#[path = "../src/helpers/mod.rs"]
mod helpers;

// defines broc_alloc and friends
pub use helpers::platform_functions::*;

use bumpalo::Bump;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use broc_gen_llvm::{llvm::build::LlvmBackendMode, run_broc::BrocCallResult, run_broc_dylib};
use broc_mono::ir::OptLevel;
use broc_std::BrocList;

// results July 6, 2022
//
//    broc sum map             time:   [612.73 ns 614.24 ns 615.98 ns]
//    broc sum map_with_index  time:   [5.3177 us 5.3218 us 5.3255 us]
//    rust (debug)            time:   [24.081 us 24.163 us 24.268 us]
//
// results April 9, 2023
//
//    broc sum map             time:   [510.77 ns 517.47 ns 524.47 ns]
//    broc sum map_with_index  time:   [573.49 ns 578.17 ns 583.76 ns]

type Input = BrocList<i64>;
type Output = i64;

type Main<I, O> = unsafe extern "C" fn(I, *mut BrocCallResult<O>);

const ROC_LIST_MAP: &str = indoc::indoc!(
    r#"
    app "bench" provides [main] to "./platform"

    main : List I64 -> Nat
    main = \list ->
        list
            |> List.map (\x -> x + 2)
            |> List.len
    "#
);

const ROC_LIST_MAP_WITH_INDEX: &str = indoc::indoc!(
    r#"
    app "bench" provides [main] to "./platform"

    main : List I64 -> Nat
    main = \list ->
        list
        |> List.mapWithIndex (\x, _ -> x + 2)
        |> List.len
    "#
);

fn broc_function<'a, 'b>(
    arena: &'a Bump,
    source: &str,
) -> libloading::Symbol<'a, Main<&'b Input, Output>> {
    let config = helpers::llvm::HelperConfig {
        mode: LlvmBackendMode::GenTest,
        ignore_problems: false,
        add_debug_info: true,
        opt_level: OptLevel::Optimize,
    };

    let context = inkwell::context::Context::create();
    let (main_fn_name, errors, lib) =
        helpers::llvm::helper(arena, config, source, arena.alloc(context));

    assert!(errors.is_empty(), "Encountered errors:\n{}", errors);

    run_broc_dylib!(arena.alloc(lib), main_fn_name, &Input, Output)
}

fn create_input_list() -> BrocList<i64> {
    let numbers = Vec::from_iter(0..1_000);

    BrocList::from_slice(&numbers)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let arena = Bump::new();

    let list_map_main = broc_function(&arena, ROC_LIST_MAP);
    let list_map_with_index_main = broc_function(&arena, ROC_LIST_MAP_WITH_INDEX);

    let input = &*arena.alloc(create_input_list());

    c.bench_function("broc sum map", |b| {
        b.iter(|| unsafe {
            let mut main_result = BrocCallResult::default();

            // the broc code will dec this list, so inc it first so it is not free'd
            std::mem::forget(input.clone());

            list_map_main(black_box(input), &mut main_result);
        })
    });

    c.bench_function("broc sum map_with_index", |b| {
        b.iter(|| unsafe {
            let mut main_result = BrocCallResult::default();

            // the broc code will dec this list, so inc it first so it is not free'd
            std::mem::forget(input.clone());

            list_map_with_index_main(black_box(input), &mut main_result);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
