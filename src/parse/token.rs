use std::sync::LazyLock;

use regex::Regex;

static TOKEN_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s*(\[|\]|\{|\}|[^\{\[\}\]\s]+)\s*").unwrap());


pub fn get_tokens(s: &str) -> Vec<&str> {
	TOKEN_PATTERN.captures_iter(s)
		.map(|caps| caps.get(1).unwrap().as_str())
		.collect()
}