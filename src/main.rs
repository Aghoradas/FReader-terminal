/*****************************************************************
*  Beginning practice and learning in Rust programming language.
*
******************************************************************/

use ::colored::Colorize;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, execute};
use keyboard::dynamic_keyboard;
use std::io::{Read, Write, BufReader, BufRead};

mod histories;
mod session_info;
mod keyboard;
mod list_mechanics;
mod shell_error;

use session_info::UserInfo;
use list_mechanics::list;
use list_mechanics::list_columns;
use shell_error::ProgramError;
use shell_error::ProgResult;




/* RENDERING THE HOST NAME
*****************************/
fn hostname_render() -> String {
    gethostname::gethostname().into_string().unwrap()
}


/* SESSION BUIILDER
********************/
fn session_builder() -> ProgResult<UserInfo>{
    let command_hist = histories::new_hist();
    let mut current_session = UserInfo::new(command_hist);
    let username = current_session
        .directory()
        .clone()
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .expect("anon");
    let host = hostname_render();
    current_session.new_user(username);
    current_session.new_host(host);

    // loading history
    let path = "./src/histories/history.dat";
    if !std::fs::exists(path)? {
        return Err(ProgramError::Construct("missing history.dat".into()));
    }

    let file_obj = std::fs::File::open(path)
        .expect("-error accessing history.dat");
    let file_reader = BufReader::new(file_obj);
    for line in file_reader.lines() {
        current_session.add_line(&line.unwrap().to_string());
 
    }
    
    Ok(current_session)
}


/* Help menu
**************/
fn help(command_line: &[String]) {
    if command_line.len() < 2 {
        println!();
        println!("- exit                 -- exits FReader");
        println!("- help                 -- this help menu");
        println!("- clear                -- clears the screen");
        println!("- ls                   -- lists directories in current path");
        println!("- org                  -- switches 'ls' function to neat columns");
        println!("- cd                   -- changes directory to home/user directory");
        println!("- cd [directory name]  -- changes directory to [directory name]");
        println!("- hist                 -- lists recently used commands");
        println!("- keypress             -- toggle on/off dynamic typing (default: on)");
    } else if command_line[1] == "exit" {
        println!("\n-exit    this exits FReader. This can also be done by holding CTRL");
        println!("         button down and pressing 'e'.");
    } else if command_line[1] == "cls" {
        println!("\n-clear    clears the screen");
    } else if command_line[1] == "ls" {
        println!("\n-ls    lists directories in current path");
    } else if command_line[1] == "org" {
        println!("\n-org    toggles 'ls' from listing directories out into rows to");
        println!("        listing them out in neatly arranged columns. [on/off]");
    } else if command_line[1] == "cd" {
        println!("\n-cd    changes directory to home/[user_directory]");
        println!("       When the 'cd' command is used with an argument it will open");
        println!("       the directory corresponding to that argument: as in 'cd Documents'");
        println!("       will open the 'Documents' directory in current path, if it exists.");
    } else if command_line[1] == "hist" {
        println!("\n-hist    this without any other arguments will list all previously");
        println!("         typed commands. There will be a way in the future to be able to");
        println!("         be able to re-enact these commands from the list.");
        println!("             --see keypress command for related topics");
    } else if command_line[1] == "keypress" {
        println!("\n-keypress    turns dynamic keyboard off. The dynamic aspect of the input");
        println!("             method is experimental for now, (command history, etc).");
    } else {
        println!(
            "\n-help {}    is not a command, or is not yet included in the help docs",
            command_line[1].red()
        );
        println!("      report to developer if it needs to be. Thank you.");
    }
}


/* CLEAR SCREEN
*****************/
fn cls() {
    std::process::Command::new("clear").status().unwrap();
}


