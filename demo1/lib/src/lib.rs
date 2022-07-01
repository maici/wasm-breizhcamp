use pulldown_cmark::{html, Parser};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_markdown(makrdown: String) -> String {
    let mut html_buf = String::new();
    let parser = Parser::new(&makrdown);
    html::push_html(&mut html_buf, parser);

    html_buf
}
