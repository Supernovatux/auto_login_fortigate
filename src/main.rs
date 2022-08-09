use thirtyfour::prelude::*;
#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let path = auto_login::cli_parser::get_path().await;
    let mut web_status = true;
    if !auto_login::warp_control::chck_warp() {
        auto_login::warp_control::warpctl(auto_login::warp_control::WarpModes::Stop);
        web_status = auto_login::login::login(auto_login::get_pass::get_pass(&path).await).await;
        auto_login::warp_control::warpctl(auto_login::warp_control::WarpModes::Start);
    } else {
        println!("Logged in and connected to warp");
    }
    if web_status {
        Ok(())
    } else {
        Err(WebDriverError::CustomError("".to_string()))
    }
}
