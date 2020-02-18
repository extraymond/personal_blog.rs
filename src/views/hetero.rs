use crate::component::article;
use crate::elements::movable_tag;
use dodrio_ext::prelude::*;

pub enum Compat {
    Empty,
    Article(Entity<article::Model, article::Msg, ()>),
    MovingTag(Entity<movable_tag::Model, movable_tag::Msg, ()>),
}

impl Component<(), ()> for Compat {
    fn new(root_tx: mpsc::UnboundedSender<bool>) -> Self {
        Compat::Empty
    }
}

impl Render<(), ()> for Compat {
    fn render<'a>(
        &self,
        ctx: &mut RenderContext<'a>,
        data_tx: mpsc::UnboundedSender<()>,
        self_tx: mpsc::UnboundedSender<()>,
        root_tx: mpsc::UnboundedSender<bool>,
    ) -> Node<'a> {
        let bump = ctx.bump;
        let view = match self {
            Compat::Empty => None,
            Compat::Article(ent) => Some(<_ as dodrio::Render>::render(&ent, ctx)),
            Compat::MovingTag(ent) => Some(<_ as dodrio::Render>::render(&ent, ctx)),
        };
        dodrio!(bump, <div style="display: block">{ view }</div>)
    }
}
