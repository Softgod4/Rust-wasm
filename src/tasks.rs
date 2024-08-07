use wasm_bindgen::prelude::*;
use crate::input;
use crate::task_inner;
use crate::dom;

pub(crate) struct Method {
	task_name: String,
}

impl Method {
	pub fn new(task_name: String) -> Method {
		Method { task_name }
	}

	pub fn delete_task() -> Result<(), JsError> {
		let item = dom::Dom
			::get_doc()
			.query_selector(".tasks-list__item")
			.map_err(|_| JsError::new("Query selector failed"))?
			.ok_or_else(|| JsError::new("No tasks found"))?;

		item.remove();

		Ok(())
	}

	pub fn create_task(task_name: String) -> Result<(), JsValue> {
		let _ = input::Method::delete_not_valide();
		let tasks_list: web_sys::Element = dom::Dom
			::get_body()
			.query_selector(".tasks-list")
			.unwrap()
			.expect("");

		let task: web_sys::Element = dom::Dom
			::get_doc()
			.create_element("label")?;
		task.set_class_name("tasks-list__item");
		task.set_inner_html(&task_inner::get_inner(task_name.clone()));
		tasks_list.append_child(&task)?;
		Ok(())
	}
}
