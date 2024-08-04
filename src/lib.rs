mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{js_sys::JsString, Document, Element, HtmlElement};

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
    let container: Element = body.query_selector(".container").unwrap().expect("");

    // event-listener
    let closure: Closure<dyn FnMut(HtmlElement)> =
        Closure::<dyn FnMut(_)>::new(move |event: web_sys::HtmlElement| {
            let input = dcmnt()
                .get_element_by_id("input-task")
                .unwrap()
                .dyn_into::<HtmlElement>();
            console_log!("{:?}", input);
        });

    let button = render
        .get_element_by_id("add-task")
        .unwrap()
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());

    closure.forget();

    // testing
    let i: u32 = 1;
    for _n in i..5 {
        let val: Element = render.create_element("p")?;
        val.set_text_content(Some("abc"));
        container.append_child(&val)?;
    }

    Ok(())
}
