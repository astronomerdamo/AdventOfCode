use std::fs::File;
use std::io::Read;

/// Check and open file if present.
fn open_file(arg: &str) -> File {
    match File::open(arg) {
        Ok(f) => f,
        Err(e) => panic!("FAILURE : OPEN FILE {}", e),
    }
}

/// Read in file from cli arg path and pass back as raw string.
/// Note: `File` closes when scope closes.
fn read_file(arg: &str) -> String {
    let mut file = open_file(&arg);
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(s) => s,
        Err(e) => panic!("FAILURE : READ FILE {}", e),
    };
    buffer
}

/// Public function to read file from cli arg path.
pub fn read_inputs(arg: &str) -> String {
    read_file(arg)
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