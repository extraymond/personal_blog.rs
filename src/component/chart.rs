use anyhow::Result;
use dodrio_ext::prelude::*;
use futures_timer::Delay;
use std::time::Duration;
use vega_lite_3::*;

#[derive(Default)]
pub struct Model(pub Option<Vegalite>);

pub enum Msg {
    Init(Sender<Msg>),
    ChartChanged(Vegalite, Sender<Msg>),
    ChartUpdated(String),
    Watcher(Sender<Msg>, [i32; 2]),
}

pub enum Charts {
    Simple,
}

pub fn simple_chart() -> Result<Vegalite, String> {
    let chart = VegaliteBuilder::default()
        .title("Weather")
        .autosize(AutosizeType::Fit)
        .data(
            UrlDataBuilder::default()
                .url("https://raw.githubusercontent.com/vega/vega-datasets/master/data/weather.csv")
                .build()?,
        )
        // .transform(vec![TransformBuilder::default()
        //     .filter("datum.symbol==='GOOG'")
        //     .build()?])
        .mark(Mark::Circle)
        .encoding(
            EncodingBuilder::default()
                .x(XClassBuilder::default()
                    .field("date")
                    .def_type(StandardType::Temporal)
                    .build()?)
                .y(YClassBuilder::default()
                    .field("wind")
                    .def_type(StandardType::Ordinal)
                    .build()?)
                .color(
                    DefWithConditionMarkPropFieldDefStringNullBuilder::default()
                        .field("weather")
                        .def_with_condition_mark_prop_field_def_string_null_type(
                            StandardType::Nominal,
                        )
                        .build()?,
                )
                .column(
                    FacetFieldDefBuilder::default()
                        .field("location")
                        .facet_field_def_type(StandardType::Nominal)
                        .build()?,
                )
                // .size(
                //     DefWithConditionMarkPropFieldDefNumberBuilder::default()
                //         .field("weather")
                //         .build()?,
                // )
                .build()?,
        )
        .build()?;

    Ok(chart)
}

impl Component<Msg, ()> for Model {
    fn new(_: Sender<bool>) -> Self {
        Model::default()
    }

    fn mounted(tx_handle: Sender<Msg>, _: Sender<()>, _: Sender<bool>) {
        log::info!("mounted");

        let mut tx = tx_handle.clone();
        spawn_local(async move {
            tx.send(Msg::Init(tx.clone())).await.unwrap();
            let chart = simple_chart().unwrap();
            tx.send(Msg::ChartChanged(chart, tx.clone())).await.unwrap();
        });
        let doc = web_sys::window().unwrap().document().unwrap();

        let mut tx = tx_handle;
        let mut dimension = [0; 2];
        let watch_job = async move {
            loop {
                if let Ok(nodes) = doc.query_selector_all("[data-watchresize=true]") {
                    Delay::new(Duration::from_millis(50)).await;
                    if let Some(node) = nodes.get(0) {
                        let node: web_sys::EventTarget = node.unchecked_into();
                        let el: web_sys::HtmlElement = node.clone().unchecked_into();
                        let new_dim = [el.client_width(), el.client_height()];
                        if (dimension != new_dim) && (new_dim[0] != 0 && new_dim[1] != 0) {
                            dimension = new_dim;
                            tx.send(Msg::Watcher(tx.clone(), dimension)).await.unwrap();
                        }
                    }
                };
            }
        };
        // spawn_local(watch_job);
    }

    fn update(&mut self, msg: Msg) -> bool {
        match msg {
            Msg::Init(mut tx) => {
                if let Some(chart) = &self.0 {
                    let spec = chart.to_string().unwrap();
                    spawn_local(async move {
                        tx.send(Msg::ChartUpdated(spec)).await.unwrap();
                    });
                }
                true
            }
            Msg::ChartChanged(chart, mut tx) => {
                let spec = chart.to_string().unwrap();
                self.0.replace(chart);
                spawn_local(async move {
                    tx.send(Msg::ChartUpdated(spec)).await.unwrap();
                });
                false
            }
            Msg::ChartUpdated(spec) => {
                let win = web_sys::window().unwrap();
                let win_target: web_sys::EventTarget = win.unchecked_into();
                let mut event_init = web_sys::CustomEventInit::new();

                event_init.detail(&JsValue::from_str(&spec));
                let event: web_sys::Event =
                    web_sys::CustomEvent::new_with_event_init_dict("vega", &event_init)
                        .unwrap()
                        .unchecked_into();
                win_target.dispatch_event(&event).unwrap();
                false
            }
            Msg::Watcher(mut tx, dim) => {
                if let Some(chart) = &mut self.0 {
                    chart.width.replace(dim[0] as f64);
                    chart.height.replace(dim[1] as f64);
                    let spec = chart.to_string().unwrap();
                    spawn_local(async move {
                        tx.send(Msg::ChartUpdated(spec)).await.unwrap();
                    });
                }
                false
            }
        }
    }
}

impl Render<Msg, ()> for Model {
    fn render<'a>(
        &self,
        ctx: &mut RenderContext<'a>,
        _: Sender<Msg>,
        _: Sender<()>,
        _: Sender<bool>,
    ) -> Node<'a> {
        let bump = ctx.bump;
        use dodrio::builder::*;
        let chart = dodrio!(bump, <div id="vega"></div>);
        let mid = div(bump)
            .attr("data-watchresize", "true")
            .attr("style", "width: 100%; height: 100%")
            .child(chart)
            .finish();
        dodrio!(bump,<div class="box is-marginless" style="width: 100vw; height: 100vh">{ mid } </div>)
    }
}
