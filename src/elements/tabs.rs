use dodrio_ext::prelude::*;

#[derive(Default)]
pub struct Model {
    pub idx: usize,
    pub tabs: Vec<String>,
}

pub enum Msg {
    SwitchTab(usize),
}

impl Component<Msg, ()> for Model {
    fn new(root_tx: mpsc::UnboundedSender<bool>) -> Self {
        Self::default()
    }

    fn update(&mut self, msg: Msg) -> bool {
        match msg {
            Msg::SwitchTab(idx) => {
                self.idx = idx;
                true
            }
            _ => false,
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
        use dodrio::{
            builder::*,
            bumpalo::{collections::Vec as BVec, format as bf},
        };

        let mut inner = BVec::with_capacity_in(self.tabs.len(), bump);
        let data_tx_handle = data_tx.clone();
        inner.extend(
            self.tabs
                .iter()
                .enumerate()
                .map(|(idx, tab)| {
                    let inner = bf!(in bump, "{}", tab);
                    dodrio!(bump,
                        <li class={
                            if idx == self.idx {
                                "is-active"
                            } else {
                                ""
                            }
                        }>
                        <a onclick={
                            let tx = data_tx_handle.clone();
                            move |_, _, _| {
                                log::info!("{:?}", idx);
                                let mut tx = tx.clone();
                            let task = async move {
                                tx.send(Msg::SwitchTab(idx)).await.unwrap();
                            };
                            spawn_local(task);
                        }}>{ text(inner.into_bump_str()) }</a>
                        </li>
                    )
                })
                .collect::<Vec<Node<'a>>>(),
        );

        dodrio!(bump,
            <div class="tabs">
                <ul>{ inner }</ul>
            </div>
        )
    }
}
