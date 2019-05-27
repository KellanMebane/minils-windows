extern crate chrono;
extern crate filetime;

use minils::Config;
use minils::FileDisplayInfo;
use minils::InputOptions;

use chrono::prelude::*;
use filetime::FileTime;
use std::env;
use std::fs;
use std::os::windows::fs::MetadataExt;
use std::path::Path;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let show_hidden = config.options.contains(&InputOptions::ShowHidden);

    let path = Path::new(&config.filename);
    let directory = fs::read_dir(&path).unwrap_or_else(|err| {
        eprintln!("Problem opening `{}`, {}", path.to_str().unwrap(), err);
        process::exit(1);
    });

    let mut longest: u64 = 0;
    let mut lines: Vec<FileDisplayInfo> = Vec::new();

    for entry in directory {
        if let Ok(entry) = entry {
            let name = entry.file_name().into_string().unwrap();

            if let Ok(metadata) = entry.metadata() {
                let file_attributes =
                    minils::format_windows_file_attributes(metadata.file_attributes());

                let length = metadata.len();

                if length > longest {
                    longest = length;
                }

                let last_write_time = Local.timestamp(
                    FileTime::from_last_modification_time(&metadata).seconds_relative_to_1970()
                        as i64,
                    0,
                );

                lines.push(FileDisplayInfo {
                    hidden: file_attributes.contains("h"),
                    last_write_time: last_write_time,
                    name: name,
                    file_attributes: file_attributes,
                    length: length,
                });
            }
        }
    }

    for line in &lines {
        if !line.hidden || show_hidden {
            println!(
                "{:5}       {:19}       {:>longest$}      {}",
                line.file_attributes,
                line.last_write_time.format("%m/%d/%Y  %l:%M %p"),
                match line.length {
                    length if length > 0 => length.to_string(),
                    _ => String::from(""),
                },
                line.name,
                longest = minils::count_digits(longest)
            );
        }
    }
}
