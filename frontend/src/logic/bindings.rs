use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use js_sys::Promise;

// For screenshot
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = html2canvas)]
    pub fn html2canvas(element: &HtmlElement, options: &JsValue) -> Promise;
}
