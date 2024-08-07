use once_cell::sync::Lazy;
use regex::Regex;
use wasm_bindgen::JsValue;
use web_sys::Element;
use crate::log;

use crate::{ dom, r#macro };

pub(crate) struct Method {
	input_value: String,
}

impl Method {
	pub fn new(input_value: String) -> Method {
		Method { input_value }
	}

	pub fn value_test(input_value: String) -> bool {
		let copy_value: String = input_value.to_string();
		static RE: Lazy<Regex> = Lazy::new(||
			Regex::new(r"(^$|[^a-zA-Z\s])").unwrap()
		);
		RE.is_match(&copy_value)
	}

	pub fn not_valide() -> Result<(), JsValue> {
		let not_valide_message: web_sys::Element = dom::Dom
			::get_body()
			.query_selector(".todo")
			.unwrap()
			.expect("");

		match dom::Dom::get_body().query_selector(".not-valid") {
			Ok(Some(_)) => {}
			Ok(None) => {
				r#macro::log!("not exists");
				let val: Element = dom::Dom::get_doc().create_element("p")?;
				val.set_text_content(Some("Not valide input!"));
				val.set_class_name("not-valid");
				not_valide_message.append_child(&val)?;
			}
			Err(e) => {
				r#macro::log!("{:?}", e);
			}
		}
		Ok(())
	}

	pub fn delete_not_valide() -> Result<(), JsValue> {
		match dom::Dom::get_body().query_selector(".not-valid") {
			Ok(Some(_)) => {
				let valid = dom::Dom
					::get_body()
					.query_selector(".not-valid")
					.unwrap()
					.expect("");
				valid.remove();
			}
			Ok(None) => {
				r#macro::log!("not exists");
			}
			Err(e) => {
				r#macro::log!("{:?}", e);
			}
		}
		Ok(())
	}
}
