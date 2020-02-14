#![recursion_limit = "1024"]

mod component;
mod elements;
mod views;
use cfg_if::*;
use dodrio_ext::prelude::*;
use vega_lite_3::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn start() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    if cfg!(debug_assertions) {
        femme::start(log::LevelFilter::Debug).unwrap();
    } else {
        femme::start(log::LevelFilter::Warn).unwrap();
    }

    let mut hub = MessageHub::new();
    hub.bind_root_el(component::chart::Model::default(), None);
    hub.mount_hub_rx();
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_md() {
        start();
    }
}
