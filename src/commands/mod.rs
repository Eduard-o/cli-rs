mod help;
mod about;
mod commands;

use core::str;

use help::call_help;
use about::call_about;
use commands::call_commands;

pub enum Command {
    About,
    Help,
    Commands,
    NotImplemented,
}

impl Command {
    pub fn from(command_str: &str) -> Command {
        let command: Command = match command_str {
            "help" | "H" => Command::Help,
            "about" | "A" => Command::About,
            "commands" | "C" => Command::Commands,
            _ => Command::NotImplemented,
        };

        command
    }

    pub fn call(&self) {
        match self {
            Command::Help => call_help(),
            Command::About => call_about(),
            Command::Commands => call_commands(),
            Command::NotImplemented => call_help(),
        }
    }
}

pub struct Arg {
    command: Command,
}

impl Arg {
    pub fn new(cli_arg: &String) -> Arg {
        let mut arg_str: String = String::from(cli_arg);

        let binding: String = arg_str.split_off(arg_str.matches("-").count());
        let command_str: &str = binding.as_str();

        let command: Command = Command::from(command_str);

        let arg: Arg = Arg {
            command
        };

        arg
    }

    pub fn get_command(&self) -> &Command {
        &self.command
    }
}
