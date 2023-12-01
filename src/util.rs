use std::{
    fs::File,
    io::{Read, Result}, path::Path,
};

pub fn read_input<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file_handle = File::open(path)?;
    let mut content = String::new();
    file_handle.read_to_string(&mut content)?;
    Ok(content)
}
