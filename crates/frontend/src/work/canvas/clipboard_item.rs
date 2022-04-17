use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "export const ClipboardItem = window.ClipboardItem")]
extern "C" {
    pub type ClipboardItem;

    #[wasm_bindgen(constructor, js_class = "ClipboardItem")]
    pub fn new(files: &js_sys::Object) -> ClipboardItem;
}
