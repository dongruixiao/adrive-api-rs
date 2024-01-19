fn main() {
    use std::path::PathBuf;
    let path = PathBuf::from("../../");
    let path = path.canonicalize().unwrap();
    println!("{:?}", path);
}
