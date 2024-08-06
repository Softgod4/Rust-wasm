pub fn get_inner(text: String) -> String {
	let inner_task_text =
		format!(r#"
    
        <article class="flex align-center justify-center">
            <p>{}</p>
        </article>
        <button>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24">
                <path d="M18 6L6 18M6 6l12 12" stroke='#B0B0B0' stroke-width='2' stroke-linecap="round" />
            </svg>
        </button>
"#, text);

	inner_task_text
}
