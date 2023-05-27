//!
//! regex
//!  

trait MatchesHelper {
    fn opt_string(&self, key: &str) -> String;
}

impl MatchesHelper for getopts::Matches {
    fn opt_string(&self, key: &str) -> String {
        if !self.opt_present(key) {
            return "".to_string()
        }
        let value = self.opt_str(key).unwrap();
        return value;
    }
}

/// Retrieve the result of the regular expression.
fn get_regex_result(string_value: String, expression_string: String) {
	let expression = regex::Regex::new(&expression_string);
	if expression.is_err() {
		eprint!("ERROR: regex compile error. {}", expression.err().unwrap());
		return;
	}
	let expression = expression.unwrap();

	// try to capture by "(...)".
	let capture_result = expression.captures_at(&string_value, 0);
	if capture_result.is_none() {
		eprint!("not match.");
		return;
	}

	// result
	let capture_result = capture_result.unwrap();
	if capture_result.len() <= 1 {
		eprint!("not match.");
		return;
	}

	let captured = capture_result.get(1).unwrap();
	let result = captured.as_str();
	print!("{}", result);
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
		eprint!("{}", options.usage(""));
		return;
	}
	let input = result.unwrap();

	if input.opt_present("help") {
		eprint!("{}", options.usage(""));
		return;
	}

	let string_value = if input.opt_present("string") {
		let string: String = input.opt_str("string").unwrap();
		string
	} else {
		"".to_string()
	};

	let expression_string = if input.opt_present("regex") {
		let string: String = input.opt_str("regex").unwrap();
		string
	} else {
		"".to_string()
	};

	get_regex_result(string_value, expression_string);
}
