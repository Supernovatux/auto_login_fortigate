use auto_login::{
    cli_parser,
    warp_control::{self, InternetStatus, WarpModes},
};
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
    match warp_control::internet_status() {
        InternetStatus::Some(warp_status) => match warp_status {
            WarpModes::On => log::info!("Logged in and connected to warp"),
            WarpModes::Off => {
                auto_login::warp_control::warpctl(WarpModes::On);
                log::info!("Logged in and connected to warp(1)");
            }
        },
        InternetStatus::None => {
            warp_control::warpctl(auto_login::warp_control::WarpModes::Off);
            web_status =
                auto_login::login::login(auto_login::get_pass::get_pass(&path).await).await;
            warp_control::warpctl(auto_login::warp_control::WarpModes::On);
            log::info!("Logged in and connected to warp");
        }
    }
    if web_status {
        Ok(())
    } else {
        Err(WebDriverError::CustomError("".to_string()))
    }
}
