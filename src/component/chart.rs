use anyhow::Result;
use dodrio_ext::prelude::*;
use futures_timer::Delay;
use ndarray::{Array, Array2};

use std::{collections::HashMap, time::Duration};
use vega_lite_3::*;

#[derive(Default)]
pub struct Model(pub Option<Vegalite>);

#[derive(Debug)]
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
                // .column(
                //     FacetFieldDefBuilder::default()
                //         .field("location")
                //         .facet_field_def_type(StandardType::Nominal)
                //         .build()?,
                // )
                // .size(
                //     DefWithConditionMarkPropFieldDefNumberBuilder::default()
                //         .field("weather")
                //         .build()?,
                // )
                .build()?,
        )
        .build()?;

    let mut selector_1 = HashMap::new();
    selector_1.insert(
        "brush".to_string(),
        SelectionDefBuilder::default()
            .encodings(vec![SingleDefUnitChannel::X])
            .selection_def_type(SelectionDefType::Interval)
            .build()?,
    );
    let mut selector_2 = HashMap::new();
    selector_2.insert(
        "click".to_string(),
        SelectionDefBuilder::default()
            .encodings(vec![SingleDefUnitChannel::Color])
            .selection_def_type(SelectionDefType::Multi)
            .build()?,
    );

    let chart = VegaliteBuilder::default()
        .title("Seattle Weather, 2012-2015")
        .data(
          UrlDataBuilder::default()
            .url("https://raw.githubusercontent.com/vega/vega-datasets/master/data/seattle-weather.csv")
            .build()?,
        )
        .vconcat(vec![
          SpecBuilder::default()
            .selection(selector_1)
            .transform(vec![TransformBuilder::default()
              .filter(PurpleLogicalOperandPredicate::Predicate(
                PredicateBuilder::default()
                  .selection(PurpleSelectionOperand::String("click".to_string()))
                  .build()?,
              ))
              .build()?])
            .mark(Mark::Point)
            .width(600)
            .height(300)
            .encoding(
              EncodingBuilder::default()
                .color(
                  DefWithConditionMarkPropFieldDefStringNullBuilder::default()
                    .condition(
                      ConditionalPredicateStringValueDefClassBuilder::default()
                        .selection(PurpleSelectionOperand::String("brush".to_string()))
                        .conditional_def_type(StandardType::Nominal)
                        .field("weather")
                        .title("Weather")
                        .scale(
                          ScaleBuilder::default()
                            .domain(vec![
                              SelectionInitIntervalElement::String("sun".to_string()),
                              SelectionInitIntervalElement::String("fog".to_string()),
                              SelectionInitIntervalElement::String("drizzle".to_string()),
                              SelectionInitIntervalElement::String("rain".to_string()),
                              SelectionInitIntervalElement::String("snow".to_string()),
                            ])
                            .range(vec![
                              RangeRange::String("#e7ba52".to_string()),
                              RangeRange::String("#c7c7c7".to_string()),
                              RangeRange::String("#aec7e8".to_string()),
                              RangeRange::String("#1f77b4".to_string()),
                              RangeRange::String("#9467bd".to_string()),
                            ])
                            .build()?,
                        )
                        .build()?,
                    )
                    .value("lightgray")
                    .build()?,
                )
                .x(
                  XClassBuilder::default()
                    .field("date")
                    .def_type(StandardType::Temporal)
                    .time_unit(TimeUnit::Monthdate)
                    .axis(AxisBuilder::default().title("date").format("%b").build()?)
                    .build()?,
                )
                .y(
                  YClassBuilder::default()
                    .field("temp_max")
                    .def_type(StandardType::Quantitative)
                    .scale(
                      ScaleBuilder::default()
                        .domain(vec![
                          SelectionInitIntervalElement::Double(-5.0),
                          SelectionInitIntervalElement::Double(40.0),
                        ])
                        .build()?,
                    )
                    .axis(
                      AxisBuilder::default()
                        .title("Maximum Daily Temperature (C)")
                        .build()?,
                    )
                    .build()?,
                )
                .size(
                  DefWithConditionMarkPropFieldDefNumberBuilder::default()
                    .title("Precipitation")
                    .field("precipitation")
                    .def_with_condition_mark_prop_field_def_number_type(StandardType::Quantitative)
                    .scale(
                      ScaleBuilder::default()
                        .domain(vec![
                          SelectionInitIntervalElement::Double(-1.0),
                          SelectionInitIntervalElement::Double(50.0),
                        ])
                        .build()?,
                    )
                    .build()?,
                )
                .build()?,
            )
            .build()?,
          SpecBuilder::default()
            .width(600)
            .mark(Mark::Bar)
            .selection(selector_2)
            .transform(vec![TransformBuilder::default()
              .filter(PurpleLogicalOperandPredicate::Predicate(
                PredicateBuilder::default()
                  .selection(PurpleSelectionOperand::String("brush".to_string()))
                  .build()?,
              ))
              .build()?])
            .encoding(
              EncodingBuilder::default()
                .color(
                  DefWithConditionMarkPropFieldDefStringNullBuilder::default()
                    .condition(
                      ConditionalPredicateStringValueDefClassBuilder::default()
                        .selection(PurpleSelectionOperand::String("click".to_string()))
                        .conditional_def_type(StandardType::Nominal)
                        .field("weather")
                        .title("Weather")
                        .build()?,
                    )
                    .build()?,
                )
                .x(
                  XClassBuilder::default()
                    .aggregate(AggregateOp::Count)
                    .def_type(StandardType::Quantitative)
                    .build()?,
                )
                .y(
                  YClassBuilder::default()
                    .title("Weather")
                    .field("weather")
                    .def_type(StandardType::Nominal)
                    .build()?,
                )
                .build()?,
            )
            .build()?,
        ])
        .build()?;

    // let mut buf = vec![[0, 0]; 100];
    // buf.iter_mut().for_each(|arr| {
    //     getrandom::getrandom(arr).unwrap();
    // });
    // let values = Array2::from(buf);
    // // let values: Array2<f64> = buf.into();

    // // the chart
    // let chart = VegaliteBuilder::default()
    //     .title("Random points")
    //     .data(values)
    //     .mark(Mark::Point)
    //     .autosize(AutosizeType::Fit)
    //     .encoding(
    //         EncodingBuilder::default()
    //             .x(XClassBuilder::default()
    //                 .title("x axis")
    //                 .field("data.0")
    //                 .def_type(StandardType::Quantitative)
    //                 .build()?)
    //             .y(YClassBuilder::default()
    //                 .title("y axis")
    //                 .field("data.1")
    //                 .def_type(StandardType::Quantitative)
    //                 .build()?)
    //             .build()?,
    //     )
    //     .build()?;
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
        spawn_local(watch_job);
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
