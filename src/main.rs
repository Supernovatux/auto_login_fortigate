use auto_login::cli_parser;
use log::info;
use thirtyfour::prelude::*;
use tokio::join;
#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let (path, log_lev) = join!(cli_parser::get_path(), cli_parser::get_verbosity());
    simple_logger::init_with_level(log_lev).unwrap();
    info!("Starting with log_lev:- {:?}", log_lev);
    info!("Path for password:- {}", path);
    let mut web_status = true;
    if !auto_login::warp_control::chck_warp() {
        auto_login::warp_control::warpctl(auto_login::warp_control::WarpModes::Stop);
        web_status = auto_login::login::login(auto_login::get_pass::get_pass(&path).await).await;
        auto_login::warp_control::warpctl(auto_login::warp_control::WarpModes::Start);
    } else {
        log::info!("Logged in and connected to warp");
    }
    if web_status {
        Ok(())
    } else {
        Err(WebDriverError::CustomError("".to_string()))
    }
}
