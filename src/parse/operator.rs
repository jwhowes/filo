use std::{collections::HashSet, sync::LazyLock};

use regex::Regex;

use crate::parse::{error::{ParseError, ParseResult}, item::{parse_item, Atom, Item, Value}, token::get_tokens};

pub struct Operator(Vec<OperatorEntry>);

struct OperatorEntry {
	pattern: Vec<PatternItem>,
	state: Vec<StateItem>
}

enum PatternItem {
	Atom(PatternAtom),
	Stack(Vec<PatternItem>)
}

enum PatternAtom {
	Variable(String),
	ListVariable(String),
	StackEnd,
	Remainder,
	Value(Value),
	Operator(String),
}

enum StateItem {
	Atom(StateAtom),
	Stack(Vec<StateItem>),
	Block(Vec<StateItem>)
}

enum StateAtom {
	Variable(String),
	StackEnd,
	Remainder,
	Value(Value),
	Operator(String)
}

impl Operator {
	pub fn parse(lines: &[&str], operators: &HashSet<String>) -> ParseResult<Self> {
		Ok(Self(
			lines.iter().map(|&line| OperatorEntry::parse(line, operators))
			.collect::<ParseResult<_>>()?
		))
	}
}

static LIST_VARIABLE_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\.\.\.(\S*)?").unwrap());

impl OperatorEntry {
	fn parse(line: &str, operators: &HashSet<String>) -> ParseResult<Self> {
		let [pattern_str, state_str] = line.split("=>").collect::<Vec<&str>>().try_into().map_err(|_| ParseError::new("Bad entry"))?;

		Ok(Self {
			pattern: Self::parse_pattern(
				if let Item::Stack(stack) = parse_item(get_tokens(pattern_str).as_slice())?.0 {
					stack
				} else {
					return Err(ParseError::new("Invalid pattern"))
				},
				operators
			)?,

			state: Self::parse_state(
				if let Item::Stack(stack) = parse_item(get_tokens(state_str).as_slice())?.0 {
					stack
				} else {
					return Err(ParseError::new("Invalid state"))
				},
				operators
			)?
		})
	}

	fn parse_state(items: Vec<Item>, operators: &HashSet<String>) -> ParseResult<Vec<StateItem>> {
		Ok(items.into_iter().map(|item| match item {
			Item::Atom(atom) => Ok(StateItem::Atom(match atom {
				Atom::Value(value) => Ok(StateAtom::Value(value)),

				Atom::Word(word) => if word.as_str() == "_" {
					Ok(StateAtom::StackEnd)
				} else if word.as_str() == "..." {
					Ok(StateAtom::Remainder)
				} else if LIST_VARIABLE_PATTERN.is_match(word.as_str()) {
					Err(ParseError::new("List variables can't be instantiated in states"))
				} else if operators.contains(word.as_str()) {
					Ok(StateAtom::Operator(word))
				} else {
					Ok(StateAtom::Variable(word))
				}
			}?)),

			Item::Stack(stack) => Ok(StateItem::Stack(Self::parse_state(stack, operators)?)),
			Item::Block(block) => Ok(StateItem::Block(Self::parse_state(block, operators)?)),
		}).collect::<ParseResult<_>>()?)
	}

	fn parse_pattern(items: Vec<Item>, operators: &HashSet<String>) -> ParseResult<Vec<PatternItem>> {
		Ok(items.into_iter().map(|item| match item {
			Item::Atom(atom) => Ok(PatternItem::Atom(match atom {
				Atom::Value(value) => Ok(PatternAtom::Value(value)),
				Atom::Word(word) => if word.as_str() == "_" {
					Ok(PatternAtom::StackEnd)
				} else {
					if let Some(capture) = LIST_VARIABLE_PATTERN.captures(word.as_str()) {
						if let Some(list_variable) = capture.get(1) {
							if operators.contains(list_variable.as_str()) {
								Err(ParseError::new("Can't use an operator as a variable name"))
							} else {
								Ok(PatternAtom::ListVariable(list_variable.as_str().to_string()))
							}
						} else {
							Ok(PatternAtom::Remainder)
						}
					} else if operators.contains(word.as_str()) {
						Ok(PatternAtom::Operator(word))
					} else {
						Ok(PatternAtom::Variable(word))
					}
				}
			}?)),

			Item::Stack(stack) => Ok(PatternItem::Stack(Self::parse_pattern(stack, operators)?)),

			Item::Block(_) => Err(ParseError::new("Can't include blocks in patterns"))
		}).collect::<ParseResult<_>>()?)
	}
}