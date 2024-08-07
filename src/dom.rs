use web_sys::{Document, HtmlElement};

pub(crate) struct Dom {}

impl Dom {
	pub fn get_doc() -> Document {
		let window = web_sys::window().expect("no global `window` exists");
		let document = window
			.document()
			.expect("should have a document on window");
		return document;
	}

	pub fn get_body() -> HtmlElement {
		let body: web_sys::HtmlElement = Dom::get_doc().body().expect("document dsnt have a body");
		body
	}
}

// let body: HtmlElement = dom
// 			::doc()
// 			.body()
// 			.expect("document should have a body");
