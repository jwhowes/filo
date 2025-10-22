use std::{collections::{HashMap, HashSet}, fs, sync::LazyLock};
use regex::Regex;

use crate::parse::{error::{ParseError, ParseResult}, item::{parse_item, Item}, operator::Operator, token::get_tokens};

mod token;
mod error;
pub mod item;
pub mod operator;

static OPERATOR_DEF_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^def (\S+):$").unwrap());

pub fn parse_source(path: &str) -> ParseResult<HashMap<String, Operator>> {
	let lines = fs::read_to_string(path).map_err(|_| ParseError::new("Couldn't read file"))?.lines().map(String::from).collect::<Vec<String>>();

	let operator_names: Vec<(usize, &str)> = lines.iter().enumerate()
		.filter(|(_, line)| line.starts_with("def"))
		.map(|(i, line)| match OPERATOR_DEF_PATTERN.captures(line) {
			None => Err(ParseError::new("Bad function definition")),
			Some(captures) => {
				match captures.get(1) {
					None => Err(ParseError::new("Bad function definition")),
					Some(name) => Ok((i, name.as_str()))
				}
			}
		}).collect::<ParseResult<_>>()?;
	
	let operators = operator_names.iter().map(|(_, name)| name.to_string()).collect::<HashSet<String>>();

	let mut source = HashMap::new();

	for (i, name) in operator_names {
		let operator_lines = lines.iter().skip(i).take_while(|line| !line.trim().is_empty()).map(|line| line.as_str()).collect::<Vec<_>>();

		source.insert(name.to_string(), Operator::parse(operator_lines.as_slice(), &operators)?);
	}

	Ok(source)
}

pub fn parse_command(command: &str) -> ParseResult<Vec<Item>> {
	if let Item::Block(block) = parse_item(get_tokens(command).as_slice())?.0 {
		Ok(block)
	} else {
		Err(ParseError::new("Expected block"))
	}
}