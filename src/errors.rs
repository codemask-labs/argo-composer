use cliclack::log;
use console::style;

pub const ARGO_COMPOSER_NOT_INITIALIZED: &str = "Argo composer not initialized\nPlease make sure you have initialized the project with `argo-composer init` command.";

pub fn terminate_with_message(message: &str) -> ! {
    let message = style(message).red();
    log::remark(message).unwrap();
    println!("{}", style("x").red());

    std::process::exit(1);
}
