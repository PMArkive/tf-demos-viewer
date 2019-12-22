use tf_demo_parser::demo::parser::gamestateanalyser::GameStateAnalyser;
use tf_demo_parser::{Demo, DemoParser, ParseError};
use wasm_bindgen::__rt::std::time::Instant;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn parse_demo(buffer: &[u8]) -> Result<(), JsValue> {
    let buffer = buffer.to_vec();
    parse_demo_inner(buffer).map_err(|e| e.to_string().into())
}

pub fn parse_demo_inner(buffer: Vec<u8>) -> Result<(), ParseError> {
    console::log_1(&JsValue::from_str(&format!("len: {}", buffer.len())));
    let demo = Demo::new(buffer);
    let parser = DemoParser::new_with_analyser(demo.get_stream(), GameStateAnalyser::default());
    let (header, mut ticker) = parser.ticker()?;
    while ticker.tick()? {
        // noop
    }

    console::log_1(&JsValue::from_str(&format!("{:?}", header)));
    Ok(())
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
