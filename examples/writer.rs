use std::{
    fs,
    io::SeekFrom,
    io::{Seek, Write},
};
fn main() {
    let mut file = fs::File::create("foo.txt").unwrap();
    file.seek(SeekFrom::Start(2)).unwrap();
    file.write_all(b"llo!").unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    file.write_all(b"he").unwrap();
}
