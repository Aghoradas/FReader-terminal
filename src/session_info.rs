/**********************************************************
* Any amount of information that the information about
* the user-session should be contained here.
***********************************************************/


use crate::histories::ComHistory;

pub struct UserInfo {
    keys: bool,
    user_name: String,
    host_name: String,
    current_directory: std::path::PathBuf,
    command_history: ComHistory,
    //command: String,
}

impl UserInfo {

    /* INITIAL CONSTRUCTOR
    *************************/

    pub fn new(com_hist: ComHistory) -> Self {
        UserInfo {
            keys: true,
            user_name: "anon".to_string(),
            host_name: "unknown".to_string(),
            current_directory: home::home_dir().unwrap(),
            command_history: com_hist,
        }
    }

    /* DYNAMIC KEYBOARD SETTING
    ******************************/

    pub fn on_keys(&self) -> bool {
        self.keys
    }
    pub fn switch_keys(&mut self) {
        self.keys = !self.keys;
    }

    /* SESSION INFO
    ******************/

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

    /* DIRECTORY
    ***************/

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

    /* COMMAND HISTORY
    *********************/

    pub fn map_size(&self) -> usize {
        self.command_history.map_size()
    }
    pub fn add_line(&mut self, line: &String) {
            self.command_history.add_line(&line);
    }
    pub fn show_history(&mut self) {
        self.command_history.show_history();
    }
    pub fn get_history(&self, num: usize) -> String {
        self.command_history.get_history(num)
    }

    /* Unused impl-functions
    ***************************

    pub fn drop_line(&mut self) {
        self.command_history.drop_line();
    }
    pub fn is_end(&mut self, num: usize) -> bool {
        self.command_history.is_end(num)
    }
    */
}
