use wasm_bindgen::prelude::*;
use web_sys::console;
use tf_demo_parser::{DemoParser, Demo};

#[wasm_bindgen]
pub fn parse_demo(buffer: &[u8]) {
    let buffer = buffer.to_vec();
    console::log_1(&JsValue::from_str(&format!("len: {}", buffer.len())));
    let demo = Demo::new(buffer);
    let (header, _) = DemoParser::parse_all(demo.get_stream()).unwrap();
    console::log_1(&JsValue::from_str(&format!("{:?}", header)));
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}
