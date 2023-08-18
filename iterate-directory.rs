#!/usr/bin/env -S cargo +nightly -Zscript

//! ```cargo
//! 
//! ```

use std::{env, fs};

fn main() -> Result<(),Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    // Read the contents of the directory
    let entries = fs::read_dir(current_dir)?;

    // Iterate over the directory entries and print their names
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }

    Ok(())

}