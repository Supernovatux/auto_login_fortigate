pub async fn get_pass(path: &str) -> Option<Secret> {
    let pass = match tokio::fs::read_to_string(path).await {
        Ok(pass) => pass,
        Err(_) => return None,
    };
    let pass:Vec<&str> = pass.trim().split('\n').collect();
    println!("{:?}",pass);
    Some(Secret::new(pass[0],pass[1]))
}

pub struct Secret {
    username : String,
    password : String
}
impl Secret {
    fn new(username :&str,password: &str) -> Secret{
        Secret { username: username.to_string(), password: password.to_string() }
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }
}