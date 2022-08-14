use log::error;
use thirtyfour::prelude::*;

use crate::setup_chrome_driver;
pub async fn login(secrets: Option<crate::get_pass::Secret>) -> bool {
    match login_int(secrets.clone()).await {
        Ok(_) => true,
        Err(num) => {
            error!("{:?}", num);
            if let Err(_) = login_int(secrets.clone()).await {
                match login_int(secrets).await {
                    Ok(_) => return true,
                    Err(_) => return false,
                }
            }
            return false;
        }
    }
}
async fn login_int(secrets: Option<crate::get_pass::Secret>) -> Result<(), WebDriverError> {
    let (web_driver, mut chrome_driver) = crate::setup_chrome_driver::get_tools(true).await;
    if let Ok(driver) = web_driver {
        driver
            .goto("https://172.25.0.1:1000/login?") // Direct link to login, works even after signing in
            .await?;
        if let Ok(title) = driver.title().await {
            if title == "Fortigate :: Login" {
                let secrets = match secrets {
                    Some(some) => some,
                    None => {
                        log::error!("Unable to accure secrets");
                        return Err(WebDriverError::CustomError(
                            "Accuing secrets failed".to_string(),
                        ));
                    }
                };
                log::debug!("Title of webpage :- {}", title);
                let user_f = driver.find(By::Name("username")).await?;
                user_f.send_keys(secrets.get_username()).await?;
                let pass_f = driver.find(By::Name("password")).await?;
                pass_f.send_keys(secrets.get_password()).await?;
                let but_f = driver.find(By::XPath(r#"//*[@id="Form1"]/table/tbody/tr[4]/th/div/table/tbody/tr[2]/th/div/p[3]/input"#)).await?;
                but_f.click().await?;
                driver.quit().await?;
            } else {
                log::error!("Unable to view login page!"); // We don't need this now
            }
        }
    } else {
        error!("Driver Failed");
        setup_chrome_driver::kill_chrome(&mut chrome_driver).await;
        return Err(WebDriverError::CustomError("Driver Failed!".to_string()));
    }
    setup_chrome_driver::kill_chrome(&mut chrome_driver).await;
    Ok(())
}
