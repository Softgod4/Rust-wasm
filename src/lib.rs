mod utils;
mod task_inner;
mod tasks;
pub mod r#macro;
pub mod dom;
pub mod input;
mod closure;

use wasm_bindgen::prelude::*;
use web_sys::{ HtmlElement, HtmlInputElement };

#[wasm_bindgen]
extern "C" {
	fn alert(s: &str);
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

// main function
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
	// event-listener
	let add_closure: Closure<dyn FnMut(HtmlElement)> = Closure::<
		dyn FnMut(_)
	>::new(move |_: web_sys::HtmlElement| {
		let input_value: String = dom::Dom
			::get_doc()
			.get_element_by_id("input-task")
			.unwrap()
			.dyn_into::<HtmlInputElement>()
			.unwrap()
			.value();

		if !input::Method::value_test(input_value.clone()) {
			let _ = tasks::Method::create_task(input_value.clone());
		} else {
			let _ = input::Method::not_valide();
		}
	});

	// event-listener
	let delete_closure: Closure<dyn FnMut(HtmlElement)> = Closure::<
		dyn FnMut(_)
	>::new(move |_: web_sys::HtmlElement| {
		let _ = tasks::Method::delete_task();
	});

	// buttons
	let _button: Result<(), JsValue> = dom::Dom
		::get_doc()
		.get_element_by_id("add-task")
		.unwrap()
		.add_event_listener_with_callback(
			"click",
			add_closure.as_ref().unchecked_ref()
		);

	let _delete_btn: Result<(), JsValue> = dom::Dom
		::get_doc()
		.get_element_by_id("remove-all")
		.unwrap()
		.add_event_listener_with_callback(
			"click",
			delete_closure.as_ref().unchecked_ref()
		);

	add_closure.forget();
	delete_closure.forget();
	Ok(())
}
