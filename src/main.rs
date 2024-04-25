use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{Read, stdin, Write};
use std::path::{Path, PathBuf};
use std::process::{exit, Stdio};
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use reqwest::blocking::Client;
use reqwest::StatusCode;

use crate::process::Process;
mod config;
mod process;
mod logging;

fn main() {
    logging::Logger::load_default();
    let app_name = env!("CARGO_PKG_NAME").to_string();
    let app_header = text_to_ascii_art::convert(app_name.clone()).unwrap();
    let version = env!("CARGO_PKG_VERSION").to_string();
    let current_exe: PathBuf = std::env::current_exe().unwrap();
    let exe_dir: &Path = current_exe.parent().unwrap();
    
    loop {
        clearscreen::clear().expect("Failed to clear screen");
        println!("{}", app_header.blue());
        if !is_elevated::is_elevated() {
            println!("{}", "Please restart the program with elevated privileges (e.g., as an administrator or root user).".bright_red());
            pause();
            continue;
        }
        if !Path::new(&exe_dir.join(config::CMD_LOADER_PATH)).exists(){
            println!("== CMD LOADER Installation ==");

            println!("The CMD LOADER is a crucial component for this software.");
            println!("It allows you to execute essential commands.");

            println!("Do you want to install the CMD LOADER?");
            println!("(Type 'yes' to install or 'no' to cancel)");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            match input.trim().to_lowercase().as_str() {
                "yes" | "y" => {},
                "no" | "n" => {
                    pause();
                    exit(0);
                },
                _ => println!("Invalid input. Please enter 'yes' or 'no'.")
            }
            
            match download_file(config::CMD_LOADER_DOWNLOAD_LINK, exe_dir.join(config::CMD_LOADER_PATH).as_path().to_str().unwrap()) {
                Ok(_) => {
                    println!("CMD LOADER installed successfully");
                    pause();
                }
                Err(_) => {
                    println!("Failed to install cmd loader!");
                    pause();
                    exit(0);
                }
            }
            continue;
        }
        let processes = process::ProcessLoader::load();

        if let Err(_) = processes {
            println!("Failed to load processes!");
            println!("== System AutoFix ==");
            println!("DO you want to delete old cache and files?");
            println!("This can help to fix the error");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            match input.trim().to_lowercase().as_str() {
                "yes" | "y" => {
                    std::fs::remove_file(&exe_dir.join(config::CMD_LOADER_PATH)).unwrap();
                },
                "no" | "n" => {},
                _ => println!("Invalid input. Please enter 'yes' or 'no'.")
            }
            pause();
            continue;
        }

        let mut processes_map = HashMap::<usize, Process>::new();
        let info = Process::new("Information", "info", "");

        let line = format!("{}. ({}) {}", 0, info.short_name, info.long_name);
        println!("{}", line.blue());
        processes_map.insert(0, info);
        let mut counter = 1;
        for process in processes.unwrap() {
            let line = format!("{}. ({}) {}", counter, process.short_name, process.long_name);
            println!("{}", line.blue());
            processes_map.insert(counter, process);
            counter += 1;
        }

        println!("{}", "Please enter your choices: ".blue().bold());

        let mut user_input = String::new();
        print!("{}", "".italic().bright_cyan());
        stdin().read_line(&mut user_input).unwrap();


        let choices: Vec<usize> = user_input
            .split_whitespace()
            .filter_map(|choice| choice.parse().ok())
            .collect();

        if choices.contains(&0) {
            clearscreen::clear().expect("Failed to clear screen");
            println!("{}", app_header.blue());
            println!("== INFORMATION ==");
            println!("Version {}", version);
            println!("{} is an open-source software developed by AMMARDEV.", &app_name);
            println!("This application is provided free of charge, but all copyrights are reserved by AMMARDEV.");
            println!("Disclaimer: This application utilizes Windows built-in tools for system optimization and repair.");
            println!("Users are advised to exercise caution and understand that any actions performed with {} are at their own discretion and risk.", &app_name);
            println!("For security and authenticity, ensure that you download the software only from the official GitHub repository:");
            println!("{}", "https://github.com/ammardevz".italic());

            pause();
            continue;
        }
        for choice in choices {

            if let Some(process) = processes_map.get(&choice) {
                let mut cmd = std::process::Command::new("cmd.exe")
                    .arg("/c")
                    .args(process.cmd.split(" "))
                    .stdout(Stdio::piped())
                    .stdin(Stdio::piped())
                    .spawn().unwrap();

                if let Some(mut stdin) = cmd.stdin.take() {
                    stdin.write_all(b"y").unwrap();
                }

                let waiting_bar = ProgressBar::new_spinner();
                waiting_bar.enable_steady_tick(Duration::from_millis(100));
                waiting_bar.set_message(format!("Running {}", process.short_name).yellow().to_string());
                let output = cmd.wait_with_output().unwrap();

                waiting_bar.finish_with_message(format!("Operation '{}' completed", &process.short_name).green().to_string());
                if !output.stdout.is_empty() {
                    if let Ok(stdout) = String::from_utf8(output.stdout) {
                        log::info!("Command[{}] output:\n{}", process.short_name, stdout);
                    }
                }
            }
        }
        println!("{}", "It is highly recommended to restart the operating system for optimal results.".yellow());
        pause();
    }
}


fn download_file(url: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    // Create a progress bar
    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {percent}%")
            .unwrap(),
    );

    // Create a reqwest client
    let client = Client::new();

    // Send the GET request and download the file
    let mut response = client.get(url).send()?;

    // Check if the server returned a successful response
    if response.status() != StatusCode::OK {
        panic!("Failed to download file: {}", response.status());
    }

    // Create a file to save the downloaded content
    let mut file = File::create(file_name)?;

    // Get the total file size from the Content-Length header
    let total_size = response.content_length().unwrap_or(0);
    let total_size_mb = total_size as f64 / (1024.0 * 1024.0);
    pb.set_length(total_size);

    // Read the response body and save it to the file
    let mut downloaded_bytes = 0;
    let mut buffer = [0; 4096];
    loop {
        let bytes_read = response.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
        downloaded_bytes += bytes_read as u64;
        pb.set_position(downloaded_bytes);
    }

    // Finish the progress bar
    pb.finish();

    println!("File downloaded successfully! Total Size: {:.2} MB", total_size_mb);

    Ok(())
}

fn pause() {
    println!("{}", "Press Enter to continue...".bold().cyan());
    stdin().read_line(&mut "".to_string()).unwrap();
}
