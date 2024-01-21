// use dirs;
use chrono::prelude::{DateTime, Utc};
use std::path::{PathBuf};

fn iso8601(st: &std::time::SystemTime) -> (String, String, String) {
    let dt: DateTime<Utc> = st.clone().into();
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
	if dir.chars().next() == Some('~') {
		if let Some(home_dir) = dirs::home_dir() {
			dir = format!("{}{}{}",home_dir.display(), std::path::MAIN_SEPARATOR,dir[1..].to_owned());
		}
	}

	// ensure that the per-month subdirectory is present
	let (month, _day, timestamp) = iso8601((&std::time::SystemTime::now()).into());
	let path = PathBuf::from(&format!("{}{}{}", dir, std::path::MAIN_SEPARATOR, month));
	std::fs::create_dir_all(path);

	let args = std::env::args().skip(1).collect::<Vec<String>>();

	let output = std::process::Command::new("git").args(args.clone()).output().expect("failed to run process");
	if output.stdout.len() == 0 {
		let Ok(error) = String::from_utf8(output.stderr.clone()) else {
			panic!("{:?}", output.stderr);
		};
		println!("Error running command: {}", error);
		std::process::exit(1);
	} else {
		let Ok(stdout) = String::from_utf8(output.stdout.clone()) else {
			panic!("{:?}", output.stdout);
		};
	}



	let log_line = format!("{}\t{}\t{}", timestamp, std::env::current_dir().expect("unable to get present working directory path").display(), args.join(" "));
	println!("hey git log {}", log_line);
}
