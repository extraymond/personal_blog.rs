use dodrio_ext::prelude::*;

#[derive(Default)]
pub struct Model {}

impl Component<(), ()> for Model {
    fn new(root_tx: Sender<bool>) -> Self {
        Model::default()
    }
}

impl Render<(), ()> for Model {
    fn render<'a>(
        &self,
        ctx: &mut RenderContext<'a>,
        data_tx: mpsc::UnboundedSender<()>,
        self_tx: mpsc::UnboundedSender<()>,
        root_tx: mpsc::UnboundedSender<bool>,
    ) -> Node<'a> {
        let bump = ctx.bump;
        dodrio!(bump, <div></div>)
    }
}
