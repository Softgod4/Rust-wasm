macro_rules! log {
	($($t:tt)*) => (log(&format_args!($($t)*).to_string()));
}

pub(crate) use log;