use dodrio_ext::prelude::*;
use vega_lite_3::*;

#[derive(Default)]
pub struct Model(pub String);

pub enum Msg {
    Init(Sender<Msg>),
    ChartChanged(String),
    ChartUpdated,
}

impl Component<Msg, ()> for Model {
    fn new(_: Sender<bool>) -> Self {
        Model::default()
    }

    fn mounted(mut tx: Sender<Msg>, _: Sender<()>, _: Sender<bool>) {
        spawn_local(async move {
            tx.send(Msg::Init(tx.clone())).await.unwrap();
        });
    }

    fn update(&mut self, msg: Msg) -> bool {
        match msg {
            Msg::Init(mut tx) => {
                if &self.0 != "" {
                    spawn_local(async move {
                        tx.send(Msg::ChartUpdated).await.unwrap();
                    });
                }
                true
            }
            Msg::ChartChanged(spec) => {
                self.0 = spec;
                false
            }
            Msg::ChartUpdated => {
                let win = web_sys::window().unwrap();
                let win_target: web_sys::EventTarget = win.unchecked_into();
                let mut event_init = web_sys::CustomEventInit::new();

                event_init.detail(&JsValue::from_str(&self.0));
                let event: web_sys::Event =
                    web_sys::CustomEvent::new_with_event_init_dict("vega", &event_init)
                        .unwrap()
                        .unchecked_into();
                win_target.dispatch_event(&event).unwrap();
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
        dodrio!(bump, <div id="vega"></div>)
    }
}
