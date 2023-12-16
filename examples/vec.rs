fn main() {
    let mut vec: Vec<f32> = Vec::with_capacity(10);
    let mut x = 0;
    loop {
        if x == 10 {
            break;
        }
        x += 1;
        vec.push(1.0);
        println!("{:?}", vec);
    }
    vec.push(2.0);
    println!("{:?}", vec);
}
