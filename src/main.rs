use thirtyfour::prelude::*;


fn run_chromedriver(path: String) -> std::process::Child{
    let output = 
        std::process::Command::new(&path)
        .arg("--ip=localhost")
        .arg("--port=9515")
        .spawn()
        .expect("Command failed");
    output
}


#[tokio::main]
async fn main() -> WebDriverResult<()> {
    std::process::Command::new("warp-cli").arg("disconnect").spawn().unwrap();
    let secrets = auto_login::get_pass::get_pass("/home/thulashitharan/Documents/iiitdm/sem_3/Projects/auto_login/target/debug/pass"); 
    let mut chromedriver = run_chromedriver(String::from("/bin/chromedriver"));
    let mut caps = DesiredCapabilities::chrome();
    caps.set_disable_gpu().unwrap();
    caps.set_ignore_certificate_errors().unwrap();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    // Navigate to https://wikipedia.org.
    driver.goto("http://www.seleniumeasy.com/test/basic-first-form-demo.html").await?;
    if let Ok(title) = driver.title().await {
        let secrets = secrets.await.unwrap();
        println!("good {}",title);
        let user_f = driver.find(By::Name("username")).await.unwrap();
        user_f.send_keys(secrets.get_username()).await.unwrap();
        let pass_f = driver.find(By::Name("password")).await.unwrap();
        pass_f.send_keys(secrets.get_password()).await.unwrap();
        let but_f = driver.find(By::XPath(r#"//*[@id="Form1"]/table/tbody/tr[4]/th/div/table/tbody/tr[2]/th/div/p[3]/input"#)).await.unwrap();
        but_f.click().await.unwrap();
    }
    else {
        panic!();
    }
    // Find element from element.

    // Type in the search terms.

    driver.quit().await?;
    chromedriver.kill().expect("chromedriver server process not killed, do manually"); 
    std::process::Command::new("warp-cli").arg("connect").spawn().unwrap();
    Ok(())
}
