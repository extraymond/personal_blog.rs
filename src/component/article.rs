use dodrio_ext::prelude::*;
use futures::{
    channel::oneshot,
    future::{AbortHandle, AbortRegistration, Abortable},
};
use futures_timer::Delay;
use gloo::events;
use pulldown_cmark::{html, Options, Parser};
use std::time::Duration;

#[derive(Default)]
pub struct Model(pub String);

pub enum Msg {
    Init(Sender<Msg>),
    // Creation,
    // Setup(AbortHandle),
    Clicked,
    ContentChanged(String, web_sys::Element),
}

impl Component<Msg, ()> for Model {
    fn new(tx: Sender<bool>) -> Self {
        Model::default()
    }

    fn mounted(data_tx: Sender<Msg>, _: Sender<()>, _: Sender<bool>) {
        spawn_local(async move {
            data_tx.clone().send(Msg::Init(data_tx)).await.unwrap();
        });
    }

    fn update(&mut self, msg: Msg) -> bool {
        match msg {
            Msg::Init(mut data_tx) => {
                let content = self.0.clone();
                spawn_local(async move {
                    let mut found = false;
                    while found == false {
                        Delay::new(Duration::from_micros(500)).await;
                        let doc = web_sys::window().unwrap().document().unwrap();
                        let nodes = doc.query_selector_all("[data-content]").unwrap();
                        for idx in 0..nodes.length() {
                            let node = nodes.item(idx).unwrap();
                            let el: web_sys::Element = node.unchecked_into();
                            if let Some(attr) = el.get_attribute("data-content") {
                                if attr == content {
                                    data_tx
                                        .send(Msg::ContentChanged(content.clone(), el))
                                        .await
                                        .unwrap();
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }
                });
                false
            }
            Msg::Clicked => true,
            Msg::ContentChanged(content, el) => {
                dbg!("hey");
                self.0 = content;
                let options = Options::empty();
                let parser = Parser::new_ext(&self.0, options);
                let mut md = String::new();
                html::push_html(&mut md, parser);
                el.set_inner_html(&md);
                true
            }
        }
    }
}

impl Render<Msg, ()> for Model {
    fn render<'a>(
        &self,
        ctx: &mut RenderContext<'a>,
        data_tx: Sender<Msg>,
        self_tx: Sender<()>,
        root_tx: Sender<bool>,
    ) -> Node<'a> {
        let bump = ctx.bump;
        use dodrio::bumpalo as bp;
        let content = bp::format!(in bump, "{}", self.0);

        use dodrio::builder::*;

        dodrio!(bump,
            <div class="card">
                <div class="card-content">
                {
                    div(bump)
                    .attr("data-content", content.into_bump_str())
                    .attr("class", "content is-unselectable")
                    .finish()
                }
                </div>
                <div class="card-footer">
                    <div class="card-footer-item">
                        <button class="button" onclick={move |_, _, _| {
                            let mut data_tx = data_tx.clone();
                            let task = async move {
                                data_tx.send(Msg::Clicked).await.unwrap();
                            };
                            spawn_local(task);
                        }}>
                            "this is awesome"
                        </button>
                    </div>
                </div>
            </div>)
    }
}
