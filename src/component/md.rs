use dodrio_ext::prelude::*;

#[derive(Default)]
pub struct Model(pub String, pub bool);
pub enum Msg {
    Toggle,
}

impl Component<(Msg), ()> for Model {
    fn new(_: Sender<bool>) -> Self {
        Model(String::new(), false)
    }

    fn mounted(_: Sender<Msg>, _: Sender<()>, _: Sender<bool>) {}

    fn update(&mut self, _: Msg) -> bool {
        self.1 = !self.1;
        true
    }
}

impl Render<Msg, ()> for Model {
    fn render<'a>(
        &self,
        ctx: &mut RenderContext<'a>,
        data_tx: Sender<Msg>,
        _: Sender<()>,
        _: Sender<bool>,
    ) -> Node<'a> {
        use dodrio::{builder::text, bumpalo::format as bf};
        let bump = ctx.bump;
        dodrio!(bump,
            <div style={ if self.1 { "" } else { "display: none" }}>
                <div class="box has-background-dark has-text-white" style="white-space: pre-wrap">{ vec![text(bf!(in bump, "{}", self.0).into_bump_str())] }</div>
            </div>
        )
    }
}
