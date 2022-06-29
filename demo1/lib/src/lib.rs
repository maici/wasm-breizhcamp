mod utils;

use pulldown_cmark::{html, Parser};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn parse_markdown(makrdown: String) -> String {
    let mut html_buf = String::new();
    let parser = Parser::new(&makrdown);
    html::push_html(&mut html_buf, parser);

    html_buf
}
