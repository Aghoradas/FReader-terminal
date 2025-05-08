/*****************************************************************
*  Beginning practice and learning in Rust programming language.
*
******************************************************************/

use ::colored::Colorize;
use crossterm::event::{self, read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, window_size, Clear, ClearType};
use crossterm::{cursor, execute};
use session_info::UserInfo;
use std::fs;
use std::io::{Read, Write, BufReader, BufRead};

mod histories;
mod session_info;

/* RENDERING THE HOST NAME
*****************************/
fn hostname_render() -> String {
    gethostname::gethostname().into_string().unwrap()
}

/* SESSION BULDER
********************/
fn session_builder() -> UserInfo {
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
    let file_obj = std::fs::File::open("./src/histories/history.dat")
        .expect("-error accessing history.dat");
    let file_reader = BufReader::new(file_obj);
    for line in file_reader.lines() {
        current_session.add_line(&line.unwrap().to_string());
    }
    
    current_session
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
        println!("- cd                   -- changes directory to home/user directory");
        println!("- cd [directory name]  -- changes directory to [directory name]");
        println!("- hist                 -- lists recently used commands");
        println!("- keypress             -- toggle off/on dynamic typing (default: on)");
    } else if command_line[1] == "cls" {
        println!("\n-clear    clears the screen");
    } else if command_line[1] == "ls" {
        println!("\n-ls    lists directories in current path");
    } else if command_line[1] == "cd" {
        println!("\n-cd    changes directory to home/[user_directory]");
        println!("When the 'cd' command is used with an arguemn it will open");
        println!("the directory corresponding to that argument: as in 'cd Documents");
        println!("will open the 'Documents' directory in current path, if it exists.");
    } else if command_line[1] == "hist" {
        println!("\n-hist    this without any other arguments will list all previously");
        println!("  typed commands. There will be a way in the future to be able to");
        println!("  be able to re-enact these commands from the list.");
        println!("          --see keypress command for related topics");
    } else if command_line[1] == "keypress" {
        println!("\n-keypress    turns dynamic keyboard off. The dynamic aspect of the input");
        println!("  method is experimental for now, (command history, etc).");
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

/* LIST MECHANICS
*******************/
fn ls(current_dir: &std::path::PathBuf) {
    let win_size = window_size().unwrap().columns;
    let mut win_width = win_size;
    let paths = fs::read_dir(current_dir).expect("-none");
    for path in paths {
        let entry = path.unwrap();
        let path_dir = entry.path().is_dir();
        let file_path: String = entry.file_name().into_string().unwrap();
        let path_size16 = u16::try_from(file_path.len() + 2).unwrap();
        if path_dir && !file_path.as_str().starts_with('.') {
            print!(" {} ", file_path.bright_blue().underline());
            win_width -= path_size16;
        }
        if win_width < path_size16 {
            win_width = win_size;
            println!();
        }
    }
    let paths = fs::read_dir(current_dir).expect("-none");
    for path in paths {
        let entry = path.unwrap();
        let file_path: String = entry.file_name().into_string().unwrap();
        let path_size16 = u16::try_from(file_path.len() + 2).unwrap();
        if win_width < path_size16 {
            win_width = win_size;
            println!();
        }
        if entry.path().is_file() {
            let file_path: String = entry.file_name().into_string().unwrap();
            if !file_path.as_str().starts_with('.') {
                print!(" {} ", file_path.white());
                win_width -= path_size16;
            }
        }
    }
    println!();
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

/* DYNAMIC KEYPRESS
**********************/
fn check_key(current_session: &mut UserInfo, command: &mut String) -> String {
    let mut hist_num = current_session.map_size();
    enable_raw_mode().unwrap();
    let mut stdout = std::io::stdout();
    let mut com_pos;
    if !command.is_empty() {
        com_pos = command.len();
    } else {
        com_pos = 0;
    }
    loop {
        let _pos_limit = command.len();
        if let Event::Key(key_event) = read().unwrap() {
            match key_event.code {
                KeyCode::Enter => {
                    stdout.flush().unwrap();
                    break;
                }
                KeyCode::Up => {
                    if hist_num >= 1 {
                        let init_num = command.trim().len();
                        if init_num > 0 {
                            let init_num16: u16 = u16::try_from(init_num).unwrap();
                            execute!(stdout, cursor::MoveLeft(init_num16 + 1)).expect("-nope");
                            execute!(stdout, Clear(ClearType::UntilNewLine)).expect("-nope");
                        } else {
                            execute!(stdout, cursor::MoveLeft(1)).expect("-nope");
                        }
                        *command = current_session.get_history(hist_num);
                        write!(stdout, " {}", current_session.get_history(hist_num)).expect("-nope");
                        if hist_num != 1 {
                            hist_num -= 1;
                        }
                        stdout.flush().unwrap();
                        com_pos = command.len();
                    } else {
                        stdout.flush().unwrap();
                    }
                }
                KeyCode::Down => {
                    if hist_num <= current_session.map_size() {
                        let init_num = command.trim().len();
                        if init_num > 0 {
                            let init_num16: u16 = u16::try_from(init_num).unwrap();
                            execute!(stdout, cursor::MoveLeft(init_num16 + 1)).expect("-nope");
                            execute!(stdout, Clear(ClearType::UntilNewLine)).expect("-nope");
                        } else {
                            execute!(stdout, cursor::MoveLeft(1)).expect("-nope");
                        }
                        *command = current_session.get_history(hist_num);
                        write!(stdout, " {}", current_session.get_history(hist_num))
                            .expect("-nope");
                        if hist_num < current_session.map_size() {
                            hist_num += 1;
                        }
                        stdout.flush().unwrap();
                        com_pos = command.len();
                    } else {
                        stdout.flush().unwrap();
                    }
                }
                KeyCode::Char(letter) => {
                    if key_event.modifiers.contains(event::KeyModifiers::CONTROL) {
                        execute!(stdout, cursor::MoveToNextLine(1)).expect("-nope");
                        disable_raw_mode().unwrap();
                        std::process::exit(0);
                    }

                    command.push(letter);
                    print!("{}", letter);
                    stdout.flush().unwrap();
                    com_pos += 1;
                }
                KeyCode::Backspace => {
                    if !command.trim().is_empty() {
                        command.pop();
                        execute!(stdout, cursor::MoveLeft(1), Clear(ClearType::UntilNewLine))
                            .unwrap();
                        stdout.flush().unwrap();
                        com_pos -= 1;
                    }
                }
                /*
                KeyCode::Left => {
                    if com_pos > 0 {
                        execute!(stdout, cursor::MoveLeft(1)).unwrap();
                        stdout.flush().unwrap();
                        com_pos -= 1;
                    }
                }
                KeyCode::Right => {
                    if com_pos < pos_limit {
                        execute!(stdout, cursor::MoveRight(1)).unwrap();
                        stdout.flush().unwrap();
                        com_pos += 1;
                    }
                }
                */
                KeyCode::End => {
                    execute!(stdout, cursor::RestorePosition).unwrap();
                    stdout.flush().unwrap();
                    com_pos = command.len();
                }
                KeyCode::Home => {
                    let num: usize = command.len();
                    let num16: u16 = u16::try_from(num).unwrap();
                    execute!(stdout, cursor::SavePosition).unwrap();
                    execute!(stdout, cursor::MoveLeft(num16)).unwrap();
                    stdout.flush().unwrap();
                    com_pos = 0;
                }
                _ => {
                    stdout.flush().unwrap();
                }
            }
        }
    }
    disable_raw_mode().unwrap();
    println!();
    command.to_string()
}

/* MAIN ENTRY
***************/
fn main() {
    let mut hist_entry: String = "".to_string();
    cls();

    let mut current_session = session_builder();
    let mut command = String::new();

    println!("    ..{}..", "FReader".cyan());
    println!(" *****************");
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
            command = check_key(&mut current_session, &mut command);
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
                    ls(current_session.directory());
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
