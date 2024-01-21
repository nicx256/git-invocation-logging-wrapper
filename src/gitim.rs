// use dirs;
use chrono::prelude::{DateTime, Utc};
use std::io::prelude::*;
use std::path::PathBuf;

fn datetime_strings(st: &std::time::SystemTime) -> (String, String, String) {
    let dt: DateTime<Utc> = (*st).into();
    let month = format!("{}", dt.format("%Y-%m")); // e.g. "2024-01"
    let day = format!("{}{}", month, dt.format("-%d")); // "2024-01-21"
    let timestamp = format!("{}{}", day, dt.format(" %R")); // "2024-01-24 04:06"
    (month, day, timestamp)
}

fn main() {
    let Ok(mut dir) = std::env::var("TIMESHEET_DIR") else {
        println!("Environment variable TIMESHEET_DIR not set. Example value: `~/sync/timesheets/`");
        std::process::exit(1);
    };

    // replace tilde at start of configured dir with user's home directory
    if dir.starts_with('~') {
        if let Some(home_dir) = dirs::home_dir() {
            dir = format!(
                "{}{}{}",
                home_dir.display(),
                std::path::MAIN_SEPARATOR,
                dir[1..].to_owned()
            );
        }
    }

    // ensure that the per-month subdirectory is present
    let (month, day, timestamp) = datetime_strings(&std::time::SystemTime::now());
    let mut path = PathBuf::from(&format!("{}{}{}", dir, std::path::MAIN_SEPARATOR, month));
    std::fs::create_dir_all(path.clone()).unwrap_or_else(|_| panic!("Unable to create path: {}", path.display()));

    // invoke git, collect and show output
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let output = std::process::Command::new("git")
        .args(args.clone())
        .output()
        .expect("failed to run process");

    if output.stdout.is_empty() {
        let Ok(error) = String::from_utf8(output.stderr.clone()) else {
            panic!("{:?}", output.stderr);
        };
        println!("Error running command: {}", error);
        std::process::exit(1);
    }

    let output_string = String::from_utf8(output.stdout.clone()).unwrap_or_else(|_| panic!("Unable to show output of command: {:?}", output.stdout));
    println!("{}", output_string);

    let log_line = format!(
        "{}\t{}\t{}",
        timestamp,
        std::env::current_dir()
            .expect("unable to get present working directory path")
            .display(),
        args.join(" ")
    );

    path.push(day);

    let mut file = if path.exists() {
        std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(path.clone())
            .unwrap_or_else(|_| panic!("Unable to open file for writing: {:?}", path))
    } else {
        std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path.clone())
            .unwrap_or_else(|_| panic!("Unable to open file for writing: {:?}", path))
    };

    if let Err(_e) = writeln!(file, "{}", log_line) {
        eprintln!("Couldn't write to file: {:?}", path);
    }
}
