use yew::prelude::*;
use web_sys::window;

pub enum Msg {
    OkClicked
}

pub struct Disclaimer {

}

impl Component for Disclaimer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OkClicked => {
                window().expect("window not found")
                    .document().expect("document not found")
                    .get_element_by_id("mobile_disclaimer_cntr")
                    .expect("mobile disclaimer cntr not found")
                    .set_class_name("mobile_disclaimer_cntr_hidden");
               true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::OkClicked);

        html![<div class="mobile_disclaimer_cntr" id="mobile_disclaimer_cntr">
                <h1>{"Sorry - this page is currently not yet mobile friendly"}</h1>
                <p class="disclaimer_text">
                {"\
By design calculating and displaying fractals requires ample processing power and solid screen resolution.
Also using a touchscreen makes getting the events for dragging rectangles on the drawing area kind of tricky."}
                </p>
                <p class="disclaimer_text">
                    {"\
So far I have not gotten around to creating alternative layouts and solutions for small screens and touch input so this page
is best viewed on a computer."}
                </p>
                <button type="button" {onclick} class="disclaimer_button" id="no_mobile_ok">
                    {"Ok"}
                </button>
            </div>
        ]
    }
}
