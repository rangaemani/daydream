use std::fs::{self, read_dir, remove_file};
use std::io::{self, ErrorKind};
use std::path::Path;

fn main() -> io::Result<()> {
    // Define the path to the 'entries' directory and the 'app.log' file.
    let entries_dir = Path::new("entries");
    let log_file = Path::new("app.log");

    // Empty the 'entries' directory.
    if entries_dir.is_dir() {
        for entry in read_dir(entries_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                fs::remove_file(path)?;
            }
        }
    }

    // Delete the 'app.log' file.
    match remove_file(log_file) {
        Ok(_) => println!("Removed 'app.log'."),
        Err(e) if e.kind() == ErrorKind::NotFound => println!("'app.log' not found."),
        Err(e) => return Err(e),
    }

    println!("'entries' directory has been emptied.");
    Ok(())
}
