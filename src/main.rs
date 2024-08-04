use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::result::Result;

fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1)
    }
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(format!("Usage: {} [one or more filenames]", args[0]).into());
    }

    for filename in args.iter().skip(1) {
        let os_filename: OsString = OsString::from(filename);
        let metadata = fs::metadata(os_filename)?;

        if metadata.is_file() {
            println!("{} ({} B)", filename, metadata.len());
        } else {
            return Err(format!("{} is not a regular file", filename).into());
        }
    }

    Ok(())
}
