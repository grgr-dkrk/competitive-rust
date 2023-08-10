fn convert_case(s: &str) -> String {
	s.chars()
		.map(|c| {
			if c.is_ascii_lowercase() {
				c.to_ascii_uppercase()
			} else if c.is_ascii_uppercase() {
				c.to_ascii_lowercase()
			} else {
				c
			}
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::convert_case;

	#[test]
	fn should_be_return_correctly() {
		assert_eq!(convert_case("Hello, World!"), "hELLO, wORLD!");
		assert_eq!(convert_case("CowAbuNga!"), "cOWaBUnGA!");
	}
}
