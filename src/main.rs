use std::{
    fs,
    path::{Path, PathBuf},
};

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

    /// Add zeros like problem-1 & problem-11 -> problem-01 & problem-011 instead of -> problem-01 &
    /// problem-11
    #[arg(short, long)]
    basic_add: bool,
}

fn main() {
    let args = Cli::parse();
    let path = args.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            let num =
                change_leading_zeros(&path, args.decrease, args.number as usize, args.basic_add);
            if num != 0 {
                println!(
                    "{}{}{}",
                    "Successfully changed ".green(),
                    num.green(),
                    " filenames.".green()
                );
            } else {
                println!("{}", "Changed zero filenames.".red());
            }
        } else {
            println!("{}", "Path does not exist".red());
        }
    } else {
        println!("{}", "Error reading directory".red());
    }
}

fn change_leading_zeros(path: &Path, dec: bool, count: usize, basic: bool) -> usize {
    let filenames = get_filenames(path);

    if basic && dec {
        println!(
            "{}",
            "Basic is only an option for increasing. Decreasing normally.".yellow()
        );
    }

    let mut target: usize = 0;

    if !basic && !dec {
        let mut minimum = usize::MAX;
        for file in &filenames {
            if !file.chars().last().unwrap_or(' ').is_ascii_digit() {
                continue;
            }

            let mut split_index = 0;
            for (i, j) in file.char_indices().rev() {
                if !j.is_ascii_digit() {
                    split_index = i + j.len_utf8();
                    break;
                }
            }

            minimum = minimum.min(file[split_index..].len())
        }

        target = minimum + count;
    }

    let mut files_changed: usize = 0;

    for file in filenames {
        if !file.chars().last().unwrap_or(' ').is_ascii_digit() {
            continue;
        }

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
            let new = if basic {
                format!("{}{}{}", prefix, "0".repeat(count), numeric)
            } else {
                let pad = target.saturating_sub(numeric.len());
                format!("{}{}{}", prefix, "0".repeat(pad), numeric)
            };

            let success = fs::rename(path.join(&file), path.join(&new));

            if let Err(fail) = success {
                println!(
                    "File rename {} -> {} failed with: {}",
                    &file.blue(),
                    &new.blue(),
                    fail.red()
                );
            } else {
                files_changed += 1;
            }

            continue;
        }

        let remove = numeric
            .chars()
            .take_while(|&c| c == '0')
            .count()
            .min(count)
            .min(numeric.len() - 1);
        let new = format!("{}{}", prefix, &numeric[remove..]);
        let success = fs::rename(path.join(&file), path.join(&new));

        if let Err(fail) = success {
            println!(
                "File rename {} -> {} failed with: {}",
                &file.blue(),
                &new.blue(),
                fail.red()
            );
        } else {
            files_changed += 1;
        }
    }

    files_changed
}

fn get_filenames(path: &Path) -> Vec<String> {
    let mut names = Vec::new();
    if let Ok(read_dir) = fs::read_dir(path) {
        for file in read_dir.flatten() {
            names.push(
                file.file_name()
                    .into_string()
                    .unwrap_or("unknown name".into()),
            );
        }
    }
    names
}
