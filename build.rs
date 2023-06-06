use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use walkdir::WalkDir;

fn main() {
    let mut output_file = File::create("concat.scss").expect("failed to create output file");

    // Walk through src directory and search for SCSS files
    for entry in WalkDir::new("src") {
        let entry = entry.expect("failed to read directory entry");

        if entry.path().extension().map_or(false, |e| e == "scss") {
            let contents = fs::read_to_string(entry.path()).expect("failed to read scss file");

            // Write to the output file
            output_file
                .write_all(contents.as_bytes())
                .expect("failed to write to output file");

            output_file
                .write_all(b"\n")
                .expect("failed to write newline to output file");
        }
    }

    // Execute the sass command
    let output = Command::new("sass")
        .args(&["concat.scss", "public/css/style.css"])
        .output()
        .expect("failed to execute sass");

    fs::remove_file("concat.scss").expect("failed to remove concat.scss");

    // Print sass's stdout and stderr
    println!("{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
}
