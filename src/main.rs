// use std::fs;
// use std::fs::File;
// use std::io::{Write, BufReader, BufRead, Error};
use std::process::Command;

use fzf_wrapped::Fzf;
use fzf_wrapped::run_with_output;

mod args;

use args::EntityType;
use args::RclonerArgs;
use clap::Parser;

// fn read_file() {
//     let sources_filenames = "/home/darkelectron/Falcon/Tools/rcloner/sources_filenames.txt";
//
//     let info = fs::read_to_string(sources_filenames).expect("The file could not be read");
//     println!("{}", info);
// }
//
// fn write_file() -> Result<(), Error> {
//     let sources_filenames = "/home/darkelectron/Falcon/Tools/rcloner/sources_filenames.txt";
//
//     let mut output = File::create(sources_filenames)?; // opens file for writing
//     write!(output, "New thing")?;
//
//     /* opens file for reading */
//     let input = File::open(sources_filenames)?;
//     let buffered = BufReader::new(input);
//
//     for line in buffered.lines() {
//         println!("{}", line?);
//     }
//
//     Ok(())
// }

fn get_remote() -> String {

    let remotes = vec!["megadrive:", "gdrive:", "nextcloud:", "proton:"];

    let users_selection = run_with_output(Fzf::default(), remotes).expect("Something went wrong!");

    println!("Using Cloud Service: {}", users_selection);

    return users_selection;
}

fn list_files() {
    // get_remote();
    let remote = get_remote();

    let output = Command::new("rclone")
        .arg("ls")
        .arg(remote)
        .output()
        .expect("failed");

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("{}", output_str);
    } else {
        let error_str = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", error_str);
    }
}

fn copy_files(source: String, target: String) {
    let remote = get_remote();

    let output = Command::new("rclone")
        .arg("copy")
        .arg("--progress")
        .arg("--verbose")
        .arg(source)
        .arg(remote + &target)
        .output()
        .expect("failed");

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("{}", output_str);
    } else {
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

    let output = Command::new("rclone")
        .arg("mount")
        .arg("--daemon")
        .arg(remote)
        .arg(String::from("/home/darkelectron/Cloud/") + mount_point)
        .output()
        .expect("failed");

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("{}", output_str);
    } else {
        let error_str = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", error_str);
    }
}

fn main() {
    // list_files();
    let args: RclonerArgs = RclonerArgs::parse();

    match args.entity_type {
        EntityType::Copy(copy) => copy_files(copy.source, copy.target),
        EntityType::Mount(_) => mount_cloud_service(),
        EntityType::List(_) => list_files(),
        // EntityType::Video(video) => handle_video_command(video),
        // EntityType::View(view) => handle_view_command(view),
    }
}

