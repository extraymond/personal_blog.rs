use dodrio_ext::prelude::*;
use include_dir::{include_dir, Dir, DirEntry};
type ArticleEnt = Entity<super::article::Model, super::article::Msg, ()>;
use uuid::Uuid;
use dodrio::{bumpalo::format as bf, builder::*};

pub const PROJECT_DIR: Dir = include_dir!("src/articles");
pub enum Msg {
    AddContent(Sender<bool>),
    AddPager(Sender<Msg>, Sender<bool>),
    Prev,
    Next,
    Goto(usize)
}

#[derive(Default)]
pub struct Model {
    articles: Vec<ArticleEnt>,
    page_id: usize,
    size: usize,
    pager: Option<Entity<PagerModel, PagerMsg, ()>>,
}

impl Component<(), Msg> for Model {
    fn new(mut root_tx: Sender<bool>) -> Self {
        spawn_local(async move {
            root_tx.send(true).await.unwrap();
        });

        Model {
            size: 5,
            ..Model::default()
        }
    }

    fn mounted(_: Sender<()>, mut ent_tx: Sender<Msg>, root_tx: Sender<bool>) {
        spawn_local(async move {
            ent_tx.send(Msg::AddContent(root_tx.clone())).await.unwrap();
            ent_tx
                .send(Msg::AddPager(ent_tx.clone(), root_tx))
                .await
                .unwrap();
        });
    }

    fn update_el(&mut self, msg: Msg) -> bool {
        match msg {
            Msg::AddContent(root_tx) => {
                let glob = "**/*.md";
                for entry in PROJECT_DIR.find(glob).unwrap() {
                    log::info!("{:?}", &entry);
                    if let DirEntry::File(file) = entry {
                        let title = file.path().file_stem().map(|name| name.to_os_string()).map(|name| name.into_string().ok()).flatten().unwrap();
                        let md = file.contents_utf8().unwrap();
                        let uid = Uuid::new_v4().to_string();
                        let article = super::article::Model(title, md.to_string(), uid);
                        let ent = Entity::new(article, root_tx.clone());
                        self.articles.push(ent);
                    }
                }
                true
            }
            Msg::AddPager(self_tx, root_tx) => {
                let pager = Entity::new(
                    PagerModel {
                        current: 0,
                        size: self.size,
                        total: self.articles.len(),
                        parent_tx: Some(self_tx),
                    },
                    root_tx.clone(),
                );
                self.pager.replace(pager);
                true
            }
            Msg::Prev => {
                self.page_id -= 1;
                true
            }
            Msg::Next => {
                self.page_id += 1;
                true
            }
            Msg::Goto(idx) => {
                self.page_id = idx;
                true
            }
        }
    }
}

impl Render<(), Msg> for Model {
    fn render<'a>(
        &self,
        ctx: &mut RenderContext<'a>,
        _: Sender<()>,
        _: Sender<Msg>,
        _: Sender<bool>,
    ) -> Node<'a> {
        let bump = ctx.bump;
        let view = self
            .articles
            .iter()
            .enumerate()
            .map(|(idx, content)| 
            {
                if (&idx >= &(self.size*self.page_id)) && (&idx < &((1+self.page_id) * self.size)) {
                    dodrio!(bump, <div style="padding-bottom: 1rem">{<_ as dodrio::Render>::render(content, ctx)}</div>)

                } else {
                    dodrio!(bump, <div style="padding-bottom: 1rem; display: none">{<_ as dodrio::Render>::render(content, ctx)}</div>)

                }
            }
        )
        .collect::<Vec<_>>();
        let pager = self
            .pager
            .as_ref()
            .map(|node| <_ as dodrio::Render>::render(&node, ctx));
        let header = dodrio!(bump,
            <nav class="level">
                <div class="level-left">
                    <div class="level-item"></div>
                    <strong class="subtitle">"A blog build using wasm/rust."</strong>
                </div>
                <div class="level-right">
                    <div class="level-item">
                        <div class="tabs">
                            <ul>
                                <li class="is-active"><a>"Articles"</a></li>
                                <li><a href="https://github.com/extraymond/">"Contact"</a></li>
                            </ul>
                        </div>
                    </div>
                </div>
            </nav>
        );
        dodrio!(bump,
        <div style="padding: 2rem; min-height: 100vh">
            { header }
            { view }
            { pager }
            </div>
        )
    }
}

