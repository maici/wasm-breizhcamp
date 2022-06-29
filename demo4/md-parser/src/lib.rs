use pulldown_cmark::{html, Parser};

wit_bindgen_rust::export!("./md-parser.wit");
wit_bindgen_rust::import!("../runtime/fetch.wit");

struct MdParser;

impl md_parser::MdParser for MdParser {
    fn parse_markdown(url: String) -> String {
        let markdown = fetch::fetch(&url);
        let mut html_buf = String::new();
        let parser = Parser::new(&markdown);

        html::push_html(&mut html_buf, parser);

        html_buf
    }
}
