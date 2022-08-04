use std::process::Child;

use thirtyfour::prelude::*;
pub async fn get_tools(headless: bool) -> (Result<WebDriver, WebDriverError>, Child) {
    let chromedriver = run_chromedriver(String::from("/bin/chromedriver"));
    let mut caps = DesiredCapabilities::chrome();
    caps.set_disable_gpu().unwrap();
    caps.set_ignore_certificate_errors().unwrap();
    if headless {
        caps.set_headless().unwrap_or(());
        caps.set_no_sandbox().unwrap_or(());
    }
    (
        WebDriver::new("http://localhost:9515", caps).await,
        chromedriver,
    )
}

fn run_chromedriver(path: String) -> std::process::Child {
    let output = std::process::Command::new(&path)
        .arg("--ip=localhost")
        .arg("--port=9515")
        .spawn()
        .expect("Command failed");
    output
}
