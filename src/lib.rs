#![feature(const_generics)]
#![allow(incomplete_features)]
#![macro_use]

use crate::state::ParsedDemo;
use tf_demo_parser::demo::parser::gamestateanalyser::GameStateAnalyser;
use tf_demo_parser::{Demo, DemoParser, ParseError};
use wasm_bindgen::prelude::*;

mod state;

macro_rules! log {
    ($($arg:tt)*) => (web_sys::console::log_1(&JsValue::from(format!($($arg)*))))
}

#[wasm_bindgen]
pub fn parse_demo(buffer: Box<[u8]>) -> Result<(), JsValue> {
    let buffer = buffer.into_vec();
    let parsed = parse_demo_inner(buffer).map_err(|e| JsValue::from(e.to_string()))?;

    log!("{:?}, size {}", parsed.players[2].get(10), parsed.size());

    Ok(())
}

pub fn parse_demo_inner(buffer: Vec<u8>) -> Result<ParsedDemo, ParseError> {
    let demo = Demo::new(buffer);
    let parser = DemoParser::new_with_analyser(demo.get_stream(), GameStateAnalyser::default());
    let (_header, mut ticker) = parser.ticker()?;

    let mut parsed_demo = ParsedDemo::new();

    let mut skip = false;
    while ticker.tick()? {
        if !skip {
            // halve framerate
            parsed_demo.push_state(ticker.state());
        }
        skip = !skip;
    }

    Ok(parsed_demo)
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    log!("Hello world!");

    Ok(())
}
