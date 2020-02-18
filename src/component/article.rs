use dodrio::builder::*;
use dodrio_ext::prelude::*;
use futures_timer::Delay;
use pulldown_cmark::{html, Options, Parser};
use std::time::Duration;
use uuid::Uuid;

#[derive(Default)]
pub struct Model(pub String, pub String, pub String);

impl Model {
    pub fn new_with_id(title: String, content: String) -> Self {
        Self(title, content, Uuid::new_v4().to_string())
    }
}

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
                let content = self.1.clone();
                let uid = self.2.clone();
                spawn_local(async move {
                    let mut found = false;
                    while found == false {
                        Delay::new(Duration::from_micros(500)).await;
                        let doc = web_sys::window().unwrap().document().unwrap();
                        let id = &uid.clone();
                        if let Some(node) = doc
                            .query_selector_all(&format!("[content='{}']", id.clone()))
                            .map(|nodes| nodes.get(0))
                            .ok()
                            .flatten()
                        {
                            data_tx
                                .send(Msg::ContentChanged(content.clone(), node.unchecked_into()))
                                .await
                                .unwrap();
                            found = true;
                        }
                    }
                });
                true
            }
            Msg::ContentChanged(content, el) => {
                self.1 = content;
                let options = Options::empty();
                let parser = Parser::new_ext(&self.1, options);
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
        let id = bp::format!(in bump, "{}", self.2);

        dodrio!(bump,
            <div class="card">
                <div class="card-header">
                    <div class="card-header-title">
                        <p class="is-capitalized">{ vec![text(bp::format!(in bump, "{}", self.0).into_bump_str())]}</p>
                    </div>
                </div>
                <div class="card-content">
                    <section class="section">
                    <div class="content" data-content=id.into_bump_str()></div>
                    </section>
                </div>
            </div>)
    }
}
