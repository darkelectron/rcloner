use std::process::Command;
use std::process::Stdio;

use fzf_wrapped::Fzf;
use fzf_wrapped::run_with_output;

mod args;
mod config;

use args::EntityType;
use args::RclonerArgs;
use clap::Parser;

use config::read_config;

fn get_remote() -> String {
    let mut remotes = Vec::new();

    match read_config() {
        Ok(config) => {
            remotes = config.rdrives;
        }
        Err(e) => println!("Error reading config: {}", e),
    }

    let users_selection = run_with_output(Fzf::default(), remotes).expect("Something went wrong!");

    println!("Using Cloud Service: {}", users_selection);

    return users_selection;
}

fn list_files() {
    // get_remote();
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

    let remotes = vec!["megadrive:", "gdrive:", "nextcloud:", "proton:"];
    let mut mount_point_index = 0;

    // let target = "banana".to_string();
    // Iterate over the Vec with indices
    for (index, value) in remotes.iter().enumerate() {
        // Check if the current value matches the target
        if *value == remote {
            mount_point_index = index;
            // Print the index
            println!("Found '{}' at index {}", remote, index);
            // Optionally, break the loop if you only want the first occurrence
            break;
        }
    }

    let mount_points = vec!["Mega", "Google", "Nextcloud", "Proton"];

    let mount_point = mount_points[mount_point_index];

    let mut command = Command::new("rclone");
        command.arg("mount");
        command.arg("--daemon");
        command.arg(remote);
        command.arg(String::from("/home/darkelectron/Cloud/") + mount_point);

    let output = command.output().unwrap();

    println!("{}", String::from_utf8(output.stdout).unwrap());

    if !output.status.success() {
        let error_str = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", error_str);
    }
}

fn main() {
    let args: RclonerArgs = RclonerArgs::parse();

    match args.entity_type {
        EntityType::Copy(copy) => copy_files(copy.source, copy.target),
        EntityType::Mount(_) => mount_cloud_service(),
        EntityType::List(_) => list_files(),
    }
}

