#![recursion_limit = "256"]

mod component;
use dodrio_ext::prelude::*;

#[wasm_bindgen]
pub fn start() {
    console_error_panic_hook::set_once();
    femme::start(log::LevelFilter::Debug).unwrap();
    log::info!("start testing");
    let mut hub = MessageHub::new();
    hub.bind_root_el(component::blog_page::Model::default(), None);
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
