use std::error::Error;

use filo::parse::command::parse_command;

fn main() -> Result<(), Box<dyn Error>> {
	println!("{:?}", parse_command("foo bar false [{if < 1 2} 10]")?);

	Ok(())
}