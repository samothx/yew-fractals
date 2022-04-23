use yew::prelude::*;
use yew::{Component, Context, Html, html, NodeRef, Properties};
use web_sys::HtmlDivElement;
use super::{ModalMsg, DEFAULT_COLOR};

// TODO: pretty up modal


pub(crate) struct ModalOk {
    modal_ref: NodeRef,
    content_ref: NodeRef,
    title_ref: NodeRef,
}

impl Component for ModalOk {
    type Message = Msg;
    type Properties = ModalOkProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            modal_ref: NodeRef::default(),
            content_ref: NodeRef::default(),
            title_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Ok => {
                ctx.props().on_ok.emit(());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = if ctx.props().visible {
            "modal modal_visible"
        } else {
            "modal modal_invisible"
        };
        let on_ok = ctx.link().callback(|_| Msg::Ok);

        html![
            <div class={class} ref={self.modal_ref.clone()} >
                <div class="modal-content" ref={self.content_ref.clone()}>
                    <h2 ref={self.title_ref.clone()}>{ctx.props().title.as_str()}</h2>
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
                    <button class="menu_button" id="modal_ok_btn" onclick={on_ok}>
                        {"Ok"}
                    </button>
                </div>
            </div>
        ]
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        info!("ModalOk::rendered: bgcolor {:?}", ctx.props().background_color.as_ref());
        let _res = self.content_ref.cast::<HtmlDivElement>().expect("Could not cast to HtmlDivElement")
        .set_attribute("style",format!("background-color:{};",
            if let Some(color) = ctx.props().background_color.as_ref() {
                color
            } else {
                DEFAULT_COLOR
             }).as_str());
    }
}

pub enum Msg {
    Ok
}

#[derive(Properties, PartialEq, Clone)]
pub struct ModalOkProps {
    pub visible: bool,
    pub title: String,
    pub message: ModalMsg,
    pub background_color: Option<String>,
    pub on_ok: Callback<()>,
    // pub btn_text: String
}
