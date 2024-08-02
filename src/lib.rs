mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()));
}

fn dcmnt() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    return document;
}

#[wasm_bindgen]
pub fn greet() -> Result<(), JsValue> {
    let render: Document = dcmnt();
    let body: HtmlElement = render.body().expect("document should have a body");
    let container = body.query_selector(".container").unwrap().expect("");
    let i: u32 = 1;
    for _n in i..100 {
        let val: Element = render.create_element("p")?;
        val.set_text_content(Some("abc"));
        container.append_child(&val)?;
    }

    Ok(())
}
