mod utils;

use adventofcode_2022::days;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn log(s: &str);
    fn ctx_log(s: &str);
}

#[wasm_bindgen]
pub fn day_1(input: &str) -> Vec<JsValue> {
    let results = days::day_01::main(input);
    ctx_log(&format!("Logging! {:?}", results));
    vec![results.0.into(), results.1.into()]
}

#[wasm_bindgen]
pub fn day_2(input: &str) -> Vec<JsValue> {
    let results = days::day_02::main(input);
    vec![results.0.into(), results.1.into()]
}
