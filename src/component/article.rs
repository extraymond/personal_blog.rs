use dodrio_ext::prelude::*;
use futures::channel::oneshot;
use futures_timer::Delay;
use gloo::events;
use pulldown_cmark::{html, Options, Parser};
use std::time::Duration;

type EntMd = Entity<super::md::Model, super::md::Msg, ()>;

#[derive(Default)]
pub struct Model(pub String, pub Option<EntMd>);

pub enum Msg {
    Init(Sender<Msg>, Sender<bool>),
    // Clicked,
    ContentChanged(String, web_sys::Element),
}

impl Component<Msg, ()> for Model {
    fn new(tx: Sender<bool>) -> Self {
        Model::default()
    }

    fn mounted(data_tx: Sender<Msg>, _: Sender<()>, root_tx: Sender<bool>) {
        spawn_local(async move {
            data_tx
                .clone()
                .send(Msg::Init(data_tx, root_tx))
                .await
                .unwrap();
        });
    }

    fn update(&mut self, msg: Msg) -> bool {
        match msg {
            Msg::Init(mut data_tx, root_tx) => {
                let content = self.0.clone();
                let ent_md = Entity::new(super::md::Model(content.clone(), true), root_tx);
                self.1 = Some(ent_md);
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

    fn update_el(&mut self, msg: ()) -> bool {
        if let Some(ent) = &self.1 {
            let mut tx = ent.data_tx.clone();
            let fut = async move {
                tx.send(super::md::Msg::Toggle).await.unwrap();
            };
            spawn_local(fut);
            true
        } else {
            false
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

        let md_view = self
            .1
            .as_ref()
            .map(|view| <_ as dodrio::Render>::render(&view, ctx));

        let toggle_text = self
            .1
            .as_ref()
            .map(|ent| ent.data.try_lock().unwrap().1)
            .map(|val| {
                if val {
                    text("toggle markdown off")
                } else {
                    text("toggle markdown on")
                }
            });

        dodrio!(bump,
            <div class="card">
                <div class="card-content">
                {
                    div(bump)
                    .attr("data-content", content.into_bump_str())
                    .attr("class", "content is-unselectable")
                    .finish()
                }
                { md_view }
                </div>
                <div class="card-footer">
                    <div class="card-footer-item">
                        <button class="button" onclick={move |_, _, _| {
                            let mut self_tx =self_tx.clone();
                            let task = async move {
                                self_tx.send(()).await.unwrap();
                            };
                            spawn_local(task);
                        }}> { toggle_text }
                        </button>
                    </div>
                </div>
            </div>)
    }
}
