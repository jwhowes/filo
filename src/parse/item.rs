use std::{fmt::Debug, str::FromStr};

use crate::parse::error::{ParseError, ParseResult};

#[derive(Debug)]
pub enum Item<Atom> {
	Atom(Atom),
	Stack(Vec<Item<Atom>>),
	Block(Vec<Item<Atom>>)
}

pub fn parse_items<'a, Atom: FromStr<Err = ParseError>>(mut tokens: &'a [&'a str]) -> ParseResult<(Item<Atom>, &'a [&'a str])> {
	let mut items = Vec::new();

	while !tokens.is_empty() {
		let &token = tokens.first().unwrap();

		match token {
			"[" | "{" => {
				let (child_items, remainder) = parse_items(&tokens[1..])?;

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

	Ok((Item::Stack(items), tokens))
}