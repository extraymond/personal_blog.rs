#![recursion_limit = "1024"]

mod component;
mod elements;
use cfg_if::*;
use dodrio_ext::prelude::*;
use vega_lite_3::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

pub fn show() -> String {
    let chart = VegaliteBuilder::default()
    .title("Stock price")
    .description("Google's stock price over time.")
    .data(UrlDataBuilder::default().url(
        "https://raw.githubusercontent.com/davidB/vega_lite_3.rs/master/examples/res/data/stocks.csv"
    ).build().unwrap())
    .transform(vec![
        TransformBuilder::default().filter("datum.symbol==='GOOG'")
    .build().unwrap()])
    .mark(Mark::Line)
    .encoding(EncodingBuilder::default()
        .x(XClassBuilder::default()
            .field("date")
            .def_type(StandardType::Temporal)
            .build().unwrap())
        .y(YClassBuilder::default()
            .field("price")
            .def_type(StandardType::Quantitative)
            .build().unwrap()).build().unwrap()).build().unwrap();

    chart.to_string().unwrap()
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

    show();

    let mut hub = MessageHub::new();
    hub.bind_root_el(component::chart::Model(show()), None);
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
