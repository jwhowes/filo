use std::{str::FromStr, sync::LazyLock};

use regex::Regex;

use crate::parse::{error::{ParseError, ParseResult}, item::{parse_items, Item}, token::get_tokens};

static INT_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[0-9]+$").unwrap());
static FLOAT_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[0-9]+\.[0-9]*$").unwrap());

#[derive(Debug)]
pub enum CommandAtom {
	Int(i32),
	Float(f32),
	Bool(bool),
	Operator(String)
}

impl FromStr for CommandAtom {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if INT_PATTERN.is_match(s) {
			Ok(CommandAtom::Int(s.parse().unwrap()))
		} else if FLOAT_PATTERN.is_match(s) {
			Ok(CommandAtom::Float(s.parse().unwrap()))
		} else {
			match s {
				"true" => Ok(CommandAtom::Bool(true)),
				"false" => Ok(CommandAtom::Bool(false)),

				word => Ok(CommandAtom::Operator(word.to_string()))
			}
		}
	}
}

pub fn parse_command(s: &str) -> ParseResult<Vec<Item<CommandAtom>>> {
	let tokens = get_tokens(s);

	if let (Item::Stack(command_items), []) = parse_items(tokens.as_slice())? {
		Ok(command_items)
	} else {
		Err(ParseError::new("Expected stack"))
	}
}