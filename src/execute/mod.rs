mod error;
mod operator;

use std::collections::HashMap;

use crate::{execute::error::{RuntimeError, RuntimeResult}, parse::{item::{Atom, Item}, operator::Operator}};

pub fn execute_block(mut block: Vec<Item>, operators: &HashMap<String, Operator>) -> RuntimeResult<Vec<Item>> {
	let mut out_stack = Vec::new();

	while !block.is_empty() {
		let item = block.remove(0);
		match item {
			Item::Atom(atom) => match atom {
				Atom::Value(value) => {out_stack.push(Item::Atom(Atom::Value(value.clone())));},
				Atom::Word(word) => {
					match operators.get(word.as_str()) {
						None => {return Err(RuntimeError::new(format!("Unrecognized operator {}", word)))},

						Some(operator) => {
							block = operator.fill_pattern(block)?;
						}
					}
				}
			},

			Item::Stack(stack) => {out_stack.push(Item::Stack(stack));},
			Item::Block(item_block) => {
				block.extend(execute_block(item_block, operators)?);
			}
		}
	}

	Ok(out_stack)
}