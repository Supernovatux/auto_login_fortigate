use clap::Parser;

#[derive(Parser, Default, Debug)]
pub struct Cli {
    //Path to password
    #[clap(
        short,
        long,
        value_parser,
        default_value = "/home/thulashitharan/.config/pass"
    )]
    pub path: std::path::PathBuf,
}

pub async fn get_path() -> String {
    let arg = Cli::parse();
    arg.path.to_str().unwrap().to_string()
}
