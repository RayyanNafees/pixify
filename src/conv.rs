use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

fn read_file_buffer(path: &str) -> io::Result<usize> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    
    // Read file into vector.
    return reader.read_to_end(&mut buffer);   
}

pub fn bytes_from_file(path: &str){
    let bytes = read_file_buffer(path);
    match bytes {
        Ok(bins) => println!("{bins:?}"),
        Err(e) => println!("Error finding the file: {e:?}"),
    }
}

pub fn get_args() -> String{
    let Some(value) = std::env::args().next() else {panic!()};
    value
}