#[derive(Default)]
pub struct PagerModel {
    current: usize,
    total: usize,
    size: usize,
    parent_tx: Option<Sender<Msg>>,
}

pub enum PagerMsg {
    Prev,
    Next,
    Goto(usize),
}

impl Component<PagerMsg, ()> for PagerModel {
    fn new(root_tx: mpsc::UnboundedSender<bool>) -> Self {
        Self::default()
    }

    fn update(&mut self, msg: PagerMsg) -> bool {
        match msg {
            PagerMsg::Prev => {
                if self.current > 0 {
                    self.current -= 1;
                    if let Some(tx) = &self.parent_tx {
                        let mut tx = tx.clone();
                        spawn_local(async move { tx.send(Msg::Prev).await.unwrap() });
                    }
                    true
                } else {
                    false
                }
            }
            PagerMsg::Next => {
                if self.current < (self.total / self.size) {
                    self.current += 1;
                    if let Some(tx) = &self.parent_tx {
                        let mut tx = tx.clone();
                        spawn_local(async move { tx.send(Msg::Next).await.unwrap() });
                    }
                    true
                } else {
                    false
                }
            }
            PagerMsg::Goto(idx) => {
                self.current = idx;
                 if let Some(tx) = &self.parent_tx {
                        let mut tx = tx.clone();
                        spawn_local(async move { tx.send(Msg::Goto(idx)).await.unwrap() });
                    }
                true
            }
        }
    }
}

impl Render<PagerMsg, ()> for PagerModel {
    fn render<'a>(
        &self,
        ctx: &mut RenderContext<'a>,
        data_tx: mpsc::UnboundedSender<PagerMsg>,
        self_tx: mpsc::UnboundedSender<()>,
        root_tx: mpsc::UnboundedSender<bool>,
    ) -> Node<'a> {
        let bump = ctx.bump;

        let tx_handle = data_tx.clone();
        let prev = if self.current > 0 {
            let id = bf!(in bump, "{}", self.current);
            let idx = self.current -1;
            Some(dodrio!(bump, 
                <li>
                    <a 
                    onclick={
                        let  tx = tx_handle.clone();
                        move |_, _, _| {
                            let mut tx = tx.clone();
                            spawn_local(async move {
                                tx.send(PagerMsg::Goto(idx)).await.unwrap();
                            });
                    }}
                    class="pagination-link">{ vec![text(id.into_bump_str())] }</a>
                </li>
            ))
        } else {
            None
        };

        let id = bf!(in bump, "{}", self.current+1);
        let current = dodrio!(bump, 
                <li>
                    <a 
                    class="pagination-link is-current">{ vec![text(id.into_bump_str())] }</a>
                </li>
            );
        
        let next = if self.current < (self.total / self.size) {
            let id = bf!(in bump, "{}", self.current +1 +1);
            let idx = self.current +1;
            
            Some(dodrio!(bump, 
                <li>
                    <a 
                    onclick={
                        let  tx = tx_handle.clone();
                        move |_, _, _| {
                            let mut tx = tx.clone();
                            spawn_local(async move {
                                tx.send(PagerMsg::Goto(idx)).await.unwrap();
                            });
                    }}
                    class="pagination-link">{ vec![text(id.into_bump_str())] }</a>
                </li>
            ))
        } else {
            None
        };
        


        dodrio!(bump,
        <div style="position: sticky; top: 100%">
        <nav class="pagination is-centered" role="navigation">
          <a class="pagination-previous"
          onclick={
              let tx = tx_handle.clone();
              move |_, _, _| {
                let mut tx = tx.clone();
                spawn_local(async move {
                    tx.send(PagerMsg::Prev).await.unwrap();
                });
          }}
          >"Previous"</a>
          <a class="pagination-next"
          onclick={
              let  tx = tx_handle.clone();
              move |_, _, _| {
                let mut tx = tx.clone();
                spawn_local(async move {
                    tx.send(PagerMsg::Next).await.unwrap();
                });
          }}

          >"Next page"</a>
          <ul class="pagination-list">
            { prev }
            { current }
            { next }
          </ul>
        </nav>
        </div>

        )
    }
}
