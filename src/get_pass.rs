/// Returns a [Secret]
///
/// The value is `Some` if [Secret] is found or else its a `None`
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to username and password
///
pub async fn get_pass(path: &str) -> Option<Secret> {
    let pass = match tokio::fs::read_to_string(path).await {
        Ok(pass) => pass,
        Err(_) => return None,
    };
    let pass: Vec<&str> = pass.trim().split('\n').collect();
    log::debug!("Username:- {}\nPassword:- {}", pass[0], pass[1]);
    Some(Secret::new(pass[0], pass[1]))
}

///Struct to hold username and password as a [String]
#[derive(Clone)]
pub struct Secret {
    username: String,
    password: String,
}
impl Secret {
    ///Creates and returns a new [Secret]
    ///
    /// # Arguments
    ///
    /// * `username` - A string slice to username
    /// * `password` - A string slice to password
    ///
    /// # Examples
    /// ```
    /// use auto_login::get_pass::Secret;
    /// let new_secret = Secret::new("name","pass");
    /// assert_eq!(new_secret.get_username(),"name");
    /// assert_eq!(new_secret.get_password(),"pass");
    /// ```  
    pub fn new(username: &str, password: &str) -> Secret {
        Secret {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
    ///Returns `username` field of [Secret]
    /// # Examples
    /// ```
    /// use auto_login::get_pass::Secret;
    /// let new_secret = Secret::new("name","pass");
    /// assert_eq!(new_secret.get_username(),"name");
    /// ```    
    pub fn get_username(&self) -> &str {
        &self.username
    }
    ///Returns `password` field of [Secret]
    ///
    /// # Examples
    /// ```
    /// use auto_login::get_pass::Secret;
    /// let new_secret = Secret::new("name","pass");
    /// assert_eq!(new_secret.get_password(),"pass");
    /// ```
    pub fn get_password(&self) -> &str {
        &self.password
    }
}
