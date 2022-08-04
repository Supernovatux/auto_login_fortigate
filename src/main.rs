use thirtyfour::prelude::*;
#[tokio::main]
async fn main() -> WebDriverResult<()> {
    std::process::Command::new("warp-cli")
        .arg("disconnect")
        .spawn()
        .unwrap();
    let secrets = auto_login::get_pass::get_pass(
        "~/.config/pass",
    );
    let (web_driver, mut chrome_driver) = auto_login::setup_chrome_driver::get_tools(true).await;
    if let Ok(driver) = web_driver {
        driver
            .goto("http://www.seleniumeasy.com/test/basic-first-form-demo.html")
            .await?;
        if let Ok(title) = driver.title().await {
            if title == "Fortigate :: Login" {
            let secrets = secrets.await.unwrap();
            println!("good {}", title);
            let user_f = driver.find(By::Name("username")).await.unwrap();
            user_f.send_keys(secrets.get_username()).await.unwrap();
            let pass_f = driver.find(By::Name("password")).await.unwrap();
            pass_f.send_keys(secrets.get_password()).await.unwrap();
            let but_f = driver.find(By::XPath(r#"//*[@id="Form1"]/table/tbody/tr[4]/th/div/table/tbody/tr[2]/th/div/p[3]/input"#)).await.unwrap();
            but_f.click().await.unwrap();
            driver.quit().await?;
            }
            else {
                println!("Already logged in!");
            }
        }
    }

    chrome_driver
        .kill()
        .expect("chromedriver server process not killed, do manually");
    std::process::Command::new("warp-cli")
        .arg("connect")
        .spawn()
        .unwrap();
    Ok(())
}
