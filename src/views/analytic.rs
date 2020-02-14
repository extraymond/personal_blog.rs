use crate::component::{article, chart};
use dodrio_ext::prelude::*;

#[derive(Default)]
pub struct Model {
    pub article: Option<Entity<article::Model, article::Msg, ()>>,
    pub chart: Option<Entity<chart::Model, chart::Msg, ()>>,
}

pub enum Msg {
    GetArticle(article::Model, Sender<bool>),
    GetChart(chart::Model, Sender<bool>),
}

impl Component<Msg, ()> for Model {
    fn new(_: Sender<bool>) -> Self {
        Self::default()
    }

    fn mounted(tx: Sender<Msg>, _: Sender<()>, root_tx: Sender<bool>) {
        let content = include_str!("../articles/01_intro.md");
        let art_comp = article::Model(content.to_string(), None);
        let chart_comp = chart::Model::default();
        let mut tx_clone = tx.clone();
        spawn_local(async move {
            tx_clone
                .send(Msg::GetArticle(art_comp, root_tx.clone()))
                .await
                .unwrap();
            tx_clone
                .send(Msg::GetChart(chart_comp, root_tx))
                .await
                .unwrap();
        });
    }

    fn update(&mut self, msg: Msg) -> bool {
        match msg {
            Msg::GetArticle(content, tx) => {
                let ent = Entity::new(content, tx);
                self.article.replace(ent);
            }
            Msg::GetChart(content, tx) => {
                let ent = Entity::new(content, tx);
                self.chart.replace(ent);
            }
        }
        true
    }
}

impl Render<Msg, ()> for Model {
    fn render<'a>(
        &self,
        ctx: &mut RenderContext<'a>,
        _: Sender<Msg>,
        _: Sender<()>,
        _: Sender<bool>,
    ) -> Node<'a> {
        let bump = ctx.bump;
        let article_view = self
            .article
            .as_ref()
            .map(|article| <_ as dodrio::Render>::render(&article, ctx));
        let chart_view = self
            .chart
            .as_ref()
            .map(|chart| <_ as dodrio::Render>::render(&chart, ctx));
        dodrio!(bump,
            <div class="columns">
                <div class="column is-6 card">{ article_view }</div>
                <div class="column is-6 card">{ chart_view }</div>
            </div>
        )
    }
}
