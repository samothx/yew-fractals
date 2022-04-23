use yew::{Component, Context, Html, html, NodeRef, Properties};
use super::{ModalMsg, DEFAULT_COLOR};
use web_sys::HtmlDivElement;

// TODO: pretty up modal

pub(crate) struct ModalPlain {
    modal_ref: NodeRef,
    content_ref: NodeRef,
}

impl Component for ModalPlain {
    type Message = ();
    type Properties = ModalPlainProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            modal_ref: NodeRef::default(),
            content_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        info!("ModalPlain::view called");
        let class = if ctx.props().visible {
            "modal modal_visible"
        } else {
            "modal modal_invisible"
        };

        html![
            <div class={class} ref={self.modal_ref.clone()} >
                <div class="modal-content" ref={self.content_ref.clone()}>
                    <h2>{ctx.props().title.as_str()}</h2>
                    {   match &ctx.props().message {
                            ModalMsg::String(msg) => html![<p>{msg}</p>],
                            ModalMsg::StringList(msg_list) => {
                                msg_list.iter().map(|msg| {
                                    html!{<p>{ msg }</p>}
                                }).collect::<Html>() },
                            ModalMsg::Html(html) => {
                                html.clone()
                            }
                        }
                    }
                </div>
            </div>
        ]
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        info!("ModalPlain::rendered: bgcolor {:?}", ctx.props().background_color.as_ref());
        let _res = self.content_ref.cast::<HtmlDivElement>().expect("Could not cast to HtmlDivElement")
            .set_attribute("style",format!("background-color:{};",
                   if let Some(color) = ctx.props().background_color.as_ref() {
                       color
                   } else {
                       DEFAULT_COLOR
                   }).as_str());
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ModalPlainProps {
    pub visible: bool,
    pub title: String,
    pub message: ModalMsg,
    pub background_color: Option<String>,
    // pub btn_text: String
}
