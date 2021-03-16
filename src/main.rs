use std::fs::{self, metadata, DirEntry};
use std::io;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let mut got_file = false;
    let mut chosen_file: String = String::from("/home/john");
    let mut md;
    while !got_file {
        chosen_file = browse(&chosen_file);
        let temp = chosen_file.clone();
        md = metadata(temp).unwrap();
        got_file = !md.is_dir();
    }
    println!("{:?}", chosen_file);
}

fn browse(browse_path: &str) -> String {
    //, chosen_file: &String) {
    let mut line_num = 0; // real entry number of currently selected / highlighted line
    let mut offset = 0; // Tracks scrolling. Equals index of entry at the top of the page
    const MAX_LINES: usize = 30; // How many entries to display in one page
    let mut entry_index = 0; // index used to access correct range of entries when displaying. Will start from offset and finish at offset + MAX_LINES -1

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
    entries = fs::read_dir(browse_path)
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
        entries = fs::read_dir(browse_path)
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
            Key::Char(' ') => break,
            Key::Ctrl('q') => break,
            Key::Alt('t') => println!("termion is cool"),
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
                    //as u16 - offset as u16 <= 0 {
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
    String::from(temp2)
    //String::from(entries[line_num].to_str());
}
