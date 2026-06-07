use std::{fs, path::{Path, PathBuf}};

use clap::Parser;

use owo_colors::OwoColorize;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Add and remove leading zeros from filenames",
    long_about = "A utility to add and remove leading zeros from the numbers at the ends of filenames"
)]
struct Cli {
    path: Option<PathBuf>,
    /// The amount of zeros being added or removed
    #[arg(short, long, default_value_t = 1)]
    number: u8,

    /// Decrease the amount of leading zeros rather than raising it
    #[arg(short, long)]
    decrease: bool,
}

fn main() {
    let args = Cli::parse();
    let path = args.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            change_leading_zeros(&path, args.decrease, args.number);
        } else {
            println!("{}", "Path does not exist".red());
        }
    } else {
        println!("{}", "Error reading directory".red());
    }
}

fn change_leading_zeros(path: &Path, dec: bool, count: u8) {
    let filenames = get_filenames(path);

    for file in filenames {
        if !file.chars().last().unwrap_or(' ').is_ascii_digit() { continue; }

        let mut split_index = 0;
        for (i, j) in file.char_indices().rev() {
            if !j.is_ascii_digit() {
                split_index = i + j.len_utf8();
                break;
            }
        }

        let prefix = &file[..split_index];
        let numeric = &file[split_index..];
        
        if !dec {
            let new = format!("{}{}{}", prefix, "0".repeat(count as usize), numeric);
            let _ = fs::rename(path.join(&file), path.join(new));
            continue;
        }

        let remove = numeric
            .chars()
            .take_while(|&c| c == '0')
            .count()
            .min(count as usize)
            .min(numeric.len() - 1);
        let new = format!("{}{}", prefix, &numeric[remove..]);
        let _ = fs::rename(path.join(&file), path.join(new));
    }
}

fn get_filenames(path: &Path) -> Vec<String> {
    let mut names = Vec::new();
    if let Ok(read_dir) = fs::read_dir(path) {
        for file in read_dir.flatten() {
                names.push(file
                    .file_name()
                    .into_string()
                    .unwrap_or("unknown name".into()));
        }
    }
    names
}
