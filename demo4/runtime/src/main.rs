use md_parser::MdParserData;
use wit_bindgen_wasmtime::{
    rt::{get_func, get_memory, RawMem},
    wasmtime::*,
};

wit_bindgen_wasmtime::import!("../md-parser/md-parser.wit");

fn main() {
    let engine = Engine::default();
    let mut linker = Linker::<MdParserData>::new(&engine);
    let module = Module::from_file(
        &engine,
        "../md-parser/target/wasm32-unknown-unknown/release/md_parser.wasm",
    )
    .unwrap();
    let mut store = Store::<MdParserData>::new(&engine, MdParserData::default());

    linker
        .func_wrap(
            "fetch",
            "fetch",
            move |mut caller: Caller<'_, MdParserData>, arg0: i32, arg1: i32, arg2: i32| {
                let func = get_func(&mut caller, "canonical_abi_realloc")?;
                let func_canonical_abi_realloc =
                    func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
                let memory = &get_memory(&mut caller, "memory")?;
                let (mem, _data) = memory.data_and_store_mut(&mut caller);
                let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
                let ptr0 = arg0;
                let len0 = arg1;
                let param0 = _bc.slice_str(ptr0, len0)?;
                let result = fetch_markdown(param0);
                let vec1 = result;
                let ptr1 =
                    func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, vec1.len() as i32))?;
                let caller_memory = memory.data_mut(&mut caller);
                caller_memory.store_many(ptr1, vec1.as_bytes())?;
                caller_memory.store(
                    arg2 + 4,
                    wit_bindgen_wasmtime::rt::as_i32(vec1.len() as i32),
                )?;
                caller_memory.store(arg2 + 0, wit_bindgen_wasmtime::rt::as_i32(ptr1))?;
                Ok(())
            },
        )
        .unwrap();

    let result =
        md_parser::MdParser::<MdParserData>::instantiate(&mut store, &module, &mut linker, |s| s)
            .unwrap();

    let res = result
        .0
        .parse_markdown(
            &mut store,
            "https://raw.githubusercontent.com/WebAssembly/spec/main/README.md",
        )
        .unwrap();
    println!("{}", res);
}

fn fetch_markdown(url: &str) -> String {
    let response = reqwest::blocking::get(url).unwrap();
    let markdown = response.text().unwrap();

    markdown
}
