use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use walkdir::WalkDir;
use rand::prelude::*;
use clap::Parser;

/// Random Wallpaper picker using swww
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of your wallpaper directory
    #[arg(short, long)]
    path: String,

    /// Seconds between wallpaper changes
    #[arg(short, long, default_value_t = 300)]
    duration: u64,
}

fn main() {
    let args = Args::parse();
    let mut rng = thread_rng();

    loop {
        let mut files = Vec::new();

        for entry in WalkDir::new(&args.path) {
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

        sleep(Duration::from_secs(args.duration));
    }
}
