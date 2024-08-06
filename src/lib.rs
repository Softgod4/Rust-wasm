mod utils;
mod task_inner;

use std::fmt::Error;

use once_cell::sync::Lazy;
use regex::Regex;
use wasm_bindgen::prelude::*;
use web_sys::{ Document, Element, HtmlElement, HtmlInputElement };

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

fn value_test(str: String) -> bool {
	static RE: Lazy<Regex> = Lazy::new(||
		Regex::new(r"(^$|[^a-zA-Z\s])").unwrap()
	);
	RE.is_match(&str)
}

// create a new task in todo (onclick button)
#[wasm_bindgen]
pub fn create_task(task_name: String) -> Result<(), JsValue> {
	let render: Document = dcmnt();
	let body: HtmlElement = render.body().expect("document should have a body");
	let _ = delete_not_valide();
	let tasks_list: web_sys::Element = body
		.query_selector(".tasks-list")
		.unwrap()
		.expect("");

	let task = render.create_element("label")?;
	task.set_class_name("tasks-list__item");
	task.set_inner_html(&task_inner::get_inner(task_name));
	tasks_list.append_child(&task)?;
	Ok(())
}

#[wasm_bindgen]
pub fn delete_task() -> Result<(), JsError> {
	let item = dcmnt()
		.query_selector(".tasks-list__item")
		.map_err(|_| JsError::new("Query selector failed"))?
		.ok_or_else(|| JsError::new("No tasks found"))?;

	item.remove();

	Ok(())
}

// not valide input
#[wasm_bindgen]
pub fn not_valide() -> Result<(), JsValue> {
	let render: Document = dcmnt();
	let body: HtmlElement = render.body().expect("document should have a body");
	let not_valide_message: web_sys::Element = body
		.query_selector(".todo")
		.unwrap()
		.expect("");

	match body.query_selector(".not-valid") {
		Ok(Some(_)) => {}
		Ok(None) => {
			console_log!("not exists");
			let val: Element = render.create_element("p")?;
			val.set_text_content(Some("Not valide input!"));
			val.set_class_name("not-valid");
			not_valide_message.append_child(&val)?;
		}
		Err(e) => {
			console_log!("{:?}", e);
		}
	}
	Ok(())
}

// for delete not valide input
#[wasm_bindgen]
pub fn delete_not_valide() -> Result<(), JsValue> {
	let render: Document = dcmnt();
	let body: HtmlElement = render.body().expect("document should have a body");
	match body.query_selector(".not-valid") {
		Ok(Some(_)) => {
			let valid = body.query_selector(".not-valid").unwrap().expect("");
			valid.remove();
		}
		Ok(None) => {
			console_log!("not exists");
		}
		Err(e) => {
			console_log!("{:?}", e);
		}
	}
	Ok(())
}

// main function
#[wasm_bindgen]
pub fn greet() -> Result<(), JsValue> {
	let render: Document = dcmnt();

	// event-listener
	let add_closure: Closure<dyn FnMut(HtmlElement)> = Closure::<
		dyn FnMut(_)
	>::new(move |_: web_sys::HtmlElement| {
		let input_value: String = dcmnt()
			.get_element_by_id("input-task")
			.unwrap()
			.dyn_into::<HtmlInputElement>()
			.unwrap()
			.value();

		if !value_test(input_value.clone()) {
			console_log!("{}", input_value);
			let _ = create_task(input_value);
		} else {
			let _ = not_valide();
		}
	});

	// event-listener
	let delete_closure: Closure<dyn FnMut(HtmlElement)> = Closure::<
		dyn FnMut(_)
	>::new(move |_: web_sys::HtmlElement| {
		let _ = delete_task();
	});

	// buttons
	let _button: Result<(), JsValue> = render
		.get_element_by_id("add-task")
		.unwrap()
		.add_event_listener_with_callback(
			"click",
			add_closure.as_ref().unchecked_ref()
		);

	let _delete_btn: Result<(), JsValue> = render
		.get_element_by_id("remove-all")
		.unwrap()
		.add_event_listener_with_callback(
			"click",
			delete_closure.as_ref().unchecked_ref()
		);

	add_closure.forget();
	delete_closure.forget();

	// testing
	// let i: u32 = 1;
	// for _n in i..5 {
	// 	let val: Element = render.create_element("p")?;
	// 	val.set_text_content(Some("abc"));
	// 	container.append_child(&val)?;
	// }

	Ok(())
}
