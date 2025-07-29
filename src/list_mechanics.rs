/***********************************************************************
* LIST MECHANICS - 'ls'
*   This will primarily be responsible for printing to the user
* a list of directories and files contained in the current path.
************************************************************************/
use ::colored::Colorize;
use crossterm::cursor;
use crossterm::{execute, terminal::window_size};
use std::io::Write;
use std::{fs::{self}, usize};


fn column_length(current_dir: &std::path::PathBuf) -> (u16, u16) {
    let paths = fs::read_dir(current_dir).expect("-none");
    let mut size_column: usize = 0;
    let mut path_size: usize;
    let mut skip_count: u16 = 0;
    for path in paths {
        let entry = path.unwrap().file_name()
                        .into_string().unwrap();
        path_size = entry.len(); 
        if path_size > size_column && !entry.starts_with('.') {
            size_column = path_size.clone();
            println!("{}", entry);
        } else if entry.starts_with('.'){
            skip_count += 1;
        }
        
    }
    println!("{}", size_column);
    let column_size = 2 + u16::try_from(size_column)
        .unwrap();

    (column_size, skip_count)

}


// Create neat columns of alphabetic list
pub fn list_columns(current_dir: &std::path::PathBuf) {
    let pad: u16 = 0;
    let (column_size, skip_count) = column_length(current_dir);
    let columns_width = pad + column_size;
    let win_size = window_size().unwrap().columns;


    let mut win_width = win_size;
    let paths = fs::read_dir(current_dir).expect("-none")
        .collect::<Result<Vec<_>, _>>().expect("-none");
    let entry_amount = u16::try_from(paths.len()).unwrap() - skip_count;
    println!("entry_amount:   {}", entry_amount);


    let column_number:  u16 = win_width / columns_width;
    println!("column_number:  {}", column_number);
    let row_per_column: u16 = entry_amount / column_number;
    println!("row_per_column: {}", row_per_column);
    let remaining_row:    u16 = entry_amount - (column_number * row_per_column);
    println!("remaining_row:  {}", remaining_row);


    let mut row:      u16 = 0;
    let mut column:   u16 = 1;
    let mut row_flag: u16 = 0;
    for path in paths {

        let _path_dir = path.path().is_dir();
        let file_path: String = path.file_name().into_string().unwrap();

        if !file_path.as_str().starts_with('.') {
            execute!(std::io::stdout(), cursor::MoveToColumn(column)).unwrap();

            if row <= row_per_column {
                row += 1;
                println!("{}", file_path.bright_blue().underline());
            } else if remaining_row >= column && row_flag == 0 {
                row_flag = 1;
                println!("{}", file_path.bright_blue().underline());
            } else {
                column += columns_width;
                execute!(std::io::stdout(), cursor::MoveToColumn(column)).unwrap();
                execute!(std::io::stdout(), cursor::MoveToPreviousLine(row)).unwrap();
                row = 1;
                println!("{}", file_path.bright_blue().underline());
            }

        }
    }
}


pub fn list(current_dir: &std::path::PathBuf) {
    let pad: u16 = 0;
    let columns_size = u16::try_from(column_length(current_dir).0)
        .unwrap();
    let win_size = window_size().unwrap().columns;


    let mut win_width = win_size;
    println!("{}", win_width);


    let paths = fs::read_dir(current_dir).expect("-none");
    
    for path in paths {
        let entry = path.unwrap();
        let path_dir = entry.path().is_dir();
        let file_path: String = entry.file_name().into_string().unwrap();
        let path_size16 = pad + u16::try_from(file_path.len()).unwrap();
        if win_width < columns_size {
            win_width = win_size;
            println!();
        }
        if path_dir && !file_path.as_str().starts_with('.') {
            print!("{}", file_path.bright_blue());
            win_width -= columns_size;
            let mut cur_point = columns_size - path_size16;
            if cur_point != 0 {
                while cur_point > 0 {
                    cur_point -= 1;
                    print!("{}", " ");
                } 
            }
        } else if entry.path().is_file() {
            let file_path: String = entry.file_name().into_string().unwrap();
            if !file_path.as_str().starts_with('.') {
                print!("{}", file_path.white());
                win_width -= columns_size;
                let mut cur_point = columns_size - path_size16;
                if cur_point != 0 {
                    while cur_point > 0 {
                        cur_point -= 1;
                        print!("{}", " ");
                    } 
                }
            }
        }

    }

    println!("\n");
}
