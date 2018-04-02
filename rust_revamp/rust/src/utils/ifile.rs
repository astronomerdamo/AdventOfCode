use std::fs::File;
use std::io;
use std::io::Read;

/// Read in file from cli arg path and pass back as raw string.
/// Bubble up errors to calling public function.
/// Note: `File` closes when scope closes.
fn read_file(arg: &str) -> Result<String, io::Error> {
    let mut buffer = String::new();
    File::open(arg)?.read_to_string(&mut buffer)?;
    Ok(buffer)
}

/// Public function to read file from cli arg path.
pub fn read_inputs(arg: &str) -> String {
    match read_file(arg) {
        Ok(buffer) => buffer,
        Err(e) => panic!("ERROR: Input file -> {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn panic_read_inputs() {
        read_inputs(&"this/file/isnt/here");
    }
}
