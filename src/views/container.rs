use crate::component::article;
use crate::elements::movable_tag;
use crate::views::hetero;
use dodrio_ext::prelude::*;
#[derive(Default)]
pub struct Model {
    comps: Vec<Entity<hetero::Compat, (), ()>>,
}

pub enum Msg {
    Init(Sender<bool>),
}

impl Component<Msg, ()> for Model {
    fn new(tx: mpsc::UnboundedSender<bool>) -> Self {
        let art = Entity::new(
            article::Model(
                String::new(),
                include_str!("../articles/01_intro.md").to_string(),
                String::new(),
            ),
            tx.clone(),
        );
        let hetero_art = hetero::Compat::Article(art);
        let ent_art = Entity::new(hetero_art, tx.clone());

        let tag = Entity::new(movable_tag::Model::default(), tx.clone());
        let hetero_tag = hetero::Compat::MovingTag(tag);
        let ent_tag = Entity::new(hetero_tag, tx.clone());

        Self {
            comps: vec![ent_art, ent_tag],
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
        let raw = article::Model::default();
        // let com_raw: Box<dyn Component<article::Msg, ()>> = Box::new(raw);
        let bump = ctx.bump;
        let views = self
            .comps
            .iter()
            .map(|ent| <_ as dodrio::Render>::render(&ent, ctx))
            .collect::<Vec<_>>();

        dodrio!(bump, <div style="display: block;">{ views }</div>)
    }
}
