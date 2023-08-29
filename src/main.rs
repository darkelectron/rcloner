use std::fs;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

fn read_file() {
    let sources_filenames = "/home/darkelectron/Falcon/Tools/rcloner/sources_filenames.txt";

    let info = fs::read_to_string(sources_filenames).expect("The file could not be read");
    println!("{}", info);
}

fn write_file() -> Result<(), Error> {
    let sources_filenames = "/home/darkelectron/Falcon/Tools/rcloner/sources_filenames.txt";

    let mut output = File::create(sources_filenames)?; // opens file for writing
    write!(output, "New thing")?;

    /* opens file for reading */
    let input = File::open(sources_filenames)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn main() {
    read_file();
    let _ = write_file();
}
