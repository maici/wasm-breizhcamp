use pulldown_cmark::{html, Parser};
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;


#[no_mangle]
pub extern "C" fn render_markdown(markdownchar: *const c_char) -> *const c_char {
    let c_str: &CStr = unsafe { CStr::from_ptr(markdownchar) };
    let mut html_buf = String::new();
    let markdown = c_str.to_str().unwrap();
    let parser = Parser::new(&markdown[..]);
    html::push_html(&mut html_buf, parser);
    // println!("{}",html_buf);
    let result = CString::new(html_buf).unwrap();        
     return result.into_raw();    
}