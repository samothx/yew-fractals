use yew::prelude::*;
use web_sys::{Element};

pub enum Msg {
    OkClicked
}

pub struct Disclaimer {
    container_ref: NodeRef,
}

impl Component for Disclaimer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            container_ref: NodeRef::default()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OkClicked => {
                self.container_ref.cast::<Element>()
                    .expect("mobile disclaimer cntr not found")
                    .set_class_name("mobile_disclaimer_cntr_hidden");
               true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::OkClicked);

        html![<div class="mobile_disclaimer_cntr" id="mobile_disclaimer_cntr" ref={self.container_ref.clone()}>
                <h1>{"Sorry - this page is currently not yet mobile friendly"}</h1>
                <p class="disclaimer_text">
                {"\
By design calculating and displaying fractals requires ample processing power and solid screen resolution."
                }
                </p>
                <p class="disclaimer_text">
                    {"\
So far I have not gotten around to creating alternative layouts and solutions for small screens so this page
is best viewed on a computer."}
                </p>
                <button type="button" {onclick} class="disclaimer_button" id="no_mobile_ok">
                    {"Ok"}
                </button>
            </div>
        ]
    }
}
