use std::path::PathBuf;

use md_parser::MdParserData;
use structopt::StructOpt;
use wit_bindgen_wasmtime::wasmtime::*;

wit_bindgen_wasmtime::import!("../md-parser/md-parser.wit");
wit_bindgen_wasmtime::export!("./fetch.wit");

#[derive(StructOpt)]
#[structopt(
    name = "custom runtime with fetch",
    about = "Markdown to HTML renderer CLI, written with Rust & custom ABI"
)]
pub struct Options {
    #[structopt(parse(from_os_str))]
    wasm: PathBuf,

    #[structopt(short, long, parse(from_str))]
    url: String,
}

impl fetch::Fetch for MdParserData {
    fn fetch(&mut self, url: &str) -> String {
        let response = reqwest::blocking::get(url).unwrap();
        let markdown = response.text().unwrap();

        markdown
    }
}

fn main() {
    let options = Options::from_args();
    let engine = Engine::default();
    let module = Module::from_file(&engine, options.wasm).unwrap();
    let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, MdParserData::default());

    fetch::add_to_linker(&mut linker, |s| s).unwrap();

    let (wasm_md_parser, _) =
        md_parser::MdParser::instantiate(&mut store, &module, &mut linker, |s| s).unwrap();

    let markdown = wasm_md_parser
        .parse_markdown(&mut store, options.url.as_str())
        .unwrap();

    println!("{}", markdown);
}
