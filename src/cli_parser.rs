use cached::proc_macro::cached;
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
///Struct to parse cli arguments
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Cli {
    ///Verbosity flag
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
    ///Path to password
    #[clap(
        short,
        long,
        value_parser,
        default_value_t = get_def_path(),
    )]
    pub path: String,
}
///Return path to password as [String]
///
/// If path was not provided by cli defaults to `~/.config/pass`
///
pub async fn get_path() -> String {
    let arg = init_cli();
    arg.path
}
///Returns [log::Level] from verbosity flag passed via cli
///
/// Defaults to [log::Level::Info]
pub async fn get_verbosity() -> log::Level {
    let arg = init_cli();
    arg.verbose.log_level().unwrap()
}
pub async fn get_headless() -> bool {
    todo!();
}
pub async fn get_login_url() -> String {
    todo!();
}
#[cached]
fn get_def_path() -> String {
    dirs::config_dir()
        .unwrap()
        .join("pass")
        .to_str()
        .unwrap()
        .to_string()
}
#[cached]
fn init_cli() -> Cli {
    Cli::parse()
}
