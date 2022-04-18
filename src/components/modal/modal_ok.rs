use yew::{Component, Context, Html, html, NodeRef, Properties};

pub(crate) struct ModalOk {
    modal_ref: NodeRef,
    msg_ref: NodeRef,
    title_ref: NodeRef,
    btn_ref: NodeRef
}

impl Component for ModalOk {
    type Message = ();
    type Properties = ModalOkProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self{
            modal_ref: NodeRef::default(),
            msg_ref: NodeRef::default(),
            title_ref: NodeRef::default(),
            btn_ref: NodeRef::default()

        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = if ctx.props().visible {
            "modal modal_visible"
        } else {
            "modal modal_invisible"
        };

        html![
            <div class={class} ref={self.modal_ref.clone()} >
                <div class="modal-content">
                    <span class="modal_ok_close" ref={self.btn_ref.clone()}>{"&times;"}</span>
                    <p ref={self.msg_ref.clone()}>{ctx.props().message.as_str()}</p>
                </div>
            </div>
        ]
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ModalOkProps {
    pub visible: bool,
    pub title: String,
    pub message: String,
    // pub btn_text: String
}
