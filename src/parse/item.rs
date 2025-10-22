use std::{fmt::Debug, str::FromStr, sync::LazyLock};
use regex::Regex;
use crate::parse::error::{ParseError, ParseResult};

#[derive(Debug)]
pub enum Item {
	Atom(Atom),
	Stack(Vec<Item>),
	Block(Vec<Item>)
}

#[derive(Debug)]
pub enum Atom {
	Value(Value),
	Word(String),
}

#[derive(Debug, Clone)]
pub enum Value {
	Int(i32),
	Float(f32),
	Bool(bool),
}

pub fn parse_item<'a>(mut tokens: &'a [&'a str]) -> ParseResult<(Item, &'a [&'a str])> {
	let mut items = Vec::new();

	while !tokens.is_empty() {
		let &token = tokens.first().unwrap();

		match token {
			"[" | "{" => {
				let (child_items, remainder) = parse_item(&tokens[1..])?;

				match child_items {
					Item::Stack(stack) => {
						if token != "[" {
							return Err(ParseError::new("Inconsistent brackets"))
						}

						items.push(Item::Stack(stack));
					},

					Item::Block(block) => {
						if token != "{" {
							return Err(ParseError::new("Inconsistent brackets"))
						}
						
						items.push(Item::Block(block));
					},

					_ => {return Err(ParseError::new("Inconsistent brackets"))}
				}

				tokens = remainder;
			},

			"]" => {
				return Ok((
					Item::Stack(items),
					&tokens[1..]
				))
			},

			"}" => {
				return Ok((
					Item::Block(items),
					&tokens[1..]
				))
			},

			word => {
				items.push(Item::Atom(
					word.parse()?
				));

				tokens = &tokens[1..];
			},
		}
	}

	Ok((Item::Block(items), tokens))
}

static INT_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[0-9]+$").unwrap());
static FLOAT_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[0-9]+\.[0-9]*$").unwrap());

impl FromStr for Atom {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if INT_PATTERN.is_match(s) {
			Ok(Atom::Value(Value::Int(s.parse().unwrap())))
		} else if FLOAT_PATTERN.is_match(s) {
			Ok(Atom::Value(Value::Float(s.parse().unwrap())))
		} else {
			match s {
				"true" => Ok(Atom::Value(Value::Bool(true))),
				"false" => Ok(Atom::Value(Value::Bool(false))),

				word => Ok(Atom::Word(word.to_string()))
			}
		}
	}
}