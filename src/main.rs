use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use walkdir::WalkDir;
use rand::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut rng = thread_rng();
    let mut file_path:&str;

    match args.len() {
        0 | 1 => {
            println!("No argument was supplied");
            return;
        },
        _ => {
            file_path = &args[1];
        }
    }

    loop {
        let mut files = Vec::new();

        for entry in WalkDir::new(file_path) {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                files.push(entry.path().to_owned());
            }
        }

        if !files.is_empty() {
            let random_file = files.choose(&mut rng).unwrap();

            let _output = Command::new("swww")
                .arg("img")
                .arg(random_file)
                .output()
                .expect("Failed to execute command");
        } else {
            println!("No files found");
        }

        sleep(Duration::from_secs(300));
    }
}
