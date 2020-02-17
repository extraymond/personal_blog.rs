use dodrio_ext::prelude::*;
use futures::future::{AbortHandle, AbortRegistration, Abortable};
use thiserror::Error;

use futures_timer::Delay;
use std::time::Duration;

#[derive(Default)]
pub struct Model {
    editable: bool,
    pos: i32,
    width: Option<i32>,
}

pub enum Msg {
    DragStart(web_sys::Event),
    DragEnd(web_sys::Event),
    Moving(web_sys::Event),
}

impl<B> Component<Msg, B> for Model {
    fn new(_: Sender<bool>) -> Self {
        Model::default()
    }

    fn update(&mut self, msg: Msg) -> bool {
        match msg {
            Msg::DragStart(e) => {
                if self.width.is_none() {
                    let target = e.current_target().unwrap();
                    let el: web_sys::Element = target.unchecked_into();
                    let rect = el.get_bounding_client_rect();
                    self.width.replace(rect.width() as i32);
                    log::info!("{:?}", self.width);
                }
                self.editable = true;
                false
            }
            Msg::DragEnd(_) => {
                self.editable = false;
                false
            }
            Msg::Moving(e) => {
                if self.editable {
                    let target = e.current_target().unwrap();
                    let el: web_sys::Element = target.unchecked_into();
                    let rect = el.get_bounding_client_rect();
                    let e: web_sys::MouseEvent = e.unchecked_into();
                    let mouse_pos = e.client_x() - rect.left() as i32;
                    self.pos = mouse_pos - self.width.unwrap() / 2;
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl<B> Render<Msg, B> for Model {
    fn render<'a>(
        &self,
        ctx: &mut RenderContext<'a>,
        data_tx: Sender<Msg>,
        _: Sender<B>,
        _: Sender<bool>,
    ) -> Node<'a> {
        let bump = ctx.bump;
        let tx_handle = data_tx.clone();
        dodrio!(bump,
            <div onmousemove={
                let tx = tx_handle.clone();
                move |_, _, e| {
                let mut tx = tx.clone();
                let fut = async move {
                    e.stop_propagation();
                    tx.send(Msg::Moving(e)).await.unwrap();
                };
                spawn_local(fut);
            }}

            onmouseup={
                let tx = tx_handle.clone();
                move |_, _, e| {
                let mut tx = tx.clone();
                let fut = async move {
                    Delay::new(Duration::from_micros(200)).await;
                    tx.send(Msg::DragEnd(e)).await.unwrap();
                };
                spawn_local(fut);
            }}

            onmouseleave={
                let tx = tx_handle.clone();
                move |_, _, e| {
                let mut tx = tx.clone();
                let fut = async move {
                    Delay::new(Duration::from_micros(200)).await;
                    tx.send(Msg::DragEnd(e)).await.unwrap();
                };
                spawn_local(fut);
            }}

            class="box">
                <div
                onmousedown={
                    let tx = tx_handle.clone();
                    move |_, _, e| {
                    let mut tx = tx.clone();
                    let fut = async move {
                        tx.send(Msg::DragStart(e)).await.unwrap();
                    };
                    spawn_local(fut);
                }}
                onmouseup={
                    let tx = tx_handle.clone();
                    move |_, _, e| {
                    let mut tx = tx.clone();
                    let fut = async move {
                        Delay::new(Duration::from_micros(200)).await;

                        tx.send(Msg::DragEnd(e)).await.unwrap();
                    };
                    spawn_local(fut);
                }}

                class="tag is-dark is-unselectable"
                style={ format!("position: relative; left: {};", self.pos) }>
                    {
                        let nodes = vec![dodrio::builder::text(dodrio::bumpalo::format!(in bump, "{}", self.pos).into_bump_str())];
                        nodes
                     }
                </div>
            </div>)
    }
}

#[derive(Error, Debug)]
pub enum LibError {
    #[error("Error generated from Js: {0}")]
    FromJs(String),
    #[error("Nothing can be found...")]
    Missing,
}

impl From<JsValue> for LibError {
    fn from(target: JsValue) -> Self {
        LibError::FromJs(target.as_string().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    fn inject_css() -> Result<(), LibError> {
        let window = web_sys::window().ok_or(LibError::Missing)?;
        let document = window.document().ok_or(LibError::Missing)?;
        let head = document.head().ok_or(LibError::Missing)?;
        let link = document.create_element("link")?;
        let link: web_sys::HtmlLinkElement = link.unchecked_into();
        link.set_rel("stylesheet");
        link.set_href("https://cdn.jsdelivr.net/npm/bulma@0.8.0/css/bulma.min.css");
        let link: web_sys::Node = link.unchecked_into();
        let head: web_sys::Element = head.unchecked_into();
        head.append_with_node_1(&link)?;
        Ok(())
    }

    #[wasm_bindgen_test]
    fn test_ui() {
        console_error_panic_hook::set_once();
        femme::start(log::LevelFilter::Debug).unwrap();
        inject_css().expect("it can't work");
        let mut hub = MessageHub::new();
        hub.bind_root_el::<Model, Msg, ()>(
            Model {
                pos: 0,
                ..Model::default()
            },
            None,
        );
        hub.mount_hub_rx();
    }
}
