use std::fs::{self, metadata, DirEntry};
use std::io;
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// TODO 
// page down 
// page up 
// home
// end
// highlight previous directory when navigating up the tree
// filter
// enter key
// crashes going up from root
// sorting
// tidy output, 
// dates, 
// sizes
// resume from previously selected file on first call
// navigation tree
// disks
// home etc
// bookmarks
// mouse?

fn main() {
    let mut got_file = false;
    let mut chosen_file_string: String = String::from("/home/john");
    let mut chosen_file_clone;
    let mut cancelled = false;
    let mut go_parent = false;
    let mut da_parent;
    let mut browse_result;
    let mut md;
    while !got_file && !cancelled {
        browse_result = browse(&chosen_file_string);
        // save the original variable to use in order to get .parent() if necessary
        chosen_file_clone = chosen_file_string.clone();
        chosen_file_string = browse_result.0;
        go_parent = browse_result.1;
        cancelled = browse_result.2;
        if go_parent {
            da_parent = PathBuf::from(chosen_file_clone)
                .parent()
                .unwrap()
                .to_path_buf();
            chosen_file_string = da_parent.clone().into_os_string().into_string().unwrap();
        }
        let temp = chosen_file_string.clone();
        md = metadata(temp).unwrap();
        got_file = !md.is_dir();
    }
    if cancelled {
        println!("Operation was cancelled");
    } else if got_file {
        println!("{:?}", chosen_file_string);
    }
}

fn browse(browse_path: &str) -> (String, bool, bool) {
    let mut line_num = 0; // real entry number of currently selected / highlighted line
    let mut offset = 0; // Tracks scrolling. Equals index of entry at the top of the page
    const MAX_LINES: usize = 30; // How many entries to display in one page
    let mut entry_index = 0; // index used to access correct range of entries when displaying. Will start from offset and finish at offset + MAX_LINES -1
    let mut cancelled = false; // flag to say whether routine ended with picking a file or cancelling the operation
    let mut go_parent = false; // flag saying when user wants to navigate back up a directory

    let stdin = stdin();
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    // clearing the screen and going to left top corner with the cursor
    write!(
        stdout,
        r#"{}{}"#,
        termion::cursor::Goto(1, 1),
        termion::clear::All
    )
    .unwrap();
    stdout.flush().unwrap();
    let mut entries;
    let temp_path = browse_path.clone();
    entries = fs::read_dir(temp_path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
    entries.sort();
    entry_index = offset;
    for i in offset..offset + MAX_LINES {
        if i < entries.len() {
            if entries[i].is_dir() {
                print!("[FOLDER] ");
            }
            if entry_index == line_num {
                println!(
                    "{invert}{:?}{reset}\r",
                    entries[i],
                    reset = termion::style::Reset,
                    invert = termion::style::Invert
                );
            } else {
                println!("{:?}\r", entries[i]);
            }
            entry_index += 1;
        }
    }
    stdout.flush().unwrap();
    //detecting keydown events
    for c in stdin.keys() {
        entries = fs::read_dir(temp_path)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();

        //clearing the screen and going to top left corner
        entries.sort();
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        match c.unwrap() {
            Key::Char(' ') => {
                cancelled = false;
                break;
            }
            Key::Esc => {
                cancelled = true;
                break;
            }
            Key::Backspace => {
                go_parent = true;
                break;
            }
            Key::Down => {
                if line_num as u16 - offset as u16 <= MAX_LINES as u16 {
                    line_num += 1;
                }
                if line_num >= entries.len() {
                    line_num = entries.len() - 1;
                }
            }
            Key::Up => {
                if line_num > 0 {
                    line_num -= 1;
                }
            }
            _ => (),
        }
        if line_num >= offset + MAX_LINES {
            offset += 1;
        }
        if line_num < offset {
            offset = line_num;
        }
        // println!("line_num={}, offset={}\r", line_num, offset);
        entry_index = offset;
        for i in offset..offset + MAX_LINES {
            if i < entries.len() {
                if entries[i].is_dir() {
                    print!("[FOLDER] ");
                }
                if entry_index == line_num {
                    println!(
                        "{invert}{:?}{reset}\r",
                        entries[i],
                        reset = termion::style::Reset,
                        invert = termion::style::Invert
                    );
                } else {
                    println!("{:?}\r", entries[i]);
                }
                entry_index += 1;
            }
        }
        stdout.flush().unwrap();
    }
    let temp2 = entries[line_num]
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    //*browse_path = entries[line_num].clone();

    (String::from(temp2), go_parent, cancelled)
}
