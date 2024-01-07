fn main() {
    use std::path::PathBuf;
    let path = PathBuf::from("/Users/dongruixiao/PlayGround/adrive-api-rs/Cargo.lock");
    let result = std::fs::metadata(path).unwrap().is_dir();
    println!("{:?}", result);
}
