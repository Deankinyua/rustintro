use std::fs::File;
use std::io::{self, Read};

pub fn _main() -> io::Result<()> {
    let file_path = "example.txt";
    let mut buffer = [0u8; 1024];  // A buffer with a capacity of 1024 bytes

    let mut file = File::open(file_path)?;

    let bytes_read = file.read(&mut buffer)?;

    // Now, 'buffer' contains the data read from the file.

    println!("Read {} bytes from the file.", bytes_read);

    Ok(())
}