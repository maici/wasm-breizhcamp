// Import our CLI parsing libraries (And PathBuf for reading paths)
extern crate structopt;

// Import our markdown parser library, crate
extern crate pulldown_cmark;

use pulldown_cmark::{html, Parser};


// Our entrypoint into our WASI module
fn main() {

    // Read the markdown file into a string
    let contents = "* Hello Breizhcamp".to_string();

    // Run our parsing function to get back an HTML string
    let result = render_markdown(contents);

    // Print out the resulting HTML to standard out
    println!("{}", result);
}

pub fn render_markdown(markdown: String) -> String {
    let mut html_buf = String::new();
    let parser = Parser::new(&markdown[..]);
    html::push_html(&mut html_buf, parser);
    html_buf
}
