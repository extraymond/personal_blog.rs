use dodrio_ext::prelude::*;
use include_dir::{include_dir, Dir, DirEntry};
type ArticleEnt = Entity<super::article::Model, super::article::Msg, ()>;

const PROJECT_DIR: Dir = include_dir!("src/articles");
pub enum Msg {
    AddContent(Sender<bool>),
}

#[derive(Default)]
pub struct Model {
    articles: Vec<ArticleEnt>,
}

impl Component<(), Msg> for Model {
    fn new(mut root_tx: Sender<bool>) -> Self {
        spawn_local(async move {
            root_tx.send(true).await.unwrap();
        });
        Model::default()
    }

    fn mounted(_: Sender<()>, mut ent_tx: Sender<Msg>, root_tx: Sender<bool>) {
        spawn_local(async move {
            ent_tx.send(Msg::AddContent(root_tx)).await.unwrap();
            log::info!("we need to add some nodes");
        });
    }

    fn update_el(&mut self, msg: Msg) -> bool {
        match msg {
            Msg::AddContent(root_tx) => {
                let glob = "**/*.md";
                for entry in PROJECT_DIR.find(glob).unwrap() {
                    if let DirEntry::File(file) = entry {
                        let md = file.contents_utf8().unwrap();
                        let article = super::article::Model(md.to_string());
                        let ent = Entity::new(article, root_tx.clone());
                        self.articles.push(ent);
                    }
                }
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
            .map(|content: &ArticleEnt| <_ as dodrio::Render>::render(content, ctx))
            .collect::<Vec<Node<'a>>>();
        dodrio!(bump, <div class="box">{ view }</div>)
    }
}
