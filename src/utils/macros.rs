#[macro_export]
macro_rules! required_var {
	($name: expr) => {
		std::env::var($name).wrap_err_with(|| format!("Couldn't find {} in environment!", $name))?
	};
}
