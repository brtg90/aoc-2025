use std::env::current_dir;
use std::fs;
use std::io::{Write, Result};
use std::path::PathBuf;
use std::process::Command;
use regex::Regex;

fn main() {
    let cwd = current_dir().expect("Failed to read current directory");
    let new_day = get_new_day_number(&cwd).expect("Failed to get new day");
    println!("Creating new day: {}", new_day);
    let new_day_path = cwd.join(new_day.to_string());
    println!("Setting up files for the project");
    let _ = create_new_day(&new_day).expect("Failed to create new day");
    let _ = setup_files(&new_day_path).expect("Failed to setup files");
    let _ = add_input_files(&new_day);
}

fn create_new_day(day: &str) -> Result<()> {
    let output = Command::new("cargo")
        .args(["new", &day])
        .output()
        .expect("Failed to create new day");

    if !output.status.success() {
        eprintln!("Cargo command failed with status: {}", output.status);
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

fn get_new_day_number(dir: &PathBuf) -> Result<String> {
    let mut day_vec = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() && is_day_folder(&path) {
            day_vec.push(path.file_name().unwrap().to_owned());
        }
    }

    if day_vec.is_empty() {
        return Ok(String::from("day01"));
    }

    day_vec.sort();
    let last_day = day_vec[day_vec.len() - 1]
        .to_str()
        .expect("Could not parse str from OsString")
        .trim_start_matches("day")
        .parse::<usize>()
        .expect("Could not parse day number");
    let new_day = format!("day{:02}", last_day + 1);

    Ok(new_day)
}

fn is_day_folder(path: &PathBuf) -> bool {
    let re = Regex::new(r"day\d\d").unwrap();
    let folder_name = path.file_name().expect("Could not get filename").to_string_lossy();
    re.is_match(&folder_name)
}

fn setup_files(dir: &PathBuf) -> Result<()> {
    let main_path = dir.clone().join("src").join("main.rs");
    let toml_path = dir.join("Cargo.toml");
    setup_main(&main_path)?;
    setup_cargo_toml(&toml_path)?;
    Ok(())
}

fn setup_main(path: &PathBuf) -> Result<()> {
    let cwd = current_dir()?;
    let new_main = cwd.join("add-day").join("main-template.rs");

    fs::copy(new_main, path)?;
    Ok(())
}

fn setup_cargo_toml(path: &PathBuf) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(path)?;

    write!(file, "{}", "utils = { path = \"../utils\" }")?;
    Ok(())
}

fn add_input_files(day: &str) -> Result<()> {
    if !fs::exists("/inputs")? {
        fs::create_dir("/inputs")?;
    }

    for i in 0..3 {
        let filename = format!("{day}pt{}.txt", i + 1);
        fs::File::create(format!("inputs/{filename}"))?;
    }

    Ok(())
}

