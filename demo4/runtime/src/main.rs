use md_parser::MdParserData;
use wit_bindgen_wasmtime::wasmtime::*;

wit_bindgen_wasmtime::import!("../md-parser/md-parser.wit");
wit_bindgen_wasmtime::export!("./fetch.wit");

impl fetch::Fetch for MdParserData {
    fn fetch(&mut self, url: &str) -> String {
        let response = reqwest::blocking::get(url).unwrap();
        let markdown = response.text().unwrap();

        markdown
    }
}

fn main() {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    let module = Module::from_file(
        &engine,
        "../md-parser/target/wasm32-unknown-unknown/release/md_parser.wasm",
    )
    .unwrap();
    let mut store = Store::new(&engine, MdParserData::default());

    fetch::add_to_linker(&mut linker, |s| s).unwrap();

    let (wasm_md_parser, _) =
        md_parser::MdParser::instantiate(&mut store, &module, &mut linker, |s| s).unwrap();

    let markdown = wasm_md_parser
        .parse_markdown(
            &mut store,
            "https://raw.githubusercontent.com/WebAssembly/spec/main/README.md",
        )
        .unwrap();

    println!("{}", markdown);
}
