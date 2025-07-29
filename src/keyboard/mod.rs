/*!****************************************************************
*  DYNAMIC KEYPRESS
*    This is going to be able to handle not
*    only basic-typed commands, but also "special"
*    keyed-commands up arrow, ctrl+key, etc.
******************************************************************/

use crossterm::event::{self, read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{cursor, execute};
use std::io::Write;

use crate::session_info::UserInfo;

pub fn dynamic_keyboard(current_session: &mut UserInfo, command: &mut String) -> String {
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
                        write!(stdout, " {}", current_session.get_history(hist_num))
                            .expect("-nope");
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
                        if hist_num <= current_session.map_size() {
                            hist_num += 1;
                        }
                        let init_num = command.trim().len();
                        let init_num16: u16 = u16::try_from(init_num).unwrap();
                        if init_num > 0 {
                            execute!(stdout, cursor::MoveLeft(init_num16 + 1)).expect("-nope");
                            execute!(stdout, Clear(ClearType::UntilNewLine)).expect("-nope");
                        } else {
                            execute!(stdout, cursor::MoveLeft(1)).expect("-nope");
                        }
                        *command = current_session.get_history(hist_num);
                        write!(stdout, " {}", current_session.get_history(hist_num))
                            .expect("-nope");
                        stdout.flush().unwrap();
                        com_pos = command.len();
                    } else {
                        stdout.flush().unwrap();
                    }
                }
                KeyCode::Char(letter) => {
                    if key_event.modifiers == event::KeyModifiers::CONTROL && key_event.code == KeyCode::Char('c') {
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
