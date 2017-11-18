#[macro_use]
extern crate clap;
extern crate chrono;
use clap::App;
use std::process::Command;
use std::env::home_dir;


static LOG_FILE_NAME: &'static str = "terminal_log";


fn main() {
    let cli_yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(cli_yaml).get_matches();

    match matches.subcommand_matches("start") {
        Some(_) => start_logging(),
        None => {},
    }

    match matches.subcommand_matches("history") {
        Some(_) => show_history(),
        None => {},
    }
}

fn start_logging() {
    match home_dir() {
        Some(mut path) => {
            path.push(LOG_FILE_NAME);
            let log_path = path.to_str().unwrap();
            Command::new("script").arg(log_path).status().unwrap();
        },
        None => {},
    }
}

fn show_history() {
    // TODO: stop script when launching such as vim
    match home_dir() {
        Some(mut path) => {
            path.push(LOG_FILE_NAME);
            if !path.exists() {
                return;
            }
            let log_path = path.to_str().unwrap();
            let cleansed_log_path = cleanse_log(log_path);
            Command::new("sh").arg("-c").arg(format!("vim \"+normal G$\" {}", cleansed_log_path)).status().unwrap();
        },
        None => {},
    }
}

fn cleanse_log(log_file_path: &str) -> &str {
    let cleansed_path = "/tmp/cleansed_log";
    let cmd = format!("col -bp <{} | grep -v -e '^$' >{}", log_file_path, cleansed_path);
    Command::new("sh").arg("-c").arg(cmd).status().unwrap();
    return cleansed_path;
}
