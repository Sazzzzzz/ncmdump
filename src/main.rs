mod crypto;
mod ncm;

use anyhow::Result;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Collect arguments, skipping the binary name
    let args: Vec<String> = env::args().skip(1).collect();

    let mut files_to_process = Vec::new();

    if args.is_empty() {
        // Fallback: scan current working directory
        let entries = fs::read_dir(env::current_dir()?)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("ncm") {
                files_to_process.push(path);
            }
        }

        if files_to_process.is_empty() {
            println!("No .ncm files found in the current directory.");
            return Ok(());
        }
    } else {
        for arg in args {
            let path = PathBuf::from(arg);
            if path.is_file() {
                files_to_process.push(path);
            } else {
                eprintln!("Warning: {} is not a valid file.", path.display());
            }
        }
    }

    for path in files_to_process {
        println!("Processing: {}", path.display());
        match ncm::dump(&path) {
            Ok(out_path) => println!("Success! Saved to {}", out_path.display()),
            Err(e) => eprintln!("Failed to decrypt {}: {:?}", path.display(), e),
        }
    }

    Ok(())
}
