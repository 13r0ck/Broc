use bumpalo::Bump;
use broc_load::{ExecutionMode, LoadConfig, LoadedModule, Threading};
use broc_packaging::cache::BrocCacheDir;
use broc_reporting::report::DEFAULT_PALETTE;
use broc_target::TargetInfo;
use std::path::Path;

pub fn load_module(
    src_file: &Path,
    broc_cache_dir: BrocCacheDir<'_>,
    threading: Threading,
) -> LoadedModule {
    let load_config = LoadConfig {
        target_info: TargetInfo::default_x86_64(), // editor only needs type info, so this is unused
        render: broc_reporting::report::RenderTarget::ColorTerminal,
        palette: DEFAULT_PALETTE,
        threading,
        exec_mode: ExecutionMode::Check,
    };

    let arena = Bump::new();
    let loaded =
        broc_load::load_and_typecheck(&arena, src_file.to_path_buf(), broc_cache_dir, load_config);

    match loaded {
        Ok(x) => x,
        Err(broc_load::LoadingProblem::FormattedReport(report)) => {
            panic!(
                "Failed to load module from src_file: {:?}. Report: {}",
                src_file, report
            );
        }
        Err(e) => panic!(
            "Failed to load module from src_file {:?}: {:?}",
            src_file, e
        ),
    }
}
