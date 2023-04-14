mod graphics;
mod gui;
mod rects_and_texts;
mod broc;

use crate::broc::BrocElem;

extern "C" {
    #[link_name = "broc__renderForHost_1_exposed"]
    fn broc_render() -> BrocElem;
}

#[no_mangle]
pub extern "C" fn rust_main() -> i32 {
    let root_elem = unsafe { broc_render() };

    gui::render("test title".into(), root_elem);

    // Exit code
    0
}
