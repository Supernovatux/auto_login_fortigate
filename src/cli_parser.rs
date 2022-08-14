use cached::proc_macro::cached;
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
///Automatically log in to iiitdmk's fortigate login portal
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
    ///Path to password
    #[clap(
        short,
        long,
        value_parser,
        default_value_t = get_def_path() 
    )]
    pub path: String,
}

pub async fn get_path() -> String {
    let arg = init_cli().await;
    arg.path
}
pub async fn get_verbosity() -> log::Level {
    let arg = init_cli().await;
    arg.verbose.log_level().unwrap()
}
fn get_def_path() -> String {
    dirs::config_dir().unwrap().join("pass").to_str().unwrap().to_string()
}
#[cached]
async fn init_cli() -> Cli {
    Cli::parse()
}