/* CHANGE DISK (CD) MECHANICS
*******************************/
fn cd(dir_change: &str, current_session: &mut UserInfo) -> bool {
    if dir_change == ".." {
        current_session.back_directory();
        return true;
    }
    let mut test_dir = current_session.directory().clone();
    test_dir.push(dir_change);

    if test_dir.is_dir() {
        current_session.change_directory(dir_change);
        true
    } else {
        println!("-directory does not exist");
        false
    }
}


/* MAIN ENTRY
***************/
fn main() {
    let mut hist_entry: String = "".to_string();
    cls();

    let new_session = session_builder();
    match &new_session {
        Ok(session) => {
            println!("Success: {}", session.user());
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    }
    let mut current_session = new_session.unwrap();
    let mut command = String::new();

    loop {
        let dir_display = current_session
            .directory()
            .file_name()
            .unwrap()
            .to_str();
        print!(
            "[{}@{}: {:}]$ ",
            current_session.user().yellow(),
            current_session.host(),
            dir_display.unwrap()
        );
        std::io::stdout().flush().expect("Erm...");

        if !hist_entry.is_empty() {
            command = hist_entry.clone();
            hist_entry.clear();

            enable_raw_mode().unwrap();
            write!(std::io::stdout(), "{}", command.trim().green()).expect("-nope");
            execute!(std::io::stdout(), cursor::MoveToNextLine(1)).unwrap();
            std::io::stdout().flush().unwrap();
            disable_raw_mode().unwrap();
        } else if !current_session.on_keys() {
            std::io::stdin()
                .read_line(&mut command)
                .expect("Failed to read line");
            command = command.trim_end().to_string();
        } else {
            command = dynamic_keyboard(&mut current_session, &mut command);
        }

        let command_line: Vec<String> = command.split_whitespace().map(String::from).collect();
        if command.trim().len() > 0 {
            current_session.add_line(&command);
        }
        if let Some(first_word) = command_line.first() {
            match first_word.as_str() {
                "exit" => {
                    cls();
                    println!("-{} is closed\n", "FReader".yellow());
                    break;
                }
                "keypress" => match current_session.on_keys() {
                    false => {
                        current_session.switch_keys();
                        println!("-keypress: {}", "on".green());
                    }
                    true => {
                        current_session.switch_keys();
                        std::io::stdout().flush().unwrap();
                        println!("keypress: {}", "off".green());
                    }
                },
                "clear" => {
                    cls();
                }
                "help" => {
                    help(&command_line);
                }
                "read" => {
                    if command_line.len() > 1 {
                        let mut target = current_session.directory().clone();
                        target.push(&command_line[1]);
                        if !target.exists() {
                            println!("-file {} does not exist", command_line[1].yellow());
                            continue;
                        }
                        let mut file = std::fs::File::open(&target).unwrap();
                        let mut contents = String::new();
                        file.read_to_string(&mut contents).unwrap();
                        print!("{}", contents.green());
                    } else {
                        println!("-needs a file to read");
                    }
                }
                "ls" => {
                    if *current_session.ls_type() == true {
                        list_columns(current_session.directory());
                    } else {
                        list(current_session.directory());
                    }
                }
                "org" => {
                    current_session.switch_ls_type();

                }
                "cd" => {
                    if command_line.len() > 1 {
                        cd(command_line[1].as_str(), &mut current_session);
                    } else {
                        current_session.home();
                    }
                }
                "pwd" => {
                    println!("{}", current_session.directory().display());
                }
                "hist" => {
                    match command_line.len() {
                        1 => current_session.show_history(),
                        2 => {
                            let num_entry: usize = command_line[1].parse().unwrap();
                            hist_entry = current_session.get_history(num_entry);
                        }
                        _ => println!("-no history command {}", command_line[2]),
                    };
                }
                ".hist" => {}
                _ => println!("\n-command '{}' does not exist", first_word.yellow()),
            }
        }
        /* If let some() */
        else {
            println!("-no input detected");
        }
        command.clear();
    } // main loop
}
