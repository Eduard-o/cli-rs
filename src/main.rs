mod commands;

use std::env;

use commands::{Arg, Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print!("Use --help for usage info.");
        return;
    }

    let arg: Arg = Arg::new(&args[1].clone());
    let command: &Command = arg.get_command();
    command.call();
}
