use std::{collections::HashMap, error::Error};

use filo::{execute_block, parse_command};

fn main() -> Result<(), Box<dyn Error>> {
	let command = parse_command("foo bar [100 false]")?;

	println!("{:?}", execute_block(command, &HashMap::new())?);
	Ok(())
}