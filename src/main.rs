use thirtyfour::prelude::*;
#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut web_status = true;
    if !auto_login::warp_control::chck_warp() {
        auto_login::warp_control::warpctl(auto_login::warp_control::WarpModes::Stop);
        web_status = auto_login::login::login(
            auto_login::get_pass::get_pass("/home/thulashitharan/.config/pass").await,
        )
        .await;
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
