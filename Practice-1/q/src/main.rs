use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io::ErrorKind;
fn main() {
    let file_result = File::open("hello.txt");
    let file_result = match file_result {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("Error creating file: {}", e),
            }
            other_error => panic!("Error creating file: {}", other_error),
        },
    };
}
