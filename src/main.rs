use thirtyfour::prelude::*;
#[tokio::main]
async fn main() -> WebDriverResult<()> {
    auto_login::warp_control::warpctl(auto_login::warp_control::WarpModes::Stop);
    let secrets = auto_login::get_pass::get_pass("/home/thulashitharan/.config/pass");
    let web_status = auto_login::login::login(secrets).await;
    auto_login::warp_control::warpctl(auto_login::warp_control::WarpModes::Stop);
    if web_status {
        Ok(())
    } else {
        Err(WebDriverError::CustomError("".to_string()))
    }
}
