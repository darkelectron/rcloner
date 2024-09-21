use std::process::exit;
use std::process::Command;
use std::process::Stdio;
use std::path::Path;
use std::fs;

extern crate notify_rust;
use notify_rust::Notification;

use fzf_wrapped::Fzf;
use fzf_wrapped::run_with_output;

mod args;

use args::EntityType;
use args::RclonerArgs;
use clap::Parser;

fn list_remotes() -> Vec<String> {
    let output = Command::new("rclone")
        .arg("listremotes")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);    let remotes: Vec<String> = stdout
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    remotes
}

fn get_remote() -> String {
    let remotes = list_remotes();

    let users_selection = run_with_output(Fzf::default(), remotes).expect("Something went wrong!");

    if !users_selection.is_empty() {
        println!("Using Cloud Service: {}", users_selection);
        return users_selection;
    } else {
        println!("Nothing Selected");
        exit(1);
    }
}

fn list_files() {
    let remote = get_remote();

    let mut command = Command::new("rclone");
        command.arg("ls");
        command.arg(remote);
        command.stdout(Stdio::inherit());

    let output = command.output().unwrap();

    if !output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", output_str);
    }
}

fn copy_files(source: String, target: String) {
    let remote = get_remote();

    let mut command = Command::new("rclone");
        command.arg("copy");
        command.arg("--progress");
        command.arg("--verbose");
        command.arg(source);
        command.arg(remote + "/" + &target);
        command.stdout(Stdio::inherit());

    let output = command.output().unwrap();

    println!("{}", String::from_utf8(output.stdout).unwrap());

    if !output.status.success() {
        let error_str = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", error_str);
    }
}

fn mount_cloud_service() {
    let remote = get_remote();

    // let mut mount_point = remote.clone();
    let mount_point = remote.trim_matches(|c| c == '"' || c == ':').to_string();

    let path_name = "/home/darkelectron/Cloud/".to_owned() + &mount_point;
    let dir_path = Path::new(&path_name);

    if dir_path.is_dir() {
        println!("Directory exists!");
    } else {
        println!("Directory does not exist.");
        fs::create_dir(dir_path).expect("Failed to create Directory");
    }

    let mut command = Command::new("rclone");
        command.arg("mount");
        command.arg("--daemon");
        command.arg(remote.clone());
        command.arg(dir_path);

    let output = command.output().unwrap();

    println!("{}", String::from_utf8(output.stdout).unwrap());

    if !output.status.success() {
        let error_str = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", error_str);
    }
    Notification::new()
        .summary("Drive Mounted")
        .body("Drive was mounted successfully")
        .icon("rclone-browser")
        .show().unwrap();

}

fn main() {
    let args: RclonerArgs = RclonerArgs::parse();

    match args.entity_type {
        EntityType::Copy(copy) => copy_files(copy.source, copy.target),
        EntityType::Mount(_) => mount_cloud_service(),
        EntityType::List(_) => list_files(),
    }
}

