/**********************************************************
 * Any amount of information that the information about
 * the user-session should be contatined here.
 **********************************************************/

pub struct UserInfo {
  
    keys: bool,
    user_name: String,
    host_name: String,
    current_directory: std::path::PathBuf,
    //command_history: HashMap<i32, String>,
    //command: String,

}

impl UserInfo {
    // INITIAL CONSTRUCTOR
    pub fn new() -> Self {
        UserInfo {
            keys: true,
            user_name: "anon".to_string(),
            host_name: "unknown".to_string(),
            current_directory: home::home_dir().unwrap(),
        }
        
    }
    
    // DYNAMIC KEYBOARD SETTING
    pub fn on_keys(&self) -> bool {
        self.keys
    }
    pub fn switch_keys(&mut self) {
        if false == self.keys {
    self.keys = true;
        } else {
            self.keys = false;
        }
    }
    
    // SESSION INFO
    pub fn user(&self) -> &String {
        &self.user_name
    }
    pub fn new_user(&mut self, name: String) {
        self.user_name = name;
    }
    pub fn host(&self) -> &String {
        &self.host_name
    }
    pub fn new_host(&mut self, name: String) {
        self.host_name = name;
    }

    // DIRECTORY
    pub fn directory(&self) -> &std::path::PathBuf {
        &self.current_directory
    }
    pub fn back_directory(&mut self) {
        let path_check = std::path::PathBuf::from("/home");
        if self.current_directory != path_check {
            self.current_directory.pop();
        } else {
            println!("-no access to root: at this time");
        }
    }
    pub fn change_directory(&mut self, dir: &str) {
        self.current_directory.push(dir);
    }
    pub fn home(&mut self) {
        self.current_directory = home::home_dir().unwrap();
    }
}

