
include!("main.rs");

use wasm_bindgen::prelude::*;

// Wrap through to main (pretend it's a normal rust executable)
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_log::init_with_level(log::Level::Debug).unwrap();

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    main();
    Ok(())
}