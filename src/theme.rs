use cliclack::log;
use console::style;

pub fn terminate_with_message(message: &str) -> ! {
    let message = style(message).red();
    log::remark(message).unwrap();
    println!("{}", style("x").red());

    std::process::exit(1);
}
