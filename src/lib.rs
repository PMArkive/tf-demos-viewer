#![feature(const_generics)]
#![allow(incomplete_features)]
#![macro_use]

use crate::state::ParsedDemo;
use tf_demo_parser::demo::parser::gamestateanalyser::{GameStateAnalyser, World};
use tf_demo_parser::demo::vector::Vector;
use tf_demo_parser::{Demo, DemoParser, ParseError};
use wasm_bindgen::prelude::*;

mod state;

macro_rules! log {
    ($($arg:tt)*) => (web_sys::console::log_1(&JsValue::from(format!($($arg)*))))
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct XY {
    pub x: f32,
    pub y: f32,
}

impl From<Vector> for XY {
    fn from(vec: Vector) -> Self {
        XY { x: vec.x, y: vec.y }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct WorldBoundaries {
    pub boundary_min: XY,
    pub boundary_max: XY,
}

impl From<World> for WorldBoundaries {
    fn from(world: World) -> Self {
        WorldBoundaries {
            boundary_min: world.boundary_min.into(),
            boundary_max: world.boundary_max.into(),
        }
    }
}

#[wasm_bindgen]
pub struct FlatState {
    player_count: usize,
    data: Box<[u8]>,
    boundaries: WorldBoundaries,
}

impl FlatState {
    pub fn new(parsed: ParsedDemo, world: World) -> Self {
        FlatState {
            player_count: parsed.players.len(),
            boundaries: world.into(),
            data: parsed.flat().into_boxed_slice(),
        }
    }
}

#[wasm_bindgen]
pub fn parse_demo(buffer: Box<[u8]>) -> Result<FlatState, JsValue> {
    let buffer = buffer.into_vec();
    let (parsed, world) = parse_demo_inner(buffer).map_err(|e| JsValue::from(e.to_string()))?;

    let world = world.ok_or_else(|| JsValue::from_str("No world defined in demo"))?;

    Ok(FlatState::new(parsed, world))
}

#[wasm_bindgen]
pub fn get_boundaries(state: &FlatState) -> WorldBoundaries {
    state.boundaries.clone()
}

#[wasm_bindgen]
pub fn get_player_count(state: &FlatState) -> usize {
    state.player_count
}

#[wasm_bindgen]
pub fn get_data(state: FlatState) -> Box<[u8]> {
    state.data
}

pub fn parse_demo_inner(buffer: Vec<u8>) -> Result<(ParsedDemo, Option<World>), ParseError> {
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

    let world: Option<&World> = ticker.state().world.as_ref();
    Ok((parsed_demo, world.map(|w| w.clone())))
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
