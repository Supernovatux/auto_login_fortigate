use std::{
    io::{BufRead, BufReader, Read},
    process::{Child, ChildStderr, ChildStdout, Stdio},
};

use log::debug;
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
pub async fn kill_chrome(process: &mut Child) -> bool {
    let out = get_buff_read(process).await;
    let mut handles = Vec::new();
    handles.push(tokio::spawn(async move {write_buf(out.0).await}));
    handles.push(tokio::spawn(async move {write_buf(out.1).await}));
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    for handle in handles {
        handle.abort();
    }
    match process.kill() {
        Ok(_) => true,
        Err(n) => {
            log::error!("Kill failed\n {}", n);
            false
        }
    }
}
async fn get_buff_read(
    process: &mut Child,
) -> (
    Option<BufReader<ChildStdout>>,
    Option<BufReader<ChildStderr>>,
) {
    let mut ret = (None, None);
    if let Some(buff) = process.stdout.take() {
        ret.0 = Some(BufReader::new(buff));
    }
    if let Some(buff) = process.stderr.take() {
        ret.1 = Some(BufReader::new(buff));
    }
    ret
}
async fn write_buf<R: Read>(f: Option<BufReader<R>>) {
    if let Some(f) = f {
        for line in f.lines() {
            let line = match line {
                Ok(num) => num,
                Err(_) => return,
            };
            debug!("Line: {}", line);
        }
    }
}
fn run_chromedriver(path: String) -> std::process::Child {
    let output = std::process::Command::new(&path)
        .arg("--ip=localhost")
        .arg("--port=9515")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Command failed");
    output
}
