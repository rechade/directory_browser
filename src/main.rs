use std::fs::{self, metadata, DirEntry};
use std::io;
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// TODO
// highlight previous directory when navigating up the tree
// enter key
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
// presentation - folder at top line, stems in the list,
// make into a crate, with less boiler plate around call
// try to call read_dir only when necessary
// home directory as default

fn main() {
    let mut got_file = false;
    let mut chosen_file_string: String = String::from("/home/john/Music/scratches/doot-wah.wav");
    //let mut chosen_file_clone;
    let mut cancelled = false;
    let mut go_parent = false;
    let mut da_parent;
    let mut browse_result: (String, bool, bool);
    //let mut md: u8;
    let mut load_location_clone = chosen_file_string.clone();
    while !got_file && !cancelled {
        let browse_result = browse(&load_location_clone);
        // save original to use to navigate to parent if necessary
        let prev_string = load_location_clone.clone();
        load_location_clone = browse_result.0;
        go_parent = browse_result.1;
        cancelled = browse_result.2;
        if go_parent {
            if prev_string == "/" {
                da_parent = PathBuf::from("/");
            } else {
                da_parent = PathBuf::from(prev_string).parent().unwrap().to_path_buf();
            }
            load_location_clone = da_parent.clone().into_os_string().into_string().unwrap();
        } else if cancelled {
            load_location_clone = prev_string.clone();
        }
        let temp = load_location_clone.clone();
        let md = metadata(temp).unwrap();
        got_file = !md.is_dir();
    }
    if !cancelled {
        println!("{:?}", load_location_clone);
        //c.send((true, load_location_clone.clone())).unwrap();
    } else {
        println!("Cancelled");
    }
}
pub fn browse(browse_string: &str) -> (String, bool, bool) {
    // check it's crashing on the bottom line somehow and up from root TODO

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
    let mut temp_string = browse_string.clone();
    let browse_buf = PathBuf::from(browse_string.clone());

    let parent_buf;
    // check if the input is a directory so that read_dir doesn't break
    // set temp_string to the parent if so
    if !browse_buf.is_dir() {
        parent_buf = browse_buf.parent().unwrap().clone().to_path_buf();
        temp_string = parent_buf.to_str().unwrap();
    }
    // output the folder name as top line
    println!("{}\r", temp_string);
    entries = fs::read_dir(temp_string)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
    entries.sort();

    // remove hidden (dot) files
    for temp_index in (0..entries.len()).rev() {
        if entries[temp_index]
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with(".")
        {
            entries.remove(temp_index);
        }
    }

    for temp_index in (0..entries.len()).rev() {
        // leave remaining dirs
        if !entries[temp_index].is_dir() {
            // filter the file if it doesn't end in accepted format extension
            if !(entries[temp_index]
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .ends_with(".mp3")
                || entries[temp_index]
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .ends_with(".MP3")
                || entries[temp_index]
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .ends_with(".wav")
                || entries[temp_index]
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .ends_with(".WAV")
                || entries[temp_index]
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .ends_with(".ogg")
                || entries[temp_index]
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .ends_with(".OGG")
                || entries[temp_index]
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .ends_with(".flac")
                || entries[temp_index]
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .ends_with(".FLAC"))
            {
                entries.remove(temp_index);
            }
        }
    }
    for temp_index in 0..entries.len() {
        let entry_string = entries[temp_index]
            .clone()
            .into_os_string()
            .into_string()
            .unwrap();
        if entry_string == browse_string {
            line_num = temp_index;
            offset = line_num;
        }
    }
    entry_index = offset;
    for i in offset..offset + MAX_LINES {
        if i < entries.len() {
            if entries[i].is_dir() {
                print!("   [FOLDER] ");
            }
            if entry_index == line_num {
                println!(
                    "   {invert}{:?}{reset}\r",
                    entries[i].file_name().unwrap(),
                    reset = termion::style::Reset,
                    invert = termion::style::Invert
                );
            } else {
                println!("   {:?}\r", entries[i].file_name().unwrap());
            }
            entry_index += 1;
        }
    }
    stdout.flush().unwrap();

    //detecting keydown events
    for c in stdin.keys() {
     
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();
        // output folder name as top line first
        println!("{}\r", temp_string);
        match c.unwrap() {
            Key::Char(' ') => {
                if entries.len() == 0 {
                    // do nothing
                } else {
                    cancelled = false;
                    break;
                }
            }
            Key::Char('+') => {
                if entries.len() == 0 {
                    // do nothing
                } else {
                    cancelled = false;
                    break;
                }
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
                if line_num > entries.len() - 1 {
                    line_num = entries.len() - 1;
                }
                if line_num >= offset + MAX_LINES {
                    offset += 1;
                }
            }
            Key::Up => {
                if line_num > 0 {
                    line_num -= 1;
                    if line_num < offset {
                        offset = line_num;
                    }
                }
            }
            Key::PageUp => {
                if line_num < MAX_LINES {
                    line_num = 0;
                    offset = 0;
                } else {
                    if offset == line_num {
                        line_num = line_num - MAX_LINES;
                        offset = offset - MAX_LINES;
                    } else {
                        line_num = offset;
                    }
                }
            }
            Key::Home => {
                line_num = 0;
                offset = 0;
            }
            Key::End => {
                line_num = entries.len() - 1;
                if offset + MAX_LINES > entries.len() - 1 {
                    // next index would be gt last index
                    // leave the offset alone if the last file is already visible
                } else {
                    offset = entries.len() - MAX_LINES + 1;
                }
            }
            Key::PageDown => {
                if line_num != entries.len() - 1 {
                    if offset + MAX_LINES > entries.len() - 1 {
                        // last dir entry is already visible on screen, so
                        // don't change the rest of the display
                        line_num = entries.len() - 1;
                    } else {
                        let old_num = line_num;
                        line_num += MAX_LINES;
                        if line_num > entries.len() - 1 {
                            line_num = entries.len() - 1;
                            offset = old_num + 1;
                        } else {
                            offset = offset + MAX_LINES;
                        }
                    }
                }
            }
            _ => (),
        }
        entry_index = offset;
        for i in offset..offset + MAX_LINES {
            if i < entries.len() {
                if entries[i].is_dir() {
                    print!("   [FOLDER] ");
                }
                if entry_index == line_num {
                    println!(
                        "   {invert}{:?}{reset}\r",
                        entries[i].file_name().unwrap(),
                        reset = termion::style::Reset,
                        invert = termion::style::Invert
                    );
                } else {
                    println!("   {:?}\r", entries[i].file_name().unwrap());
                }
                entry_index += 1;
            }
        }
        stdout.flush().unwrap();
    }
    // clearing the screen and going to left top corner with the cursor
    write!(
        stdout,
        r#"{}{}"#,
        termion::cursor::Goto(1, 1),
        termion::clear::All
    )
    .unwrap();
    stdout.flush().unwrap();

    if cancelled || go_parent {
        (String::from(""), go_parent, cancelled)
    } else {
        let temp2 = entries[line_num]
            .clone()
            .into_os_string()
            .into_string()
            .unwrap();
        //*browse_string = entries[line_num].clone();
        (String::from(temp2), go_parent, cancelled)
    }
}
