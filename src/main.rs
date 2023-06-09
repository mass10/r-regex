//!
//! r-regex
//!  

use std::io::Read;

/// Helpers for container.
trait MatchesHelper {
	fn opt_string(&self, key: &str) -> String;
}

impl MatchesHelper for getopts::Matches {
	fn opt_string(&self, key: &str) -> String {
		if !self.opt_present(key) {
			return "".to_string();
		}
		let value = self.opt_str(key).unwrap();
		return value;
	}
}

/// Retrieve the result of the regular expression.
fn get_regex_result(string_value: &str, expression_string: &str) -> Result<String, String> {
	let expression = regex::Regex::new(&expression_string);
	if expression.is_err() {
		eprintln!("ERROR: regex compile error. {}", expression.err().unwrap());
		return Err("".into());
	}
	let expression = expression.unwrap();

	// try to capture by "(...)".
	let capture_result = expression.captures_at(&string_value, 0);
	if capture_result.is_none() {
		eprintln!("not match.");
		return Err("".into());
	}

	// result
	let capture_result = capture_result.unwrap();
	if capture_result.len() <= 1 {
		eprintln!("not match.");
		return Err("".into());
	}

	// println!("[debug] captured result: {:?}", capture_result);

	let captured = capture_result.get(1).unwrap();
	let result = captured.as_str();
	return Ok(result.into());
}

/// Read whole lines from stdin.
fn read_whole_lines_from_stdin() -> String {
	let mut buffer = String::new();
	let stdin = std::io::stdin();
	let mut handle = stdin.lock();
	let result = handle.read_to_string(&mut buffer);
	if result.is_err() {
		eprintln!("ERROR: {}", result.err().unwrap());
		return "".to_string();
	}
	return buffer;
}

/// Entrypoint of a Rust application.
fn main() {
	let mut options = getopts::Options::new();
	options.optflag("h", "help", "usage");
	options.opt("s", "string", "string", "STRING", getopts::HasArg::Yes, getopts::Occur::Optional);
	options.opt("r", "regex", "expression", "REGEX", getopts::HasArg::Yes, getopts::Occur::Optional);

	// Analyzing command line arguments.
	let result = options.parse(std::env::args().skip(1));
	if result.is_err() {
		eprintln!("{}", options.usage(""));
		return;
	}
	let input = result.unwrap();

	if input.opt_present("help") {
		eprintln!("{}", options.usage(""));
		return;
	}

	let string_value = if input.opt_present("string") {
		let string: String = input.opt_str("string").unwrap();
		string
	} else {
		read_whole_lines_from_stdin()
	};

	let expression_string = if input.opt_present("regex") {
		let string: String = input.opt_str("regex").unwrap();
		string
	} else {
		"".to_string()
	};

	let result = get_regex_result(&string_value, &expression_string);
	if result.is_err() {
		return;
	}

	println!("{}", result.unwrap());
}

/// test
#[cfg(test)]
mod tests {
	use super::*;

	fn exec_test(left: &str, right: &str, expected_result: &str) {
		let result = get_regex_result(left, right);
		let result = if result.is_err() { "".to_string() } else { result.unwrap() };
		assert_eq!(result, expected_result);
	}

	#[test]
	fn test_get_regex_result() {
		exec_test("abc", "a(.*)", "bc");
		exec_test("000-1234", "^[0-9]+-([0-9]+)$", "1234");
		exec_test("jimi.hendrix@gmail.com", "@(.+)$", "gmail.com");
	}
}
