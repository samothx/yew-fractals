use crate::work::colors::{ColorRange, HslRange};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use yew::prelude::*;
use yew::{Component, Context, Html};

pub struct EditColorConfig {
    container_ref: NodeRef,
}

impl Component for EditColorConfig {
    type Message = ();
    type Properties = EditColorCfgProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            container_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cntr_class = if ctx.props().edit_mode {
            "edit_cntr_visible"
        } else {
            "edit_cntr_hidden"
        };

        html![
            <div class={cntr_class} id="color_edit_cntr" ref={self.container_ref.clone()}>
            </div>
        ]
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct ColorCfg {
    palettes: BTreeMap<String, ColorRange>,
}

impl ColorCfg {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            palettes: BTreeMap::new(),
        }
    }

    pub fn get<'a>(&'a self, name: &str) -> Option<&'a ColorRange> {
        self.palettes.get(name)
    }
}

impl Default for ColorCfg {
    fn default() -> Self {
        let palette = ColorRange::Hsl(HslRange::default());
        let mut cfg = Self {
            palettes: BTreeMap::new(),
        };

        cfg.palettes.insert("default".to_owned(), palette);
        cfg
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct EditColorCfgProps {
    pub config: ColorCfg,
    pub edit_mode: bool,
    // pub cb_saved: Callback<ColorCfg>,
    // pub cb_canceled: Callback<()>,
}
