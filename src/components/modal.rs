use yew::Html;

mod modal_plain;
pub(crate) use modal_plain::ModalPlain;

mod modal_ok;
pub(crate) use modal_ok::ModalOk;

const DEFAULT_COLOR: &str = "lightblue";

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum ModalMsg {
    StringList(Vec<String>),
    String(String),
    Html(Html),
}
