use super::super::component::{article, blog_page::PROJECT_DIR};
use dodrio_ext::prelude::*;
use include_dir::*;
use uuid::Uuid;

#[derive(Default)]
pub struct Model {
    pub idx: usize,
    pub tabs: Vec<Entity<article::Model, article::Msg, ()>>,
}

pub enum Msg {
    Init(Sender<bool>),
    SwitchTab(usize),
}

impl Component<Msg, ()> for Model {
    fn new(root_tx: mpsc::UnboundedSender<bool>) -> Self {
        Self::default()
    }

    fn mounted(mut data_tx: Sender<Msg>, _self_tx: Sender<()>, root_tx: Sender<bool>) {
        spawn_local(async move {
            data_tx.send(Msg::Init(root_tx)).await;
        });
    }

    fn update(&mut self, msg: Msg) -> bool {
        match msg {
            Msg::Init(tx) => {
                let glob = "**/*.md";
                for entry in PROJECT_DIR.find(glob).unwrap() {
                    if let DirEntry::File(file) = entry {
                        let md = file.contents_utf8().unwrap();
                        let title = file
                            .path()
                            .file_name()
                            .map(|name| name.to_os_string())
                            .map(|name| name.into_string().ok())
                            .flatten()
                            .unwrap();
                        let article = article::Model::new_with_id(title, md.to_string());
                        let ent = Entity::new(article, tx.clone());
                        self.tabs.push(ent);
                    }
                }
                true
            }
            Msg::SwitchTab(idx) => {
                self.idx = idx;
                true
            }
        }
    }
}

impl Render<Msg, ()> for Model {
    fn render<'a>(
        &self,
        ctx: &mut RenderContext<'a>,
        data_tx: mpsc::UnboundedSender<Msg>,
        self_tx: mpsc::UnboundedSender<()>,
        root_tx: mpsc::UnboundedSender<bool>,
    ) -> Node<'a> {
        let bump = ctx.bump;
        use dodrio::{builder::*, bumpalo::format as bf};

        let inner = self
            .tabs
            .iter()
            .clone()
            .enumerate()
            .map(|(idx, _)| {
                let tx = data_tx.clone();
                let inner = bf!(in bump, "no: {}", idx);
                dodrio!(bump,
                    <li class={ if idx == self.idx {
                        "is-active"
                    } else {
                        ""
                    }}>
                    <a onclick={
                        move |_, _, _| {
                        let mut tx = tx.clone();
                        let task = async move {
                            tx.send(Msg::SwitchTab(idx)).await.unwrap();
                        };
                        spawn_local(task);
                    }}>
                    { text(inner.into_bump_str()) }
                    </a>
                    </li>
                )
            })
            .collect::<Vec<_>>();

        let selected_view = self
            .tabs
            .iter()
            .enumerate()
            .map(|(idx, tab)| {
                dodrio!(bump,
                    <div style={ if self.idx != idx  {
                        "display: none"
                    } else {""}}>
                    {<_ as dodrio::Render>::render(&tab, ctx)}
                    </div>
                )
            })
            .collect::<Vec<_>>();

        dodrio!(bump,
            <div class="card">
            <div class="tabs is-centered">
            <ul>{ inner }</ul>
            </div>
            { selected_view }
            </div>
        )
    }
}
