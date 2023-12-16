use std::fs::File;
use std::io::{BufReader, Read, SeekFrom};
use std::str;

fn main() {
    let mut file =
        File::open("/Users/dongruixiao/PlayGround/adrive-api-rs/examples/credentials.rs").unwrap();
    let mut buf = [0; 10];
    loop {
        let size = file.read(&mut buf).unwrap();
        if size == 0 {
            break;
        }
        println!("{}", size);
        println!("{:#?}", &buf[..size]);
        println!("{:?}", str::from_utf8(&buf[..size]).unwrap());
    }
}
