use crate::{execute::error::RuntimeResult, parse::{item::Item, operator::Operator}};

impl Operator {
	pub fn fill_pattern(&self, stack: Vec<Item>) -> RuntimeResult<Vec<Item>> {
		Ok(stack)
	}
}