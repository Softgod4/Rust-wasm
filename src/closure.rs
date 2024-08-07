// in the future such event listener implementation will be added

use wasm_bindgen::prelude::*;
use web_sys::{ HtmlElement, HtmlInputElement };

use crate::{ dom, input, tasks };

pub(crate) struct EventListener {}

impl EventListener {
	pub fn get_input_value() {
		let _clos: Closure<dyn FnMut(HtmlElement)> = Closure::<
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

		_clos.forget();
	}

	pub fn delete_value() {
		let _clos: Closure<dyn FnMut(HtmlElement)> = Closure::<
			dyn FnMut(_)
		>::new(move |_: web_sys::HtmlElement| {
			let _ = tasks::Method::delete_task();
		});

		_clos.forget();
	}
}
