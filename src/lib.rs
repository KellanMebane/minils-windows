pub enum WindowsFileAttributes {
    Archive = 32,
    Compressed = 2048,
    Device = 64,
    Directory = 16,
    Encrypted = 16834,
    Hidden = 2,
    IntegrityStream = 32768,
    Normal = 128,
    NoScrubData = 131072,
    NotContentIndexed = 8192,
    ReadOnly = 1,
    ReparsePoint = 1024,
    SparseFile = 512,
    System = 4,
    Temporary = 256,
}

pub fn format_windows_file_attributes(file_attributes: u32) -> String {
    let mut formatted = vec!['-', '-', '-', '-', '-', '-'];

    if file_attributes & WindowsFileAttributes::Directory as u32 > 0 {
        formatted[0] = 'd';
    }
    if file_attributes & WindowsFileAttributes::Archive as u32 > 0 {
        formatted[1] = 'a';
    }
    if file_attributes & WindowsFileAttributes::ReadOnly as u32 > 0 {
        formatted[2] = 'r';
    }
    if file_attributes & WindowsFileAttributes::Hidden as u32 > 0 {
        formatted[3] = 'h';
    }
    if file_attributes & WindowsFileAttributes::System as u32 > 0 {
        formatted[4] = 's';
    }
    if file_attributes & WindowsFileAttributes::ReparsePoint as u32 > 0 {
        formatted[5] = 'l';
    }

    formatted.into_iter().collect()
}

pub struct FileDisplayInfo {
    pub hidden: bool,
    pub last_write_time: chrono::DateTime<chrono::Local>,
    pub name: String,
    pub file_attributes: String,
    pub length: u64,
}

pub fn count_digits(mut n: u64) -> usize {
    let mut digits: usize = 0;

    while n > 0 {
        n = n / 10;
        digits += 1;
    }

    digits
}

#[derive(PartialEq)]
pub enum InputOptions {
    ShowHidden,
}

pub struct Config {
    pub filename: String,
    pub options: Vec<InputOptions>,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, String> {
        args.next(); // command call
        let inputs: Vec<String> = args.collect();

        let mut options: Vec<InputOptions> = Vec::new();
        let mut filename = String::new();

        for input in &inputs {
            if input.starts_with("-") {
                match input.as_str() {
                    "-h" => options.push(InputOptions::ShowHidden),
                    _ => {
                        let bad_input = format!("Unrecognized input option: `{}`", input);
                        return Err(bad_input);
                    }
                }
            } else {
                if filename.len() == 0 {
                    filename = input.clone();
                }
            }
        }

        if filename.len() == 0 {
            filename = String::from(".");
        }

        Ok(Config { filename, options })
    }
}