use yew::{Component, Context, Html, html, NodeRef, Properties};

pub(crate) struct ModalPlain {
    modal_ref: NodeRef,
    msg_ref: NodeRef,
    title_ref: NodeRef,
}

impl Component for ModalPlain {
    type Message = ();
    type Properties = ModalPlainProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            modal_ref: NodeRef::default(),
            msg_ref: NodeRef::default(),
            title_ref: NodeRef::default(),
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
                <div class="modal-content">
                    <h2 ref={self.title_ref.clone()}>{ctx.props().title.as_str()}</h2>
                    <p ref={self.msg_ref.clone()}>{ctx.props().message.as_str()}</p>
                </div>
            </div>
        ]
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ModalPlainProps {
    pub visible: bool,
    pub title: String,
    pub message: String,
    // pub btn_text: String
}
