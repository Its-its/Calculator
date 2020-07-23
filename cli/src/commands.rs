use console::style;

use conversion_parser::Factory;


use crate::ColorTypes;

pub enum Command {
	Help,
	Constants,
	Functions,
	Units
}

impl Command {
	pub fn display(&self, factory: &Factory) -> String {
		match self {
			Command::Help => display_help(),
			Command::Constants => display_const(factory),
			Command::Functions => display_func(factory),
			Command::Units => display_units(factory),
		}
	}
}



pub fn get_command(value: &str) -> Option<Command> {
	Some(match value {
		"help" => Command::Help,
		"constants" => Command::Constants,
		"functions" => Command::Functions,
		"units" => Command::Units,

		_ => return None
	})
}

fn display_help() -> String {
	vec![
		"help"
	]
	.join("\n")
}

fn display_const(factory: &Factory) -> String {
	factory.get_constants()
	.iter()
	.map(|(v, _)| ColorTypes::Default.str(v))
	.map(|s| format!("{} {}", style(">").red(), s))
	.collect::<Vec<String>>()
	.join("\n")
}

fn display_func(factory: &Factory) -> String {
	factory.get_functions()
	.iter()
	.map(|(v, _)| ColorTypes::Default.str(v))
	.map(|s| format!("{} {}", style(">").red(), s))
	.collect::<Vec<String>>()
	.join("\n")
}

fn display_units(factory: &Factory) -> String {
	factory.get_units()
	.iter()
	.map(|u| ColorTypes::Default.str(u.base_long()))
	.map(|s| format!("{} {}", style(">").red(), s))
	.collect::<Vec<String>>()
	.join("\n")
}